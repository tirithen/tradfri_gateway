use crate::impl_from;

#[derive(Debug)]
pub struct Error {
    cause: String,
}

impl Error {
    pub fn new<C: Into<String>>(cause: C) -> Self {
        Self {
            cause: cause.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.cause
    }
}

impl_from!(crate::udp_dtls::Error);
impl_from!(coap::message::packet::PackageError);
impl_from!(coap::message::packet::ParseError);
impl_from!(std::io::Error);
impl_from!(serde_json::Error);

impl From<crate::udp_dtls::HandshakeError<crate::udp_dtls::UdpChannel>> for Error {
    fn from(err: crate::udp_dtls::HandshakeError<crate::udp_dtls::UdpChannel>) -> Self {
        Self {
            cause: format!("{:?}", err),
        }
    }
}
