use leptos::*;

#[component]
pub fn Parent() -> impl IntoView { // так указываем свой тип для первоначального состояния 
    // let (counter, set_counter) = create_signal::<u16>(0);

    // let increment_counter = |_| set_counter.update(|c| *c += 1);


    view! { <p>"Parent Child Write Signal"</p> } // это надпись в компоненте Parent появляется при клике на слово Write Signal
} // <p>"Сигнал записи Родитель-Ребенок"</p>
