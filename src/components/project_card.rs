use crate::contexts::project::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectCard(_project: Project) -> impl IntoView {
    view! {
        <a
            href=_project.url.get_value()
            target="_blank"
            class="block p-6 bg-gray-800 rounded-lg shadow-md transition hover:bg-gray-700 hover:-translate-y-0.5"
        >
            <img
                src=_project.image.get_value()
                class="object-contain mx-auto mb-5 w-full h-auto rounded-3xl max-w-[300px]"
            />
            <h3 class="mb-2 text-xl font-semibold">{_project.title.get_value()}</h3>
            <p class="text-gray-400">{_project.description.get_value()}</p>
        </a>
    }
}
