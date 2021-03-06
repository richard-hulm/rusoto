#![cfg(feature = "ram")]

extern crate rusoto_core;
extern crate rusoto_ram;

use rusoto_core::Region;
use rusoto_ram::{ListResourcesRequest, Ram, RamClient};

#[test]
fn should_work() {
    let client = RamClient::new(Region::UsEast1);
    let response = client
        .list_resources(ListResourcesRequest {
            resource_owner: "SELF".into(),
            ..ListResourcesRequest::default()
        })
        .sync()
        .expect("expected an ok response");
    println!("response is {:#?}", response);
}
