//! An rusty abstraction over OpenSSL DTLS.

mod certificate;
mod certificate_fingerprint;
mod dtls_connection_builder;
mod dtls_connector;
mod dtls_stream;
mod error;
mod identity;
mod midhandshake_dtls_steam;
mod openssl;
mod protocol;
mod udp_channel;

pub(crate) use self::certificate::Certificate;
pub(crate) use self::certificate_fingerprint::{CertificateFingerprint, SignatureAlgorithm};
pub(crate) use self::dtls_connection_builder::DtlsConnectorBuilder;
pub(crate) use self::dtls_connector::DtlsConnector;
pub(crate) use self::dtls_stream::DtlsStream;
pub(crate) use self::error::{Error, HandshakeError, Result};
pub(crate) use self::identity::{ConnectorIdentity, PskIdentity};
pub(crate) use self::midhandshake_dtls_steam::MidHandshakeDtlsStream;
pub(crate) use self::protocol::Protocol;
pub(crate) use self::udp_channel::UdpChannel;
