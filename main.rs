use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    project_type: String,
    path: std::path::PathBuf,
}

fn main() {
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
    for file_names in file_names_collection {
        let origin_file_path = format!(
            "{}/{}/{}",
            std::fs::canonicalize("./Dockerfiles/")
                .expect("could not read file")
                .into_os_string()
                .into_string()
                .unwrap(),
            &args.project_type,
            file_names[1]
        );
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
