use leptos::prelude::*;

#[component]
pub fn Tag(url: &'static str, text: &'static str, img: &'static str) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            class="flex flex-row gap-1 py-1 px-3 text-sm font-medium rounded-full transition duration-150 hover:-translate-y-0.5 bg-slate-700 hover:bg-slate-500"
        >
            <img src=img class="max-w-[20] max-h-[20]" />
            {text}
        </a>
    }
}
