use crate::ui::{
    Card, FormCheckbox, FormH1, FormInput, FormSubmit, SideBarAction, SideBarItem,
    SideBarItemCircle, SideBarItems,
};
use brainace_core::auth::User;
use leptos::{
    component, server, view, Action, IntoView, Resource, ServerFnError, SignalGet, Suspense,
};
use leptos_router::ActionForm;

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

#[component]
pub fn Login(action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Card class="mx-auto w-1/3 p-6">
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
pub fn Signup(action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        <Card class="mx-auto w-1/3 p-6">
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
pub fn LoginSection(
    user: Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>,
    logout: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let login_signup_buttons = move || {
        view! {
            <SideBarItems>
                <SideBarItem href="/login" icon=icondata::FiLogIn text="LOG IN"/>
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
                                        icon=icondata::FaUserSolid
                                        text="PROFILE"
                                    />
                                    <SideBarAction
                                        action=logout
                                        icon=icondata::FiLogOut
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
