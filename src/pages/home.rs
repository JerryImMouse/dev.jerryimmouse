use crate::components::{Greet, ProjectsShowcase};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Greet />
        <ProjectsShowcase />
    }
}
