use std::process;

use directories::ProjectDirs;

pub fn setup_logging() {
  if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

  pretty_env_logger::init();
}

pub fn setup_directories() -> ProjectDirs {
  let project_dirs = ProjectDirs::from("codes", "juancsu", "flatboat");

  if project_dirs.is_none() {
      error!("Unable to locate application folders");
      process::exit(1);
  } else {
      debug!("Located folders {:?}", &project_dirs);
  }

  let project_dirs = project_dirs.unwrap();
  let data_dir = project_dirs.data_dir();

  debug!("Data Directory: {:?}", data_dir);

  return project_dirs;
}

pub fn output_serialized(output: &impl serde::Serialize) {
    println!("{}", serde_json::to_string(&output).expect("JSON Serializer"));
}
