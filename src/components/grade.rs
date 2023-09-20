use leptos::*;

#[component]
pub fn Grade(grade: ReadSignal<f32>,set_grade: WriteSignal<f32>) -> impl IntoView {
    view! {
        <input type="number"
            style=move || {
                if 4.0 > grade.get()  {
                    "background-color: red"
                } else if 5.0 < grade.get() {
                    "background-color: green"
                } else {
                    "background-color: yellow"
                }
            }
            on:input=move |ev| {
                let new_value = event_target_value(&ev).parse::<f32>().unwrap();
                set_grade.set(event_target_value(&ev).parse::<f32>().unwrap());
                if 6.0 < new_value {
                    set_grade.set(6.0);
                } else if 1.0 > new_value {
                    set_grade.set(1.0)
                }
            }
            step="0.25"

        prop:value=grade
        />
    }
    // add code here
}
