mod lang;

use gfx_hal::Backend;
use app_dirs::{AppInfo, get_app_root, AppDataType, app_root};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, Read, BufReader};
use unic_langid::LanguageIdentifier;
use fluent::{FluentBundle, FluentResource};
use std::path::PathBuf;
use std::env;
use std::error::Error;

#[cfg(target_family = "windows")]
const BACKEND_ATTEMPTS: &'static [&'static str] = &["vulkan", "directx12"];

#[cfg(target_os = "macos")]
const BACKEND_ATTEMPTS: &'static [&'static str] = &["metal"];

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
const BACKEND_ATTEMPTS: &'static [&'static str] = &["vulkan", "gl"];

const APP_NAME: &'static str = "unnamedracinggame";
const APP_AUTHOR: &'static str = "s5bug";
const APP_INFO: AppInfo = AppInfo {
    name: APP_NAME,
    author: APP_AUTHOR
};

#[derive(Serialize, Deserialize)]
struct GlobalConfig {
    language: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let configuration_dir = app_root(AppDataType::UserConfig, &APP_INFO)?;

    let config_file_path = configuration_dir.join("global_config.json");

    if !config_file_path.exists() {
        let mut config_file = File::create(&config_file_path)?;

        let default_language =
            whoami::lang().next().unwrap_or(String::from("en-US"));

        let initial_config = GlobalConfig {
            language: default_language
        };

        let config_json =
            serde_json::to_string(&initial_config)?;

        config_file.write_all(config_json.as_bytes())?
    }

    let config_file = File::open(&config_file_path)?;

    let config_reader = BufReader::new(config_file);

    let config: GlobalConfig =
        serde_json::from_reader(config_reader)?;

    let language = config.language;
    let langid: LanguageIdentifier =
        language.parse()?;

    let langhandler = lang::LanguageHandler::new("lang", &langid);

    Ok(println!("{}", langhandler.format("game_title", None)))
}
