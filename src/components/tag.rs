use leptos::prelude::*;

#[component]
pub fn Tag(url: &'static str, text: &'static str, img: &'static str) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            class="px-3 py-1 bg-slate-700 rounded-full text-sm font-medium hover:bg-slate-500 hover:-translate-y-0.5 transition duration-150 flex flex-row gap-1"
        >
            <img src=img class="max-w-[20] max-h-[20]" />
            {text}
        </a>
    }
}
