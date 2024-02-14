use crate::{
    error_template::ErrorTemplate,
    garden::leaf::{get_all_leaves, Leaf, ReviewLeaf},
    ui::{ActionBtn, ServerAction},
};
use chrono::Utc;
use leptos::{
    component, create_resource, create_server_action, create_signal, view, ErrorBoundary, IntoView,
    SignalGet, SignalUpdate, Transition,
};

#[component]
pub fn Review() -> impl IntoView {
    let (revealed, set_revealed) = create_signal(false);
    let (i, set_i) = create_signal(0);

    let review_leaf = create_server_action::<ReviewLeaf>();

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
    let id = move || leaf().unwrap().unwrap().unwrap().id();
    let now = move || Utc::now().timestamp_millis();

    let next = move |_| {
        set_revealed.update(|x| *x = false);
        set_i.update(|i| *i += 1);
    };

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
                        class="w-full h-4 rounded-full bg-gray-750 [&::-webkit-progress-value]:rounded-full [&::-webkit-progress-value]:bg-violet-400 [&::-moz-progress-bar]:rounded-full [&::-moz-progress-bar]:bg-violet-400 text-violet-400"
                    ></progress>
                    <label for="progress" class="shrink-0 text-xl text-white">
                        {move || format!("{} / {}", i(), length())}
                    </label>
                </header>
                <main class="w-3/5 mx-auto flex-1 flex justify-center items-center">
                    {move || {
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
                    }}

                </main>
                <footer class="border-t-2 border-gray-750" class=("bg-gray-750", revealed)>
                    <div class="h-36 w-3/5 mx-auto flex items-center">
                        {move || {
                            if revealed() {
                                view! {
                                    <div class="w-full flex justify-center space-x-12">
                                        <ServerAction
                                            action=review_leaf
                                            msg="AGAIN"
                                            on_click=Box::new(next)
                                        >
                                            <input type="hidden" name="id" value=id/>
                                            <input type="hidden" name="rating" value=1/>
                                            <input type="hidden" name="now" value=now/>
                                        </ServerAction>
                                        <ServerAction
                                            action=review_leaf
                                            msg="HARD"
                                            on_click=Box::new(next)
                                        >
                                            <input type="hidden" name="id" value=id/>
                                            <input type="hidden" name="rating" value=2/>
                                            <input type="hidden" name="now" value=now/>
                                        </ServerAction>
                                        <ServerAction
                                            action=review_leaf
                                            msg="GOOD"
                                            on_click=Box::new(next)
                                        >
                                            <input type="hidden" name="id" value=id/>
                                            <input type="hidden" name="rating" value=3/>
                                            <input type="hidden" name="now" value=now/>
                                        </ServerAction>
                                        <ServerAction
                                            action=review_leaf
                                            msg="EASY"
                                            on_click=Box::new(next)
                                        >
                                            <input type="hidden" name="id" value=id/>
                                            <input type="hidden" name="rating" value=4/>
                                            <input type="hidden" name="now" value=now/>
                                        </ServerAction>
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
