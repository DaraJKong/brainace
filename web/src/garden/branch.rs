use crate::{
    garden::stem::{AddStem, Stems},
    ui::{Card, ControlAction, ControlBtn, Controls, FormH1, FormInput, FormSubmit, Modal},
};
use brainace_core::{Branch, Tree};
use leptos::{
    component, create_server_action, create_server_multi_action, create_signal, server,
    use_context, view, IntoView, Params, RwSignal, ServerFnError, SignalGet, SignalUpdate,
    SignalWith,
};
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

#[server(AddBranch, "/api")]
pub async fn add_branch(tree_id: u32, name: String) -> Result<(), ServerFnError> {
    use crate::app::ssr::pool;

    let pool = pool()?;

    Ok(
        sqlx::query("INSERT INTO branches (tree_id, name) VALUES (?, ?)")
            .bind(tree_id)
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
pub fn Branch() -> impl IntoView {
    let tree = use_context::<RwSignal<Tree>>();

    let (editing, set_editing) = create_signal(false);
    let (adding_stem, set_adding_stem) = create_signal(false);

    let edit_branch = create_server_multi_action::<EditBranch>();
    let delete_branch = create_server_action::<DeleteBranch>();
    let add_stem = create_server_multi_action::<AddStem>();

    let params = use_params::<BranchParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let branch = move || tree.map(|tree| tree.get().find_branch(id()));

    view! {
        {move || {
            branch()
                .map(|branch| match branch {
                    Some(branch) => {
                        view! {
                            <div class="flex items-center h-16 px-8 py-1 mb-8 border-b-2 border-primary-500">
                                <p class="text-4xl font-bold text-white tracking-wide">
                                    {branch.name()}
                                </p>
                                <div class="grow"></div>
                                <Controls>
                                    <ControlBtn
                                        on_click=move |_| set_editing.update(|x| *x = true)
                                        size="5"
                                        icon=icondata::FaPencilSolid
                                    />
                                    <ControlBtn
                                        on_click=move |_| set_adding_stem.update(|x| *x = true)
                                        size="5"
                                        icon=icondata::FaPlusSolid
                                    />
                                    <ControlAction
                                        action=delete_branch
                                        on_submit=move |_| {
                                            use_navigate()("/", Default::default());
                                        }

                                        size="5"
                                        icon=icondata::FaTrashCanRegular
                                    >
                                        <input type="hidden" name="id" value=id/>
                                    </ControlAction>
                                </Controls>
                            </div>
                            <Stems branch/>
                        }
                            .into_view()
                    }
                    None => {
                        view! {
                            <pre class="text-xl text-white">
                                {move || format!("No branch with id {} found.", id())}
                            </pre>
                        }
                            .into_view()
                    }
                })
        }}

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
            class="block py-6 px-9 text-2xl text-white rounded-xl outline outline-2 outline-secondary-750 hover:outline-primary-500 hover:scale-105 transition ease-out"
        >
            {branch.name()}
        </A>
    }
}

#[component]
pub fn PendingBranch(input: Option<AddBranch>) -> impl IntoView {
    let text = input.map_or("LOADING".to_string(), |input| input.name);

    view! {
        <div class="block py-6 px-9 rounded-xl border-2 border-secondary-750">
            <p class="text-2xl text-secondary-750 animate-pulse">{text}</p>
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
