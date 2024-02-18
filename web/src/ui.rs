use icondata::Icon;
use leptos::{
    component,
    ev::{MouseEvent, SubmitEvent},
    event_target,
    server_fn::{
        client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
    },
    view, Action, AttributeValue, Children, IntoView, ReadSignal, Serializable, ServerFnError,
};
use leptos_icons::*;
use leptos_router::{ActionForm, A};
use serde::de::DeserializeOwned;
use web_sys::{Element, FormData};

#[component]
pub fn SideBar(children: Children) -> impl IntoView {
    view! {
        <aside class="fixed top-0 left-0 z-40 w-64 h-screen border-r-2 border-secondary-750">
            <div class="h-full px-4 overflow-y-auto">{children()}</div>
        </aside>
    }
}

#[component]
pub fn SideBarItems(children: Children) -> impl IntoView {
    view! { <ul class="space-y-2">{children()}</ul> }
}

#[component]
pub fn SideBarItem<'a>(href: &'a str, icon: Icon, text: &'a str) -> impl IntoView {
    let href = href.to_string();
    let text = text.to_string();

    view! {
        <li>
            <A
                href
                class="group block flex items-center px-4 py-3 space-x-6 rounded-xl hover:bg-secondary-750 focus:bg-secondary-750 focus:outline focus:outline-2 focus:outline-primary-400"
            >
                <Icon icon class="size-6 text-primary-500 group-focus:text-white"/>
                <span class="text-lg text-white font-medium">{text}</span>
            </A>
        </li>
    }
}

#[component]
pub fn SideBarItemCircle<'a>(href: &'a str, icon: Icon, text: &'a str) -> impl IntoView {
    let href = href.to_string();
    let text = text.to_string();

    view! {
        <li>
            <A
                href
                class="group block flex items-center px-4 py-3 space-x-6 rounded-xl hover:bg-secondary-750 focus:bg-secondary-750 focus:outline focus:outline-2 focus:outline-primary-400"
            >
                <div class="relative size-6">
                    <div class="absolute -top-2 -left-2 size-10 rounded-full bg-primary-500 group-focus:bg-white"></div>
                    <Icon
                        icon
                        class="absolute top-0 left-0 z-10 size-6 text-white group-focus:text-primary-500"
                    />
                </div>
                <span class="text-lg text-white font-medium">{text}</span>
            </A>
        </li>
    }
}

#[component]
pub fn SideBarAction<'a, ServFn>(
    action: Action<ServFn, Result<ServFn::Output, ServerFnError<ServFn::Error>>>,
    icon: Icon,
    text: &'a str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    ServFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
        From<FormData>,
{
    let text = text.to_string();

    view! {
        <li>
            <ActionForm action>
                {children.map(|children| children())}
                <button
                    type="submit"
                    class="group w-full flex items-center px-4 py-3 space-x-6 rounded-xl hover:bg-secondary-750 focus:bg-secondary-750 focus:outline focus:outline-2 focus:outline-primary-400"
                >
                    <Icon icon class="size-6 text-primary-500 group-focus:text-white"/>
                    <span class="text-lg text-white font-medium">{text}</span>
                </button>
            </ActionForm>
        </li>
    }
}

#[component]
pub fn SideBarSeparator() -> impl IntoView {
    view! { <hr class="my-6 border-secondary-750"/> }
}

#[component]
pub fn SideContent(children: Children) -> impl IntoView {
    view! { <div class="h-full ml-64 flex flex-col">{children()}</div> }
}

#[component]
pub fn Card<'a>(children: Children, #[prop(optional)] class: Option<&'a str>) -> impl IntoView {
    let classes = "rounded-xl bg-secondary-870 border border-secondary-750 shadow-lg";
    let class = class.map_or(format!("{}", classes), |str| format!("{} {}", str, classes));

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn Modal<'a, F: Fn(MouseEvent) + 'static>(
    id: &'a str,
    show: ReadSignal<bool>,
    on_blur: F,
    children: Children,
) -> impl IntoView {
    let id = id.to_string();
    let selector = format!("#{id}");

    view! {
        <div class:hidden=move || !show()>
            <div class="fixed top-0 left-0 size-full bg-black opacity-25"></div>
            <div
                on:click=move |e| {
                    if event_target::<Element>(&e.clone().into())
                        .closest(&selector)
                        .unwrap()
                        .is_none()
                    {
                        on_blur(e);
                    }
                }

                class="fixed top-0 left-0 size-full flex justify-center items-center"
            >
                <div id=id class="contents">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Controls<'a>(#[prop(optional)] class: Option<&'a str>, children: Children) -> impl IntoView {
    let classes = "flex rounded-xl bg-primary-600 overflow-hidden";
    let class = class.map_or(format!("{}", classes), |str| format!("{} {}", str, classes));

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn ControlBtn<F, 'a>(on_click: F, size: &'a str, icon: Icon) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    view! {
        <button on:click=on_click class="group size-8 p-1.5 text-white hover:bg-primary-500">
            <Icon icon=icon class=format!("size-{} group-hover:scale-105", size)/>
        </button>
    }
}

#[component]
pub fn ControlAction<'a, I, O, F>(
    action: Action<I, Result<O, ServerFnError>>,
    on_submit: F,
    size: &'a str,
    icon: Icon,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    I: Clone
        + ServerFn<InputEncoding = PostUrl, Output = O, Error = NoCustomError>
        + DeserializeOwned
        + 'static,
    O: Clone + Serializable + 'static,
    <<<I as ServerFn>::Client as Client<<I as ServerFn>::Error>>::Request as ClientReq<
        <I as ServerFn>::Error,
    >>::FormData: From<web_sys::FormData>,
    F: FnMut(SubmitEvent) + 'static,
{
    let size = size.to_string();

    view! {
        <ActionForm
            action=action
            on:submit=on_submit
            class="group size-8 p-1.5 hover:bg-primary-500"
        >
            {children.map(|children| children())}
            <button type="submit" class="text-white">
                <Icon icon=icon class=format!("size-{} group-hover:scale-105", size)/>
            </button>
        </ActionForm>
    }
}

#[component]
pub fn ActionBtn<'a, F>(
    msg: &'a str,
    on_click: F,
    #[prop(optional)] color: Option<&'a str>,
    #[prop(optional)] hover_color: Option<&'a str>,
) -> impl IntoView
where
    F: FnMut(MouseEvent) + 'static,
{
    let msg = msg.to_string();
    let color = color.unwrap_or("bg-primary-500").to_string();
    let hover_color = hover_color.unwrap_or("hover:bg-primary-400").to_string();

    view! {
        <button
            on:click=on_click
            class=format!(
                "px-6 py-2 rounded-md {color} text-white hover:scale-105 {hover_color} focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-primary-300 focus:ring-offset-secondary-870 transition ease-out",
            )
        >

            {msg}
        </button>
    }
}

#[component]
pub fn ActionA<'a>(href: &'a str, msg: &'a str) -> impl IntoView {
    let href = href.to_string();
    let msg = msg.to_string();

    view! {
        <A
            href=href
            class="block px-6 py-2 rounded-md bg-primary-500 text-white hover:scale-105 hover:bg-primary-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-primary-300 focus:ring-offset-secondary-870 transition ease-out"
        >
            {msg}
        </A>
    }
}

#[component]
pub fn FormH1<'a>(text: &'a str) -> impl IntoView {
    let text = text.to_string();

    view! { <h1 class="text-center text-4xl mb-4 text-white">{text}</h1> }
}

#[component]
pub fn FormInput<'a>(
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
            <label for=id.clone() class="block mb-2 text-lg font-bold text-white">
                {label}
            </label>
            <input
                type=input_type
                id=id
                placeholder=placeholder
                name=name
                maxlength=maxlength
                class="w-full p-2 rounded-md bg-transparent text-white outline outline-2 outline-primary-500 caret-primary-400 selection:bg-primary-400 focus:outline-offset-2 focus:outline-primary-300 transition-all ease-out"
            />
        </div>
    }
}

#[component]
pub fn FormCheckbox<'a>(label: &'a str, name: &'a str) -> impl IntoView {
    let label = label.to_string();
    let name = name.to_string();

    view! {
        <div class="flex spacing-x-10 mb-6">
            <label class="flex items-center text-lg font-bold text-white">
                <input
                    type="checkbox"
                    name=name
                    class="appearance-none relative peer size-5 shrink-0 rounded border-2 border-secondary-630 checked:bg-primary-400 checked:border-0 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-primary-300 focus:ring-offset-secondary-870 transition ease-out"
                />
                <Icon
                    icon=icondata::FaCheckSolid
                    class="absolute size-5 scale-90 hidden peer-checked:block pointer-events-none outline-none"
                />
                <span class="ml-2">{label}</span>
            </label>
        </div>
    }
}

#[component]
pub fn FormSubmit<'a>(msg: &'a str) -> impl IntoView {
    let msg = msg.to_string();

    view! {
        <button
            type="submit"
            class="w-full py-2 rounded-md bg-primary-500 text-white hover:bg-primary-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-primary-300 focus:ring-offset-secondary-870 transition ease-out"
        >
            {msg}
        </button>
    }
}

#[component]
pub fn ServerAction<I, O, 'a>(
    action: Action<I, Result<O, ServerFnError>>,
    msg: &'a str,
    #[prop(optional)] color: Option<&'a str>,
    #[prop(optional)] hover_color: Option<&'a str>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    I: Clone
        + ServerFn<InputEncoding = PostUrl, Output = O, Error = NoCustomError>
        + DeserializeOwned
        + 'static,
    O: Clone + Serializable + 'static,
    <<<I as ServerFn>::Client as Client<<I as ServerFn>::Error>>::Request as ClientReq<
        <I as ServerFn>::Error,
    >>::FormData: From<web_sys::FormData>,
{
    let msg = msg.to_string();
    let color = color.unwrap_or("bg-primary-500").to_string();
    let hover_color = hover_color.unwrap_or("hover:bg-primary-400").to_string();

    view! {
        <ActionForm action=action>
            {children.map(|children| children())}
            <button
                type="submit"
                class=format!(
                    "px-6 py-2 rounded-md {color} text-white hover:scale-105 {hover_color} focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-primary-300 focus:ring-offset-secondary-870 transition ease-out",
                )
            >

                {&msg}
            </button>
        </ActionForm>
    }
}
