use fluent::{FluentBundle, FluentResource, FluentArgs, FluentError, FluentMessage};
use std::borrow::Cow;
use unic_langid::LanguageIdentifier;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;

pub struct LanguageHandler {
    bundle: FluentBundle<FluentResource>
}

impl LanguageHandler {

    pub fn new(
        bundle_id: &str,
        language: &LanguageIdentifier
    ) -> LanguageHandler {
        let mut bundle = FluentBundle::new(vec![language.clone()]);

        match std::env::current_dir() {
            Ok(curdir) => {
                let mut lang_file_name = String::new();
                lang_file_name.push_str(bundle_id);
                lang_file_name.push_str("_");
                lang_file_name.push_str(&language.to_string());
                lang_file_name.push_str(".ftl");

                let mut lang_file_path = PathBuf::new();
                lang_file_path.push(curdir);
                lang_file_path.push("res");
                lang_file_path.push(lang_file_name);

                match File::open(&lang_file_path) {
                    Ok(mut lang_file) => {
                        let mut content = String::new();
                        match lang_file.read_to_string(&mut content) {
                            Ok(_) => {
                                match FluentResource::try_new(content) {
                                    Ok(res) => {
                                        match bundle.add_resource(res) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                eprintln!("Localization error: couldn't add language file {:?} to language bundle: {:?}", &lang_file_path.to_str(), err);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Localization error: couldn't parse language file {:?}: {:?}", &lang_file_path.to_str(), err.1);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("Localization error: couldn't read language file {:?}: {}", &lang_file_path.to_str(), err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Localization error: couldn't open language file {:?}: {}", &lang_file_path.to_str(), err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Localization error: couldn't retrieve current working directory: {}", err);
            }
        }

        LanguageHandler { bundle }
    }

    pub fn format<'handler>(
        &'handler self,
        key: &'handler str,
        args: Option<&'handler FluentArgs>,
    ) -> Cow<'handler, str> {
        let maybe_msg = self.bundle.get_message(key);
        match maybe_msg {
            None => {
                let locale_vec: Vec<_> = self.bundle.locales.iter().map(|l| l.to_string()).collect();
                eprintln!("Localization error: could not find message {} for locale {:?}.", key, locale_vec);
                Cow::Borrowed(key)
            }
            Some(msg) => {
                let mut errors = vec![];
                let maybe_pattern = msg.value();
                match maybe_pattern {
                    None => {
                        let locale_vec: Vec<_> = self.bundle.locales.iter().map(|l| l.to_string()).collect();
                        eprintln!("Localization error: could not generate a pattern for message {} for locale {:?}.", key, locale_vec);
                        Cow::Borrowed(key)
                    }
                    Some(pattern) => {
                        let result = self.bundle.format_pattern(pattern, args, &mut errors);
                        if !errors.is_empty() {
                            let locale_vec: Vec<_> = self.bundle.locales.iter().map(|l| l.to_string()).collect();
                            eprintln!(
                                "Localization error: encountered errors {:?} when formatting {} for locale {:?}.",
                                errors,
                                key,
                                locale_vec
                            );
                        }
                        result
                    }
                }
            }
        }
    }

}
