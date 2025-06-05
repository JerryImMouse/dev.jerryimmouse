use leptos::prelude::*;

use crate::components::{Footer, NavBar, ParticlesBackground};

#[component]
pub fn HotFoundPage() -> impl IntoView {
    view! {
        <ParticlesBackground />
        <NavBar />
        <div class="flex z-10 flex-col justify-center items-center h-screen text-center text-red-400 bg-transparent">
            <h1 class="mb-2 text-4xl font-bold">"404 - Hot Found"</h1>
            <p class="text-lg text-gray-300">"Requested page not found."</p>
        </div>
        <Footer />
    }
}
