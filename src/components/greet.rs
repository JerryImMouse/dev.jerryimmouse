use crate::components::Tag;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Greet() -> impl IntoView {
    view! {
        <Title text="dev.jerryimmouse" />
        <main class="flex flex-col items-center justify-center min-h-screen p-6 bg-transparent text-gray-100 relative z-10">
            <div class="animate-fade-in-up text-center space-y-4">
                <h1 class="text-5xl font-bold text-white drop-shadow-md">"JerryImMouse"</h1>
                <p class="text-xl text-white">"I build some shit in Rust just for fun!"</p>

                <div class="flex flex-wrap justify-center gap-3 mt-4">
                    <Tag url="https://youtube.com" img="./public/images/rust-logo-512x512.png" text="Rust"/>
                    <Tag url="https://youtube.com" img="./public/images/typescript-icon-2048x2048.png" text="TypeScript"/>
                    <Tag url="https://youtube.com" img="./public/images/csharp-logo-1820x2048.png" text="C#"/>
                </div>

                <a
                    href="#projects"
                    class="mt-6 inline-block px-6 py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white rounded-xl shadow transition duration-300 hover:-translate-y-0.5"
                >
                    "View My Projects"
                </a>
            </div>
            <div class="absolute -top-32 -left-32 w-[400px] h-[400px] bg-purple-600 rounded-full mix-blend-overlay filter blur-3xl opacity-30 animate-pulse-fast"></div>
            <div class="absolute -bottom-32 -right-32 w-[400px] h-[400px] bg-blue-600 rounded-full mix-blend-overlay filter blur-3xl opacity-30 animate-pulse-slow"></div>
        </main>
    }
}
