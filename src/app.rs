use crate::components::Page;
use crate::contexts::toaster::provide_toaster_context;
use crate::pages::{Home, HotFoundPage};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_toaster_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="public/favicon.ico"/>
        <Title text="dev.jerryimmouse" />
        <Router base="/dev.jerryimmouse">
            <Routes fallback=HotFoundPage>
                <ParentRoute path=path!("/") view=Page>
                    <Route path=path!("") view=Home />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
