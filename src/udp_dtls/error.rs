use super::MidHandshakeDtlsStream;
use openssl::{error::ErrorStack, ssl, x509::X509VerifyResult};
use std::{error, fmt, result};

/// A typedef of the result-type returned by many methods.
pub type Result<T> = result::Result<T, Error>;

/// An error returned from the DTLS implementation.
#[derive(Debug)]
pub enum Error {
    /// Collection of [`Error`]s from OpenSSL.
    ///
    /// [`Error`]: struct.Error.html
    Normal(ErrorStack),
    /// An SLL error with the result of peer certificate verification.
    Ssl(ssl::Error, X509VerifyResult),
    /// Bad SRTP profile
    SrtpProfile(SrtpProfileError),
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Normal(ref e) => error::Error::source(e),
            Error::Ssl(ref e, _) => error::Error::source(e),
            Error::SrtpProfile(ref e) => error::Error::source(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Normal(ref e) => fmt::Display::fmt(e, fmt),
            Error::Ssl(ref e, X509VerifyResult::OK) => fmt::Display::fmt(e, fmt),
            Error::Ssl(ref e, v) => write!(fmt, "{} ({})", e, v),
            Error::SrtpProfile(ref e) => fmt::Display::fmt(e, fmt),
        }
    }
}

impl From<ErrorStack> for Error {
    fn from(err: ErrorStack) -> Error {
        Error::Normal(err)
    }
}

impl From<SrtpProfileError> for Error {
    fn from(err: SrtpProfileError) -> Error {
        Error::SrtpProfile(err)
    }
}

/// An error that can occur during the handshake-process.
#[derive(Debug)]
pub enum HandshakeError<S: fmt::Debug> {
    /// An error occurred during the handshake process, see inner error for more information.
    Failure(Error),

    /// A stream interrupted midway through the handshake process due to a
    /// `WouldBlock` error.
    ///
    /// Note that this is not a fatal error and it should be safe to call
    /// `handshake` at a later time once the stream is ready to perform I/O
    /// again.
    WouldBlock(MidHandshakeDtlsStream<S>),
}

impl<S: fmt::Debug> From<ssl::HandshakeError<S>> for HandshakeError<S> {
    fn from(e: ssl::HandshakeError<S>) -> HandshakeError<S> {
        match e {
            ssl::HandshakeError::SetupFailure(e) => HandshakeError::Failure(e.into()),
            ssl::HandshakeError::Failure(e) => {
                let v = e.ssl().verify_result();
                HandshakeError::Failure(Error::Ssl(e.into_error(), v))
            }
            ssl::HandshakeError::WouldBlock(s) => {
                HandshakeError::WouldBlock(MidHandshakeDtlsStream::from(s))
            }
        }
    }
}

impl<S: fmt::Debug> From<ErrorStack> for HandshakeError<S> {
    fn from(e: ErrorStack) -> HandshakeError<S> {
        HandshakeError::Failure(e.into())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SrtpProfileError {
    BadProfile,
}

impl error::Error for SrtpProfileError {
    fn description(&self) -> &str {
        match *self {
            SrtpProfileError::BadProfile => "bad SRTP profile",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for SrtpProfileError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SrtpProfileError::BadProfile => fmt.write_str("bad SRTP profile"),
        }
    }
}
