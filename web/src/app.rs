use brainace_core::{auth::User, Leaf};
use leptos::{
    component, create_resource, create_server_action, create_server_multi_action, server, view,
    Action, AttributeValue, Children, CollectView, ErrorBoundary, IntoView, Resource,
    ServerFnError, SignalGet, Transition,
};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{ActionForm, MultiActionForm, Route, Router, Routes, A};

use crate::{
    auth::{get_user, Login, Logout, Signup},
    error_template::ErrorTemplate,
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

#[server(GetLeaves, "/api")]
pub async fn get_leaves() -> Result<Vec<Leaf>, ServerFnError> {
    use self::ssr::pool;
    use brainace_core::SqlLeaf;
    use futures::future::join_all;

    let pool = pool()?;

    Ok(join_all(
        sqlx::query_as::<_, SqlLeaf>("SELECT * FROM leaves")
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|leaf: &SqlLeaf| leaf.clone().into_leaf(&pool)),
    )
    .await)
}

#[server(AddLeaf, "/api")]
pub async fn add_leaf(front: String, back: String) -> Result<(), ServerFnError> {
    use self::ssr::pool;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    let card = brainace_core::Card::default();
    let card_json: sqlx::types::Json<brainace_core::Card> =
        sqlx::types::Json::decode_from_string(&serde_json::to_string(&card).unwrap()).unwrap();

    Ok(
        sqlx::query("INSERT INTO leaves (user_id, front, back, card) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(front)
            .bind(back)
            .bind(card_json)
            .execute(&pool)
            .await
            .map(|_| ())?,
    )
}

#[server(DeleteLeaf, "/api")]
pub async fn delete_leaf(id: u32) -> Result<(), ServerFnError> {
    use self::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM leaves WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
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
            <html class="h-screen">
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
                            <Route path="" view=HelloWorld/>
                            <Route path="/login" view=move || view! { <Login action=login/> }/>
                            <Route path="/signup" view=move || view! { <Signup action=signup/> }/>
                        </Routes>
                    </main>
                </body>
            </html>
        </Router>
    }
}

#[component]
fn HelloWorld() -> impl IntoView {
    let add_leaf = create_server_multi_action::<AddLeaf>();
    let delete_leaf = create_server_action::<DeleteLeaf>();
    let submissions = add_leaf.submissions();

    let leaves = create_resource(
        move || (add_leaf.version().get(), delete_leaf.version().get()),
        move |_| get_leaves(),
    );

    view! {
        <div>
            <MultiActionForm action=add_leaf>
                <label>"Front" <input type="text" name="front"/></label>
                <label>"Back" <input type="text" name="back"/></label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorTemplate errors=errors/> }
                }>
                    {move || {
                        let existing_leaves = {
                            move || {
                                leaves
                                    .get()
                                    .map(move |leaves| match leaves {
                                        Err(e) => {
                                            view! {
                                                <pre class="error">"Server Error: " {e.to_string()}</pre>
                                            }
                                                .into_view()
                                        }
                                        Ok(leaves) => {
                                            if leaves.is_empty() {
                                                view! { <p>"No tasks were found."</p> }.into_view()
                                            } else {
                                                leaves
                                                    .into_iter()
                                                    .map(move |leaf| {
                                                        view! {
                                                            <li>
                                                                {leaf.front()} "|" {leaf.back()} ": Created at "
                                                                {leaf.created_at()} " by "
                                                                {leaf.user().unwrap_or_default().username}
                                                                <ActionForm action=delete_leaf>
                                                                    <input type="hidden" name="id" value=leaf.id()/>
                                                                    <input type="submit" value="X"/>
                                                                </ActionForm>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };
                        let pending_leaves = move || {
                            submissions
                                .get()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        <li class="pending">
                                            {move || submission.input.get().map(|data| data.front)}
                                        </li>
                                    }
                                })
                                .collect_view()
                        };
                        view! { <ul>{existing_leaves} {pending_leaves}</ul> }
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Card>
            <ActionForm action=action>
                <FormH1 text="Log in"/>
                <FormInput
                    input_type="text"
                    id="username"
                    label="Username"
                    placeholder="Username"
                    name="username"
                    maxlength="32"
                />
                <FormInput
                    input_type="password"
                    id="password"
                    label="Password"
                    placeholder="Password"
                    name="password"
                />
                <FormCheckbox label="Remember me?" name="remember"/>
                <FormSubmit msg="LOG IN"/>
            </ActionForm>
        </Card>
    }
}

#[component]
fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Card>
            <ActionForm action=action>
                <FormH1 text="Create your account"/>
                <FormInput
                    input_type="text"
                    id="username"
                    label="Username"
                    placeholder="Username"
                    name="username"
                    maxlength="32"
                />
                <FormInput
                    input_type="password"
                    id="password"
                    label="Password"
                    placeholder="Password"
                    name="password"
                />
                <FormInput
                    input_type="password"
                    id="password_confirmation"
                    label="Confirm Password"
                    placeholder="Password again"
                    name="password_confirmation"
                />
                <FormCheckbox label="Remember me?" name="remember"/>
                <FormSubmit msg="SIGN UP"/>
            </ActionForm>
        </Card>
    }
}

#[component]
fn LoginSection(
    user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
    logout: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let login_signup_buttons = move || {
        view! {
            <ActionA href="/signup" msg="SIGN UP"/>
            <ActionA href="/login" msg="LOG IN"/>
        }
    };

    let login_section = move || {
        user.get().map(|user| match user {
            Err(_) => login_signup_buttons.into_view(),
            Ok(None) => login_signup_buttons.into_view(),
            Ok(Some(user)) => view! {
                <p class="text-2xl text-white">{user.username}</p>
                <ActionBtn action=logout msg="LOG OUT"/>
            }
            .into_view(),
        })
    };

    view! {
        <Transition fallback=move || {
            view! { "Loading..." }
        }>{login_section}</Transition>
    }
}

#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="mx-auto w-1/3 p-6 rounded-xl bg-gray-870 border border-gray-750 shadow-lg">
            {children()}
        </div>
    }
}

#[component]
fn ActionA<'a>(href: &'a str, msg: &'a str) -> impl IntoView {
    let href = href.to_string();
    let msg = msg.to_string();

    view! {
        <A
            href=href
            class="px-6 py-2 rounded-md bg-violet-500 text-white hover:scale-105 hover:bg-violet-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
        >
            {msg}
        </A>
    }
}

#[component]
fn ActionBtn<'a>(action: Action<Logout, Result<(), ServerFnError>>, msg: &'a str) -> impl IntoView {
    let msg = msg.to_string();

    view! {
        <ActionForm action=action>
            <button
                type="submit"
                class="px-6 py-2 rounded-md bg-violet-500 text-white hover:scale-105 hover:bg-violet-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
            >
                {msg}
            </button>
        </ActionForm>
    }
}

#[component]
fn FormH1<'a>(text: &'a str) -> impl IntoView {
    let text = text.to_string();

    view! { <h1 class="text-center text-4xl mb-4 text-white">{text}</h1> }
}

#[component]
fn FormInput<'a>(
    input_type: &'a str,
    id: &'a str,
    label: &'a str,
    placeholder: &'a str,
    name: &'a str,
    #[prop(optional, into)] maxlength: Option<AttributeValue>,
) -> impl IntoView {
    let input_type = input_type.to_string();
    let id = id.to_string();
    let label = label.to_string();
    let placeholder = placeholder.to_string();
    let name = name.to_string();

    view! {
        <div class="mb-4">
            <label for=id.clone() class="block mb-2 text-lg text-bold text-white">
                {label}
            </label>
            <input
                type=input_type
                id=id
                placeholder=placeholder
                name=name
                maxlength=maxlength
                class="w-full p-2 rounded-md bg-transparent text-white outline outline-2 outline-violet-500 caret-violet-400 selection:bg-violet-400 focus:outline-offset-2 focus:outline-violet-300 transition-all ease-out"
            />
        </div>
    }
}

#[component]
fn FormCheckbox<'a>(label: &'a str, name: &'a str) -> impl IntoView {
    let label = label.to_string();
    let name = name.to_string();

    view! {
        <div class="flex spacing-x-10 mb-6">
            <label class="flex items-center text-lg text-bold text-white">
                <input
                    type="checkbox"
                    name=name
                    class="appearance-none relative peer size-5 shrink-0 rounded border-2 border-gray-630 checked:bg-violet-400 checked:border-0 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
                />
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="2"
                    stroke="currentColor"
                    class="absolute size-5 hidden peer-checked:block pointer-events-none outline-none"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="m4.5 12.75 6 6 9-13.5"
                    ></path>
                </svg>
                <span class="ml-2">{label}</span>
            </label>
        </div>
    }
}

#[component]
fn FormSubmit<'a>(msg: &'a str) -> impl IntoView {
    let msg = msg.to_string();

    view! {
        <button
            type="submit"
            class="w-full py-2 rounded-md bg-violet-500 text-white hover:bg-violet-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
        >
            {msg}
        </button>
    }
}
