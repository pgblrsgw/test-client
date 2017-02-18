extern crate reqwest;
extern crate a4_proto as proto;
extern crate serde;

use std::env;
use std::process;
use reqwest::{Client, RequestBuilder};
use serde::Deserialize;
use std::fmt::Debug;

fn main() {
    let domain = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: test-client <domain/ip>");
        process::exit(1);
    });
    let client = Client::new().expect("Unable to create client");

    fn dispatch_and_print(request: RequestBuilder) {
        println!("Request:\n{:?}\n", request);
        println!("Response:\n{:?}\n", request.send());
    }

    fn dispatch_and_print_native_response<T: Deserialize + Debug>(request: RequestBuilder) {
        println!("Request:\n{:?}\n", request);
        let response = request.send();
        println!("Response:\n{:?}\n", response);
        println!("Response Native Object:\n{:?}\n", response.map(|mut r| r.json::<T>()));
    }

    println!("##########");
    println!("########## Remove 'test' from the problems in case it existed before.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test")));
    println!("##########");
    println!("########## Add 'test' to the problems.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test")));
    println!("##########");
    println!("########## Create the boundary.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test/Boundary")).json(&proto::Boundary{
        width: 10.0,
        length: 10.0,
        point: proto::Point{ x: -5.0, y: -5.0 },
    }));
    println!("##########");
    println!("########## Create the robot.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test/Robot")).json(&proto::Robot{
        point: proto::Point{ x: -1.0, y: -1.0 },
        radius: 0.2,
    }));
    println!("##########");
    println!("########## Create the goal.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test/Goal")).json(&proto::Goal{
        point: proto::Point{ x: 2.0, y: 2.0 },
    }));
    println!("##########");
    println!("########## Add 'block' to the obstacles of 'test'.");
    println!("##########");
    dispatch_and_print(client.post(&(domain.clone() + "/test/Obstacles/block")).json(&proto::Obstacle{
        length: 1.0,
        width: 1.0,
        point: proto::Point{ x: 0.0, y: 0.0 },
    }));
    println!("##########");
    println!("########## Acquire the path back.");
    println!("##########");
    dispatch_and_print_native_response::<proto::Path>(client.get(&(domain.clone() + "/test/Path")));
}
