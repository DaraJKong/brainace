use crate::{
    error_template::ErrorTemplate,
    garden::leaf::{get_all_leaves, review_leaf, Leaf},
    ui::{ActionA, ActionBtn},
};
use brainace_core::{Leaf, Rating};
use chrono::Utc;
use leptos::{
    component, create_resource, create_signal, spawn_local, view, ErrorBoundary, IntoView,
    SignalGet, SignalUpdate, Transition, WriteSignal,
};

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

#[component]
pub fn Review() -> impl IntoView {
    let (revealed, set_revealed) = create_signal(false);
    let (i, set_i) = create_signal(0);

    let leaves = create_resource(|| (), move |_| get_all_leaves());

    let length = move || {
        leaves
            .get()
            .unwrap_or(Ok(Vec::new()))
            .unwrap_or_default()
            .len()
    };
    let leaf = move || {
        leaves
            .get()
            .map(|leaves| leaves.map(|leaves| leaves.get(i()).cloned()))
    };
    let leaf_unwrap = move || leaf().unwrap().unwrap().unwrap();

    // TODO: filter leaves by due date

    view! {
        <Transition fallback=move || view! { <p class="text-white">"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
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
                                    <p class="text-xl text-white">
                                        "All leaves have been reviewed."
                                    </p>
                                </div>
                            }
                                .into_view()
                        } else {
                            leaf()
                                .map(|leaves| match leaves {
                                    Err(e) => {
                                        view! {
                                            <pre class="text-white">
                                                "Server Error: " {e.to_string()}
                                            </pre>
                                        }
                                            .into_view()
                                    }
                                    Ok(None) => {
                                        view! {
                                            <p class="text-2xl text-white">"No leaf was found."</p>
                                        }
                                            .into_view()
                                    }
                                    Ok(Some(leaf)) => view! { <Leaf leaf revealed/> }.into_view(),
                                })
                                .unwrap_or_default()
                        }
                    }}

                </main>
                <footer
                    class="border-t-2 border-secondary-750"
                    class=("bg-secondary-750", revealed)
                >
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
                                        <ActionBtn
                                            msg="SKIP"
                                            on_click=move |_| set_i.update(|i| *i += 1)
                                        />
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
            </ErrorBoundary>
        </Transition>
    }
}
