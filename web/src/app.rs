use leptos::{create_resource, create_server_action, view, IntoView, SignalGet};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::Router;

use crate::auth::{get_user, Login, Logout, Signup};

#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::auth::ssr::AuthSession;
    use leptos::{use_context, ServerFnError};
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }
}

pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Brainace"/>
        <Link rel="shortcut icon" type_="image/ico" href="/Brainace_Icon_Dark.ico"/>
        <Stylesheet id="leptos" href="/pkg/brainace_web.css"/>
        <Router>
            <header>
            </header>
            <main class="h-full bg-gray-870">
                <h1 class="text-violet-500">Hello, World!</h1>
            </main>
        </Router>
    }
}
