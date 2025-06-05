use leptos::prelude::*;

use crate::{components::ProjectCard, contexts::project::Project};

#[component]
pub fn ProjectsShowcase() -> impl IntoView {
    // TODO: Fetch projects dynamically from JSON or something
    let projects = RwSignal::new(vec![
        Project::new(
            "Ultor",
            "Discord bot written in Rust with serenity-lib for SS14 authorization & server management. Currently closed source, because im too shy.",
            "https://shattereddisk.github.io/rickroll/rickroll.mp4",
            "./public/images/serenity-logo.png",
        ),
        Project::new(
            "dev.jerryimmouse",
            "Written in learning purposes. Simple site you're on now.",
            "https://github.com/JerryImMouse/dev.jerryimmouse",
            "./public/images/sample-ferris.jpg",
        ),
        Project::new(
            "TypeAuthD",
            "SS14 & Discord OAuth2 \"linker\". Provides interface for the game to link discord and SS14 account written in TypeScript",
            "https://github.com/JerryImMouse/typeauthd",
            "./public/images/typeauthd-login-page.png",
        ),
        Project::new(
            "S.T.A.L.K.E.R 14",
            "SS14 build with original S.T.A.L.K.E.R motives written in C#",
            "https://github.com/stalker14-project/stalker-14",
            "./public/images/stalker-logo.png",
        ),
    ]);

    view! {
        <section
            id="projects"
            class="relative z-10 py-12 px-4 min-h-screen text-white bg-transparent sm:px-6 scroll-mt-10"
        >
            <h2 class="mb-10 text-3xl font-bold text-center sm:text-4xl">"My Projects"</h2>

            <div class="grid grid-cols-1 gap-8 mx-auto max-w-7xl sm:grid-cols-2 lg:grid-cols-3">
                <For each=move || projects.get() key=move |x| *x let:project>
                    <ProjectCard _project=project />
                </For>
            </div>
        </section>
    }
}
