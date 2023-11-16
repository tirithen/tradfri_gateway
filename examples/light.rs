use tradfri_gateway::{Device, TradfriGateway};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect with gateway code
    let gateway_code = "enter gateway code from the underside of your TRÅDFRI gateway";
    let mut gateway = TradfriGateway::from_gateway_code(gateway_code)?;
    println!("{:#?}", gateway);

    // Connect with identifier and session key once created
    // let session_key = "enter pre shared key generated from gateway code";
    // let identifier = "enter identifier generated along with the pre shared key";
    // let mut gateway =
    //     TradfriGateway::from_identifier_and_session_key(identifier, session_key)?;
    // println!("{:#?}", gateway);

    // Toggle all your lights
    for i in 0..10 {
        for device in gateway.devices()? {
            match device {
                Ok(Device::RemoteControl) => (),
                Ok(Device::Light(mut light)) => {
                    if i % 2 == 0 {
                        light.on()?;
                    } else {
                        light.off()?;
                    }
                    println!("light {:#?}", light);
                }
                Err(error) => panic!("{}", error),
            }
        }
    }

    // Turn on one specific light, if id exists
    for i in 0..10 {
        if let Ok(Device::Light(mut light)) = gateway.device(65568) {
            if i % 2 == 0 {
                light.on()?;
            } else {
                light.off()?;
            }
        }
    }

    Ok(())
}
