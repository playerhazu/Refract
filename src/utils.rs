use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;

fn help() {}

fn build() {
    let user_env = env::var("USER").unwrap_or_default();

    let base_dir = format!("/home/{}/.refract", user_env);
    let selected_profile = fs::read_to_string(format!("{}/.profile", base_dir))
        .expect("Error trying to get current profile");

    let profile_dir = format!("{}/profiles/{}", base_dir, selected_profile);
    let linker_path = PathBuf::from(&profile_dir).join("linker");
    let linker_file = fs::File::open(&linker_path).expect("Error trying to open linker file");
    let reader = io::BufReader::new(linker_file);

    for line in reader.lines() {
        let line = line.unwrap_or("".to_string());

        if !line.is_empty() {
            let mut parts: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();

            parts[0] = format!("{}/{}", profile_dir, &parts[0]);

            if parts[1].starts_with("~") {
                parts[1] = format!("/home/{}/{}", user_env, &parts[1][2..]);
            }

            let path = Path::new(parts[1].as_str());

            if path.exists() && path.is_dir() && fs::metadata(path).is_ok() {
                fs::remove_dir_all(path).expect("Error trying to replace directory");
            }

            Command::new("cp")
                .arg("-r")
                .arg(parts[0].as_str().trim())
                .arg(parts[1].as_str().trim())
                .status()
                .expect("Error trying to link file or directory");
        }
    }
}

fn profile(new_profile: &String) {
    let base_dir = format!("/home/{}/.refract", env::var("USER").unwrap_or_default());
    let mut validation: bool = false;

    let path = format!("{}/profiles", base_dir);
    let paths = fs::read_dir(path).expect("Error trying to read profiles");

    for path in paths {
        let file_name = path.expect("Error trying to get entry").file_name();

        if file_name
            .to_str()
            .expect("Error trying to transform file name to string")
            == new_profile
        {
            validation = true;
            break;
        }
    }

    if validation {
        fs::write(format!("{}/.profile", base_dir), new_profile)
            .expect("Error trying to update profile");

        println!("Building new profile...");

        build();
    } else {
        println!("This profile is not available in your list of profiles.");
    }
}

fn profile_list() {
    let path = format!(
        "/home/{}/.refract/profiles",
        env::var("USER").unwrap_or_default()
    );

    println!("Current profiles:");

    let paths = fs::read_dir(path).expect("Error trying to read package directory");

    for path in paths {
        let file_name = path.expect("Error trying to get entry").file_name();
        println!("{:?}", file_name);
    }
}

fn update() {
    let user_env = env::var("USER").unwrap_or_default();
    let core_str = format!("/home/{}/.refract/core/Refract", user_env);
    let core_dir = Path::new(core_str.as_str());

    Command::new("git")
        .current_dir(core_dir)
        .arg("pull")
        .status()
        .expect("Error trying to clone repo");
}

fn patch(name: &String) {}

fn set_env(var: &String, value: &String) {}

fn install(route: &String) {}

fn remove(name: &String) {}

fn list() {
    let path = format!(
        "/home/{}/.refract/packages",
        env::var("USER").unwrap_or_default()
    );

    println!("Installed packages:");

    let paths = fs::read_dir(path).expect("Error trying to read package directory");

    for path in paths {
        let file_name = path.expect("Error trying to get entry").file_name();
        println!("{:?}", file_name);
    }
}

fn setup() {
    let home = env::var("USER").unwrap_or_default();
    let base_dir = format!("/home/{}/.refract", home);

    if !PathBuf::from(&base_dir).is_dir() {
        fs::create_dir_all(&base_dir).expect("Error trying to create base dir");
        println!("Base directory created at {}", base_dir);
    }

    fs::write(
        format!("{}/.environment", base_dir),
        format!(
            "PackagesDir: {}/packages\nProfilesDir: {}/profiles",
            base_dir, base_dir
        ),
    )
    .expect("Error trying to create environment file");

    fs::write(format!("{}/.version", base_dir), env!("CARGO_PKG_VERSION"))
        .expect("Error trying to create versihttps://github.com/playerhazu/Refract.giton file");

    fs::write(format!("{}/.profile", base_dir), "").expect("Error trying to create profile file");

    fs::write(
        format!("{}/.repo", base_dir),
        "https://github.com/playerhazu/Refract",
    )
    .expect("Error trying to create repo file");

    let directories = ["packages", "profiles"];
    let core_dir = format!("{}/core", base_dir);

    if !PathBuf::from(&core_dir).is_dir() {
        let repo = fs::read_to_string(Path::new(format!("/home{}/.refract/.repo", home).as_str()))
            .unwrap_or_default();

        fs::create_dir_all(&core_dir).expect("Error trying to create core dir");

        Command::new("git")
            .current_dir(&core_dir)
            .args(["clone", &repo])
            .status()
            .expect("Error trying to clone repository");
    }

    for value in directories.iter() {
        let new_dir = format!("{}/{}", base_dir, value);

        println!("Creating directory {}", &new_dir);
        fs::create_dir(&new_dir).expect("Error trying to create dir");
    }

    println!("Construction of the Refract environment has been completed...");
}

pub fn process_command(args: &[String]) {
    let command = &args
        .get(1)
        .map(String::as_str)
        .unwrap_or("help")
        .to_string();

    if command != "setup"
        && command != "help"
        && (!PathBuf::from(format!(
            "/home/{}/.refract",
            env::var("USER").unwrap_or_default()
        ))
        .is_dir()
            || !PathBuf::from(format!(
                "/home/{}/.refract/.repo",
                env::var("USER").unwrap_or_default()
            ))
            .is_file())
    {
        println!(
            "No Refract environment has been created; use the `setup` command or create it manually."
        );

        return;
    }

    match command.as_str() {
        "build" => build(),
        "setup" => setup(),
        "patch" => patch(&args[2]),
        "profile" => profile(&args[2]),
        "set-env" => set_env(&args[2], &args[3]),
        "install" => install(&args[2]),
        "update" => update(),
        "remove" => remove(&args[2]),
        "list" => list(),
        "profile-list" => profile_list(),
        "help" => help(),
        _ => help(),
    }
}
