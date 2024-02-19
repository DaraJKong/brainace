use crate::{
    error_template::ErrorTemplate,
    garden::branch::{AddBranch, Branches},
    ui::{Card, FormH1, FormInput, FormSubmit, Loading, Modal},
};
use brainace_core::Tree;
use leptos::{
    component, create_resource, create_server_multi_action, create_signal, server, view,
    ErrorBoundary, IntoView, ServerFnError, SignalUpdate, Transition,
};
use leptos_router::MultiActionForm;

#[server(GetTree, "/api")]
pub async fn get_tree() -> Result<Option<Tree>, ServerFnError> {
    use crate::{app::ssr::pool, users::get_user};
    use brainace_core::SqlTree;
    use futures::future::OptionFuture;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    Ok(OptionFuture::from(
        sqlx::query_as::<_, SqlTree>("SELECT * FROM trees WHERE user_id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await?
            .map(|sql_tree| sql_tree.into_tree(&pool)),
    )
    .await)
}

#[component]
pub fn Tree() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    let add_branch = create_server_multi_action::<AddBranch>();

    let tree = create_resource(|| (), move |_| get_tree());

    view! {
        <Transition fallback=move || view! { <Loading/> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    tree()
                        .map(move |tree| match tree {
                            Err(e) => {
                                view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view()
                            }
                            Ok(None) => view! { <pre>"No tree was found."</pre> }.into_view(),
                            Ok(Some(tree)) => {
                                view! { <Branches tree_id=tree.id() set_show_modal add_branch/> }
                            }
                        })
                        .unwrap_or_default()
                }}
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
                            {move || {
                                tree()
                                    .map(|tree| {
                                        tree.map(|tree| {
                                            tree.map(|tree| {
                                                view! {
                                                    <input type="hidden" name="tree_id" value=tree.id()/>
                                                }
                                            })
                                        })
                                    })
                            }}

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
            </ErrorBoundary>
        </Transition>
    }
}
