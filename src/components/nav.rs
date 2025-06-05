use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 w-screen z-50 bg-slate-950/70 text-white shadow-md">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 py-4 flex flex-col sm:flex-row items-center justify-between gap-3 sm:gap-0">
                <h1 class="text-lg sm:text-xl font-bold whitespace-nowrap">"dev.jerryimmouse"</h1>

                <ul class="flex flex-wrap justify-center gap-3 text-sm sm:text-base text-center">
                    <li>
                        <a href="https://github.com/JerryImMouse" target="_blank" class="hover:underline">
                            "My GitHub"
                        </a>
                    </li>
                    <li>
                        <a href="#projects" class="hover:underline">
                            "My Projects"
                        </a>
                    </li>
                    <li>
                        <a href="" class="hover:underline">
                            "Home"
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
