use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/zerops-leptos.css"/>

        // sets the document title
        <Title text="Leptos with Zerops"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Say Hello to Leptos on Zerops!"</h1>
        <h3>Deploy a rust website on zerops with 2 clicks</h3>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div class="buttons">
        <a class="primarybtn" href="https://zerops.io">"Discord Community"</a>
        <a class="secondarybtn" href="https://zerops.io">"Zerops Docs"</a>
        </div>
    }
}
