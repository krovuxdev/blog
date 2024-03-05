use chrono::Datelike;
use leptos::{component, view, Children, IntoView};

use crate::{
    components::Header,
    meta::{Head, Html},
};

fn get_year() -> i32 {
    chrono::Utc::now().year()
}

#[component]
// This is a common Layout component that will be used by all pages.
pub fn Layout(
    #[prop(into, default=format!("Blog de Rust Lang en Español {}", get_year()))] title: String,
    slug: String,
    #[prop(into, default="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust.".to_string())]
    description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Html attrs=vec![("lang", "es")] class="bg-[#fed7aac9]"/>
        <Head>
            <meta charset="utf-8"/>
            <title>{title.clone()}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <meta property="og:title" content=title.clone()/>
            <meta name="description" content=description.clone()/>
            <meta property="og:description" content=description.clone()/>
            <meta
                property="og:site_name"
                content=format!("Blog de Rust Lang en Español {}", get_year())
            />
            <meta property="og:url" content="https://www.rustlang-es.org"/>
            <meta
                property="og:image"
                content=format!("https://www.rustlang-es.org/blog/articles/{slug}.png")
            />
            <meta
                property="twitter:image"
                content=format!("https://www.rustlang-es.org/blog/articles/{slug}.png")
            />
            <meta name="twitter:card" content="summary_large_image"/>
            <meta name="twitter:site" content="@rustlang"/>
            <link rel="icon" href="/LogoSegunMichael-134de58fcd9af94e.ico"/>
            {if cfg!(debug_assertions) {
                view! { <link rel="stylesheet" href="/output.css"/> }
            } else {
                view! {
                    <link rel="stylesheet" href="https://www.rustlang-es.org/blog/output.css"/>
                }
            }}

            <style>
                {"
                body {
                margin: 0 auto;
                }
                "}
            </style>
            <script type="module">
                {"
                const API = 'https://rust-lang-en-espanol-api.shuttleapp.rs';
                const previous_domain = document.referrer || 'Undefined';
                if (previous_domain != 'Undefined') { previous_domain = new URL(previous_domain).host; }
                const urlParams = new URLSearchParams(window.location.search);
                const fromParam = urlParams.get('from');
                if (fromParam != null) previous_domain = fromParam;
                await fetch(API + '/track/count?reference=' + previous_domain, { method: 'POST' });
                "}
            </script>
        </Head>

        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section class="w-full flex flex-col">
            <Header/>

            // <Async view=navigation_bar />
            <main class="container mx-auto py-5">{children()}</main>
        </section>
    }
}
