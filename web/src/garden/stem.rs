use crate::{
    error_template::ErrorTemplate,
    garden::leaf::{get_leaves, AddLeaf, DeleteLeaf, Leaves},
    ui::{
        Card, ControlA, ControlAction, ControlBtn, Controls, FormH1, FormInput, FormSubmit, Modal,
    },
};
use brainace_core::Stem;
use leptos::{
    component, create_resource, create_server_action, create_server_multi_action, create_signal,
    server, view, Action, CollectView, ErrorBoundary, IntoView, MultiAction, Params, ServerFnError,
    SignalGet, SignalUpdate, SignalWith, Transition,
};
use leptos_router::{use_navigate, use_params, MultiActionForm, Params, A};

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
            .map(|sql_stem| sql_stem.into_stem())
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
    let delete_leaf = create_server_action::<DeleteLeaf>();

    let params = use_params::<StemParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

    let stem = create_resource(
        move || (id(), edit_stem.version().get()),
        move |_| get_stem(id()),
    );

    let leaves = create_resource(
        move || (add_leaf.version().get(), delete_leaf.version().get()),
        move |_| get_leaves(id()),
    );
    let submissions = add_leaf.submissions();

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
                                    <div class="flex items-center h-16 px-8 py-1 mb-8 border-b-2 border-primary-500">
                                        <p class="text-4xl font-bold text-white tracking-wide">
                                            {stem.name()}
                                        </p>
                                        <div class="grow"></div>
                                        <Controls>
                                            <ControlBtn
                                                on_click=move |_| set_editing.update(|x| *x = true)
                                                size="5"
                                                icon=icondata::FaPencilSolid
                                            />
                                            <ControlBtn
                                                on_click=move |_| set_adding_leaf.update(|x| *x = true)
                                                size="5"
                                                icon=icondata::FaPlusSolid
                                            />
                                            <ControlAction
                                                action=delete_stem
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
                                    <Leaves
                                        leaves=leaves
                                        delete_leaf=delete_leaf
                                        submissions=submissions
                                    />
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
        <Card class="mx-auto relative w-1/3 hover:scale-105 hover:border-primary-500 transition ease-out">
            <A href=format!("/stem/{}", id) class="block p-5">
                <p class="text-2xl text-center text-white hyphens-auto">{stem.name()}</p>
            </A>
            <Controls class="absolute -top-4 right-4">
                <ControlA href=&format!("/stem/{}", id) size="5" icon=icondata::FaPencilSolid/>
                <ControlAction
                    action=delete_stem
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
pub fn PendingStem(input: Option<AddStem>) -> impl IntoView {
    let text = input.map_or("LOADING".to_string(), |input| input.name);

    view! {
        <Card class="mx-auto relative w-1/3">
            <div class="animate-pulse p-4">
                <p class="text-2xl text-center text-secondary-750">{text}</p>
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
