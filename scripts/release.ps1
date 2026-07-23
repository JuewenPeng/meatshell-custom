param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidatePattern('^v\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$')]
    [string] $Tag,

    [switch] $Push,
    [switch] $DryRun
)

$ErrorActionPreference = 'Stop'

function Run-Git {
    param([Parameter(Mandatory = $true)][string[]] $CommandArgs)

    if ($DryRun) {
        Write-Host "git $($CommandArgs -join ' ')"
        return
    }

    & git @CommandArgs
    if ($LASTEXITCODE -ne 0) {
        throw "git $($CommandArgs -join ' ') failed"
    }
}

function Run-Cargo {
    param([Parameter(Mandatory = $true)][string[]] $CommandArgs)

    if ($DryRun) {
        Write-Host "cargo $($CommandArgs -join ' ')"
        return
    }

    & cargo @CommandArgs
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($CommandArgs -join ' ') failed"
    }
}

function Confirm-DeleteTag {
    param([Parameter(Mandatory = $true)][string] $Location)

    if ($DryRun) {
        Write-Host "Would ask to delete existing $Location tag '$Tag'."
        return
    }

    $answer = Read-Host "Tag '$Tag' already exists $Location. Delete it? [y/N]"
    if ($answer -notmatch '^(?i:y|yes)$') {
        throw "Release cancelled; existing $Location tag '$Tag' was not deleted."
    }
}

function Read-TextFile {
    param([Parameter(Mandatory = $true)][string] $Path)

    $reader = New-Object System.IO.StreamReader($Path, $true)
    try {
        return $reader.ReadToEnd()
    }
    finally {
        $reader.Dispose()
    }
}

function Write-Utf8NoBom {
    param(
        [Parameter(Mandatory = $true)][string] $Path,
        [Parameter(Mandatory = $true)][string] $Content
    )

    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $utf8NoBom)
}

$repoRoot = (& git rev-parse --show-toplevel).Trim()
if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($repoRoot)) {
    throw 'This script must be run inside a git repository.'
}

Set-Location $repoRoot

& git diff --quiet --exit-code
if ($LASTEXITCODE -ne 0) {
    throw 'Tracked files have unstaged changes. Commit or stash them before releasing.'
}

& git diff --cached --quiet --exit-code
if ($LASTEXITCODE -ne 0) {
    throw 'Tracked files have staged changes. Commit or stash them before releasing.'
}

$existingLocalTag = & git tag --list $Tag
$replaceExistingTag = $false
if ($existingLocalTag) {
    Confirm-DeleteTag 'locally'
    Run-Git @('tag', '-d', $Tag)
    # Confirming replacement of a local release tag also replaces its remote
    # counterpart and pushes the rebuilt tag at the end of this script.
    $replaceExistingTag = $true
}

# A remote tag is touched for an explicit push, or when the user confirmed
# replacement of an existing local release tag.
if ($Push -or $replaceExistingTag) {
    if ($DryRun) {
        Write-Host "Would check origin for tag '$Tag'."
    }
    else {
        $existingRemoteTag = & git ls-remote --tags --refs origin "refs/tags/$Tag"
        if ($LASTEXITCODE -ne 0) {
            throw "Could not check whether tag '$Tag' exists on origin."
        }
        if ($existingRemoteTag) {
            if (-not $replaceExistingTag) {
                Confirm-DeleteTag 'on origin'
            }
            Run-Git @('push', 'origin', ":refs/tags/$Tag")
        }
    }
}

$version = $Tag.Substring(1)
$escapedVersion = [regex]::Escape($version)
$cargoTomlPath = Join-Path $repoRoot 'Cargo.toml'
$cargoLockPath = Join-Path $repoRoot 'Cargo.lock'

$cargoToml = Read-TextFile $cargoTomlPath
$newCargoToml = [regex]::Replace(
    $cargoToml,
    '(?ms)^(\[package\]\s+.*?^version\s*=\s*")[^"]+("\s*)$',
    "`${1}$version`${2}",
    1
)

$cargoTomlVersionPattern = '(?m)^version\s*=\s*"{0}"\s*$' -f $escapedVersion
if ($newCargoToml -eq $cargoToml -and $cargoToml -notmatch $cargoTomlVersionPattern) {
    throw 'Could not update [package].version in Cargo.toml.'
}

$cargoLock = Read-TextFile $cargoLockPath
$newCargoLock = [regex]::Replace(
    $cargoLock,
    '(?ms)^(name\s*=\s*"meatshell"\s*)(\r?\n)(version\s*=\s*")[^"]+("\s*)$',
    "`${1}`${2}`${3}$version`${4}",
    1
)

$cargoLockVersionPattern = '(?ms)^name\s*=\s*"meatshell"\s*\r?\nversion\s*=\s*"{0}"\s*$' -f $escapedVersion
if ($newCargoLock -eq $cargoLock -and $cargoLock -notmatch $cargoLockVersionPattern) {
    throw 'Could not update meatshell version in Cargo.lock.'
}

$androidCargoToml = Get-Content -LiteralPath $androidCargoTomlPath -Raw
$newAndroidCargoToml = [regex]::Replace(
    $androidCargoToml,
    '(?ms)^(\[package\]\s+.*?^version\s*=\s*")[^"]+(")',
    "`${1}$version`${2}",
    1
)
if ($newAndroidCargoToml -eq $androidCargoToml) {
    throw "Could not update [package].version in android/Cargo.toml."
}

$androidCargoLock = Get-Content -LiteralPath $androidCargoLockPath -Raw
$newAndroidCargoLock = [regex]::Replace(
    $androidCargoLock,
    '(?ms)^(name\s*=\s*"meatshell-android"\s*)(\r?\n)(version\s*=\s*")[^"]+(")',
    "`${1}`${2}`${3}$version`${4}",
    1
)
if ($newAndroidCargoLock -eq $androidCargoLock) {
    throw "Could not update meatshell-android version in android/Cargo.lock."
}

if ($DryRun) {
    Write-Host "Would set Cargo.toml and Cargo.lock version to $version."
}
else {
    Write-Utf8NoBom $cargoTomlPath $newCargoToml
    Write-Utf8NoBom $cargoLockPath $newCargoLock
}

Run-Cargo @('check', '--locked')

if ($DryRun) {
    Write-Host 'cargo run --locked -- --version'
    Write-Host "Would verify output equals: meatshell $version"
}
else {
    Write-Host 'Verifying executable version...'

    $versionOutput = @(& cargo run --locked -- --version)
    if ($LASTEXITCODE -ne 0) {
        throw 'cargo run --locked -- --version failed'
    }

    $actualVersion = ($versionOutput | Select-Object -Last 1).ToString().Trim()
    $expectedVersion = "meatshell $version"

    Write-Host "Expected: $expectedVersion"
    Write-Host "Actual:   $actualVersion"

    if ($actualVersion -ne $expectedVersion) {
        throw "Binary version mismatch. Expected '$expectedVersion', got '$actualVersion'."
    }
}

Run-Git @('add', 'Cargo.toml', 'Cargo.lock')

# Re-running a release for an already-versioned clean HEAD should recreate the
# tag without failing on an empty commit.
if ($DryRun) {
    Write-Host "Would commit release changes if Cargo.toml or Cargo.lock changed."
}
else {
    & git diff --cached --quiet --exit-code
    if ($LASTEXITCODE -eq 1) {
        Run-Git @('commit', '-m', "Release $Tag")
    }
    elseif ($LASTEXITCODE -eq 0) {
        Write-Host "Version is already $version; creating tag without an empty release commit."
    }
    else {
        throw 'Could not determine whether the release files changed.'
    }
}
Run-Git @('tag', '-a', $Tag, '-m', "Release $Tag")

if ($Push) {
    Run-Git @('push', 'origin', 'HEAD')
    Run-Git @('push', 'origin', $Tag)
    Write-Host "Released $Tag and pushed branch + tag."
}
elseif ($replaceExistingTag) {
    Run-Git @('push', 'origin', $Tag)
    Write-Host "Replaced and pushed tag $Tag."
}
else {
    Write-Host "Created release commit and tag $Tag."
    Write-Host "Push with: git push origin HEAD && git push origin $Tag"
}
