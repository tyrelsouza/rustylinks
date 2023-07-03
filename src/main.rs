use std::fs;
use std::io;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_yaml::{self};
use tera::{Tera,Context};


#[derive(Serialize, Deserialize, Debug)]
pub struct RustyLinks {
    config: Config,
    links: Vec<Links>,
    metadata: Option<MetaData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    last_updated: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    title: String,
    name: String,
    description: String,
    avatar: String,
    background: String,
    background_opacity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    title: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    text: String,
    icon: String,
    href: Option<String>,
    copy: Option<String>,
}


pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    // https://nick.groenen.me/notes/recursively-copy-files-in-rust/
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


fn load_links(file_name: &str) -> RustyLinks {
    let links_yaml = std::fs::File::open(file_name).expect("Could not find file");
    let mut rusty_links: RustyLinks = serde_yaml::from_reader(links_yaml).expect("Could not read values");
    let now : DateTime<Utc> = Utc::now();
    let metadata : MetaData = MetaData { last_updated: now.to_rfc2822()};
    rusty_links.metadata = Some(metadata);

    rusty_links
}

fn render_links(rusty_links: RustyLinks) -> String {
    let context: Context = Context::from_serialize(&rusty_links).expect("Could not parse");
    let tera = match Tera::new("templates/*.tera") {
      Ok(t) => t,
      Err(e) => {
          println!("Parsing error(s): {}", e);
          ::std::process::exit(1);
      }
    };
    tera.render("main.tera", &context).expect("Could not parse")
}

fn write_file(html: String ) {
    fs::remove_dir_all("./output").expect("could not remove directory");
    fs::create_dir("./output").expect("Could not create output directory");
    copy_recursively("./static", "./output").expect("Could not copy static directory");
    fs::write("./output/index.html", html).expect("Could not write to index.html");
}

fn main() {
    let rusty_links = load_links("links.yaml");
    let output : String = render_links(rusty_links);
    write_file(output);
}
