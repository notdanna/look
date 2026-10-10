use crate::config::RuntimeConfig;
use std::sync::mpsc;

pub fn discover_installed_apps(
    config: &RuntimeConfig,
    tx: mpsc::SyncSender<look_indexing::Candidate>,
) {
    crate::platform::discover_macos_installed_apps(config, tx);
}
