use std::env;
use viessmann_sdk::*;
use viessmann_sdk::viessmann_client::UserInfo;

fn main() {
    let args: Vec<String> = env::args().collect();

    let token = match env::var_os("VIESSMANN_TOKEN") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("Token is not set")
    };

    let action: &str = &args[1];

    match action {
        "--help" => print_help(),
        "--info" => {
            print_info(token.as_str())
        },
        "--refresh_token" => {
            if args.len() > 3 {
                let client_id: &str = &args[2];
                let refresh_token: &str = &args[3];
                refresh_viessmann_token(client_id, refresh_token);
            } else {
                println!("Provide the client id and the refresh token in order to refresh your token")
            }
        },
        _ => print_help()
    }


}

fn print_help() {
    println!("Usage: viessmann-sdk <action>\n");
    println!("--info");
    println!("--refresh_token");
}

fn refresh_viessmann_token(client_id: &str, refresh_token: &str) {
    let token = viessmann_client::refresh_token(client_id, refresh_token)
        .expect("Refresh token failed, please make sure your refresh token is valid");

    println!("Your new token {}", token.access_token);
    println!("Your new refresh token {}", token.refresh_token);
}

 fn print_info(token: &str) {
     let user: UserInfo = viessmann_client::get_user_info(token).expect("fail to fetch user info");
     let installations = viessmann_client::get_installations(token).expect("fail to fetch user installations");
     let installation_id = installations.data[0].id;
     let gateways = viessmann_client::get_gateways(token).expect("fail to fetch user gateways");
     let gateway_serial= gateways.data[0].serial.as_str();
     let devices = viessmann_client::get_devices(token, installation_id, gateway_serial.parse().unwrap()).expect("fail to fetch devices");

     println!("Account name {} \n", user.login_id);
     println!("Devices ({}) \n", devices.data.len());
     for d in devices.data {
         println!("device {}", d.id);
     }
 }
