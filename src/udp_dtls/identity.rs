use bytes::Bytes;

/// Identity/key for client PSK authentication.
///
/// Defaults to None
///
/// # Hint
/// You should specify one of the PSK_* ciphers, i.e. PSK-AES128-CCM8
#[derive(Clone)]
pub struct PskIdentity(pub(crate) Bytes, pub(crate) Bytes);

impl PskIdentity {
    pub fn new(identity: &[u8], key: &[u8]) -> PskIdentity {
        PskIdentity(Bytes::from(identity), Bytes::from(key))
    }
}

/// Possible identities for DTLS connector (client)
pub enum ConnectorIdentity {
    Psk(PskIdentity),
}
