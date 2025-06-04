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
            <a href="/" class="mt-6 inline-block px-6 py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white rounded-xl shadow transition duration-300 hover:-translate-y-0.5">"Go Back"</a>
        </div>
        <Footer />
    }
}
