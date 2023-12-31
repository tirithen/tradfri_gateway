use super::{
    openssl::{init_trust, try_set_supported_protocols},
    ConnectorIdentity, DtlsConnectorBuilder, DtlsStream, Error, HandshakeError, Protocol,
};
use log::debug;
use openssl::error::ErrorStack;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::{fmt, io, io::Write};

/// Connector to an UDP endpoint secured with DTLS.
#[derive(Clone)]
pub struct DtlsConnector {
    connector: SslConnector,
    use_sni: bool,
    accept_invalid_hostnames: bool,
    accept_invalid_certs: bool,
}

impl DtlsConnector {
    /// Creates a new `DtlsConnector`.
    ///
    /// The `DtlsConnector` will use the settings from the given builder.
    ///
    /// The following propperties will be applied from the builder:
    /// - Sets minimal/maximal protocol version
    /// - Sets srtp profile by enabling the DTLS extension 'use_srtp'
    /// - Sets the certificate and private key
    /// - Adds the root certificates to the certificate store.
    pub fn new(builder: &DtlsConnectorBuilder) -> Result<DtlsConnector, Error> {
        init_trust();

        let mut connector = SslConnector::builder(SslMethod::dtls()).unwrap();

        if let Some(ref identity) = builder.identity {
            match identity {
                ConnectorIdentity::Psk(identity_) => {
                    let identity_ = identity_.clone();

                    connector.set_psk_client_callback(move |_, _, mut identity, mut psk| {
                        if let Err(err) = identity.write_all(&identity_.0) {
                            debug!("psk_client_callback error (identity): {:?}", err);
                            return Err(ErrorStack::get());
                        }

                        if let Err(err) = psk.write_all(&identity_.1) {
                            debug!("psk_client_callback error (psk): {:?}", err);
                            return Err(ErrorStack::get());
                        }

                        Ok(identity_.1.len())
                    });
                }
            }
        }

        if !builder.cipher_list.is_empty() {
            connector.set_cipher_list(&builder.cipher_list.join(":"))?;
        }

        try_set_supported_protocols(builder.min_protocol, builder.max_protocol, &mut connector)?;

        for cert in &builder.root_certificates {
            if let Err(err) = connector.cert_store_mut().add_cert((cert.as_ref()).clone()) {
                debug!("add_cert error: {:?}", err);
            }
        }

        Ok(DtlsConnector {
            connector: connector.build(),
            use_sni: builder.use_sni,
            accept_invalid_hostnames: builder.accept_invalid_hostnames,
            accept_invalid_certs: builder.accept_invalid_certs,
        })
    }

    /// Returns a new builder for a `DtlsConnector` from which you can create the `DtlsConnector`.
    pub fn builder() -> DtlsConnectorBuilder {
        DtlsConnectorBuilder {
            identity: None,
            min_protocol: Some(Protocol::Dtlsv10),
            max_protocol: None,
            root_certificates: vec![],
            use_sni: true,
            accept_invalid_certs: false,
            accept_invalid_hostnames: false,
            cipher_list: vec![],
        }
    }

    /// Initiates a DTLS handshake.
    ///
    /// The provided domain will be used for both SNI and certificate hostname
    /// validation.
    ///
    /// If the socket is nonblocking and a `WouldBlock` error is returned during
    /// the handshake, a `HandshakeError::WouldBlock` error will be returned
    /// which can be used to restart the handshake when the socket is ready
    /// again.
    ///
    /// The domain is ignored if both SNI and hostname verification are
    /// disabled.
    pub fn connect<S: fmt::Debug>(
        &self,
        domain: &str,
        stream: S,
    ) -> Result<DtlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        let mut ssl = self
            .connector
            .configure()?
            .use_server_name_indication(self.use_sni)
            .verify_hostname(!self.accept_invalid_hostnames);
        if self.accept_invalid_certs {
            ssl.set_verify(SslVerifyMode::NONE);
        }

        let stream = ssl.connect(domain, stream)?;
        Ok(DtlsStream::from(stream))
    }
}

impl AsRef<SslConnector> for DtlsConnector {
    fn as_ref(&self) -> &SslConnector {
        &self.connector
    }
}
