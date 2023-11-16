use tradfri_gateway::{Device, TradfriGatewayConnector};

fn main() {
    // Connect with gateway code
    let gateway_code = "enter gateway code from the underside of your TRÅDFRI gateway";
    let mut gateway = TradfriGatewayConnector::from_gateway_code(gateway_code)
        .unwrap()
        .connect()
        .unwrap();
    println!("{:#?}", gateway);

    // Connect with identifier and session key
    // let session_key = "enter pre shared key generated from gateway code";
    // let identifier = "enter identifier generated along with the pre shared key";
    // let mut gateway =
    //     TradfriGatewayConnector::from_identifier_and_session_key(identifier, session_key)
    //         .unwrap()
    //         .connect()
    //         .unwrap();
    // println!("{:#?}", gateway);

    // Get all device ids
    let ids = gateway.device_ids().unwrap();
    println!("ids {:#?}", ids);

    // Turn on all your lights
    for id in ids {
        let device = gateway.device(id).unwrap();
        println!("device {:#?}", device);

        match device {
            Device::RemoteControl => (),
            Device::Light(mut light) => {
                light.on().unwrap();
                println!("light {:#?}", light);
            },
        }
    }
}
