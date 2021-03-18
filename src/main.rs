use gfx_hal::Backend;
use app_dirs::{AppInfo, get_app_root, AppDataType, app_root};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, Read, BufReader};
use unic_langid::LanguageIdentifier;
use fluent::{FluentBundle, FluentResource};
use std::path::PathBuf;
use std::env;

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

fn main() {
    let configuration_dir =
        app_root(AppDataType::UserConfig, &APP_INFO)
            .expect("Unable to generate a configuration directory.");

    let config_file_path = configuration_dir.join("global_config.json");

    if !config_file_path.exists() {
        let mut config_file = File::create(&config_file_path)
            .expect("Unable to create configuration file.");

        let default_language =
            whoami::lang().next().unwrap_or(String::from("en-US"));

        let initial_config = GlobalConfig {
            language: default_language
        };

        let config_json =
            serde_json::to_string(&initial_config)
                .expect("Unable to serialize the default configuration.");

        config_file
            .write_all(config_json.as_bytes())
            .expect("Unable to write the default configuration.")
    }

    let config_file = File::open(&config_file_path)
        .expect("Unable to open configuration file.");

    let config_reader = BufReader::new(config_file);

    let config: GlobalConfig =
        serde_json::from_reader(config_reader)
            .expect("Unable to parse configuration file.");

    let language = config.language;
    let langid: LanguageIdentifier =
        language.parse()
            .expect("The language string in the configuration file is invalid.");

    let mut bundle = FluentBundle::new(vec![langid]);

    let mut lang_file_name = String::new();
    lang_file_name.push_str("lang_");
    lang_file_name.push_str(&language);
    lang_file_name.push_str(".ftl");

    let curdir = env::current_dir()
        .expect("Unable to retrieve the current directory.");

    let mut lang_file_path = PathBuf::new();
    lang_file_path.push(curdir);
    lang_file_path.push("res");
    lang_file_path.push(lang_file_name);

    let mut lang_file = File::open(lang_file_path)
        .expect("Could not find the correct language file.");

    let mut content = String::new();

    lang_file.read_to_string(&mut content)
        .expect("Unable to read language file.");

    let res =
        FluentResource::try_new(content)
            .expect("Language file contents are invalid.");

    bundle
        .add_resource(res)
        .expect("Unable to add the language resource to the language bundle.");

    let title_msg = bundle.get_message("game_title")
        .expect("game_title missing from the language bundle.");
    let mut errors = vec![];
    let pattern = title_msg.value()
        .expect("game_title is missing a value in the language bundle.");
    let title = bundle.format_pattern(&pattern, None, &mut errors);

    println!("Hello from {}!", title);
}
