use leptos::prelude::*;

use crate::components::{Footer, NavBar, ParticlesBackground};

#[component]
pub fn HotFoundPage() -> impl IntoView {
    view! {
        <ParticlesBackground />
        <NavBar />
        <div class="flex flex-col items-center justify-center h-screen text-center text-red-400 z-10 bg-transparent">
            <h1 class="text-4xl font-bold mb-2">"404 - Hot Found"</h1>
            <p class="text-lg text-gray-300">"Requested page not found."</p>
        </div>
        <Footer />
    }
}
