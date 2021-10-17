#[derive(Clone, Debug)]
pub struct ImageId {
    pub name: String,
    pub tag: String,
}

pub fn parse_image_id(s: &str) -> ImageId {
    match s.split_once(":") {
        Some((name, tag)) => {
            let name = normalize_image_name(name);

            ImageId {
                name,
                tag: tag.to_string(),
            }
        }
        None => {
            let name = normalize_image_name(&s);

            ImageId {
                name,
                tag: String::from("latest"),
            }
        }
    }
}

pub fn normalize_image_name(name: &str) -> String {
    if name.contains("/") {
        name.to_string()
    } else {
        format!("library/{}", name)
    }
}
