extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::{Handlebars, RenderError, TemplateError};
use serde_json::value::Value;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};

const TEMPLATES: [&'static str; 2] = ["html5", "rust-cli"];

pub fn list_templates() {
    for template in &TEMPLATES {
        println!(" - {}", template);
    }
}

fn sanitize_rust_name(s: &str) -> String {
    s.replace("-", "_")
}

pub fn rust_cli_template(name: Option<&str>, description: Option<&str>, lib_name: Option<&str>, bin_name: Option<&str>) -> io::Result<()> {
    let name = if let Some(n) = name { n } else { "example" };

    let description = if let Some(d) = description { d } else { "TBD" };

    let lib_name = if let Some(ln) = lib_name {
        ln.to_owned()
    } else {
        sanitize_rust_name(name)
    };

    let bin_name = if let Some(bn) = bin_name {
        bn.to_owned()
    } else {
        sanitize_rust_name(name)
    };

    let template_data = json!({
                "name": &name,
                "description": &description,
                "lib-name": &lib_name,
                "bin-name": &bin_name
    });

    // TODO Custom error type that implements what io::Error needs
    let mut reg = Handlebars::new();
    // Register Templates
    reg.register_template_string(
        "Cargo.toml_template",
        include_str!("../templates/rust-cli/Cargo.toml"),
    ).expect("could not register Cargo.toml template");
    reg.register_template_string(
        "README.md_template",
        include_str!("../templates/rust-cli/README.md"),
    ).expect("could not register README.md template");
    reg.register_template_string(
        "lib.rs_template",
        include_str!("../templates/rust-cli/lib.rs"),
    ).expect("could not register lib.rs template");
    reg.register_template_string(
        "main.rs_template",
        include_str!("../templates/rust-cli/main.rs"),
    ).expect("could not render main.rs template");

    // Render Templates
    let cargo_toml_content = reg.render("Cargo.toml_template", &template_data).expect(
        "could not compile Cargo.toml template",
    );
    let readme_md_content = reg.render("README.md_template", &template_data).expect(
        "could not compile README.md template",
    );
    let lib_rs_content = reg.render("lib.rs_template", &template_data).expect(
        "could not compile lib.rs template",
    );
    let main_rs_content = reg.render("main.rs_template", &template_data).expect(
        "could not compile main.rs template",
    );
    let mut cwd = env::current_dir().expect(
        "current directory either doesn't exist or you don't have sufficient permissions.",
    );
    // Root
    cwd.push(name);

    fs::create_dir(&cwd)?;

    // Cargo.toml
    let mut cargo_toml_path = cwd.clone();
    cargo_toml_path.push("Cargo");
    cargo_toml_path.set_extension("toml");
    let mut cargo_toml_file = File::create(&cargo_toml_path)?;
    cargo_toml_file.write_all(cargo_toml_content.as_bytes())?;

    let mut readme_md_path = cwd.clone();
    readme_md_path.push("README");
    readme_md_path.set_extension("md");
    let mut readme_md_file = File::create(&readme_md_path)?;
    readme_md_file.write_all(readme_md_content.as_bytes())?;

    // `src` Directory
    cwd.push("src");
    fs::create_dir(&cwd)?;

    let mut lib_rs_path = cwd.clone();
    lib_rs_path.push("lib");
    lib_rs_path.set_extension("rs");
    let mut lib_rs_file = File::create(&lib_rs_path)?;
    lib_rs_file.write_all(lib_rs_content.as_bytes())?;

    let mut main_rs_path = cwd.clone();
    main_rs_path.push("main");
    main_rs_path.set_extension("rs");
    let mut main_rs_file = File::create(&main_rs_path)?;
    main_rs_file.write_all(main_rs_content.as_bytes())?;

    Ok(())
}

#[derive(Debug)]
pub struct Html5Error {
    pub render_error: Option<RenderError>,
    pub template_error: Option<TemplateError>,
}

/// Print my HTML5 template to stdout
pub fn html5_template(web_page_title: Option<&str>) -> Result<(), Html5Error> {
    let mut reg = Handlebars::new();
    match reg.register_template_string(
        "html5_template",
        include_str!("../templates/html5.html.hbs"),
    ) {
        Ok(_) => {
            let template_data: Value;
            match web_page_title {
                Some(title) => {
                    template_data = json!({
                        "title": &title
                    });
                }
                None => {
                    template_data = json!({});
                }
            }
            match reg.render("html5_template", &template_data) {
                Ok(result) => {
                    println!("{}", result);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("[ERROR] Failed to render HTML5 remplate: {:?}", e);
                    Err(Html5Error {
                        render_error: Some(e),
                        template_error: None,
                    })
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to register HTML5 template: {:?}", e);
            Err(Html5Error {
                render_error: None,
                template_error: Some(e),
            })
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn all_templates_runs() {
        list_templates();
        assert!(true);
    }
}
