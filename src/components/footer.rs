use leptos::prelude::*;

use crate::contexts::toaster::{Toast, ToastDuration, Toaster};

#[component]
pub fn Footer() -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let send_toast = move |_| {
        let toast = Toast::new(
            "hypercube is now in 3D >w<".to_string(),
            Some(ToastDuration::Long),
            Some(crate::contexts::toaster::ToastLevel::Info),
        );
        toaster.add_toast(toast);
    };

    view! {
        <footer class="flex z-10 flex-row gap-1 justify-center py-4 mt-auto text-center text-gray-300 shadow-inner bg-slate-950/70">
            <p class="text-sm">"Powered with ❤️ by Leptos"</p>
            <button on:click=send_toast>":3"</button>
        </footer>
    }
}
