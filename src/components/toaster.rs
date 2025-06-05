use crate::contexts::toaster::{Toast, ToastLevel, Toaster};
use leptos::prelude::*;

#[component]
pub fn Toaster<V, DV, IV, WV, EV>(
    default_view: DV,
    info_view: IV,
    warning_view: WV,
    error_view: EV,
) -> impl IntoView
where
    V: IntoView + 'static,
    DV: Fn(Toast) -> V + 'static + Copy + Send + Sync,
    IV: Fn(Toast) -> V + 'static + Copy + Send + Sync,
    WV: Fn(Toast) -> V + 'static + Copy + Send + Sync,
    EV: Fn(Toast) -> V + 'static + Copy + Send + Sync,
{
    let toaster = expect_context::<Toaster>();
    view! {
        <div class="fixed right-4 bottom-5 z-50 w-full max-w-md px-4 md:px-0 space-y-2">
            <For
                each=move || toaster.toasts.get() key=|toast| *toast let:toast
            >
                {
                    set_timeout(
                        move || toaster.remove_toast(&toast),
                        toast.duration.unwrap_or_default().into()
                    );
                    match toast.level.unwrap_or_default() {
                        ToastLevel::Normal => default_view(toast),
                        ToastLevel::Info => info_view(toast),
                        ToastLevel::Warn => warning_view(toast),
                        ToastLevel::Error => error_view(toast),
                    }
                }
            </For>
        </div>
    }
}

#[component]
pub fn DefaultToaster() -> impl IntoView {
    view! {
        <Toaster
            default_view=move |toast: Toast| {
                view! {
                    <div class="bg-slate-700 text-white px-4 py-2 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }.into_any()
            }
            info_view=move |toast: Toast| {
                view! {
                    <div class="bg-blue-500 text-white px-4 py-2 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }.into_any()
            }
            warning_view=move |toast: Toast| {
                view! {
                    <div class="bg-yellow-400 text-black px-4 py-2 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }.into_any()
            }
            error_view=move |toast: Toast| {
                view! {
                    <div class="bg-red-600 text-white px-4 py-2 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }.into_any()
            }
        />
    }
}
