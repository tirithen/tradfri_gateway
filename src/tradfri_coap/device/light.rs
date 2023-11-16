use {
    super::super::{device::colournames::ColourName, device_worker::DeviceWorker},
    coap::{message::request::Method, CoAPRequest},
    serde::Deserialize,
};

const BODY_ON: &'static str = "{ \"3311\": [{ \"5850\": 1 }] }";
const BODY_OFF: &'static str = "{ \"3311\": [{ \"5850\": 0 }] }";

#[derive(Debug, Deserialize)]
struct InternalLight {
    #[serde(rename = "9001")]
    name: String,
    #[serde(rename = "9003")]
    id: u32,
    #[serde(rename = "3")]
    device_info: super::DeviceInfo,
    #[serde(rename = "3311")]
    lights: Vec<LightState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LightState {
    #[serde(rename = "5850")]
    pub state: u8,
    #[serde(rename = "5851")]
    pub dimmer: u8,
    #[serde(rename = "5706", default)]
    pub colour_name: ColourName,
    #[serde(rename = "5709", default)]
    pub colour_x: u32,
    #[serde(rename = "5710", default)]
    pub colour_y: u32,
}

#[derive(Debug, Clone)]
pub struct Light {
    _worker: DeviceWorker,
    pub name: String,
    pub id: u32,
    pub lights: Vec<LightState>,
    pub mfr: String,
    pub device_name: String,
    pub version: String,
}

impl Light {
    pub fn new(worker: DeviceWorker, bytes: &[u8]) -> super::super::Result<Self> {
        let internal: InternalLight = serde_json::from_slice(bytes)?;

        Ok(Self {
            _worker: worker,
            name: internal.name,
            id: internal.id,
            lights: internal.lights,
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

    pub fn dim(&self, level: u8) -> super::super::Result<()> {
        let mut req = CoAPRequest::new();
        req.set_path(&format!("15001/{}", self.id));
        req.set_method(Method::Put);
        req.message.set_payload(
            format!("{{ \"3311\": [{{ \"5851\": {} }}] }}", level)
                .as_bytes()
                .to_vec(),
        );
        self._worker.send(req)?;

        Ok(())
    }

    //NOTE uses colourname instead of string as unknown hexcodes default to "Glow"
    pub fn colour(&self, colour: ColourName) -> super::super::Result<()> {
        let mut req = CoAPRequest::new();
        req.set_path(&format!("15001/{}", self.id));
        req.set_method(Method::Put);
        req.message.set_payload(
            format!(
                "{{ \"3311\": [{{ \"5706\": \"{}\" }}] }}",
                serde_json::to_string(&colour)?
            )
            .as_bytes()
            .to_vec(),
        );
        self._worker.send(req)?;

        Ok(())
    }

    pub fn colour_xy(&self, x: u32, y: u32) -> super::super::Result<()> {
        let mut req = CoAPRequest::new();
        req.set_path(&format!("15001/{}", self.id));
        req.set_method(Method::Put);
        req.message.set_payload(
            format!("{{ \"3311\": [{{ \"5709\": {}, \"5710\": {} }}] }}", x, y)
                .as_bytes()
                .to_vec(),
        );
        self._worker.send(req)?;

        Ok(())
    }
}
