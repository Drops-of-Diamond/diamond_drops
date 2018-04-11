use cli::config;

#[derive(PartialEq)]
enum ConfigType {
    Mode,
    Nil
}

pub fn parse_args(args: Vec<String>) -> Result<config::Config, &'static str> {
    let mut config_type = ConfigType::Nil;
    
    // Default Case
    let mut mode = config::Mode::Both;

    for arg in args {
        // Match `-` prefixed args to a list of valid configuration points and set their values with the following non `-` prefixed value
        if arg.starts_with("-") {
            match arg.to_lowercase().as_ref() {
                    "-mode" => { config_type = ConfigType::Mode; },
                    _ => { return Err("Invalid configuration argument, try -mode."); }
                }
        } else if config_type == ConfigType::Mode {
            // Match provided value to mode type
            match arg.to_lowercase().as_ref() {
                "proposer" | "p" => { mode = config::Mode::Proposer; },
                "notary" | "n" => { mode = config::Mode::Notary; },
                "both" | "b" => { mode = config::Mode::Both; }
                _ => { return Err("Invalid configuration value"); }
            }

            config_type = ConfigType::Nil;
        } else {
            return Err("No configuration argument supplied");
        }
    }

    if config_type == ConfigType::Nil {
        Ok(config::Config::new(mode))
    } else {
        Err("No configuration value supplied")
    }
}
