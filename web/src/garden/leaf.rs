use super::i;
use crate::{
    error_template::ErrorTemplate,
    ui::{Card, ControlAction, ControlBtn, Controls},
};
use brainace_core::Leaf;
use leptos::{
    component, create_signal, leptos_server::Submission, server, view, Action, CollectView,
    ErrorBoundary, IntoView, ReadSignal, Resource, ServerFnError, SignalGet, SignalUpdate,
    Transition,
};

#[server(GetLeaves, "/api")]
pub async fn get_leaves(stem_id: u32) -> Result<Vec<Leaf>, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlLeaf;

    let pool = pool()?;

    Ok(
        sqlx::query_as::<_, SqlLeaf>("SELECT * FROM leaves WHERE stem_id = ?")
            .bind(stem_id)
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|leaf| leaf.into_leaf())
            .collect(),
    )
}

#[server(AddLeaf, "/api")]
pub async fn add_leaf(stem_id: u32, front: String, back: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    let card = brainace_core::Card::default();
    let card_json: sqlx::types::Json<brainace_core::Card> =
        sqlx::types::Json::decode_from_string(&serde_json::to_string(&card).unwrap()).unwrap();

    Ok(
        sqlx::query("INSERT INTO leaves (stem_id, front, back, card) VALUES (?, ?, ?, ?)")
            .bind(stem_id)
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
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM leaves WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[component]
pub fn Leaves(
    leaves: Resource<(usize, usize), Result<Vec<Leaf>, ServerFnError>>,
    delete_leaf: Action<DeleteLeaf, Result<(), ServerFnError>>,
    submissions: ReadSignal<Vec<Submission<AddLeaf, Result<(), ServerFnError>>>>,
) -> impl IntoView {
    view! {
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
                                        view! { <pre>"Server Error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(leaves) => {
                                        if leaves.is_empty() {
                                            view! {
                                                <p class="text-2xl text-white">"No leaves were found."</p>
                                            }
                                                .into_view()
                                        } else {
                                            leaves
                                                .into_iter()
                                                .map(move |leaf| {
                                                    view! {
                                                        <li>
                                                            <LeafOverview leaf delete_leaf/>
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
                                    <li>
                                        <PendingLeaf input=submission.input.get()/>
                                    </li>
                                }
                            })
                            .collect_view()
                    };
                    view! {
                        <ul class="flex flex-col space-y-6">{existing_leaves} {pending_leaves}</ul>
                    }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn LeafOverview(
    leaf: Leaf,
    delete_leaf: Action<DeleteLeaf, Result<(), ServerFnError>>,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(true);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="p-5">
                <p class="text-2xl text-center text-white hyphens-auto">{leaf.front()}</p>
            </div>
            <div class=("hidden", hidden)>
                <hr class="border-t-1 border-gray-750"/>
                <div class="p-5">
                    <p class="text-2xl text-center text-violet-500 hyphens-auto">{leaf.back()}</p>
                </div>
            </div>
            <Controls class="absolute -top-4 right-4">
                <ControlBtn
                    on_click=move |_| { set_hidden.update(|x| *x = !*x) }
                    size="5"
                    icon=i::FaEyeRegular
                />
                <ControlAction
                    action=delete_leaf
                    on_submit=move |_| {}
                    size="5"
                    icon=i::FaTrashCanRegular
                >
                    <input type="hidden" name="id" value=leaf.id()/>
                </ControlAction>
            </Controls>
        </Card>
    }
}

#[component]
pub fn PendingLeaf(input: Option<AddLeaf>) -> impl IntoView {
    let text = input.map_or("LOADING".to_string(), |input| input.front);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="animate-pulse p-4">
                <p class="text-2xl text-center text-gray-750">{text}</p>
            </div>
        </Card>
    }
}
