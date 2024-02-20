use crate::garden::branch::AddBranch;
use brainace_core::Tree;
use leptos::{
    component, create_server_multi_action, create_signal, server, use_context, view, CollectView,
    ErrorBoundary, IntoView, RwSignal, ServerFnError, SignalUpdate,
};
use leptos_icons::Icon;
use leptos_router::MultiActionForm;

use crate::{
    error_template::ErrorTemplate,
    garden::branch::BranchOverview,
    ui::{Card, FormH1, FormInput, FormSubmit, Modal},
    users::get_user,
};

#[server(GetUserTree, "/api")]
pub async fn get_user_tree() -> Result<Tree, ServerFnError> {
    use crate::app::ssr::pool;
    use brainace_core::SqlTree;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    Ok(
        sqlx::query_as::<_, SqlTree>("SELECT * FROM trees WHERE user_id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?
            .into_tree(&pool)
            .await,
    )
}

#[component]
pub fn Tree() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    let tree = use_context::<RwSignal<Tree>>();

    let id = move || match tree {
        Some(tree) => tree().id(),
        None => 0,
    };

    let add_branch = create_server_multi_action::<AddBranch>();

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
            <ul class="flex flex-wrap items-center gap-6">
                {move || match tree {
                    None => {
                        view! { <pre class="text-xl text-white">"Loading branches..."</pre> }
                            .into_view()
                    }
                    Some(tree) => {
                        let branches = tree().branches();
                        if branches.is_empty() {
                            view! { <p class="text-xl text-white">"No branches were found."</p> }
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
                }}
                <li>
                    <button
                        on:click=move |_| set_show_modal.update(|x| *x = true)
                        class="block p-3 text-2xl text-white rounded-full bg-primary-600 hover:bg-primary-500 transition ease-out"
                    >
                        <Icon icon=icondata::FaPlusSolid/>
                    </button>
                </li>
            </ul>
        </ErrorBoundary>
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
                    <input type="hidden" name="tree_id" value=id()/>
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
