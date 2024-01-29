use leptos::{
    component, create_resource, create_server_action, view, Action, IntoView, ServerFnError,
    SignalGet, Transition,
};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{ActionForm, Route, Router, Routes, A};

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

    let login_section = move || {
        user.get().map(|user| match user {
            Err(e) => view! {
                <A href="/signup">"Signup"</A>
                <A href="/login">"Login"</A>
            }
            .into_view(),
            Ok(None) => view! {
                <A href="/signup">"Signup"</A>
                <A href="/login">"Login"</A>
            }
            .into_view(),
            Ok(Some(user)) => view! {
                <ActionForm action=logout>
                    <button type="submit">"Log Out"</button>
                </ActionForm>
            }
            .into_view(),
        })
    };

    view! {
        <Title text="Brainace"/>
        <Link rel="shortcut icon" type_="image/ico" href="/Brainace_Icon_Dark.ico"/>
        <Stylesheet id="leptos" href="/pkg/brainace_web.css"/>
        <Router>
            <body class="bg-gray-870">
                <header class="flex h-20 px-8 py-4 items-center border-b-2 border-gray-750">
                    <A href="/" class="h-full">
                        <img src="/Brainace_Banner_Dark.svg" class="h-full"/>
                    </A>
                    <Transition fallback=move || view! { "Loading..." }>
                        {login_section}
                    </Transition>
                </header>
                <main class="container h-full mx-auto py-4">
                    <Routes>
                        <Route path="" view=HelloWorld/>
                        <Route path="/login" view=move || view! { <Login action=login/> }/>
                        <Route path="/signup" view=move || view! { <Signup action=signup/> }/>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

#[component]
fn HelloWorld() -> impl IntoView {
    view! {
        <h1 class="text-6xl text-bold text-violet-400">"Hello, World!"</h1>
    }
}

#[component]
fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "Username:"
                <input
                    type="text"
                    placeholder="Username"
                    maxlength="32"
                    name="username"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password"/>
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember"/>
                "Remember me?"
            </label>
            <br/>
            <button type="submit">
                "Log In"
            </button>
        </ActionForm>
    }
}

#[component]
fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "Username:"
                <input
                    type="text"
                    placeholder="Username"
                    maxlength="32"
                    name="username"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password"/>
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input
                    type="password"
                    placeholder="Password again"
                    name="password_confirmation"
                />
            </label>
            <br/>
            <label>
                "Remember me?" <input type="checkbox" name="remember"/>
            </label>
            <br/>
            <button type="submit">
                "Sign Up"
            </button>
        </ActionForm>
    }
}
