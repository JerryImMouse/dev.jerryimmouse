use leptos::prelude::*;

use crate::{components::ProjectCard, contexts::project::Project};

#[component]
pub fn ProjectsShowcase() -> impl IntoView {
    // TODO: Fetch projects dynamically from JSON or something
    let projects = RwSignal::new(vec![
        Project::new(
            "Ultor",
            "Discord bot written in Rust with serenity-lib for SS14 authorization & server management. Currently closed source, because im too shy.",
            "url",
            "./public/images/serenity-logo.png",
        ),
        Project::new(
            "dev.jerryimmouse",
            "Written in learning purposes. Simple site you're on now.",
            "url",
            "./public/images/sample-ferris.jpg",
        ),
        Project::new(
            "TypeAuthD",
            "SS14 & Discord OAuth2 \"linker\". Provides interface for the game to link discord and SS14 account written in TypeScript",
            "url",
            "./public/images/typeauthd-login-page.png",
        ),
        Project::new(
            "S.T.A.L.K.E.R 14",
            "SS14 build with original S.T.A.L.K.E.R motives written in C#",
            "url",
            "./public/images/stalker-logo.png",
        ),
    ]);

    view! {
        <section
            id="projects"
            class="scroll-mt-10 px-6 py-12 bg-transparent text-white min-h-screen relative z-10"
        >
            <h2 class="text-4xl font-bold mb-10 text-center">"My Projects"</h2>

            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8 max-w-7xl mx-auto">
                <For each=move || projects.get() key=move |x| *x let:project >
                    <ProjectCard _project=project />
                </For>
            </div>
        </section>
    }
}
