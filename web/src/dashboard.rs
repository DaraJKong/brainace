use crate::{
    error_template::ErrorTemplate,
    garden::leaf::get_all_leaves,
    ui::{ActionA, Card, FormH1, Loading},
};
use brainace_core::{utils, State};
use leptos::{component, create_resource, view, ErrorBoundary, IntoView, Transition};

#[component]
pub fn Dashboard() -> impl IntoView {
    let leaves = create_resource(|| (), move |_| get_all_leaves());

    let total = move || leaves().unwrap().unwrap().len();
    let states = move || utils::count_states(leaves().unwrap().unwrap());
    let due_today = move || utils::count_due_today(leaves().unwrap().unwrap());
    let due_now = move || utils::count_due_now(leaves().unwrap().unwrap());

    view! {
        <Transition fallback=move || view! { <Loading/> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    leaves
                        .map(|leaves| match leaves {
                            Err(e) => {
                                view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view()
                            }
                            Ok(_) => {
                                view! {
                                    <StatesBadge total=total() states=states()/>
                                    <DueCard due_today=due_today() due_now=due_now()/>
                                }
                                    .into_view()
                            }
                        })
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn StatesBadge(total: usize, states: (u32, u32, u32, u32)) -> impl IntoView {
    let (new, learning, review, relearning) = states;

    view! {
        <div class="flex items-center rounded-full bg-secondary-750 overflow-hidden">
            <div class="px-8 py-4 font-medium text-2xl text-white rounded-full bg-primary-500">
                {total} " leaves"
            </div>
            <div class="grow"></div>
            <div class="flex px-4">
                <StateBadge qty=new state=State::New/>
                <StateBadge qty=learning state=State::Learning/>
                <StateBadge qty=review state=State::Review/>
                <StateBadge qty=relearning state=State::Relearning/>
            </div>
        </div>
    }
}

#[component]
pub fn StateBadge(qty: u32, state: State) -> impl IntoView {
    let (color, border, label) = match state {
        State::New => ("text-lime-500", "border-lime-500", "new"),
        State::Learning => ("text-cyan-500", "border-cyan-500", "learning"),
        State::Review => ("text-emerald-500", "border-emerald-500", "review"),
        State::Relearning => ("text-rose-500", "border-rose-500", "relearning"),
    };

    view! {
        <div class=format!("p-4 mr-4 font-medium text-2xl text-white border-b-2 {}", border)>
            <span class=color>{qty}</span>
            " "
            {label}
        </div>
    }
}

#[component]
pub fn DueCard(due_today: usize, due_now: usize) -> impl IntoView {
    view! {
        <div class="h-full flex justify-center items-center">
            <Card class="w-1/2">
                <div class="grid grid-cols-2">
                    <div class="p-6">
                        <FormH1 text="Due today"/>
                        <FormH1 text=&format!("{}", due_today)/>
                        <ActionA href="/review-today" msg="REVIEW ALL"/>
                    </div>
                    <div class="p-6">
                        <FormH1 text="Due now"/>
                        <FormH1 text=&format!("{}", due_now)/>
                        <ActionA href="/review" msg="REVIEW"/>
                    </div>
                </div>
            </Card>
        </div>
    }
}
