use ddc::Ddc;
use ddc_winapi::Monitor;
use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "monitor-switcher")]
#[command(about = "Switch monitor input via DDC/CI")]
struct Args {
    /// Monitor type (main or secondary)
    #[arg(short, long, value_enum)]
    monitor: MonitorType,
    
    /// Value
    #[arg(short, long)]
    value: u16,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum MonitorType {
    Main,
    Secondary,
}

#[derive(Deserialize)]
struct Config {
    monitors: MonitorConfig,
}

#[derive(Deserialize)]
struct MonitorConfig {
    main_maximum: u16,
    secondary_maximum: u16,
}

const INPUT_SELECT_VCP_CODE: u8 = 0x60;

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn main() {
    let args = Args::parse();
    
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };
    
    for mut ddc in Monitor::enumerate().unwrap() {
        match ddc.get_vcp_feature(INPUT_SELECT_VCP_CODE) {
            Ok(value) => {
                let maximum = value.maximum();
                let is_target_monitor = match args.monitor {
                    MonitorType::Main => maximum == config.monitors.main_maximum,
                    MonitorType::Secondary => maximum == config.monitors.secondary_maximum,
                };
                
                if is_target_monitor {
                    let current_value = value.value();
                    if current_value != args.value {
                        match ddc.set_vcp_feature(INPUT_SELECT_VCP_CODE, args.value) {
                            Ok(_) => println!("Successfully set VCP code 0x{:02X} from {} to {}", INPUT_SELECT_VCP_CODE, current_value, args.value),
                            Err(e) => println!("Error setting VCP feature: {:?}", e),
                        }
                    } else {
                        println!("VCP code 0x{:02X} is already set to {}, no change needed", INPUT_SELECT_VCP_CODE, current_value);
                    }
                    break;
                }
            },
            Err(e) => println!("Error getting monitor info: {:?}", e),
        }
    }
}
