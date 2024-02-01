use leptos::{component, view, AttributeValue, Children, IntoView};
use leptos_icons::*;
use leptos_router::A;

#[component]
pub fn Card<'a>(children: Children, #[prop(optional)] class: Option<&'a str>) -> impl IntoView {
    let mut class = class.map_or(String::new(), |str| str.to_string());
    class.push_str(" rounded-xl bg-gray-870 border border-gray-750 shadow-lg");

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn ActionA<'a>(href: &'a str, msg: &'a str) -> impl IntoView {
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
pub fn FormCheckbox<'a>(label: &'a str, name: &'a str) -> impl IntoView {
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
            class="w-full py-2 rounded-md bg-violet-500 text-white hover:bg-violet-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
        >
            {msg}
        </button>
    }
}

#[component]
pub fn FormAction<'a>(msg: &'a str) -> impl IntoView {
    let msg = msg.to_string();

    view! {
        <button
            type="submit"
            class="px-6 py-2 rounded-md bg-violet-500 text-white hover:scale-105 hover:bg-violet-400 focus:outline-none focus:ring-offset-2 focus:ring-2 focus:ring-violet-300 focus:ring-offset-gray-870 transition ease-out"
        >
            {msg}
        </button>
    }
}
