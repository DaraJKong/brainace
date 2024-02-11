use brainace_core::{Branch, Leaf, Stem};
use icondata as i;
use leptos::{
    component, create_resource, create_server_action, create_server_multi_action, create_signal,
    server, view, Action, ErrorBoundary, IntoView, MultiAction, Params, ServerFnError,
    SignalUpdate, SignalWith, Transition,
};
use leptos::{CollectView, SignalGet};
use leptos_icons::Icon;
use leptos_router::{use_navigate, use_params, MultiActionForm, Params, A};

use crate::error_template::ErrorTemplate;
use crate::ui::{Card, ControlAction, ControlBtn, Controls, FormH1, FormInput, FormSubmit, Modal};
use crate::users::get_user;

#[server(GetBranch, "/api")]
pub async fn get_branch(id: u32) -> Result<Branch, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlBranch;

    let pool = pool()?;

    Ok(
        sqlx::query_as::<_, SqlBranch>("SELECT * FROM branches WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?
            .into_branch(&pool)
            .await,
    )
}

#[server(GetBranches, "/api")]
pub async fn get_branches() -> Result<Vec<Branch>, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlBranch;
    use futures::future::join_all;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    Ok(join_all(
        sqlx::query_as::<_, SqlBranch>("SELECT * FROM branches WHERE user_id = ?")
            .bind(id)
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|branch| branch.into_branch(&pool)),
    )
    .await)
}

#[server(AddBranch, "/api")]
pub async fn add_branch(name: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    Ok(
        sqlx::query("INSERT INTO branches (user_id, name) VALUES (?, ?)")
            .bind(id)
            .bind(name)
            .execute(&pool)
            .await
            .map(|_| ())?,
    )
}

#[server(EditBranch, "/api")]
pub async fn edit_branch(id: u32, name: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("UPDATE branches SET name = $2 WHERE id = $1")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[server(DeleteBranch, "/api")]
pub async fn delete_branch(id: u32) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM branches WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[server(GetStem, "/api")]
pub async fn get_stem(id: u32) -> Result<Stem, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlStem;

    let pool = pool()?;

    Ok(
        sqlx::query_as::<_, SqlStem>("SELECT * FROM stems WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?
            .into_stem(),
    )
}

#[server(GetStems, "/api")]
pub async fn get_stems(branch_id: u32) -> Result<Vec<Stem>, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlStem;

    let pool = pool()?;

    Ok(
        sqlx::query_as::<_, SqlStem>("SELECT * FROM stems WHERE branch_id = ?")
            .bind(branch_id)
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|branch| branch.into_stem())
            .collect(),
    )
}

#[server(AddStem, "/api")]
pub async fn add_stem(branch_id: u32, name: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(
        sqlx::query("INSERT INTO stems (branch_id, name) VALUES (?, ?)")
            .bind(branch_id)
            .bind(name)
            .execute(&pool)
            .await
            .map(|_| ())?,
    )
}

#[server(EditStem, "/api")]
pub async fn edit_stem(id: u32, name: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("UPDATE stems SET name = $2 WHERE id = $1")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[server(DeleteStem, "/api")]
pub async fn delete_stem(id: u32) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM stems WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
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
pub fn Branches() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    let add_branch = create_server_multi_action::<AddBranch>();
    let submissions = add_branch.submissions();

    let branches = create_resource(
        move || (add_branch.version().get()),
        move |_| get_branches(),
    );

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let existing_branches = {
                        move || {
                            branches
                                .get()
                                .map(move |branches| match branches {
                                    Err(e) => {
                                        view! { <pre>"Server Error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(branches) => {
                                        if branches.is_empty() {
                                            view! {
                                                <p class="text-xl text-white">"No branches were found."</p>
                                            }
                                                .into_view()
                                        } else {
                                            branches
                                                .into_iter()
                                                .map(move |branch| {
                                                    view! {
                                                        <li>
                                                            <BranchOverview branch=branch/>
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
                    let pending_branches = move || {
                        submissions
                            .get()
                            .into_iter()
                            .filter(|submission| submission.pending().get())
                            .map(|submission| {
                                view! {
                                    <li>
                                        <PendingBranch input=submission.input.get()/>
                                    </li>
                                }
                            })
                            .collect_view()
                    };
                    view! {
                        <ul class="flex flex-wrap items-center gap-6">
                            {existing_branches} {pending_branches} <li>
                                <button
                                    on:click=move |_| set_show_modal.update(|x| *x = true)
                                    class="block p-3 text-2xl text-white rounded-full bg-violet-600 hover:bg-violet-500 transition ease-out"
                                >
                                    <Icon icon=i::FaPlusSolid/>
                                </button>
                            </li>
                        </ul>
                    }
                }}

            </ErrorBoundary>
        </Transition>
        <Modal
            id="add_branch_modal"
            show=show_modal
            on_blur=move |_| set_show_modal.update(|x| *x = false)
        >
            <Card class="w-1/3 p-6">
                <MultiActionForm
                    action=add_branch
                    on:submit=move |_| set_show_modal.update(|x| *x = false)
                >
                    <FormH1 text="Create a new branch"/>
                    <FormInput
                        input_type="text"
                        id="Name"
                        label="Name"
                        placeholder="Name"
                        name="name"
                    />
                    <FormSubmit msg="ADD"/>
                </MultiActionForm>
            </Card>
        </Modal>
    }
}

#[derive(Params, PartialEq)]
struct BranchParams {
    id: u32,
}

#[component]
pub fn Branch() -> impl IntoView {
    let (editing, set_editing) = create_signal(false);
    let (adding_stem, set_adding_stem) = create_signal(false);

    let edit_branch = create_server_multi_action::<EditBranch>();
    let delete_branch = create_server_action::<DeleteBranch>();
    let add_stem = create_server_multi_action::<AddStem>();

    let params = use_params::<BranchParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let branch = create_resource(
        move || (id(), edit_branch.version().get()),
        move |_| get_branch(id()),
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    branch
                        .get()
                        .map(move |branch| match branch {
                            Err(e) => {
                                view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view()
                            }
                            Ok(branch) => {
                                view! {
                                    <div class="flex items-center h-16 px-8 py-1 mb-8 border-b-2 border-violet-500">
                                        <p class="text-4xl font-bold text-white tracking-wide">
                                            {branch.name()}
                                        </p>
                                        <div class="grow"></div>
                                        <Controls>
                                            <ControlBtn
                                                on_click=move |_| set_editing.update(|x| *x = true)
                                                size="5"
                                                icon=i::FaPencilSolid
                                            />
                                            <ControlBtn
                                                on_click=move |_| set_adding_stem.update(|x| *x = true)
                                                size="5"
                                                icon=i::FaPlusSolid
                                            />
                                            <ControlAction
                                                action=delete_branch
                                                on_submit=move |_| {
                                                    use_navigate()("/", Default::default());
                                                }

                                                size="5"
                                                icon=i::FaTrashCanRegular
                                            >
                                                <input type="hidden" name="id" value=id/>
                                            </ControlAction>
                                        </Controls>
                                    </div>
                                    <Stems branch_id=id() add_stem=add_stem/>
                                }
                                    .into_view()
                            }
                        })
                        .unwrap_or_default()
                }}

            </ErrorBoundary>
        </Transition>
        <Modal
            id="edit_branch_modal"
            show=editing
            on_blur=move |_| set_editing.update(|x| *x = false)
        >
            <Card class="w-1/3 p-6">
                <MultiActionForm
                    action=edit_branch
                    on:submit=move |_| set_editing.update(|x| *x = false)
                >
                    <FormH1 text="Editing branch"/>
                    <input type="hidden" name="id" value=id/>
                    <FormInput
                        input_type="text"
                        id="Name"
                        label="Name"
                        placeholder="Name"
                        name="name"
                    />
                    <FormSubmit msg="SAVE"/>
                </MultiActionForm>
            </Card>
        </Modal>
        <Modal
            id="add_stem_modal"
            show=adding_stem
            on_blur=move |_| set_adding_stem.update(|x| *x = false)
        >
            <Card class="w-1/3 p-6">
                <MultiActionForm
                    action=add_stem
                    on:submit=move |_| set_adding_stem.update(|x| *x = false)
                >
                    <FormH1 text="Grow a stem"/>
                    <input type="hidden" name="branch_id" value=id/>
                    <FormInput
                        input_type="text"
                        id="Name"
                        label="Name"
                        placeholder="Name"
                        name="name"
                    />
                    <FormSubmit msg="ADD"/>
                </MultiActionForm>
            </Card>
        </Modal>
    }
}

#[component]
pub fn BranchOverview(branch: Branch) -> impl IntoView {
    view! {
        <A
            href=format!("/branch/{}", branch.id())
            class="block py-6 px-9 text-2xl text-white rounded-xl outline outline-2 outline-gray-750 hover:outline-violet-500 hover:scale-105 transition ease-out"
        >
            {branch.name()}
        </A>
    }
}

#[component]
pub fn PendingBranch(input: Option<AddBranch>) -> impl IntoView {
    let text = input.map_or("LOADING".to_string(), |input| input.name);

    view! {
        <div class="block py-6 px-9 rounded-xl border-2 border-gray-750">
            <p class="text-2xl text-gray-750 animate-pulse">{text}</p>
        </div>
    }
}

#[component]
pub fn NoBranch() -> impl IntoView {
    view! {
        <p class="text-xl text-white">
            "No branch selected. You need to log in and navigate to one of your branches."
        </p>
    }
}

#[component]
pub fn Stems(
    branch_id: u32,
    add_stem: MultiAction<AddStem, Result<(), ServerFnError>>,
) -> impl IntoView {
    let delete_stem = create_server_action::<DeleteStem>();

    let submissions = add_stem.submissions();

    let stems = create_resource(
        move || (add_stem.version().get(), delete_stem.version().get()),
        move |_| get_stems(branch_id),
    );

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let existing_stems = move || {
                        stems
                            .get()
                            .map(|stems| match stems {
                                Err(e) => {
                                    view! { <pre>"Server Error: " {e.to_string()}</pre> }
                                        .into_view()
                                }
                                Ok(stems) => {
                                    if stems.is_empty() {
                                        view! {
                                            <p class="text-2xl text-white">"No stems were found."</p>
                                        }
                                            .into_view()
                                    } else {
                                        stems
                                            .into_iter()
                                            .map(|stem| {
                                                view! {
                                                    <li>
                                                        <StemOverview stem=stem delete_stem=delete_stem/>
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    }
                                }
                            })
                    };
                    let pending_stems = move || {
                        submissions
                            .get()
                            .into_iter()
                            .filter(|submission| submission.pending().get())
                            .map(|submission| {
                                view! {
                                    <li>
                                        <PendingStem input=submission.input.get()/>
                                    </li>
                                }
                            })
                            .collect_view()
                    };
                    view! {
                        <ul class="flex flex-col space-y-8">{existing_stems} {pending_stems}</ul>
                    }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[derive(Params, PartialEq)]
struct StemParams {
    id: u32,
}

#[component]
pub fn Stem() -> impl IntoView {
    let (editing, set_editing) = create_signal(false);
    let (adding_leaf, set_adding_leaf) = create_signal(false);

    let edit_stem = create_server_multi_action::<EditStem>();
    let delete_stem = create_server_action::<DeleteStem>();
    let add_leaf = create_server_multi_action::<AddLeaf>();

    let params = use_params::<StemParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let stem = create_resource(
        move || (id(), edit_stem.version().get()),
        move |_| get_stem(id()),
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    stem.get()
                        .map(move |stem| match stem {
                            Err(e) => {
                                view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view()
                            }
                            Ok(stem) => {
                                view! {
                                    <div class="flex items-center h-16 px-8 py-1 mb-8 border-b-2 border-violet-500">
                                        <p class="text-4xl font-bold text-white tracking-wide">
                                            {stem.name()}
                                        </p>
                                        <div class="grow"></div>
                                        <Controls>
                                            <ControlBtn
                                                on_click=move |_| set_editing.update(|x| *x = true)
                                                size="5"
                                                icon=i::FaPencilSolid
                                            />
                                            <ControlBtn
                                                on_click=move |_| set_adding_leaf.update(|x| *x = true)
                                                size="5"
                                                icon=i::FaPlusSolid
                                            />
                                            <ControlAction
                                                action=delete_stem
                                                on_submit=move |_| {
                                                    use_navigate()("/", Default::default());
                                                }

                                                size="5"
                                                icon=i::FaTrashCanRegular
                                            >
                                                <input type="hidden" name="id" value=id/>
                                            </ControlAction>
                                        </Controls>
                                    </div>
                                    <Leaves stem_id=id() add_leaf=add_leaf/>
                                }
                                    .into_view()
                            }
                        })
                        .unwrap_or_default()
                }}

            </ErrorBoundary>
        </Transition>
        <Modal
            id="edit_stem_modal"
            show=editing
            on_blur=move |_| set_editing.update(|x| *x = false)
        >
            <Card class="w-1/3 p-6">
                <MultiActionForm
                    action=edit_stem
                    on:submit=move |_| set_editing.update(|x| *x = false)
                >
                    <FormH1 text="Editing stem"/>
                    <input type="hidden" name="id" value=id/>
                    <FormInput
                        input_type="text"
                        id="Name"
                        label="Name"
                        placeholder="Name"
                        name="name"
                    />
                    <FormSubmit msg="SAVE"/>
                </MultiActionForm>
            </Card>
        </Modal>
        <Modal
            id="add_leaf_modal"
            show=adding_leaf
            on_blur=move |_| set_adding_leaf.update(|x| *x = false)
        >
            <Card class="w-1/3 p-6">
                <MultiActionForm
                    action=add_leaf
                    on:submit=move |_| set_adding_leaf.update(|x| *x = false)
                >
                    <FormH1 text="Grow a leaf"/>
                    <input type="hidden" name="stem_id" value=id/>
                    <FormInput
                        input_type="text"
                        id="Front"
                        label="Front"
                        placeholder="Front"
                        name="front"
                    />
                    <FormInput
                        input_type="text"
                        id="Back"
                        label="Back"
                        placeholder="Back"
                        name="back"
                    />
                    <FormSubmit msg="ADD"/>
                </MultiActionForm>
            </Card>
        </Modal>
    }
}

#[component]
pub fn StemOverview(
    stem: Stem,
    delete_stem: Action<DeleteStem, Result<(), ServerFnError>>,
) -> impl IntoView {
    let id = stem.id();

    view! {
        <Card class="mx-auto relative w-1/3 hover:scale-105 hover:border-violet-500 transition ease-out">
            <A href=format!("/stem/{}", id) class="block p-5">
                <p class="text-2xl text-center text-white hyphens-auto">{stem.name()}</p>
            </A>
            <Controls class="absolute -top-4 right-4">
                <ControlBtn on_click=move |_| {} size="5" icon=i::FaPencilSolid/>
                <ControlAction
                    action=delete_stem
                    on_submit=move |_| {}
                    size="5"
                    icon=i::FaTrashCanRegular
                >
                    <input type="hidden" name="id" value=id/>
                </ControlAction>
            </Controls>
        </Card>
    }
}

#[component]
pub fn PendingStem(input: Option<AddStem>) -> impl IntoView {
    let text = input.map_or("LOADING".to_string(), |input| input.name);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="animate-pulse p-4">
                <p class="text-2xl text-center text-gray-750">{text}</p>
            </div>
        </Card>
    }
}

#[component]
pub fn NoStem() -> impl IntoView {
    view! {
        <p class="text-xl text-white">
            "No stem selected. You need to log in and navigate to one of your stems."
        </p>
    }
}

#[component]
pub fn Leaves(
    stem_id: u32,
    add_leaf: MultiAction<AddLeaf, Result<(), ServerFnError>>,
) -> impl IntoView {
    let delete_leaf = create_server_action::<DeleteLeaf>();

    let submissions = add_leaf.submissions();

    let leaves = create_resource(
        move || (add_leaf.version().get(), delete_leaf.version().get()),
        move |_| get_leaves(stem_id),
    );

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
pub fn Leaf(
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
