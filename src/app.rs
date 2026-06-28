use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| set_count.update(|cont| *cont += 10)
            class:_red=move || count.get() % 2 == 1

            style="position: absolute"
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
        >
            "Click me: "
            {count}
        </button>

        <p>
            "Double count: "
            {double_count}
        </p>

        <progress
            max="50"

            value=double_count
        />
    }
}