use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    <Title text="Solid Ground Farm"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=HomePage />
      </Routes>
    </Router>
  }
}

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="flex flex-col gap-1 p-4">
      <p class="text-3xl font-semibold tracking-tight">"This site is under construction."</p>
      <p class="">"Check back later for more information!"</p>
    </div>
  }
}
