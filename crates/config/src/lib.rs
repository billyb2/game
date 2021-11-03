#![deny(clippy::all)]
#![feature(path_try_exists)]

use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "native")]
use directories_next::ProjectDirs;

#[cfg(feature = "web")]
#[wasm_bindgen(inline_js = "export function js_get_data(a){return localStorage.getItem(a)}export function js_write_data(a,b){localStorage.setItem(a,b)}export function js_delete_data(a){localStorage.removeItem(a)}export function js_key_exists(a){if(localStorage[a]){true}else{false}}")]
extern "C" {
    fn js_get_data(key: String) -> Option<String>;
    fn js_write_data(key: String, value: String);
    fn js_delete_data(key: String);
    fn js_key_exists(key: String) -> bool;

}

pub fn get_data<'a, T>(key: String) -> Option<T> where T: Deserialize<'a> {
    #[cfg(feature = "web")]
    let value = {
        let string = js_get_data(key);

        match string {
            Some(string) => {
                let string = Box::leak(string.into_boxed_str());
                let val: T = from_str(string).expect("Failed to deserialize");
                Some(val)

            },
            None => None,
        }
    };

    #[cfg(feature = "native")]
    let value = {
        use std::fs::{File, create_dir_all, read_to_string};
        use std::io::ErrorKind;

        let key_path = get_path_from_key(key);

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
                ErrorKind::PermissionDenied => panic!("Permission denied to access {:?}", key_path),
                other_err => panic!("Unknown error: {:?}", other_err), 
            }           
        }

    };

    value


}

pub fn write_data<T>(key: String, value: T) where T: Serialize {
    #[cfg(feature = "web")]
    {
        let ron_config = PrettyConfig::new();
        let value = to_string_pretty(&value, ron_config).unwrap();

        js_write_data(key, value);
    };

    #[cfg(feature = "native")]
    {
        use std::fs::write;

        let key_path = get_path_from_key(key);

        let ron_config = PrettyConfig::new();
        let value = to_string_pretty(&value, ron_config).unwrap();

        write(key_path, value).unwrap();

    };
}

pub fn delete_data(key: String) -> Result<(), Option<std::io::Error>> {
    if check_key(key.clone()) {
        // Only try to delete an item if it exists
        #[cfg(feature = "native")]
        {
            use std::fs::remove_file;

            let key_path = get_path_from_key(key);

            match remove_file(key_path) {
                Ok(_) => Ok(()),
                Err(e) => Err(Some(e)),

            }

        }

        #[cfg(feature = "web")]
        {
            js_delete_data(key);
            Ok(())

        }

    } else {
        // If the file doesn't exist, return an error
        Err(None)
    }

}

pub fn check_key(key: String) -> bool {
    #[cfg(feature = "web")]
    let value = js_key_exists(key);

    #[cfg(feature = "native")]
    let value = {
        use std::fs::try_exists;

        let key_path = get_path_from_key(key);
        try_exists(key_path).unwrap()
    };

    value
}

#[cfg(feature = "native")]
fn get_path_from_key(key: String) -> std::path::PathBuf {
    use std::fs::create_dir_all;

    let proj_dirs = ProjectDirs::from("com", "William Batista",  "game").unwrap();
    let config_dir = proj_dirs.config_dir();

    create_dir_all(config_dir.clone());

    let key_path = config_dir.join(key);

    key_path
}
