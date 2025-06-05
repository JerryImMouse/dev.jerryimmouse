use crate::components::Tag;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Greet() -> impl IntoView {
    view! {
        <Title text="dev.jerryimmouse" />
        <main class="flex relative z-10 flex-col justify-center items-center p-6 min-h-screen text-center text-gray-100 bg-transparent">
            <div class="mx-auto space-y-4 w-full max-w-xl animate-fade-in-up">
                <h1 class="text-4xl font-bold text-white sm:text-5xl drop-shadow-md">
                    "JerryImMouse"
                </h1>
                <p class="text-lg text-white sm:text-xl">
                    "I build some shit in Rust just for fun!"
                </p>

                <div class="flex flex-wrap gap-3 justify-center mt-4">
                    <Tag
                        url="https://www.rust-lang.org/"
                        img="./public/images/rust-logo-512x512.png"
                        text="Rust"
                    />
                    <Tag
                        url="https://www.typescriptlang.org/"
                        img="./public/images/typescript-icon-2048x2048.png"
                        text="TypeScript"
                    />
                    <Tag
                        url="https://dotnet.microsoft.com/en-us/"
                        img="./public/images/csharp-logo-1820x2048.png"
                        text="C#"
                    />
                </div>

                <a
                    href="#projects"
                    class="inline-block py-3 px-6 mt-6 text-white bg-blue-600 rounded-xl shadow transition duration-300 hover:bg-blue-500 hover:-translate-y-0.5 active:bg-blue-700"
                >
                    "View My Projects"
                </a>
            </div>
            <div class="absolute -top-32 -left-32 bg-purple-600 rounded-full opacity-30 w-[300px] h-[300px] mix-blend-overlay filter blur-3xl animate-pulse-fast sm:w-[400px] sm:h-[400px]"></div>
            <div class="absolute -right-32 -bottom-32 bg-blue-600 rounded-full opacity-30 w-[300px] h-[300px] mix-blend-overlay filter blur-3xl animate-pulse-slow sm:w-[400px] sm:h-[400px]"></div>
        </main>
    }
}
