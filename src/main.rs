use std::{fs,io};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde_yaml::{self};
use minijinja::{Environment};

mod links;
use links::{RustyLinks,MetaData};

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    // Credit: https://nick.groenen.me/notes/recursively-copy-files-in-rust/
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
     
    // Set last updated time to now.
    let now : DateTime<Utc> = Utc::now();
    let metadata : MetaData = MetaData { last_updated: now.to_rfc2822()};
    rusty_links.metadata = Some(metadata);

    rusty_links
}

fn render_links(rusty_links: RustyLinks) -> String {
    let main = std::fs::read_to_string("templates/main.html").expect("Could not find main.html");


    let mut env = Environment::new();
    env.add_template("main", &*main).unwrap();
    let tmpl = env.get_template("main").unwrap();
    tmpl.render(rusty_links).unwrap()
}

fn write_file(html: String) {
    if Path::new("./output").exists() {
      fs::remove_dir_all("./output").expect("could not remove directory");
    }
    fs::create_dir("./output").expect("Could not create output directory");
    // copy all files in static to output
    if Path::new("./output").exists() {
        copy_recursively("./static", "./output").expect("Could not copy static directory");
    }
    fs::write("./output/index.html", html).expect("Could not write to index.html");
}

fn main() {
    let rusty_links = load_links("links.yaml");
    let output : String = render_links(rusty_links);
    write_file(output);
}
