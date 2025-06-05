use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 z-50 w-screen text-white shadow-md bg-slate-950/70">
            <div class="flex flex-col gap-3 justify-between items-center py-4 px-4 mx-auto max-w-7xl sm:flex-row sm:gap-0 sm:px-6">
                <h1 class="text-lg font-bold whitespace-nowrap sm:text-xl">"dev.jerryimmouse"</h1>

                <ul class="flex flex-wrap gap-3 justify-center text-sm text-center sm:text-base">
                    <li>
                        <a
                            href="https://github.com/JerryImMouse"
                            target="_blank"
                            class="hover:underline"
                        >
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
