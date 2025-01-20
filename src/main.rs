use clap::{Args, Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "bump")]
#[command(version = "0.1")]
#[command(about = "bump pubspec vresion", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "patch")]
    Patch(CmdArgs),

    #[command(name = "minor")]
    Minor(CmdArgs),

    #[command(name = "major")]
    Major(CmdArgs),
}

#[derive(Args)]
pub struct CmdArgs {
    #[arg(default_value = "./pubspec.yaml")]
    pub file_path: String,
}

enum VersionType {
    Patch,
    Minor,
    Major,
}

#[derive(Debug)]
struct Version {
    pub patch: u16,
    pub minor: u8,
    pub major: u8,
    pub code: u32,
}

impl TryFrom<String> for Version {
	type Error = String;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        let split1 = str.split("+").collect::<Vec<&str>>();
        let version = split1.get(0);
        let code = split1.get(1);

        if version.is_none()
            || code.is_none()
            || version.unwrap().len() == 0
            || code.unwrap().len() == 0
        {
            return Err("invalid version string".to_string());
        }

        let split2 = version.unwrap().split(".").collect::<Vec<&str>>();
        let major = split2.get(0);
        let minor = split2.get(1);
        let patch = split2.get(2);

        if major.is_none()
            || minor.is_none()
            || patch.is_none()
            || major.unwrap().len() == 0
            || minor.unwrap().len() == 0
            || patch.unwrap().len() == 0
        {
            return Err("invalid version string".to_string());
        }

        let parsed_code = code.unwrap().parse::<u32>();
        if parsed_code.is_err() {
            return Err("invalid version code".to_string());
        }

        let parsed_major = major.unwrap().parse::<u8>();
        if parsed_major.is_err() {
            return Err("invalid major version".to_string());
        }

        let parsed_minor = minor.unwrap().parse::<u8>();
        if parsed_minor.is_err() {
            return Err("invalid minor version".to_string());
        }

        let parsed_patch = patch.unwrap().parse::<u16>();
        if parsed_patch.is_err() {
            return Err("invalid patch version".to_string());
        }

        Ok(Version {
            patch: parsed_patch.unwrap(),
            minor: parsed_minor.unwrap(),
            major: parsed_major.unwrap(),
            code: parsed_code.unwrap(),
        })
    }
}

impl ToString for Version {
	fn to_string(&self) -> String {
		format!(
			"{}.{}.{}+{}",
			self.major, self.minor, self.patch, self.code
		)
	}
}

impl Version {
	pub fn increment_patch(&mut self) {
		self.patch += 1;
		self.code += 1;
	}

	pub fn increment_minor(&mut self) {
		self.minor += 1;
		self.code += 1;
		self.patch = 0;
	}

	pub fn increment_major(&mut self) {
		self.major += 1;
		self.code += 1;
		self.minor = 0;
		self.patch = 0;
	}
}

fn get_version_str_from_file_path(file_path: &String) -> Result<String, String> {
	let file = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

	let version_line = file.lines().find(|line| line.starts_with("version: ")).ok_or("version not found")?;

	let payload = version_line.split(": ")
		.collect::<Vec<&str>>()
		.get(1)
		.map(|v| v.to_string())
		.ok_or("version not found")?;
	
	return Ok(payload);
}

fn replace_version_payload(file_path: &String, payload: Version) -> Result<(), String> {
	let file = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
	let mut lines = file.lines().collect::<Vec<&str>>();

	let version_line_idx = lines.iter().position(|line| line.starts_with("version: ")).ok_or("version not found")?;
	let new_line = format!("version: {}", payload.to_string());
	lines[version_line_idx] = &new_line;

	let new_file = lines.join("\n");
	fs::write(file_path, new_file).map_err(|e| e.to_string())?;
	Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Patch(CmdArgs { file_path }) => {
            let version_str = get_version_str_from_file_path(&file_path)?;
			let mut version = Version::try_from(version_str).map_err(|e| e.to_string())?;

			version.increment_patch();
			let vstr = version.to_string();

			replace_version_payload(&file_path, version)?;
			println!("new version is {}", vstr);
        }
        Command::Minor(CmdArgs { file_path }) => {
            let version_str = get_version_str_from_file_path(&file_path)?;
			let mut version = Version::try_from(version_str).map_err(|e| e.to_string())?;

			version.increment_minor();
			let vstr = version.to_string();

			replace_version_payload(&file_path, version)?;
			println!("new version is {}", vstr);
        }
        Command::Major(CmdArgs { file_path }) => {
			let version_str = get_version_str_from_file_path(&file_path)?;
			let mut version = Version::try_from(version_str).map_err(|e| e.to_string())?;

			version.increment_major();
			let vstr = version.to_string();

			replace_version_payload(&file_path, version)?;
			println!("new version is {}", vstr);
        }
    }
	Ok(())
}
