use openssl::{
    error::ErrorStack,
    ssl::{SslContextBuilder, SslOptions},
};
use std::sync::Once;

use super::Protocol;

/// Sets protocol version requirements for the given `SslContextBuilder`
///
/// - Clears the options used by the context
/// - Enables the min/max protocol options
pub fn try_set_supported_protocols(
    min: Option<Protocol>,
    max: Option<Protocol>,
    ctx: &mut SslContextBuilder,
) -> Result<(), ErrorStack> {
    let no_ssl_mask = SslOptions::NO_SSL_MASK;
    let allow_unsafe_legacy_renegotiation = SslOptions::ALLOW_UNSAFE_LEGACY_RENEGOTIATION;

    ctx.clear_options(no_ssl_mask);
    let mut options = SslOptions::empty();
    options |= match min {
        None | Some(Protocol::Dtlsv10) => SslOptions::empty(),
        Some(Protocol::Dtlsv12) => SslOptions::NO_DTLSV1,
    };
    options |= match max {
        None | Some(Protocol::Dtlsv12) => SslOptions::empty(),
        Some(Protocol::Dtlsv10) => SslOptions::NO_DTLSV1_2,
    };

    ctx.set_options(options | allow_unsafe_legacy_renegotiation);

    Ok(())
}

pub fn init_trust() {
    static ONCE: Once = Once::new();
    ONCE.call_once(openssl_probe::init_ssl_cert_env_vars);
}
