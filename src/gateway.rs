use std::{
    cell::RefCell,
    net::IpAddr,
    rc::Rc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use coap::{CoAPRequest, CoAPResponse};
use mdns_sd::{ServiceDaemon, ServiceEvent};

use crate::{
    device::Device,
    tradfri_coap::{TradfriAuthenticator, TradfriConnection},
    DeviceUpdate, Group, GroupUpdate
};

#[derive(Debug, Clone)]
pub struct TradfriGateway {
    address: IpAddr,
    identifier: String,
    session_key: String,
}

impl TradfriGateway {
    pub fn from_gateway_code(gateway_code: &str) -> Result<Self, TradfriGatewayError> {
        Self::from_gateway_code_and_addr(Self::discover_ip()?, gateway_code)
    }

    pub fn from_gateway_code_and_addr<A: Into<IpAddr> + Clone>(
        address: A,
        gateway_code: &str,
    ) -> Result<Self, TradfriGatewayError> {
        let identifier = Self::generate_identifier();
        let session_key =
            TradfriAuthenticator::authenticate(address.clone(), &identifier, gateway_code, 10)?;

        Ok(TradfriGateway::from_identifier_and_session_key_and_addr(
            address,
            &identifier,
            &session_key,
        ))
    }

    pub fn from_identifier_and_session_key(
        identifier: &str,
        session_key: &str,
    ) -> Result<Self, TradfriGatewayError> {
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
    ) -> Self {
        Self {
            address: address.into(),
            identifier: identifier.into(),
            session_key: session_key.into(),
        }
    }

    pub fn devices(&mut self) -> Result<DeviceIterator, TradfriGatewayError> {
        let connection = Rc::new(RefCell::new(self.create_connection()?));
        let ids = {
            let mut connection_borrowed = connection.borrow_mut();
            self.device_ids(&mut connection_borrowed)?
        };
        Ok(DeviceIterator {
            ids,
            connection,
            gateway: Rc::new(RefCell::new(self.clone())),
        })
    }

    pub fn device(&mut self, id: u32) -> Result<Device, TradfriGatewayError> {
        let mut connection = self.create_connection()?;
        self.device_with_connection(id, &mut connection)
    }

    pub fn device_with_connection(
        &mut self,
        id: u32,
        connection: &mut TradfriConnection,
    ) -> Result<Device, TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15001/{}", id));
        req.set_method(coap::Method::Get);

        let response = self.coap_request(req, Some(connection))?;
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

    fn device_ids(
        &mut self,
        connection: &mut TradfriConnection,
    ) -> Result<Vec<u32>, TradfriGatewayError> {
        let mut req = CoAPRequest::new();
        req.set_path("15001");
        req.set_method(coap::Method::Get);

        let response = self.coap_request(req, Some(connection))?;
        let device_ids: Vec<u32> = serde_json::from_slice(&response.message.payload)?;

        Ok(device_ids)
    }

    pub fn groups(&mut self) -> Result<GroupIterator, TradfriGatewayError> {
        let connection = Rc::new(RefCell::new(self.create_connection()?));
        let ids = {
            let mut connection_borrowed = connection.borrow_mut();
            self.group_ids(&mut connection_borrowed)?
        };
        Ok(GroupIterator {
            ids,
            connection,
            gateway: Rc::new(RefCell::new(self.clone())),
        })
    }

    pub fn group(&mut self, id: u32) -> Result<Group, TradfriGatewayError> {
        let mut connection = self.create_connection()?;
        self.group_with_connection(id, &mut connection)
    }

    pub fn group_with_connection(
        &mut self,
        id: u32,
        connection: &mut TradfriConnection,
    ) -> Result<Group, TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15004/{}", id));
        req.set_method(coap::Method::Get);

        let response = self.coap_request(req, Some(connection))?;
        let group = match Group::new(self.clone(), &response.message.payload) {
            Ok(d) => d,
            Err(e) => {
                return Err(TradfriGatewayError::DeviceError(
                    id.to_string(),
                    e.to_string(),
                ))
            }
        };

        Ok(group)
    }

    fn group_ids(
        &mut self,
        connection: &mut TradfriConnection,
    ) -> Result<Vec<u32>, TradfriGatewayError> {
        let mut req = CoAPRequest::new();
        req.set_path("15004");
        req.set_method(coap::Method::Get);

        let response = self.coap_request(req, Some(connection))?;
        let device_ids: Vec<u32> = serde_json::from_slice(&response.message.payload)?;

        Ok(device_ids)
    }

    pub(crate) fn create_connection(&self) -> Result<TradfriConnection, TradfriGatewayError> {
        Ok(TradfriConnection::new(
            self.address,
            self.identifier.as_bytes(),
            self.session_key.as_bytes(),
        )?)
    }

    pub(crate) fn update_device(
        &mut self,
        id: u32,
        update: &DeviceUpdate,
        connection: Option<&mut TradfriConnection>,
    ) -> Result<(), TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15001/{}", id));
        req.set_method(coap::Method::Put);
        req.message.payload = serde_json::to_vec(&update)?;

        self.coap_request(req, connection)?;

        Ok(())
    }

    pub(crate) fn update_group(
        &mut self,
        id: u32,
        update: &GroupUpdate,
        connection: Option<&mut TradfriConnection>,
    ) -> Result<(), TradfriGatewayError> {
        let mut req = coap::CoAPRequest::new();
        req.set_path(&format!("15004/{}", id));
        req.set_method(coap::Method::Put);
        req.message.payload = serde_json::to_vec(&update)?;

        self.coap_request(req, connection)?;

        Ok(())
    }

    fn coap_request(
        &self,
        req: CoAPRequest,
        connection: Option<&mut TradfriConnection>,
    ) -> Result<CoAPResponse, TradfriGatewayError> {
        if let Some(connection) = connection {
            connection.send(req)?;
            Ok(connection.receive()?)
        } else {
            let mut connection = self.create_connection()?;
            connection.send(req)?;
            Ok(connection.receive()?)
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

pub struct DeviceIterator {
    ids: Vec<u32>,
    connection: Rc<RefCell<TradfriConnection>>,
    gateway: Rc<RefCell<TradfriGateway>>,
}

impl Iterator for DeviceIterator {
    type Item = Result<Device, TradfriGatewayError>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.pop()?;
        let mut connection_borrowed = self.connection.borrow_mut();
        let mut gateway_borrowed = self.gateway.borrow_mut();
        Some(gateway_borrowed.device_with_connection(id, &mut connection_borrowed))
    }
}

pub struct GroupIterator {
    ids: Vec<u32>,
    connection: Rc<RefCell<TradfriConnection>>,
    gateway: Rc<RefCell<TradfriGateway>>,
}

impl Iterator for GroupIterator {
    type Item = Result<Group, TradfriGatewayError>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.pop()?;
        let mut connection_borrowed = self.connection.borrow_mut();
        let mut gateway_borrowed = self.gateway.borrow_mut();
        Some(gateway_borrowed.group_with_connection(id, &mut connection_borrowed))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TradfriGatewayError {
    #[error("Error getting device with id: {0}, error: {1}")]
    DeviceError(String, String),

    #[error("Error getting group with id: {0}, error: {1}")]
    GroupError(String, String),

    #[error("COAP error: {0}")]
    CoapError(#[from] crate::tradfri_coap::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Mdns error: {0}")]
    MdnsError(#[from] mdns_sd::Error),

    #[error("Gateway not found, mDNS discovery timeout")]
    DiscoveryTimeout,
}
