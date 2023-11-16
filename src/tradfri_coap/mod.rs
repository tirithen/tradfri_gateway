mod authenticator;
mod connection;
pub(crate) mod device;
mod device_worker;
mod error;
mod macros;
mod result;

pub(crate) const BUF_SIZE: usize = 8192;

pub(crate) use {
    authenticator::TradfriAuthenticator,
    coap, // Re-export coap
    connection::TradfriConnection,
    device::Device,
    error::Error,
    result::Result,
};
