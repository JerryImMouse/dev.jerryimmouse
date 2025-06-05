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
        <div class="fixed right-4 bottom-5 z-50 px-4 space-y-2 w-full max-w-md md:px-0">
            <For each=move || toaster.toasts.get() key=|toast| *toast let:toast>
                {
                    set_timeout(
                        move || toaster.remove_toast(&toast),
                        toast.duration.unwrap_or_default().into(),
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
                    <div class="py-2 px-4 text-white rounded shadow bg-slate-700">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }
                    .into_any()
            }
            info_view=move |toast: Toast| {
                view! {
                    <div class="py-2 px-4 text-white bg-blue-500 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }
                    .into_any()
            }
            warning_view=move |toast: Toast| {
                view! {
                    <div class="py-2 px-4 text-black bg-yellow-400 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }
                    .into_any()
            }
            error_view=move |toast: Toast| {
                view! {
                    <div class="py-2 px-4 text-white bg-red-600 rounded shadow">
                        <span>{toast.message.get_value()}</span>
                    </div>
                }
                    .into_any()
            }
        />
    }
}
