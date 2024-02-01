use brainace_core::Leaf;
use leptos::{
    component, create_resource, create_server_action, create_server_multi_action, create_signal,
    server, view, Action, ErrorBoundary, IntoView, ServerFnError, SignalUpdate, Transition,
};
use leptos::{CollectView, SignalGet};
use leptos_icons::Icon;
use leptos_router::{ActionForm, MultiActionForm};

use crate::error_template::ErrorTemplate;
use crate::ui::Card;
use crate::users::get_user;

#[server(GetLeaves, "/api")]
pub async fn get_leaves() -> Result<Vec<Leaf>, ServerFnError> {
    use crate::app::ssr::pool;
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
    use crate::app::ssr::pool;

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
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM leaves WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[component]
pub fn Leaves() -> impl IntoView {
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
                                                view! { <p>"No leaves were found."</p> }.into_view()
                                            } else {
                                                leaves
                                                    .into_iter()
                                                    .map(move |leaf| {
                                                        view! {
                                                            <li>
                                                                <Leaf leaf=leaf delete_leaf=delete_leaf/>
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
                                            <PendingLeaf leaf=submission.input.get()/>
                                        </li>
                                    }
                                })
                                .collect_view()
                        };
                        view! {
                            <ul class="flex flex-col space-y-4">
                                {existing_leaves} {pending_leaves}
                            </ul>
                        }
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn Leaf(
    leaf: Leaf,
    delete_leaf: Action<DeleteLeaf, Result<(), ServerFnError>>,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(true);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="p-4">
                <p class="text-2xl text-center text-white">{leaf.front()}</p>
            </div>
            <div class=("hidden", hidden)>
                <hr class="border-t-1 border-gray-750"/>
                <div class="p-4">
                    <p class="text-2xl text-center text-violet-500">{leaf.back()}</p>
                </div>
            </div>
            <div class="absolute top-2 right-2 flex space-x-2">
                <button on:click=move |_| { set_hidden.update(|x| *x = !*x) } class="text-white">
                    <Icon icon=icondata::FaEyeRegular class="size-5"/>
                </button>
                <ActionForm action=delete_leaf>
                    <input type="hidden" name="id" value=leaf.id()/>
                    <button type="submit" class="text-white">
                        <Icon icon=icondata::FaTrashCanRegular class="size-5"/>
                    </button>
                </ActionForm>
            </div>
        </Card>
    }
}

#[component]
pub fn PendingLeaf(leaf: Option<AddLeaf>) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(true);

    let text = leaf.map_or("LOADING".to_string(), |leaf| leaf.front);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="animate-pulse p-4">
                <p class="text-2xl text-center text-gray-750">{text}</p>
            </div>
        </Card>
    }
}
