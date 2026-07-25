use crate::ssh::{CredentialResponder, HostKeyResponder, MfaResponder};

pub(crate) struct PendingHostKey {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) changed: bool,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) detail: String,
    pub(crate) confirm_label: String,
    pub(crate) responders: Vec<HostKeyResponder>,
}

pub(crate) struct PendingCred {
    pub(crate) session_id: String,
    pub(crate) host: String,
    pub(crate) user: String,
    pub(crate) need_user: bool,
    pub(crate) need_password: bool,
    pub(crate) responders: Vec<CredentialResponder>,
}

pub(crate) struct PendingMfa {
    pub(crate) session_id: String,
    pub(crate) host: String,
    pub(crate) prompt: String,
    pub(crate) echo: bool,
    pub(crate) responders: Vec<MfaResponder>,
}
