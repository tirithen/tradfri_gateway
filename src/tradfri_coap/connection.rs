use std::sync::{Arc, Mutex};

use {
    crate::udp_dtls::{ConnectorIdentity, DtlsConnector, DtlsStream, PskIdentity, UdpChannel},
    coap::{
        message::{packet::Packet, response::CoAPResponse},
        CoAPRequest,
    },
    std::{
        io::{self, Read, Write},
        net::{IpAddr, SocketAddr, UdpSocket},
        time::Duration,
    },
};

const TF_PORT: u16 = 5684;

#[derive(Debug, Clone)]
pub struct TradfriConnection {
    stream: Arc<Mutex<DtlsStream<UdpChannel>>>,
}

impl TradfriConnection {
    pub fn new<A: Into<IpAddr>>(addr: A, identity: &[u8], key: &[u8]) -> super::Result<Self> {
        Self::new_with_timeout(addr, identity, key, None)
    }

    pub fn new_with_timeout<A: Into<IpAddr>>(
        addr: A,
        identity: &[u8],
        key: &[u8],
        timeout: Option<u64>,
    ) -> super::Result<Self> {
        let connector = DtlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .use_sni(false)
            .add_cipher("PSK-AES128-CCM8")
            .identity(ConnectorIdentity::Psk(PskIdentity::new(identity, key)))
            .min_protocol_version(Some(crate::udp_dtls::Protocol::Dtlsv12))
            .max_protocol_version(Some(crate::udp_dtls::Protocol::Dtlsv12))
            .build()?;

        let addr = addr.into();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_nonblocking(false).unwrap();
        if let Some(timeout) = timeout {
            socket.set_read_timeout(Some(Duration::from_secs(timeout)))?;
            socket.set_write_timeout(Some(Duration::from_secs(timeout)))?;
        }

        let client_channel = UdpChannel {
            socket,
            remote_addr: SocketAddr::new(addr, TF_PORT),
        };

        Ok(Self {
            stream: Arc::new(Mutex::new(connector.connect("", client_channel)?)),
        })
    }

    pub fn send(&mut self, req: CoAPRequest) -> super::Result<usize> {
        Ok(self.write(&req.message.to_bytes()?)?)
    }

    pub fn receive(&mut self) -> super::Result<CoAPResponse> {
        let mut buf = [0u8; super::BUF_SIZE];
        let len = self.read(&mut buf)?;
        let packet = Packet::from_bytes(&buf[0..len])?;

        Ok(CoAPResponse { message: packet })
    }
}

impl Read for TradfriConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.lock().unwrap().read(buf)
    }
}

impl Write for TradfriConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.lock().unwrap().flush()
    }
}
