use {
    super::TradfriConnection,
    coap::{message::request::Method, CoAPRequest},
    serde::Deserialize,
    std::net::IpAddr,
};

#[derive(Debug, Deserialize)]
struct AuthResponse {
    #[serde(rename = "9091")]
    pre_shared_key: String,
}

pub struct TradfriAuthenticator;

impl TradfriAuthenticator {
    pub fn authenticate<A: Into<IpAddr>>(
        addr: A,
        key_name: &str,
        security_code: &str,
        timeout: u64,
    ) -> super::Result<String> {
        let mut con = TradfriConnection::new_with_timeout(
            addr,
            b"Client_identity",
            security_code.as_bytes(),
            Some(timeout),
        )?;

        let mut req = CoAPRequest::new();
        req.set_path("15011/9063");
        req.set_method(Method::Post);
        req.message.set_payload(
            format!("{{\"9090\": \"{}\"}}", key_name)
                .as_bytes()
                .to_owned(),
        );

        con.send(req)?;

        let response = con.receive()?;
        let content: AuthResponse = serde_json::from_slice(&response.message.payload)?;

        Ok(content.pre_shared_key)
    }
}
