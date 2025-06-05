use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{DefaultToaster, Footer, NavBar, ParticlesBackground};

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen text-gray-100 bg-transparent scroll-smooth">
            <ParticlesBackground />
            <NavBar />
            <Outlet />
            <DefaultToaster />
            <Footer />
        </div>
    }
}
