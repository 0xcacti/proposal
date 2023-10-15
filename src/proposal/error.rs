use thiserror::Error;

#[derive(Error, Debug)]

pub enum ProposalError {
    #[error("Invalid configuration")]
    Config(#[source] figment::Error),
}
