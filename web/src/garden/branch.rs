use super::i;
use crate::{
    error_template::ErrorTemplate,
    garden::stem::{AddStem, Stems},
    ui::{Card, ControlAction, ControlBtn, Controls, FormH1, FormInput, FormSubmit, Modal},
    users::get_user,
};
use brainace_core::Branch;
use leptos::{
    component, create_resource, create_server_action, create_server_multi_action, create_signal,
    server, view, CollectView, ErrorBoundary, IntoView, Params, ServerFnError, SignalGet,
    SignalUpdate, SignalWith, Transition,
};
use leptos_icons::Icon;
use leptos_router::{use_navigate, use_params, MultiActionForm, Params, A};

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

#[derive(Params, PartialEq)]
struct BranchParams {
    id: u32,
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
