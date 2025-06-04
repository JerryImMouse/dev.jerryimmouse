use leptos::prelude::StoredValue;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Project {
    pub title: StoredValue<String>,
    pub description: StoredValue<String>,
    pub url: StoredValue<String>,
    pub image: StoredValue<String>,
}
impl Project {
    pub fn new<S: Into<String>>(title: S, description: S, url: S, image: S) -> Self {
        Self {
            title: StoredValue::new(title.into()),
            description: StoredValue::new(description.into()),
            url: StoredValue::new(url.into()),
            image: StoredValue::new(image.into()),
        }
    }
}
