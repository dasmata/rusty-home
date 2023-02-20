use std::env;
use async_recursion::async_recursion;
use hubitat::device::switch;
use hubitat::http::HClient;
use config::Config;

mod hubitat;
mod config;
mod rf;

#[tokio::main]
async fn main() -> ! {
    let args: Vec<String> = env::args().collect();
    for i in args {
        if i.eq("--config") {
            match config::build_config() {
                Ok(..) => (),
                Err(..) => panic!("Could not create config!")
            };
        }
    }

    let config = load_config();
    println!("{}", config.version);


    let hubitat_api: HClient = HClient::new(&config.hubitat_host, &config.hubitat_key);
    let study_switch = switch::Device::new(
        "174",
        hubitat_api
    );

    rf::init_rf();

    // init(study_switch).await;
}

#[async_recursion(?Send)]
async fn init(mut sw: switch::Device) -> () {
    let mut input = String::new();
    println!("Should I toogle?");
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim_end() == "y" {
        let toggle_result = sw.toggle().await;
        match toggle_result {
            Ok(_r) => println!("Switch toggled"),
            Err(e) => panic!("Command not processed: {:?}", e)
        };
        init(sw).await;
    } else {
        println!("pacat");
    }
}

fn load_config() -> Config {
    let config_load = config::load();
    let config: Config = match config_load {
        Err(_e) => panic!("Could not load config"),
        Ok(r) => r
    };

    if config.version > 0 as u8 {
        return config;
    }

    match config::build_config() {
        Ok(..) => (),
        Err(..) => panic!("|could not build config!")
    };
    load_config()
}
