use crate::{
    error_template::ErrorTemplate,
    garden::leaf::{get_all_leaves, review_leaf, Leaf},
    ui::{ActionA, ActionBtn, Loading},
};
use brainace_core::{utils, Leaf, Rating};
use chrono::Utc;
use leptos::{
    component, create_resource, create_signal, provide_context, spawn_local, use_context, view,
    ErrorBoundary, IntoView, SignalUpdate, Transition, WriteSignal,
};

#[component]
pub fn ReviewToday() -> impl IntoView {
    let leaves = create_resource(|| (), move |_| get_all_leaves());

    view! {
        <Transition fallback=|| view! { <Loading/> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    leaves()
                        .map(|leaves| match leaves {
                            Err(e) => {
                                view! {
                                    <pre class="text-white">"Server Error: " {e.to_string()}</pre>
                                }
                                    .into_view()
                            }
                            Ok(leaves) => {
                                if leaves.is_empty() {
                                    view! { <p class="text-white">"No leaves were found."</p> }
                                        .into_view()
                                } else {
                                    let due_today_leaves = utils::filter_due_today(leaves);
                                    provide_context(due_today_leaves);
                                    view! { <Review/> }
                                }
                            }
                        })
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn ReviewNow() -> impl IntoView {
    let leaves = create_resource(|| (), move |_| get_all_leaves());

    view! {
        <Transition fallback=|| view! { <Loading/> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    leaves()
                        .map(|leaves| match leaves {
                            Err(e) => {
                                view! {
                                    <pre class="text-white">"Server Error: " {e.to_string()}</pre>
                                }
                                    .into_view()
                            }
                            Ok(leaves) => {
                                if leaves.is_empty() {
                                    view! { <p class="text-white">"No leaves were found."</p> }
                                        .into_view()
                                } else {
                                    let due_now_leaves = utils::filter_due_now(leaves);
                                    provide_context(due_now_leaves);
                                    view! { <Review/> }
                                }
                            }
                        })
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn Review() -> impl IntoView {
    let (revealed, set_revealed) = create_signal(false);
    let (i, set_i) = create_signal(0);

    let length = move || use_context::<Vec<Leaf>>().unwrap_or_default().len();
    let leaf = move || use_context::<Vec<Leaf>>().map(|leaves| leaves.get(i()).cloned());
    let leaf_unwrap = move || {
        use_context::<Vec<Leaf>>()
            .unwrap()
            .get(i())
            .unwrap()
            .clone()
    };

    view! {
        <header class="h-36 w-3/5 mx-auto flex items-center space-x-4">
            <progress
                id="progress"
                max=length
                value=i
                class="w-full h-4 rounded-full bg-secondary-750 [&::-webkit-progress-value]:rounded-full [&::-webkit-progress-value]:bg-primary-400 [&::-moz-progress-bar]:rounded-full [&::-moz-progress-bar]:bg-primary-400 text-primary-400"
            ></progress>
            <label for="progress" class="shrink-0 text-xl text-white">
                {move || format!("{} / {}", i(), length())}
            </label>
        </header>
        <main class="w-3/5 mx-auto flex-1 flex justify-center items-center">
            {move || {
                if i() >= length() {
                    view! {
                        <div class="space-y-4">
                            <h1 class="text-5xl text-primary-500">"Congratulations!"</h1>
                            <p class="text-xl text-white">"All leaves have been reviewed."</p>
                        </div>
                    }
                        .into_view()
                } else {
                    leaf()
                        .map(|leaves| match leaves {
                            None => view! { <Loading/> }.into_view(),
                            Some(leaf) => view! { <Leaf leaf=leaf.clone() revealed/> }.into_view(),
                        })
                        .unwrap_or_default()
                }
            }}

        </main>
        <footer class="border-t-2 border-secondary-750" class=("bg-secondary-750", revealed)>
            <div class="h-36 w-3/5 mx-auto flex items-center">
                {move || {
                    if i() >= length() {
                        view! {
                            <div class="w-full flex justify-center">
                                <ActionA href="/" msg="HOME"/>
                            </div>
                        }
                    } else if revealed() {
                        view! {
                            <div class="w-full flex justify-center space-x-12">
                                <ReviewBtn
                                    leaf=leaf_unwrap()
                                    rating=Rating::Again
                                    set_i
                                    set_revealed
                                />
                                <ReviewBtn
                                    leaf=leaf_unwrap()
                                    rating=Rating::Hard
                                    set_i
                                    set_revealed
                                />
                                <ReviewBtn
                                    leaf=leaf_unwrap()
                                    rating=Rating::Good
                                    set_i
                                    set_revealed
                                />
                                <ReviewBtn
                                    leaf=leaf_unwrap()
                                    rating=Rating::Easy
                                    set_i
                                    set_revealed
                                />
                            </div>
                        }
                    } else {
                        view! {
                            <div class="w-full flex justify-between">
                                <ActionBtn msg="SKIP" on_click=move |_| set_i.update(|i| *i += 1)/>
                                <ActionBtn
                                    msg="REVEAL"
                                    on_click=move |_| set_revealed.update(|x| *x = true)
                                />
                            </div>
                        }
                    }
                }}

            </div>
        </footer>
    }
}

#[component]
pub fn ReviewBtn(
    leaf: Leaf,
    rating: Rating,
    set_i: WriteSignal<usize>,
    set_revealed: WriteSignal<bool>,
) -> impl IntoView {
    let msg = match rating {
        Rating::Again => "AGAIN",
        Rating::Hard => "HARD",
        Rating::Good => "GOOD",
        Rating::Easy => "EASY",
    };

    let (color, hover_color) = match rating {
        Rating::Again => ("bg-red-500", "hover:bg-red-400"),
        Rating::Hard => ("bg-yellow-500", "hover:bg-yellow-400"),
        Rating::Good => ("bg-blue-500", "hover:bg-blue-400"),
        Rating::Easy => ("bg-green-500", "hover:bg-green-400"),
    };

    view! {
        <ActionBtn
            msg
            on_click=move |_| {
                let leaf = leaf.clone();
                spawn_local(async move {
                    let _ = review_leaf(leaf, rating, Utc::now()).await;
                });
                set_revealed.update(|x| *x = false);
                set_i.update(|i| *i += 1);
            }

            color
            hover_color
        />
    }
}
