use std::path::{Path, PathBuf};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::sync::RwLock;
use toml;
use serde::Serialize;
use error::*;
use std::fs::OpenOptions;
use std::io::prelude::*;


lazy_static! {
    pub(crate) static ref CONFIG_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/config.toml");
        path
    };
    pub(crate) static ref DATA_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/data.toml");
        path
    };
    pub(crate) static ref APPS_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("installman_apps");
        path
    };
    pub(crate) static ref DESKTOP_FILES_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".local/share/applications");
        path
    };
    pub(crate) static ref BIN_SYMLINK_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("bin");
        path
    };

    pub static ref CONFIG : RwLock<Config> = {
        RwLock::new(Config::get())
    };
}


#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
}

/*
impl App {
    pub fn new<A: AsRef<String>> (n: A) -> Self {
        App{
        name: n.as_ref().to_os_string(),
        }
    }
}
*/

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    pub path: PathBuf,
    pub installed_apps: Vec<App>,
}

impl Data {
    fn init_store(&self) -> Result<()> {
        fs::create_dir_all(&*DATA_LOCATION.parent().unwrap())?;
        let mut f = File::create(&*DATA_LOCATION)?;
        f.write(&*toml::to_vec(self)?)?;
        Ok(())
    }

    pub fn store_data(&self) -> Result<()> {
        let mut file = ::std::fs::OpenOptions::new().write(true).open(&*DATA_LOCATION)?;
        file.write(&*toml::to_vec(self)?)?;
        /*
        match OpenOptions::new().append(true).open(LOG_FILE) {
            Ok(ref mut file) => {
                writeln!(
                    file,
                    "Hello!"
                ).is_ok();
            },
            Err(err) => { panic!("Failed to open log file: {}", err); }
        }
*/



        /*
                let mut file = File::open(&*DATA_LOCATION)?;
                file.write_all(&*toml::to_vec(self)?)?;
          */
        Ok(())
    }

    pub fn get_data<A: AsRef<Path>>(&self, path: A) -> Result<Data> {
        use std::fs::File;
        use std::io::Read;
        let mut data_file = File::open(path)?;
        let mut data_content = String::new();
        data_file.read_to_string(&mut data_content)?;
        let data: Data = toml::from_str(&data_content).unwrap();
        Ok(data)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub apps_location: PathBuf,
    pub data_location: PathBuf,
    pub desktop_files_location: PathBuf,
    pub bin_symlink_location: PathBuf,
}

impl Config {
    fn new<A: AsRef<Path>>(apps: A, data: A, desk: A, bins: A) -> Self {
        Config {
            apps_location: apps.as_ref().to_path_buf(),
            data_location: data.as_ref().to_path_buf(),
            desktop_files_location: desk.as_ref().to_path_buf(),
            bin_symlink_location: bins.as_ref().to_path_buf(),
        }
    }

    fn store() -> Result<()> {
        store_file(&*CONFIG, &*CONFIG_LOCATION);
        Ok(())
    }

    fn get() -> Config {
        fs::create_dir_all(&*CONFIG_LOCATION.parent().unwrap()).unwrap();
        match File::open(&*CONFIG_LOCATION) {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf);
                toml::from_str::<Config>(&buf).unwrap_or(Config::default())
            }
            Err(_) => {
                File::create(&*CONFIG_LOCATION).unwrap();
                let conf = Config::default();
                store_file(&conf, &*CONFIG_LOCATION).unwrap();
                conf
            }
        }
    }

    fn default() -> Self {
        Config::new(&*APPS_LOCATION,
                    &*DATA_LOCATION,
                    &*DESKTOP_FILES_LOCATION,
                    &*BIN_SYMLINK_LOCATION)
    }
}

fn store_file<S : Serialize>(s: &S, path: &Path) -> Result<()> {
    let mut file = OpenOptions::new().truncate(true).open(path)?;
    file.write(&*toml::to_vec(&s)?)?;
    Ok(())
}