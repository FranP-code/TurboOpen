use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::{
    fs::{self, canonicalize},
    process::Command,
};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    project_type: Option<String>,
}

fn main() {
    let docker_files_source = canonicalize("./Dockerfiles/")
        .expect("could not read file")
        .into_os_string()
        .into_string()
        .unwrap();
    let args = Cli::parse();
    let path = std::fs::canonicalize(args.path)
        .expect("could not read file")
        .into_os_string()
        .into_string()
        .unwrap();
    let file_names_collection = [
        ["Dockerfile", "Dockerfile"],
        ["docker-compose.yml", "docker-compose.yml"],
    ];
    let items = fs::read_dir(docker_files_source.clone())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            entry
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    let mut project_type: String = "".to_owned();
    if args.project_type.is_some() && items.contains(&args.project_type.as_ref().unwrap()) {
        project_type = args.project_type.as_ref().unwrap().to_string();
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What type is your project?")
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());
        match selection {
            Ok(Some(index)) => {
                project_type = items[index].clone();
            }
            Ok(None) => println!("User did not select anything"),
            Err(_) => todo!(),
        }
    }
    for file_names in file_names_collection {
        let origin_file_path =
            format!("{}/{}/{}", docker_files_source, project_type, file_names[1]);
        let destiny_path = format!("{}/{}", path, file_names[1]);
        std::fs::copy(origin_file_path, destiny_path).ok();
    }
    let mut child = Command::new("docker-compose")
        .arg("-f")
        .arg(format!("{}/{}", path, "docker-compose.yml"))
        .arg("up")
        .spawn()
        .expect("failed to execute docker-compose");
    let status = child.wait().expect("failed to wait for child process");
    if status.success() {
        println!("Docker Compose command executed successfully");
    } else {
        eprintln!("Docker Compose command failed with error code {}", status);
    }
}
