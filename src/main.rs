use crate::library::populate_library;
use crate::app::App;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use clap::Parser;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

mod model;
mod library;
mod db;
mod ui;
mod app;
mod audio;



#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    pub library: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub libraries: Vec<String>,
}

fn main() {
    db::initialize_database().expect("Failed to initialize database");

    let args = Args::parse();

    if let Some(library_args) = args.library {
        add_library(library_args);
        let libraries = load_config().libraries;

        for library in libraries {
        populate_library(&library);
    }
    }

    
    

    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = Arc::clone(&app);

    

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000)); // update every second
            let mut app = app_clone.lock().unwrap();
            app.update();
        }
    });

    ui::run(Arc::clone(&app)).expect("Failed to generate TUI");

    
    
}

pub fn add_library<P: AsRef<Path>>(library: P) {
    let library = library.as_ref()
        .canonicalize()
        .expect("Invalid path");

    let library = library.to_string_lossy().to_string();

    let mut config = load_config();

    if !config.libraries.contains(&library) {
        config.libraries.push(library);
        save_config(&config);
    }
}

fn config_path() -> PathBuf {
    let proj = ProjectDirs::from("com", "semuta", "semuta")
        .expect("Failed to get XDG dirs");

    let dir = proj.config_dir();
    std::fs::create_dir_all(dir).ok();

    dir.join("config.json")
}


fn load_config() -> Config {
    let path = config_path();

    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Config::default()
    }
}

fn save_config(config: &Config) {
    let path = config_path();

    let data = serde_json::to_string_pretty(config)
        .expect("Failed to serialize config");

    std::fs::write(path, data)
        .expect("Failed to write config");
}


/*
TODO:
- Make the UI more readable (make tracks bold)
- Add Mpris support
 */