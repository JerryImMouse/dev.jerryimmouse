use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{DefaultToaster, Footer, NavBar, ParticlesBackground};

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <ParticlesBackground />
        <div class="relative z-0 flex flex-col min-h-screen text-gray-100 bg-transparent scroll-smooth">
            <NavBar />
            <Outlet />
            <DefaultToaster />
            <Footer />
        </div>
    }
}
