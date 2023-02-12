use reqwest::{Response};
use crate::hubitat::http::HClient;
pub use super::Device;
use super::Attributes;
use super::Command;

pub enum SwitchValue {
    On,
    Off
}

impl SwitchValue {
    pub fn as_string(&self) -> String {
        match self {
            SwitchValue::On => "on".to_string() ,
            SwitchValue::Off => "off".to_string()
        }
    }
}


impl Device {
    pub fn new(id: &str, client: HClient) -> Self {
        Self {
            id: String::from(id),
            client,
            name: None,
            label: None,
            device_type: None,
            room: None,
            capabilities: None,
            commands: None,
            attributes: None,
            state: None
        }
    }
    pub async fn get_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let body = self.client.get(format!("/apps/api/24/devices/{}", self.id))
            .await?;
        let json_response: super::Device = body.json().await?;

        self.name = json_response.name;
        self.label = json_response.label;
        self.device_type = json_response.device_type;
        self.room = json_response.room;
        self.capabilities = json_response.capabilities;
        self.attributes = json_response.attributes;

        for i in self.attributes.as_ref().unwrap() {
            if i.name == Attributes::Switch.as_str() {
                self.state = Option::from(i.current_value.as_str().unwrap().to_string());
                break;
            }
        };

        Ok(())
    }

    pub async fn toggle(&mut self) -> Result<Response, Box<dyn std::error::Error>>{

        if self.state.is_none() {
            let device_state_result = self.get_state().await;
            match device_state_result {
                Ok(_r) => (),
                Err(e) => panic!("Could not get device state {:?}", e)
            };
        }

        let current_state = self.state.as_ref().unwrap();
        let command_name = &(if current_state == &SwitchValue::On.as_string() {
            SwitchValue::Off.as_string()
        } else {
            SwitchValue::On.as_string()
        });

        let command_result = self.send_command(Command{
            name: String::from(command_name),
            value_type: vec![]
        }).await?;

        self.state = Option::from(String::from(command_name));

        Ok(command_result)
    }

    pub async fn send_command(&self, command: super::Command) -> Result<Response, Box<dyn std::error::Error>> {
        let body = self.client.get(format!("/apps/api/24/devices/{}/{}", self.id, command.name))
            .await?;
        Ok(body)
    }

    pub async fn get_commands(&self) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
        let body = self.client.get(format!("/apps/api/24/devices/{}/commands", self.id))
            .await?;
        let json_response: Vec<super::Command> = body.json().await?;
        Ok(json_response)
    }
}
