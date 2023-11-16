mod authenticator;
mod connection;
mod error;
mod macros;
mod result;

pub(crate) const BUF_SIZE: usize = 8192;

pub(crate) use {
    authenticator::TradfriAuthenticator,
    connection::TradfriConnection,
    error::Error,
    result::Result,
};
