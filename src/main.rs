use std::io::{stdin, stdout, Write};
use std::{fs, process};
use structopt::StructOpt;
use env_logger;
use log::debug;
use toml;
use serde_json::Value;
use regex::Regex;
use colored::*;

type Result<T> = std::result::Result<T, anyhow::Error>;

/// Increment Version Flags
#[derive(StructOpt, Debug)]
#[structopt(name = "increment_version")]
struct Opt {
    /// Increase Version by Major tick
    #[structopt(short="m", long="major")]
    major: bool,
    /// Increase Version by Minor tick
    #[structopt(short="n", long="minor")]
    minor: bool,
    /// Increase Version by Patch tick
    #[structopt(short="p", long="patch")]
    patch: bool,
    /// Look for .toml in subfolder(s)
    #[structopt(short="sf", long="sub-folder")]
    sub_folder: Option<String>,
    /// Set Version to
    #[structopt(short="v", long="version")]
    version: Option<String>,
    /// Look for Version.toml instead of Cargo.toml
    #[structopt(short="a", long="version-toml")]
    version_toml: bool,
    /// Automatically tag as v{version}, commit, and push to git remote
    #[structopt(short="t", long="tag")]
    tag: bool,
    /// Shows debugging/stderr
    #[structopt(short="d", long="debug")]
    debug: bool,    
    /// Doesn't check for updates
    #[structopt(long="no-update")]
    no_update: bool,
}

enum Bump {
    Major,
    Minor,
    Patch,
    Custom(Version)
}

fn check_for_update(_current_version: &str, debug: bool) -> Result<()> {
    let resp = process::Command::new("curl")
        .arg("-s")
        .arg("https://api.github.com/repos/billy-sheppard/increment_version/releases/latest")
        .output()
    ?;

    let response: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&resp.stdout))?;
    
    // let current_version = Version::from_str(current_version)?;
    let most_recent_version = Version::from_str(response["tag_name"].as_str().unwrap())?;
    
    let current_dir = std::env::current_dir()?;
    let current_path = current_dir.to_str().unwrap();
    let exe_dir = std::env::current_exe()?;
    let exe_path = exe_dir.to_str().unwrap();

    let mut input = String::new();
    println!("{} There is a newer version of Increment Version available (v{})", "[INFO]".cyan(), most_recent_version.to_string());
    println!("{} Would you like to download it? (y/n)", "[INFO]".cyan());

    let _ = stdout().flush();
    stdin().read_line(&mut input)?;

    input = input.replace("\n", "");

    if input == "y" || input == "yes" {
        println!("{} This will download to your current directory.", "[INFO]".cyan());
        debug!("{} Current Dir: {}", "[DEBUG]".purple(), current_path);
        println!("{} Updating...", "[INFO]".cyan());
        run_cmd(&["curl", "-L", "-o", &format!("{}/increment_version", current_path), "https://github.com/billy-sheppard/increment_version/releases/latest/download/increment_version"], Color::Cyan, debug);
        println!("{} To overwrite your binary run the following command:", "[INFO]".cyan());
        println!("      {} {}/increment_version {}", "sudo cp", current_path, exe_path);
        process::exit(0);
    }
    else if input == "n" || input == "no" {
        println!("{} Skipping, not updating.", "[INFO]".cyan());
    }
    else {
        println!("{} Invalid input passed, not updating.", "[INFO]".cyan());
    }

    Ok(())
}

fn main() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("{}{}{}", "-- Increment Cargo.toml Version: v".cyan(), current_version.cyan(), " --".cyan());

    env_logger::init();
    let flags = Opt::from_args();
    debug!("Flags: \n{:#?}", flags);

    if !flags.no_update {
        check_for_update(current_version, flags.debug)?;
    };

    let version_flags: [bool; 4] = [
        flags.major,
        flags.minor,
        flags.patch,
        match flags.version {
            Some(_) => true,
            None => false
        }
    ];
    let version_flags: Vec<&bool> = version_flags.iter().filter(|f| f == &&true).collect();

    if version_flags.len() > 1 {
        println!("{} More than one version bump flag entered!", "[ERROR]".red());
        std::process::exit(1);
    };

    let sub_folder = 
        match flags.sub_folder {
            Some(sf) => format!("{}/", sf),
            None => "".into()
    };

    let bump = 
        if flags.major {
            Bump::Major
        }
        else if flags.minor {
            Bump::Minor
        }
        else if flags.patch {
            Bump::Patch
        }
        else if flags.version.is_some() {
            let validated_version = flags.version.unwrap();
            Bump::Custom(Version::from_str(&validated_version)?)
        }
        else {
            println!("{} No version bump argument passed!", "[ERROR]".red());
            std::process::exit(1);
    };
    let toml_path;
    let new_toml;
    // Version.toml or Cargo.toml
    if flags.version_toml {
        toml_path = format!("{}Version.toml", sub_folder);
        new_toml = update_toml(&toml_path, bump)?; 
        fs::write(&toml_path, new_toml.0)?
    }
    else {
        toml_path = format!("{}Cargo.toml", sub_folder);
        new_toml = update_toml(&toml_path, bump)?;
        fs::write(&toml_path, new_toml.0)?
    };

    if flags.tag {
        run_cmd(&["cargo", "check"], Color::Yellow, flags.debug);

        run_cmd(&["git", "add", &toml_path], Color::Blue, flags.debug);

        if !flags.version_toml {
            run_cmd(&["git", "add", &format!("{}Cargo.lock", sub_folder)], Color::Blue, flags.debug);
        };

        run_cmd(&["git", "commit", "-m", &format!("v{}", new_toml.1)], Color::Blue, flags.debug);

        run_cmd(&["git", "tag", &format!("v{}", new_toml.1)], Color::Blue, flags.debug);

        run_cmd(&["git", "push"], Color::Blue, flags.debug);

        run_cmd(&["git", "push", "--tags"], Color::Blue, flags.debug);
    };

    Ok(())
}

fn run_cmd(cmd: &[&str], term_col: Color, debug: bool) {
    let command = process::Command::new(&cmd[0])
        .args(&cmd[1..])
    .stdout(process::Stdio::piped())
    .stderr(process::Stdio::piped())
    .output()
    .unwrap();
    let stdout = String::from_utf8_lossy(&command.stdout);
    let stdout: Vec<&str> = stdout.lines().collect();
    let stderr = String::from_utf8_lossy(&command.stderr);
    let stderr: Vec<&str> = stderr.lines().collect();
    let col = match term_col {
        Color::Green => format!("[{}]", cmd[0].to_uppercase()).green(),
        Color::Red => format!("[{}]", cmd[0].to_uppercase()).red(),
        Color::Blue => format!("[{}]", cmd[0].to_uppercase()).blue(),
        Color::Yellow => format!("[{}]", cmd[0].to_uppercase()).yellow(),
        Color::Cyan => format!("[{}]", cmd[0].to_uppercase()).cyan(),
        _ => format!("[{}]", cmd[0].to_uppercase()).white(),
    };
    stdout.iter().map(|l| {
        println!("{} {}", col, l)
    })
    .for_each(drop);
    if debug {
        stderr.iter().map(|l| {
            println!("{} {}", col, l)
        })
        .for_each(drop);
    };
}

fn update_toml(file_path: &str, bump: Bump) -> Result<(String, String)> {    
    let toml_file = match fs::read_to_string(file_path) {
        Ok(f) => f,
        Err(e) => {
            debug!("{}", e.to_string());            
            println!("{} {} does not exist!", "[ERROR]".red(), file_path);
            std::process::exit(1);
        }
    };
    let toml: Value = toml::from_str(&toml_file)?;
    let new_ver = bump_version(bump, toml.clone())?;
    let toml_file_lines: Vec<&str> = toml_file.lines().into_iter().collect();
    let new_toml: Vec<String> = toml_file_lines.into_iter().map(|l| {
        if l.starts_with("version") {
            format!("version = \"{}\"", new_ver)
        }   
        else {
            l.into()
        }
    }).collect();
    println!("{} Updated {} from {} to {}.", "[INFO]".green(), file_path, toml["package"]["version"].as_str().unwrap().replace("\"", ""), new_ver);

    Ok((new_toml.join("\n"), new_ver))
}

#[derive(Clone, Debug)]
struct Version {
    major: i64,
    minor: i64,
    patch: i64,
    prerelease: Option<String>,
    build_metadata: Option<String>
}
impl Version {
    fn to_string(&self) -> String {
        match (&self.prerelease, &self.build_metadata) {
            (Some(pr), Some(bm)) => format!("{}.{}.{}-{}+{}", self.major, self.minor, self.patch, pr, bm),
            (Some(pr), None) => format!("{}.{}.{}-{}",  self.major, self.minor, self.patch, pr),
            _ => format!("{}.{}.{}", self.major, self.minor, self.patch),
        }
        
    }

    fn from_str(s: &str) -> Result<Self> {
        let s = s.replace("v", "");
        let re_semver: Regex = Regex::new(r#"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#).unwrap();
        let mut ver = Vec::new();
        for cap in re_semver.captures_iter(&s) {
            ver.push(Version {
                major: cap.name("major").unwrap().as_str().parse::<i64>()?,
                minor: cap.name("minor").unwrap().as_str().parse::<i64>()?,
                patch: cap.name("patch").unwrap().as_str().parse::<i64>()?,
                prerelease: match cap.name("prerelease"){
                    Some(c) => Some(c.as_str().into()),
                    None => None
                },
                build_metadata: match cap.name("buildmetadata"){
                    Some(c) => Some(c.as_str().into()),
                    None => None
                },
            })
        };
        Ok(ver[0].clone())
    }
}

fn bump_version(bump: Bump, toml: Value) -> Result<String> {
    let old_version_string = toml["package"]["version"].clone();
    let mut old_version = Version::from_str(old_version_string.as_str().unwrap())?;
    let new_version: Version = 
        match bump {
            Bump::Major => {
                old_version.major += 1;
                old_version
            },
            Bump::Minor => {
                old_version.minor += 1;
                old_version
            },
            Bump::Patch => {
                old_version.patch += 1;
                old_version
            },
            Bump::Custom(ver) => {
                ver
            },
        };
    Ok(new_version.to_string())
}