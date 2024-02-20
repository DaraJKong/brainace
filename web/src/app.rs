use leptos::{
    component, create_resource, create_rw_signal, create_server_action, provide_context, view,
    ErrorBoundary, IntoView, SignalGet, Transition,
};
use leptos_meta::{provide_meta_context, Body, Html, Link, Stylesheet, Title};
use leptos_router::{Outlet, Route, Router, Routes, A};

use crate::{
    error_template::ErrorTemplate,
    garden::{
        branch::{Branch, NoBranch},
        leaf::{LeafDetails, NoLeaf},
        stem::{NoStem, Stem},
        tree::{get_user_tree, Tree},
    },
    review::Review,
    ui::{SideBar, SideBarItem, SideBarItems, SideBarSeparator, SideContent},
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

    let tree = create_resource(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user_tree(),
    );

    provide_meta_context();

    view! {
        <Title text="Brainace"/>
        <Link rel="shortcut icon" type_="image/ico" href="/Brainace_Icon_Dark.ico"/>
        <Stylesheet id="leptos" href="/pkg/brainace_web.css"/>
        <Html lang="en" class="h-full"/>
        <Body class="h-full flex flex-col bg-secondary-870"/>
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move || {
                        view! {
                            <SideBar>
                                <A href="/" class="block px-4 py-8 focus:outline-none">
                                    <img src="/Brainace_Banner_Dark.svg" class="outline-none"/>
                                </A>
                                <SideBarItems>
                                    <SideBarItem
                                        href="/"
                                        icon=icondata::FaHouseSolid
                                        text="DASHBOARD"
                                    />
                                    <SideBarItem
                                        href="/review"
                                        icon=icondata::FaBrainSolid
                                        text="REVIEW ALL"
                                    />
                                </SideBarItems>
                                <SideBarSeparator/>
                                <LoginSection user logout/>
                            </SideBar>
                            <SideContent>
                                <main class="flex-1 flex-col container mx-auto py-8">
                                    <Transition fallback=move || {
                                        view! { <p class="text-2xl text-white">"Loading..."</p> }
                                    }>
                                        {move || {
                                            if let Some(Ok(tree)) = tree() {
                                                let tree = create_rw_signal(tree.clone());
                                                provide_context(tree);
                                            }
                                            view! { <Outlet/> }
                                        }}

                                    </Transition>
                                </main>
                            </SideContent>
                        }
                    }
                >

                    <Route path="/" view=Tree/>
                    <Route path="/branch" view=NoBranch/>
                    <Route path="/branch/:id" view=Branch/>
                    <Route path="/stem" view=NoStem/>
                    <Route path="/stem/:id" view=Stem/>
                    <Route path="/leaf" view=NoLeaf/>
                    <Route path="/leaf/:id" view=LeafDetails/>
                    <Route
                        path="/profile"
                        view=move || {
                            view! {
                                <Transition fallback=move || {
                                    view! { <p class="text-xl text-white">"Loading user..."</p> }
                                }>
                                    <ErrorBoundary fallback=|errors| {
                                        view! { <ErrorTemplate errors/> }
                                    }>
                                        <p class="text-xl text-primary-400">
                                            {user()
                                                .map(|user| {
                                                    user.map(|user| user.map(|user| { user.username }))
                                                })}

                                        </p>
                                        <p class="text-white">
                                            {user()
                                                .map(|user| {
                                                    user.map(|user| user.map(|user| { user.password }))
                                                })}

                                        </p>
                                    </ErrorBoundary>
                                </Transition>
                            }
                        }
                    />

                    <Route path="/login" view=move || view! { <Login action=login/> }/>
                    <Route path="/signup" view=move || view! { <Signup action=signup/> }/>
                </Route>
                <Route
                    path="/review"
                    view=move || {
                        view! {
                            <Transition fallback=move || {
                                view! { <p class="text-2xl text-white">"Loading..."</p> }
                            }>
                                {move || {
                                    if let Some(Ok(tree)) = tree() {
                                        let tree = create_rw_signal(tree.clone());
                                        provide_context(tree);
                                    }
                                    view! { <Review/> }
                                }}

                            </Transition>
                        }
                    }
                />

            </Routes>
        </Router>
    }
}
