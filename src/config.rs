use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "native")]
use directories_next::ProjectDirs;

#[cfg(feature = "web")]
#[wasm_bindgen(module = "/js_functions.js")]
extern "C" {
    fn js_get_data(key: String) -> Option<String>;
    fn js_write_data(key: String, value: String);

}

#[cfg(feature = "web")]
#[inline]
pub fn get_data<'a, T>(key: String) -> Option<T> where T: Deserialize<'a> {
    let string = js_get_data(key);

    match string {
        Some(string) => {
            let string = Box::leak(string.into_boxed_str());
            let val: T = from_str(string).expect("Failed to deserialize");
            Some(val)

        },
        None => None,
    }
}


#[cfg(feature = "native")]
pub fn get_data<'a, T>(key: String) -> Option<T> where T: Deserialize<'a> {
    use std::fs::{File, create_dir_all, read_to_string};
    use std::io::ErrorKind;

    let proj_dirs = ProjectDirs::from("com", "William Batista",  "game").unwrap();
    let config_dir = proj_dirs.config_dir();

    create_dir_all(config_dir).unwrap();

    let key_path = config_dir.join(key);

    match read_to_string(&key_path) {
        Ok(string) => {
            let string = Box::leak(string.into_boxed_str());
            let val: T = from_str(string).expect("Failed to deserialize");
            
            Some(val)
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                File::create(key_path).unwrap();
                None
            },
            ErrorKind::PermissionDenied => panic!("Permission denied to access {:?}", config_dir),
            other_err => panic!("Unknown error: {:?}", other_err), 
        }           
    }

}

#[cfg(feature = "web")]
#[inline]
pub fn write_data<T>(key: String, value: T) where T: Serialize {
    let ron_config = PrettyConfig::new();
    let value = to_string_pretty(&value, ron_config).unwrap();

    js_write_data(key, value);
}

#[cfg(feature = "native")]
pub fn write_data<T>(key: String, value: T) where T: Serialize {
    use std::fs::write;

    let proj_dirs = ProjectDirs::from("com", "William Batista",  "game").unwrap();
    let config_dir = proj_dirs.config_dir();

    let key_path = config_dir.join(key);

    let ron_config = PrettyConfig::new();
    let value = to_string_pretty(&value, ron_config).unwrap();

    write(key_path, value).unwrap();

}
