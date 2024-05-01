use crate::{
    error_template::ErrorTemplate,
    ui::{
        ActionA, Card, FormCheckbox, FormH1, FormInput, FormSubmit, Loading, ServerAction,
        SideBarAction, SideBarItem, SideBarItemCircle, SideBarItems,
    },
};
use brainace_core::auth::User;
use leptos::{
    component, server, view, Action, ErrorBoundary, IntoView, Resource, ServerFnError, SignalGet,
    Suspense, Transition,
};
use leptos_router::{ActionForm, A};

#[cfg(feature = "ssr")]
mod ssr {
    pub use crate::app::ssr::{auth, pool};
    pub use bcrypt::{hash, verify, DEFAULT_COST};
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::app::ssr::auth;

    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let pool = pool()?;
    let auth = auth()?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let pool = pool()?;
    let auth = auth()?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(username.clone())
        .bind(password_hashed)
        .execute(&pool)
        .await?;

    let user = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("Signup failed: User does not exist."))?;

    sqlx::query("INSERT INTO trees (user_id, name) VALUES (?, ?)")
        .bind(user.id)
        .bind("Spruce")
        .execute(&pool)
        .await?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use self::ssr::*;

    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}

#[server(ChangePassword, "/api")]
pub async fn change_password(
    password: String,
    new_password: String,
    new_password_confirmation: String,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let user = get_user().await?;
    let pool = pool()?;

    if let Some(user) = user {
        match verify(password, &user.password)? {
            true => {
                if new_password != new_password_confirmation {
                    return Err(ServerFnError::ServerError(
                        "Passwords did not match.".to_string(),
                    ));
                }

                let new_password_hashed = hash(new_password, DEFAULT_COST).unwrap();

                sqlx::query("UPDATE users SET password = $2 WHERE id = $1")
                    .bind(user.id)
                    .bind(new_password_hashed)
                    .execute(&pool)
                    .await?;

                leptos_axum::redirect("/");

                Ok(())
            }
            false => Err(ServerFnError::ServerError(
                "Wrong current password.".to_string(),
            )),
        }
    } else {
        Err(ServerFnError::ServerError(
            "User does not exist.".to_string(),
        ))
    }
}

#[component]
pub fn Profile(
    user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
    logout: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <Transition fallback=move || {
            view! { <Loading/> }
        }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    user()
                        .map(|user| match user {
                            Err(e) => {
                                view! {
                                    <pre class="text-white">"Server Error: " {e.to_string()}</pre>
                                }
                                    .into_view()
                            }
                            Ok(None) => {
                                view! {
                                    <div class="text-lg text-secondary-630">
                                        <p>"You are not connected."</p>
                                        <A
                                            href="/signup"
                                            class="text-lg font-medium text-primary-500"
                                        >
                                            "Sign In"
                                        </A>
                                        <p>"?"</p>
                                    </div>
                                }
                                    .into_view()
                            }
                            Ok(Some(user)) => {
                                view! {
                                    <div class="flex items-center space-x-4">
                                        <p class="text-2xl text-white">{user.username}</p>
                                        <ServerAction action=logout msg="LOG OUT"/>
                                        <ActionA href="/change-password" msg="CHANGE PASSWORD"/>
                                    </div>
                                }
                                    .into_view()
                            }
                        })
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="h-full flex flex-col justify-center items-center">
            <Card class="w-1/3 p-6">
                <ActionForm action=action>
                    <FormH1 text="Log in".to_string()/>
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
            <div class="mt-8 flex items-center space-x-4">
                <span class="text-lg text-secondary-630">"Don't have an account yet?"</span>
                <A href="/signup" class="text-lg font-medium text-primary-500">
                    "Sign Up"
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="h-full flex flex-col justify-center items-center">
            <Card class="w-1/3 p-6">
                <ActionForm action=action>
                    <FormH1 text="Create your account".to_string()/>
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
            <div class="mt-8 flex items-center space-x-4">
                <span class="text-lg text-secondary-630">"Already have an account?"</span>
                <A href="/login" class="text-lg font-medium text-primary-500">
                    "Log In"
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn LoginSection(
    user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
    logout: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let login_signup_buttons = move || {
        view! {
            <SideBarItems>
                <SideBarItem href="/login" icon=icondata::LuLogIn text="LOG IN"/>
            </SideBarItems>
        }
    };

    view! {
        <Suspense>
            {move || {
                user.get()
                    .map(|user| match user {
                        Err(_) => login_signup_buttons.into_view(),
                        Ok(None) => login_signup_buttons.into_view(),
                        Ok(Some(_)) => {
                            view! {
                                <SideBarItems>
                                    <SideBarItemCircle
                                        href="/profile"
                                        icon=icondata::LuUser2
                                        text="PROFILE"
                                    />
                                    <SideBarAction
                                        action=logout
                                        icon=icondata::LuLogOut
                                        text="LOG OUT"
                                    />
                                </SideBarItems>
                            }
                                .into_view()
                        }
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn ChangePassword(action: Action<ChangePassword, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <div class="h-full flex flex-col justify-center items-center">
            <Card class="w-1/3 p-6">
                <ActionForm action=action>
                    <FormH1 text="Change your password".to_string()/>
                    <FormInput
                        input_type="password"
                        id="password"
                        label="Current Password"
                        placeholder="Password"
                        name="password"
                    />
                    <FormInput
                        input_type="password"
                        id="new_password"
                        label="New Password"
                        placeholder="New password"
                        name="new_password"
                    />
                    <FormInput
                        input_type="password"
                        id="new_password_confirmation"
                        label="Confirm New Password"
                        placeholder="New password again"
                        name="new_password_confirmation"
                    />
                    <FormSubmit msg="CHANGE"/>
                </ActionForm>
            </Card>
        </div>
    }
}
