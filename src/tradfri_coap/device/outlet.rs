use {
    super::super::device_worker::DeviceWorker,
    coap::{message::request::Method, CoAPRequest},
    serde::Deserialize,
};

const BODY_ON: &'static str = "{ \"3311\": [{ \"5850\": 1 }] }";
const BODY_OFF: &'static str = "{ \"3311\": [{ \"5850\": 0 }] }";

#[derive(Debug, Deserialize)]
struct InternalOutlet {
    #[serde(rename = "9001")]
    name: String,
    #[serde(rename = "9003")]
    id: u32,
    #[serde(rename = "3")]
    device_info: super::DeviceInfo,
    #[serde(rename = "3312")]
    outlets: Vec<OutletState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OutletState {
    #[serde(rename = "5850")]
    pub state: u8,
    //NOTE currently there are no dimmable outlets
    #[serde(rename = "5851")]
    pub dimmer: u8,
}

#[derive(Debug, Clone)]
pub struct Outlet {
    _worker: DeviceWorker,
    pub name: String,
    pub id: u32,
    pub outlets: Vec<OutletState>,
    pub mfr: String,
    pub device_name: String,
    pub version: String,
}

impl Outlet {
    pub fn new(worker: DeviceWorker, bytes: &[u8]) -> super::super::Result<Self> {
        let internal: InternalOutlet = serde_json::from_slice(bytes)?;

        Ok(Self {
            _worker: worker,
            name: internal.name,
            id: internal.id,
            outlets: internal.outlets,
            mfr: internal.device_info.mfr,
            device_name: internal.device_info.device_name,
            version: internal.device_info.version,
        })
    }

    pub fn on(&self) -> super::super::Result<()> {
        let mut req = CoAPRequest::new();
        req.set_path(&format!("15001/{}", self.id));
        req.set_method(Method::Put);
        req.message.set_payload(BODY_ON.as_bytes().to_vec());
        self._worker.send(req)?;

        Ok(())
    }

    pub fn off(&self) -> super::super::Result<()> {
        let mut req = CoAPRequest::new();
        req.set_path(&format!("15001/{}", self.id));
        req.set_method(Method::Put);
        req.message.set_payload(BODY_OFF.as_bytes().to_vec());
        self._worker.send(req)?;

        Ok(())
    }
}
