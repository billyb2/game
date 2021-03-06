#![deny(clippy::all)]
#![feature(path_try_exists)]

#[cfg(any(feature = "native", not(target_arch = "wasm32")))]
use std::io::ErrorKind;

use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

#[cfg(any(feature = "web", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

#[cfg(any(feature = "native", not(target_arch = "wasm32")))]
use directories_next::ProjectDirs;

#[cfg(any(feature = "web", target_arch = "wasm32"))]
#[wasm_bindgen(inline_js = "export function js_get_data(a){return localStorage.getItem(a)}export function js_write_data(a,b){localStorage.setItem(a,b)}export function js_delete_data(a){localStorage.removeItem(a)}export function js_key_exists(a){if(localStorage[a]){true}else{false}}")]
extern "C" {
    fn js_get_data(key: &str) -> Option<String>;
    fn js_write_data(key: &str, value: String);
    fn js_delete_data(key: &str);
    fn js_key_exists(key: &str) -> bool;

}

pub fn get_data<'a, T>(key: &str) -> Option<T> where T: Deserialize<'a> {
    #[cfg(any(feature = "web", target_arch = "wasm32"))]
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

    #[cfg(any(feature = "native", not(target_arch = "wasm32")))]
    let value = {
        use std::fs::read_to_string;

        let key_path = get_path_from_key(&key);

        match read_to_string(&key_path) {
            Ok(string) => {
                let string = Box::leak(string.into_boxed_str());
                let val: T = from_str(string).expect(&format!("Failed to deserialize \"{}\" ", key));
                
                Some(val)
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    None
                },
                ErrorKind::PermissionDenied => panic!("Permission denied to access {:?}", key_path),
                other_err => panic!("Unknown error: {:?}", other_err), 
            }           
        }

    };

    value


}

pub fn write_data<T>(key: &str, value: T) where T: Serialize {
    #[cfg(any(feature = "web", target_arch = "wasm32"))]
    {
        let ron_config = PrettyConfig::new();
        let value = to_string_pretty(&value, ron_config).unwrap();

        js_write_data(key, value);
    };

    #[cfg(any(feature = "native", not(target_arch = "wasm32")))]
    {
        use std::fs::write;

        let key_path = get_path_from_key(&key);

        let ron_config = PrettyConfig::new();
        let value = to_string_pretty(&value, ron_config).unwrap();

        write(key_path, value).unwrap();

    };
}

pub fn delete_data(key: &str) -> Result<(), Option<std::io::Error>> {
    if check_key(key.clone()) {
        // Only try to delete an item if it exists
        #[cfg(any(feature = "native", not(target_arch = "wasm32")))]
        {
            use std::fs::remove_file;

            let key_path = get_path_from_key(&key);

            match remove_file(key_path) {
                Ok(_) => Ok(()),
                Err(e) => Err(Some(e)),

            }

        }

        #[cfg(any(feature = "web", target_arch = "wasm32"))]
        {
            js_delete_data(key);
            Ok(())

        }

    } else {
        // If the key doesn't exist, return an error
        Err(None)
    }

}

pub fn check_key(key: &str) -> bool {
    #[cfg(any(feature = "web", target_arch = "wasm32"))]
    let value = js_key_exists(key);

    #[cfg(any(feature = "native", not(target_arch = "wasm32")))]
    let value = {
        use std::fs::try_exists;

        let key_path = get_path_from_key(&key);
        try_exists(key_path).unwrap()
    };

    value
}

#[cfg(any(feature = "native", not(target_arch = "wasm32")))]
fn get_path_from_key(key: &str) -> std::path::PathBuf {
    use std::fs::create_dir_all;

    let proj_dirs = ProjectDirs::from("com", "William Batista",  "game").unwrap();
    let config_dir = proj_dirs.config_dir();

    // Just try to create the directory every time, it doesn't really matter if the directory already exists
    if let Err(error) = create_dir_all(&config_dir) {
        // Any error besides the directory already existing should panic
        match error.kind() {
            ErrorKind::AlreadyExists => (),
            e => panic!("Error creating config dir: {:?}", e),
        }

    }

    let key_path = config_dir.join(key);

    key_path
}
