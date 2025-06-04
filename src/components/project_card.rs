use crate::contexts::project::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectCard(_project: Project) -> impl IntoView {
    view! {
        <a
            href=_project.url.get_value()
            target="_blank"
            class="block p-6 bg-gray-800 rounded-lg shadow-md hover:bg-gray-700 transition hover:-translate-y-0.5"
        >
            <img src=_project.image.get_value() class="w-full max-w-[300px] h-auto mx-auto rounded-3xl mb-5 object-contain"/>
            <h3 class="text-xl font-semibold mb-2">{_project.title.get_value()}</h3>
            <p class="text-gray-400">{_project.description.get_value()}</p>
        </a>
    }
}
