use leptos::{component, create_resource, create_server_action, view, IntoView, SignalGet};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, A};

use crate::{
    garden::Branches,
    users::{get_user, Login, LoginSection, Logout, Signup},
};

#[cfg(feature = "ssr")]
pub mod ssr {
    use brainace_core::auth::AuthSession;
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

#[component]
pub fn App() -> impl IntoView {
    let login = create_server_action::<Login>();
    let signup = create_server_action::<Signup>();
    let logout = create_server_action::<Logout>();

    let user = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );

    provide_meta_context();

    view! {
        <Title text="Brainace"/>
        <Link rel="shortcut icon" type_="image/ico" href="/Brainace_Icon_Dark.ico"/>
        <Stylesheet id="leptos" href="/pkg/brainace_web.css"/>
        <Router>
            <html class="h-screen" lang="en">
                <body class="flex h-full flex-col bg-gray-870">
                    <header class="h-24 px-8 py-4 border-b-2 border-gray-750">
                        <A href="/" class="float-left h-full focus:outline-none">
                            <img src="/Brainace_Banner_Dark.svg" class="h-full outline-none"/>
                        </A>
                        <div class="flex float-right h-full items-center space-x-4">
                            <LoginSection user=user logout=logout/>
                        </div>
                    </header>
                    <main class="flex-1 container mx-auto py-8">
                        <Routes>
                            <Route path="" view=Branches/>
                            <Route path="/login" view=move || view! { <Login action=login/> }/>
                            <Route path="/signup" view=move || view! { <Signup action=signup/> }/>
                        </Routes>
                    </main>
                </body>
            </html>
        </Router>
    }
}
