extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::{Handlebars, RenderError, TemplateError};
use serde_json::value::Value;

#[derive(Debug)]
pub struct Html5Error {
    pub render_error: Option<RenderError>,
    pub template_error: Option<TemplateError>,
}

/// Print my HTML5 template to stdout
pub fn html5_template(web_page_title: Option<&str>) -> Result<(), Html5Error> {
    let mut reg = Handlebars::new();
    match reg.register_template_string("html5_template", include_str!("../templates/html5.html.hbs")) {
        Ok(_) => {
            let template_data: Value;
            match web_page_title {
                Some(title) => {
                    template_data = json!({"title": &title});
                }
                None => {
                    template_data = json!({});
                }
            }
            match reg.render("html5_template", &template_data) {
                Ok(result) => {
                    println!("{}", result);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("[ERROR] Failed to render HTML5 remplate: {:?}", e);
                    Err(Html5Error { render_error: Some(e), template_error: None })
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to register HTML5 template: {:?}", e);
            Err(Html5Error { render_error: None, template_error: Some(e) })
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
