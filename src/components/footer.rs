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
        <footer class="bg-slate-950/70 text-gray-300 text-center py-4 mt-auto shadow-inner flex flex-row justify-center gap-1 z-10">
            <p class="text-sm">"Powered with ❤️ by Leptos"</p>
            <button
                on:click=send_toast
            >":3"</button>
        </footer>
    }
}
