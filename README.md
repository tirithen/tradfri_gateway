# tradfri_gateway

A rust crate for connecting to the Ikea TRÅDFRI gateway, and for controlling your home
devices through the gateway.

It also allows to optionally auto discover the IP of your Gateway via mDNS to simplify
configuration.

The purpose of this crate was to create a stable build where several of the related crates
relied on external binaries or C bindings that made the build more complicated than needed.
The crate includes the coap/dtls code that works for this application, the only system
dependency is openssl via the openssl crate. Also the idea is to make it as simple as
possible to connect to the gateway rather than exposing all low level details.

The crate is under initial development so count on breaking changes every now and then
until the best API has been found.

## Usage example

Example of a simple planing program (from `examples/light.rs`):

```rust
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
```

Enter your credentials in the example tile and run the example with:
```bash
$ cargo run --example light
```

## Whishlist for new features

* Support for more devices.
* Add optional automatic storage of session key and identifier on disk to
  simplify connection process.
* Add cli to control the gateway from a terminal, it would also be a good
  demo.
* Add relevant debug logs.
* Support for the newer Ikea DIRIGERA hub.

## Contributing

You can help out in several ways:

1. Try out the crate
2. Report any issues, bugs, missing documentation or examples
3. Create issues with feedback on the ergonomy of the crate APIs
4. Extend the documentation or examples
5. Contribute code changes

Feedback on the ergonomics of this crate or its features/lack there of might
be as valuable as code contributions.

### Code contributions

That being said, code contributions are more than welcome. Create a merge
request, with your new cuts, tools, programs or other improvements and create
a pull request.

Ensure that any new/changed publicly facing APIs have proper documentation
comments.

Once a pull request is ready for merge, also squash the commits into a single
commit per feature or fix.

The commit messages in this project should comply with the
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) standard
so that the [semver](https://semver.org/) versions can be automatically
calculated and the release changelog can be automatically generated.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
