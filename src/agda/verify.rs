use std::io;
use std::io::ErrorKind;

use crate::agda::ReplState;
use crate::cmd::Cmd;
use crate::resp::DisplayInfo;

pub fn check_version(version: &str) -> io::Result<()> {
    // I don't expect earlier versions to have interaction-json :)
    if version.starts_with("2.4") || version.starts_with("2.5") || version.starts_with("2.6.0") {
        let msg = format!("Expected Agda 2.6.1 or higher, got: {}", version);
        Err(io::Error::new(ErrorKind::InvalidData, msg))
    } else {
        Ok(())
    }
}

impl ReplState {
    pub async fn validate_version_panicking(&mut self) {
        match self.validate_version().await {
            Ok(()) => {}
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    /// Validate this Agda repl.
    pub async fn validate_version(&mut self) -> io::Result<()> {
        self.command(Cmd::ShowVersion).await?;
        let version = loop {
            match self.next_display_info().await? {
                DisplayInfo::Version { version } => break version,
                _ => {}
            }
        };
        // other checks?
        check_version(&version)
    }
}
