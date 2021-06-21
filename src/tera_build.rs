use crate::db::Hits;
use tera::Context;
use tera::Tera;

pub fn tera_create(
    hits: Hits,
    file: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    let mut tera = Tera::default();

    let index = include_str!("site/templates/index.html");
    let header = include_str!("site/templates/partials/header.html");
    let footer = include_str!("site/templates/partials/footer.html");
    tera.add_raw_templates(vec![("index.html", index), ("partials/header.html", header), ("partials/footer.html", footer)])?;

    match Context::from_serialize(hits) {
        Ok(context) => match tera.render(file, &context) {
            Ok(output) => Ok(output),
            Err(e) => {
                log::error!("Error rendering {}: {}", file, e);
                Err(Box::new(e))
            }
        },
        Err(e) => {
            log::error!("Failed parsing posts from mongodb: {}", e);
            Err(Box::new(e))
        }
    }
}
