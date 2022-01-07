use super::error::OpenError;
use ini::Ini;
use std::fmt;

#[derive(Clone)]
pub struct RepoConfig(Ini);

impl AsRef<Ini> for RepoConfig {
    fn as_ref(&self) -> &Ini {
        &self.0
    }
}

impl fmt::Debug for RepoConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepoConfig").finish()
    }
}

impl TryFrom<Ini> for RepoConfig {
    type Error = OpenError;

    fn try_from(cfg: Ini) -> Result<Self, Self::Error> {
        if let Some(fmt_version) = cfg
            .section(Some("core"))
            .ok_or(OpenError::InvalidConfig(String::from(
                "missing core section",
            )))?
            .get("repositoryformatversion")
        {
            if fmt_version != "0" {
                return Err(OpenError::UnsupportedFormatVersion(Some(String::from(
                    fmt_version,
                ))));
            }
            Ok(RepoConfig(cfg))
        } else {
            return Err(OpenError::UnsupportedFormatVersion(None));
        }
    }
}

impl RepoConfig {
    pub fn default() -> Self {
        let mut cfg = Ini::new();
        cfg.with_section(Some("core"))
            .set("repositoryformatversion", "0")
            .set("filemode", "false")
            .set("bare", "false");
        RepoConfig(cfg)
    }
}
