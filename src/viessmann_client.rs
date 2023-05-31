use std::collections::HashMap;
use serde::{Deserialize};
use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    #[serde(alias = "loginId")]
    pub login_id: String,
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    expires_in: i32,
}

#[derive(Deserialize, Debug)]
pub struct Installation {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Installations {
    pub data: Vec<Installation>,
}

#[derive(Deserialize, Debug)]
pub struct Gateway {
    pub serial: String,
}

#[derive(Deserialize, Debug)]
pub struct Gateways {
    pub data: Vec<Gateway>,
}

#[derive(Deserialize, Debug)]
pub struct Features {
    data: Vec<Feature>,
}

#[derive(Deserialize, Debug)]
pub struct Feature {
    feature: String,
    #[serde(alias = "isEnabled")]
    enabled: bool,
    #[serde(alias = "isReady")]
    ready: bool
}

#[derive(Deserialize, Debug)]
pub struct Devices {
    pub data: Vec<Device>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    pub id: String
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(alias = "eventType")]
    event_type: String
}

#[derive(Deserialize, Debug)]
pub struct Events {
    data: Vec<Event>,
}

pub struct ViessmannClient {
    pub client_id: String,
    pub refresh_token: String,
    current_token: String
}

impl ViessmannClient {

    pub fn new(client_id: String, refresh_token: String) -> ViessmannClient {
        ViessmannClient { client_id, refresh_token, current_token: "".to_string()}
    }

    pub fn user_info(&self) -> Result<UserInfo, String> {
        get_user_info(self.current_token.as_str())
    }

    pub fn refresh_token(&self) -> Result<Token, String> {
        refresh_token(self.client_id.as_str(), self.refresh_token.as_str())
    }

}

pub fn refresh_token(client_id: &str, refresh_token: &str) -> Result<Token, String> {
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", refresh_token);
    params.insert("client_id", client_id);

    let data = serde_urlencoded::to_string(params).expect("serialize issue for http parameter");

    let url = "https://iam.viessmann.com/idp/v2/token";

    let builder = client.post(url).body(data).header("Content-Type", "application/x-www-form-urlencoded");

    let result = builder.send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let body = response.json::<Token>();
        match body {
            Ok(json) => Ok(json),
            Err(err) => return Err(err.to_string()),
        }
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_user_info(token: &str) -> Result<UserInfo, String> {
    let client = Client::new();

    let result = client.get("https://api.viessmann.com/users/v1/users/me")
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: UserInfo = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_installations(token: &str) -> Result<Installations, String> {
    let client = Client::new();

    let result = client.get("https://api.viessmann.com/iot/v1/equipment/installations")
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Installations = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_gateways(token: &str) -> Result<Gateways, String> {
    let client = Client::new();

    let result = client.get("https://api.viessmann.com/iot/v1/equipment/gateways")
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Gateways = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_devices(token: &str, installation_id: i32, gateway_serial: &str) -> Result<Devices, String> {
    let client = Client::new();

    let url = format!("https://api.viessmann.com/iot/v1/equipment/installations/{installationId}/gateways/{gatewaySerial}/devices",
                      installationId = installation_id, gatewaySerial = gateway_serial);
    let result = client.get(url)
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Devices = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_gateway_features(token: &str, installation_id: i32, gateway_serial: i64) -> Result<Features, String> {
    let client = Client::new();

    let url = format!("https://api.viessmann.com/iot/v1/features/installations/{installationId}/gateways/{gatewaySerial}/features",
                      installationId = installation_id, gatewaySerial = gateway_serial);
    let result = client.get(url)
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Features = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_device_features(token: &str, installation_id: i32, gateway_serial: i64, device_id: &str) -> Result<Features, String> {
    let client = Client::new();

    let url = format!("https://api.viessmann.com/iot/v1/features/installations/{installationId}/gateways/{gatewaySerial}/devices/{deviceId}/features",
                      installationId = installation_id, gatewaySerial = gateway_serial, deviceId = device_id);
    let result = client.get(url)
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Features = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_device_feature_by_name(token: &str, installation_id: i32, gateway_serial: i64, device_id: i32, feature_name: &str) -> Result<Installations, String> {
    let client = Client::new();

    let url = format!("https://api.viessmann.com/iot/v1/equipment/installations/{installationId}/gateways/{gatewaySerial}/devices/{deviceId}/features/{featureName}",
                      installationId = installation_id, gatewaySerial = gateway_serial, deviceId = device_id, featureName = feature_name);
    let result = client.get(url)
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Installations = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

pub fn get_events(token: &str) -> Result<Events, String> {
    let client = Client::new();

    let result = client.get("https://api.viessmann.com/iot/v1/events-history/events")
        .header("content-type", "application/json")
        .bearer_auth(token)
        .send();

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err.status().expect("Failed to get status message").to_string()),
    };

    if response.status().is_success() {
        let text = response.text().expect("error");
        let v: Events = serde_json::from_str(text.as_str()).expect("error");
        return Ok(v);
    } else {
        return Err(response.text().expect("Failed to get response body").to_string());
    }
}

#[cfg(test)]
mod tests {

    static REFRESH_TOKEN: &str = "";
    static CLIENT_ID: &str = "";
    static USER_NAME: &str = "";

    use super::*;

    #[test]
    fn test_user_info() {

        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_user_info(new_token.as_str());
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        let name = result.unwrap().login_id;
        assert!(name.contains(USER_NAME));
        println!("{}",name );
    }

    #[test]
    fn test_installations() {
        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_installations(new_token.as_str());
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        let installations = result.unwrap().data;
        println!("{:?}", installations);
    }

    #[test]
    fn test_list_devices() {
        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_devices(new_token.as_str(), 2219527, 7637415015415225);
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        println!("{:?}", result.unwrap());
    }

    #[test]
    fn test_device_feature() {
        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_device_features(new_token.as_str(), 2219527, 7637415015415225, "0");
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        println!("{:?}", result.unwrap());
    }

    #[test]
    fn test_gateway_feature() {
        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_gateway_features(new_token.as_str(), 2219527, 7637415015415225);
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        println!("{:?}", result.unwrap());
    }

    #[ignore]
    #[test]
    fn test_device_feature_by_name() {
        let new_token = refresh_token(CLIENT_ID, REFRESH_TOKEN).expect("failed to get new token").access_token;
        assert_eq!(new_token.is_empty(), false);
        let result = get_device_feature_by_name(new_token.as_str(), 2219527, 7637415015415225, 0, "heating.power.consumption.total");
        assert_eq!(result.is_ok(), true, "Result should not be an error");
        println!("{:?}", result.unwrap());
    }

}
