use std::env;
use viessmann_sdk::*;
use viessmann_sdk::viessmann_client::{refresh_token, UserInfo};

fn main() {
    println!("Viessmann API project");
    let args: Vec<String> = env::args().collect();

    let client_id = &args[1];
    let refresh_token = &args[2];
    println!("Client ID {}", client_id.as_str());


    let token = viessmann_client::refresh_token(client_id, refresh_token).expect("fail token");
    println!("Token {}", token.access_token.as_str());
    let user: UserInfo = viessmann_client::get_user_info(token.access_token.as_str()).expect("fail to fetch user info");
    let installations = viessmann_client::get_installations(token.access_token.as_str()).expect("fail to fetch user installations");
    let installation_id = installations.data[0].id;
    let gateways = viessmann_client::get_gateways(token.access_token.as_str()).expect("fail to fetch user gateways");
    let gateway_serial= gateways.data[0].serial.as_str();
    let devices = viessmann_client::get_devices(token.access_token.as_str(), installation_id, gateway_serial).expect("fail to fetch devices");

    println!("Account name {}", user.login_id);
    println!("Device count {}", devices.data.len());
    println!("Devices list:");
    for d in devices.data {
        println!("device {}", d.id);
    }
}
