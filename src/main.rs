use std::{env, error::Error, fs, path::{Path, PathBuf}};

use std::process::Command;

mod terminal;
use clap::{Parser};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Parser, Debug)]
#[command(name = "setrcomp")]
struct Args {
    extension: Option<String>,
    template: Option<String>,
    filename: Option<String>
}

fn get_template_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
    }

    if let Ok(path) = std::env::var("SETRCOMP_TOOLS") {
        return PathBuf::from(path).join("templates");
    }

    if let Some(dir) = dirs::data_dir() {
        return dir.join("setrcomp/templates");
    }

    panic!("doesnt exist a directory for the data")
}

fn get_extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .map(|s| s.to_string_lossy().to_string())
}

fn find_and_copy_template(
    extension: &str,
    template_file: &str,
    filename: &str
) -> Result<(), Box<dyn Error>> {

    let template_dir = get_template_dir();
    let current_dir = env::current_dir()?;

    let from = template_dir
        .join(extension)
        .join(format!("{}.{}",template_file, extension));

    fs::create_dir(filename)?;
    let to = current_dir.join(filename).join(format!("main.{}",extension));

    println!("from: {:?}", from);
    println!("to: {:?}", to);
    fs::copy(from,to)?;

    Ok(())
}

fn list_extensions() -> Result<Vec<String>, Box<dyn Error>> {
    let dir = get_template_dir();

    let mut extensions = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().
                unwrap().to_string_lossy().to_string();
            extensions.push(name);
        }
    }

    Ok(extensions)
}

fn list_templates(extension: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let template_extension_dir = get_template_dir().join(extension);

    let mut templates = Vec::new();

    for entry in fs::read_dir(template_extension_dir)? {
        let entry = entry?;

        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                    templates.push(stem.to_string_lossy().to_string());
            }
        }
    }

    Ok(templates)
}

fn resolve_template_file(
    extension: &str,
    template_name: &str
) -> Result<String, Box<dyn Error>> {

    let dir = get_template_dir().join(extension);

    for entry in fs::read_dir(dir)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                if stem == template_name {
                    return Ok(
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    );
                }
            }
        }
    }

    Err(format!("Template '{}' doesnt exist", template_name).into())
}

fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let args = Args::parse();

    let extension = match args.extension {
        Some(ext) => ext,
        None => {
            let extensions = list_extensions()?;
            let idx = terminal::options(&extensions)?;
            extensions[idx].clone()
        }
    };

    let templates = list_templates(&extension)?;
    if templates.is_empty() {
        return Err("no templates".into());
    }

    let template = match args.template {
        Some(t) => t,
        None => {
            let idx = terminal::options_template(&templates, &extension)?;
            templates[idx].clone()
        }
    };

    let real_template = resolve_template_file(&extension, &template)?;

    let filename = match args.filename {
        Some(f) => f,
        None => {
            let ext = get_extension(&real_template)
            .unwrap_or_else(|| "".to_string());

            let suggestion = if ext.is_empty() {
                "main".to_string()
            } else {
                format!("main.{}",ext)
            };

            let input = terminal::input_prompt("dirname:")?;
            if input.is_empty() {
                suggestion
            } else  {
                input
            }
        }
    };
    // TODO: better errors and (see the templates with 's')
    find_and_copy_template(&extension,&template, &filename)?;

    // #[cfg(debug_assertions)]
    // println!("{:?}", args);
    // #[cfg(debug_assertions)]
    // println!("{:?}", get_template_dir());

    disable_raw_mode()?;
    Ok(())
}

fn main() {
    if let Err(err) =  run() {
        eprintln!("{}", err);

    }
}
