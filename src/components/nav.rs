use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 w-full z-50 bg-slate-950/70 text-white px-6 py-4 shadow-md">
            <div class="container mx-auto flex justify-between items-center">
                <h1 class="text-xl font-bold">"dev.jerryimmouse"</h1>
                <ul class="flex space-x-2 gap-2">
                    <li><a href="https://github.com/JerryImMouse" target="_blank" class="hover:underline">"My GitHub"</a></li>
                    <li><a href="#projects" class="hover:underline">"My Projects"</a></li>
                    <li><a href="" class="hover:underline">"Home"</a></li>
                </ul>
            </div>
        </nav>
    }
}
