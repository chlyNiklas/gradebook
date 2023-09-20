use crate::components::grade::*;
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    let grades = vec![create_signal(4.0), create_signal(4.0), create_signal(4.0)];
    view! {
        {
            grades.into_iter().map(|(grade, set_grade)| view! {<Grade grade=grade set_grade=set_grade />}).collect::<Vec<_>>()}
    }
}
