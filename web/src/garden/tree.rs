use crate::garden::branch::{AddBranch, PendingBranch};
use brainace_core::Tree;
use leptos::{
    component, create_effect, create_server_multi_action, create_signal, server, use_context, view,
    CollectView, ErrorBoundary, IntoView, RwSignal, ServerFnError, SignalGet, SignalUpdate,
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

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

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
    let submissions = add_branch.submissions();

    create_effect(move |handled| {
        let handled = handled.unwrap_or_default();
        if add_branch.version().get() > handled {
            if let Some(submission) = submissions.get().last() {
                if let Some(Ok(value)) = submission.value.get() {
                    tree.unwrap().update(|tree| {
                        tree.add_branch(value.0, &value.1, value.2);
                    });
                    return handled + 1;
                }
            }
        }
        handled
    });

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorTemplate errors=errors/> }
        }>
            {move || {
                let existing_branches = move || match tree {
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
                };
                let pending_branches = move || {
                    submissions()
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
                                class="block p-3 text-2xl text-white rounded-full bg-primary-600 hover:bg-primary-500 transition ease-out"
                            >
                                <Icon icon=icondata::FaPlusSolid/>
                            </button>
                        </li>
                    </ul>
                }
            }}

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
