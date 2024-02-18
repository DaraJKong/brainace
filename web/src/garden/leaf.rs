use crate::{
    error_template::ErrorTemplate,
    ui::{Card, ControlA, ControlAction, ControlBtn, Controls, Loading},
};
use brainace_core::{Leaf, Rating, State};
use chrono::{DateTime, Utc};
use leptos::{
    component, create_resource, create_signal, leptos_server::Submission, server, view, Action,
    CollectView, ErrorBoundary, IntoView, Params, ReadSignal, Resource, ServerFnError, SignalGet,
    SignalUpdate, SignalWith, Transition,
};
use leptos_icons::Icon;
use leptos_router::{use_params, Params, A};

#[server(GetLeaf, "/api")]
pub async fn get_leaf(id: u32) -> Result<Leaf, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlLeaf;

    let pool = pool()?;

    Ok(
        sqlx::query_as::<_, SqlLeaf>("SELECT * FROM leaves WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?
            .into_leaf(),
    )
}

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
            .map(|sql_leaf| sql_leaf.into_leaf())
            .collect(),
    )
}

#[server(GetAllLeaves, "/api")]
pub async fn get_all_leaves() -> Result<Vec<Leaf>, ServerFnError> {
    use crate::{app::ssr::pool, users::get_user};
    use brainace_core::SqlLeaf;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    Ok(sqlx::query_as::<_, SqlLeaf>(
        "SELECT l.* FROM leaves l
            INNER JOIN stems s
                ON s.id = l.stem_id
            INNER JOIN branches b
                ON b.id = s.branch_id
            INNER JOIN trees t
                ON t.id = b.tree_id
                AND t.user_id = ?",
    )
    .bind(id)
    .fetch_all(&pool)
    .await?
    .iter()
    .map(|sql_leaf| sql_leaf.into_leaf())
    .collect())
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

#[server(ReviewLeaf, "/api")]
pub async fn review_leaf(
    leaf: Leaf,
    rating: Rating,
    now: DateTime<Utc>,
) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::Config;

    let pool = pool()?;

    let mut leaf = leaf;

    let config = Config::default();
    leaf.review(&config, rating, now);

    let card_json: sqlx::types::Json<brainace_core::Card> =
        sqlx::types::Json::decode_from_string(&serde_json::to_string(leaf.card()).unwrap())
            .unwrap();

    Ok(sqlx::query("UPDATE leaves SET card = $2 WHERE id = $1")
        .bind(leaf.id())
        .bind(card_json)
        .execute(&pool)
        .await
        .map(|_| ())?)
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
        <Transition fallback=move || {
            view! { <Loading/> }
        }>
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

#[derive(Params, PartialEq)]
struct LeafParams {
    id: u32,
}

#[component]
pub fn LeafDetails() -> impl IntoView {
    let params = use_params::<LeafParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let leaf = create_resource(id, move |_| get_leaf(id()));

    view! {
        <Transition fallback=move || {
            view! { <Loading/> }
        }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    leaf.get()
                        .map(move |leaf| match leaf {
                            Err(e) => {
                                view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view()
                            }
                            Ok(leaf) => {
                                let card = leaf.card();
                                let state = serde_json::to_string(&card.state);
                                let previous_state = serde_json::to_string(&card.previous_state);
                                let log = serde_json::to_string(&card.log);
                                view! {
                                    <div class="text-xl text-white">
                                        <p>"Front: " {leaf.front()}</p>
                                        <p>"Back: " {leaf.back()}</p>
                                        <p>"Created at: " {leaf.created_at().to_string()}</p>
                                        <p>"Due: " {card.due.to_string()}</p>
                                        <p>"Stability: " {card.stability}</p>
                                        <p>"Difficulty: " {card.difficulty}</p>
                                        <p>"Elapsed days: " {card.elapsed_days}</p>
                                        <p>"Scheduled days: " {card.scheduled_days}</p>
                                        <p>"Reps: " {card.reps}</p>
                                        <p>"Lapses: " {card.lapses}</p>
                                        <p>"State: " {state}</p>
                                        <p>"Last review: " {card.last_review.to_string()}</p>
                                        <p>"Previous state: " {previous_state}</p>
                                        <p>"Log: " {log}</p>
                                    </div>
                                }
                                    .into_view()
                            }
                        })
                        .unwrap_or_default()
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn Leaf(leaf: Leaf, revealed: ReadSignal<bool>) -> impl IntoView {
    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="p-5">
                <p class="text-2xl text-center text-white hyphens-auto">{leaf.front()}</p>
            </div>
            <div class=("hidden", move || !revealed())>
                <hr class="border-t-1 border-secondary-750"/>
                <div class="p-5">
                    <p class="text-2xl text-center text-primary-500 hyphens-auto">{leaf.back()}</p>
                </div>
            </div>
        </Card>
    }
}

#[component]
pub fn LeafOverview(
    leaf: Leaf,
    delete_leaf: Action<DeleteLeaf, Result<(), ServerFnError>>,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(true);

    let id = leaf.id();

    view! {
        <Card class="mx-auto relative w-1/2 hover:scale-105 hover:border-primary-500 transition ease-out">
            <A href=format!("/leaf/{}", id) class="flex place-items-center">
                <div class="w-40 p-5 shrink-0">
                    <Icon
                        icon=icondata::FaLeafSolid
                        class=format!(
                            "inline-block size-5 {}",
                            match leaf.card().state {
                                State::New => "text-lime-500",
                                State::Learning => "text-cyan-500",
                                State::Review => "text-emerald-500",
                                State::Relearning => "text-rose-500",
                            },
                        )
                    />

                    <span class="ml-2 font-medium text-white">
                        {format!("{:?}", leaf.card().state)}
                    </span>
                </div>
                <div class="grow border-l-2 border-secondary-750">
                    <div class="p-5">
                        <p class="text-2xl text-center text-white hyphens-auto">{leaf.front()}</p>
                    </div>
                    <div class=("hidden", hidden)>
                        <hr class="border-t-1 border-secondary-750"/>
                        <div class="p-5">
                            <p class="text-2xl text-center text-primary-500 hyphens-auto">
                                {leaf.back()}
                            </p>
                        </div>
                    </div>
                </div>
            </A>
            <Controls class="absolute -top-4 right-4">
                <ControlBtn
                    on_click=move |_| { set_hidden.update(|x| *x = !*x) }
                    size="5"
                    icon=icondata::FaEyeRegular
                />
                <ControlA href=&format!("/leaf/{}", id) size="5" icon=icondata::FaPencilSolid/>
                <ControlAction
                    action=delete_leaf
                    on_submit=move |_| {}
                    size="5"
                    icon=icondata::FaTrashCanRegular
                >
                    <input type="hidden" name="id" value=id/>
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
                <p class="text-2xl text-center text-secondary-750">{text}</p>
            </div>
        </Card>
    }
}

#[component]
pub fn NoLeaf() -> impl IntoView {
    view! {
        <p class="text-xl text-white">
            "No leaf selected. You need to log in and navigate to one of your leaves."
        </p>
    }
}
