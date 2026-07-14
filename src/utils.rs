use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn help() {}

fn build() {}

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
        fs::write(format!("{}/profile", base_dir), new_profile)
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

fn update() {}

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

    fs::create_dir_all(&base_dir).expect("Error trying to create base dir");
    println!("Base directory created at {}", base_dir);

    let directories = ["packages", "profiles", "temp"];

    for value in directories.iter() {
        fs::create_dir(format!("{}/{}", base_dir, value)).expect("Error trying to create dir");
    }

    fs::write(
        format!("{}/environment", base_dir),
        format!(
            "PackagesDir: {}/packages\nProfilesDir: {}/profiles",
            base_dir, base_dir
        ),
    )
    .expect("Error trying to create environment file");

    fs::write(format!("{}/version", base_dir), env!("CARGO_PKG_VERSION"))
        .expect("Error trying to create version file");

    fs::write(format!("{}/profile", base_dir), "").expect("Error trying to create profile file");
}

pub fn process_command(args: &[String]) {
    let command = &args
        .get(1)
        .map(String::as_str)
        .unwrap_or("help")
        .to_string();

    if command != "setup"
        && command != "help"
        && !PathBuf::from(format!(
            "/home/{}/.refract",
            env::var("USER").unwrap_or_default()
        ))
        .is_dir()
    {
        println!(
            "No Refract environment has been created; use the `setup` command or create it manually."
        );
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
