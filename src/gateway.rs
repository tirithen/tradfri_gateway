use std::{
    net::IpAddr,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use coap::CoAPRequest;
use mdns_sd::{ServiceDaemon, ServiceEvent};

use crate::{
    device::Device,
    tradfri_coap::{TradfriAuthenticator, TradfriConnection},
    Update,
};

pub type TradfriGatewayConnector = TradfriGateway<TradfriGatewayStateWithGatewayCode>;

#[derive(Debug, Clone)]
pub struct TradfriGateway<S: TradfriGatewayState> {
    address: IpAddr,
    identifier: String,
    state: S,
}

impl<S: TradfriGatewayState> TradfriGateway<S> {
    pub fn from_gateway_code(
        gateway_code: &str,
    ) -> Result<TradfriGateway<TradfriGatewayStateWithGatewayCode>, TradfriGatewayError> {
        Ok(Self::from_gateway_code_and_addr(
            Self::discover_ip()?,
            gateway_code,
        ))
    }

    pub fn from_gateway_code_and_addr<A: Into<IpAddr>>(
        address: A,
        gateway_code: &str,
    ) -> TradfriGateway<TradfriGatewayStateWithGatewayCode> {
        TradfriGateway {
            address: address.into(),
            identifier: Self::generate_identifier(),
            state: TradfriGatewayStateWithGatewayCode {
                gateway_code: gateway_code.into(),
            },
        }
    }

    pub fn from_identifier_and_session_key(
        identifier: &str,
        session_key: &str,
    ) -> Result<TradfriGateway<TradfriGatewayStateWithCredentials>, TradfriGatewayError> {
        Ok(Self::from_identifier_and_session_key_and_addr(
            Self::discover_ip()?,
            identifier,
            session_key,
        ))
    }

    pub fn from_identifier_and_session_key_and_addr<A: Into<IpAddr>>(
        address: A,
        identifier: &str,
        session_key: &str,
    ) -> TradfriGateway<TradfriGatewayStateWithCredentials> {
        TradfriGateway {
            address: address.into(),
            identifier: identifier.into(),
            state: TradfriGatewayStateWithCredentials {
                session_key: session_key.into(),
            },
        }
    }

    fn generate_identifier() -> String {
        format!(
            "user{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(123))
                .as_secs()
        )
    }

    pub fn discover_ip() -> Result<IpAddr, TradfriGatewayError> {
        let mdns = ServiceDaemon::new()?;
        let receiver = mdns.browse("_coap._udp.local.")?;

        let start_time = Instant::now();
        while let Ok(event) = receiver.recv() {
            if let ServiceEvent::ServiceResolved(info) = event {
                if info.get_hostname().starts_with("TRADFRI-Gateway-") {
                    return Ok(if let Some(ip) = info.get_addresses_v4().iter().next() {
                        std::net::IpAddr::V4(**ip)
                    } else {
                        continue;
                    });
                }
            }

            if Instant::now() - start_time > Duration::from_secs(15) {
                break;
            }
        }

        Err(TradfriGatewayError::DiscoveryTimeout)
    }
}

impl TradfriGateway<TradfriGatewayStateWithGatewayCode> {
    pub fn connect(
        &mut self,
    ) -> Result<TradfriGateway<TradfriGatewayStateConnected>, TradfriGatewayError> {
        let session_key = TradfriAuthenticator::authenticate(
            self.address,
            &self.identifier,
            &self.state.gateway_code,
            10,
        )?;

        let with_credentials =
            TradfriGateway::<TradfriGatewayStateWithCredentials>::from_identifier_and_session_key_and_addr(
                self.address,
                &self.identifier,
                &session_key,
            );

        with_credentials.connect()
    }
}

impl TradfriGateway<TradfriGatewayStateWithCredentials> {
    pub fn connect(
        &self,
    ) -> Result<TradfriGateway<TradfriGatewayStateConnected>, TradfriGatewayError> {
        let connection = TradfriConnection::new(
            self.address,
            self.identifier.as_bytes(),
            self.state.session_key.as_bytes(),
        )?;

        Ok(TradfriGateway {
            address: self.address,
            identifier: self.identifier.clone(),
            state: TradfriGatewayStateConnected {
                session_key: self.state.session_key.clone(),
                connection,
            },
        })
    }
}

impl TradfriGateway<TradfriGatewayStateConnected> {
    pub fn device_ids(&mut self) -> Result<Vec<u32>, TradfriGatewayError> {
        let mut req = CoAPRequest::new();
        req.set_path("15001");
        req.set_method(coap::Method::Get);

        self.state.connection.send(req)?;
        let response = self.state.connection.receive()?;
        let device_ids: Vec<u32> = serde_json::from_slice(&response.message.payload)?;

        Ok(device_ids)
    }

    pub fn device(
        &mut self,
        id: u32,
    ) -> Result<Device<TradfriGatewayStateConnected>, TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15001/{}", id));
        req.set_method(coap::Method::Get);

        self.state.connection.send(req)?;
        let response = self.state.connection.receive()?;

        let device = match Device::new(self.clone(), &response.message.payload) {
            Ok(d) => d,
            Err(e) => {
                return Err(TradfriGatewayError::DeviceError(
                    id.to_string(),
                    e.to_string(),
                ))
            }
        };

        Ok(device)
    }

    pub fn update_device(&mut self, id: u32, update: Update) -> Result<(), TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15001/{}", id));
        req.set_method(coap::Method::Put);

        req.message.payload = serde_json::to_vec(&update)?;

        self.state.connection.send(req)?;
        self.state.connection.receive()?;

        Ok(())
    }
}

pub trait TradfriGatewayState {}

#[derive(Debug, Clone)]
pub struct TradfriGatewayStateWithGatewayCode {
    gateway_code: String,
}

impl TradfriGatewayState for TradfriGatewayStateWithGatewayCode {}

#[derive(Debug, Clone)]
pub struct TradfriGatewayStateWithCredentials {
    session_key: String,
}

impl TradfriGatewayState for TradfriGatewayStateWithCredentials {}

#[derive(Debug, Clone)]
pub struct TradfriGatewayStateConnected {
    session_key: String,
    connection: TradfriConnection,
}

impl TradfriGatewayState for TradfriGatewayStateConnected {}

#[derive(Debug, thiserror::Error)]
pub enum TradfriGatewayError {
    #[error("Error getting device with id: {0}, error: {1}")]
    DeviceError(String, String),

    #[error("COAP error: {0}")]
    CoapError(#[from] crate::tradfri_coap::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Mdns error: {0}")]
    MdnsError(#[from] mdns_sd::Error),

    #[error("Gateway not found, mDNS discovery timeout")]
    DiscoveryTimeout,
}
