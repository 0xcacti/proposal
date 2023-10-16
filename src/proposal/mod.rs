use figment::{
    value::{Dict, Map},
    Error, Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub rpc_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let rpc_url = "http://localhost:9933".to_string();
        Self { rpc_url }
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Radon Server Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        None
    }
}

impl Config {
    pub fn from<T: Provider>(provider: T) -> Result<Config, error::ProposalError> {
        Figment::from(provider)
            .extract()
            .map_err(error::ProposalError::Config)
    }

    pub fn figment() -> Figment {
        use figment::providers::Env;
        Figment::from(Self::default()).merge(Env::prefixed("PROPOSAL_"))
    }
}
