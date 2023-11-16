use super::{Certificate, ConnectorIdentity, DtlsConnector, Protocol, Result};

/// A builder for `DtlsConnector`s.
///
/// With this builder you can configure the following DTLS properties:
/// - The identity to be used for client certificate authentication
/// - Adding and enabling the the DTLS extension 'use_srtp'
/// - Configuring min/max supported DTLS versions
/// - Adding a certificate to the set of roots that the connector will trust
/// - Allowing invalid hostnames/certs for the connection
/// - Enabling Server Name Indication (SNI)
pub struct DtlsConnectorBuilder {
    pub(crate) identity: Option<ConnectorIdentity>,
    pub(crate) min_protocol: Option<Protocol>,
    pub(crate) max_protocol: Option<Protocol>,
    pub(crate) root_certificates: Vec<Certificate>,
    pub(crate) accept_invalid_certs: bool,
    pub(crate) accept_invalid_hostnames: bool,
    pub(crate) use_sni: bool,
    pub(crate) cipher_list: Vec<String>,
}

impl DtlsConnectorBuilder {
    /// Sets the identity to be used for client certificate authentication.
    pub fn identity(&mut self, identity: ConnectorIdentity) -> &mut DtlsConnectorBuilder {
        self.identity = Some(identity);
        self
    }

    /// Sets the minimum supported protocol version.
    ///
    /// A value of `None` enables support for the oldest protocols supported by the implementation.
    ///
    /// Defaults to `Some(Protocol::Dtlsv10)`.
    ///
    /// # Underlying SSL
    /// This will be used for setting the ssl options witch corresponds to [`SSL_CTX_set_options`].
    ///
    /// [`SSL_CTX_set_options`]: https://www.openssl.org/docs/manmaster/man3/SSL_CTX_set_options.html
    pub fn min_protocol_version(
        &mut self,
        protocol: Option<Protocol>,
    ) -> &mut DtlsConnectorBuilder {
        self.min_protocol = protocol;
        self
    }

    /// Sets the maximum supported protocol version.
    ///
    /// A value of `None` enables support for the newest protocols supported by the implementation.
    ///
    /// Defaults to `None`.
    ///
    /// # Underlying SSL
    /// This will be used for setting the ssl options witch corresponds to [`SSL_CTX_set_options`].
    ///
    /// [`SSL_CTX_set_options`]: https://www.openssl.org/docs/manmaster/man3/SSL_CTX_set_options.html
    pub fn max_protocol_version(
        &mut self,
        protocol: Option<Protocol>,
    ) -> &mut DtlsConnectorBuilder {
        self.max_protocol = protocol;
        self
    }

    /// Controls the use of certificate validation.
    ///
    /// Defaults to `false`.
    ///
    /// # Warning
    ///
    /// You should think very carefully before using this method. If invalid certificates are trusted, *any*
    /// certificate for *any* site will be trusted for use. This includes expired certificates. This introduces
    /// significant vulnerabilities, and should only be used as a last resort.
    pub fn danger_accept_invalid_certs(
        &mut self,
        accept_invalid_certs: bool,
    ) -> &mut DtlsConnectorBuilder {
        self.accept_invalid_certs = accept_invalid_certs;
        self
    }

    /// Controls the use of Server Name Indication (SNI).
    ///
    /// Defaults to `true`.
    pub fn use_sni(&mut self, use_sni: bool) -> &mut DtlsConnectorBuilder {
        self.use_sni = use_sni;
        self
    }

    /// Controls the use of hostname verification.
    ///
    /// Defaults to `false`.
    ///
    /// # Warning
    ///
    /// You should think very carefully before using this method. If invalid hostnames are trusted, *any* valid
    /// certificate for *any* site will be trusted for use. This introduces significant vulnerabilities, and should
    /// only be used as a last resort.
    pub fn danger_accept_invalid_hostnames(
        &mut self,
        accept_invalid_hostnames: bool,
    ) -> &mut DtlsConnectorBuilder {
        self.accept_invalid_hostnames = accept_invalid_hostnames;
        self
    }

    /// Adds cipher name to the list of allowed ciphers.
    pub fn add_cipher<C: Into<String>>(&mut self, cipher: C) -> &mut DtlsConnectorBuilder {
        self.cipher_list.push(cipher.into());
        self
    }

    /// Creates a new `DtlsConnector` with the settings from this builder.
    pub fn build(&self) -> Result<DtlsConnector> {
        DtlsConnector::new(self)
    }
}
