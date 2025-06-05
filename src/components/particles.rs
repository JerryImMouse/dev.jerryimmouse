use leptos::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};

#[component]
pub fn ParticlesBackground() -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if node_ref.get().is_some() {
            let timeout_callback = Closure::wrap(Box::new(|| {
                let script = format!(
                    r#"
                    if (typeof particlesJS !== 'undefined') {{
                        particlesJS.load('particles-js', './public/particles.json', function() {{
                            console.log('callback - particles.js config loaded');
                            window.dispatchEvent(new Event('resize'));
                        }});
                    }} else {{
                        console.warn('particles.js library not loaded');
                    }}
                    "#
                );
                let _ = web_sys::js_sys::eval(&script);
            }) as Box<dyn Fn()>);

            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_callback.as_ref().unchecked_ref(),
                100,
            );

            timeout_callback.forget();
        }
    });

    view! {
        <div
            id="particles-js"
            node_ref=node_ref
            class="fixed top-0 left-0 inset-0 z-0 w-full h-screen bg-slate-900"
        ></div>
    }
}
