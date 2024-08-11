use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// таким образом мы дали доступ манипулировать компонентом ParentChildHome в app.rs
use crate::components::parent_child::{
    parent_child_home::ParentChildHome, parent_child_write_signal::Parent as ParentWriteSignal,
};




#[component] // объясни этот компонент в контексте остальных 
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="container">{children()} </div> }
} 

#[component] // создаем заголовок страницы и помещаем его в контейнер 
// <Container><PageHeader/></Container> - который ниже - как компонент в компонент
pub fn PageHeader() -> impl IntoView {
    view! { 
        <header id="page-header"> 
            <h1>"Leptos Universe Tools LLC"</h1>
        </header>
    }
} 
// то есть получается компоненты - это блоки html, которые заворачиваются друг в друга, как матрешки

// создаем компонент PageFooter, выполняющий функцию тэга <footer>в html</footer>
#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer id="page-footer">
            <p>"© 2024 Leptos Universe Tools LLC"</p>
        </footer>
    }
}

// это навигационная панель на боковой панели
#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav id="main-menu">
            <ul>
                <li class="menu-item">
                <a href="/">"Home"</a>
                </li>             
                <li class="menu-item">
                    <a href="/parent-child">"Parent Child"</a>
                </li> 
            </ul> // доступ к которому мы прописали выше
        </nav>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! { <aside id="sidebar">Sidebar</aside> }
}

// а в этот компонент Content мы помещаем все компоненты, которые будут отображаться на странице 
#[component]
pub fn Content() -> impl IntoView { 
    view! {        
        <Router>
            <main id="content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/parent-child" view=ParentChildHome> // иерархия вложеннных файл и папок друг в друга
                        <Route path="write-signal" view=ParentWriteSignal />
                        <Route path="" view=|| view! { <p>"Click"</p> } />
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tools-app.css"/>

        // sets the document title
        <Title text="Leptos - Я люблю Тебя!"/>

        <Container>
            <PageHeader/> // вот сюда мы поместили созданный выше компонент PageHeader
            <NavBar/> // боковая панель с меню            
            // content for this welcome page
            <Content/> // отображение всех видимых пользователю компонентов страницы 
            <SideBar/> // тут мы поместили созданный выше компонент SideBar
            <PageFooter/> // вот тут мы поместили footer в виде заранее созданного компонента PageFooter       
        </Container>
    }
}

// глянь как расписал сам для себя объяснение того, что происходит ниже, если что не так - поправь)
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { // view! - это макрос leptos для создания html подобной структуры 
        <h1>"Welcome to Leptos!"</h1> 
        <button on:click=on_click>"Click Me: " {count}</button>
    } // здесь дается кнопка, щелкнув на которую, выполняется код выше -- let on_click = move |_| и так далее
} // то есть компилятор leptos понимает, что при клике на button нужно выполнить код выше


/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}



/*
Общий Обзор
Создание Реактивного Сигнала:

let (count, set_count) = create_signal(0);
CopyInsert
create_signal(0) создает реактивный сигнал с начальным значением 0.
count — это геттер, который возвращает текущее значение сигнала.
set_count — это сеттер, который позволяет обновлять значение сигнала.
Определение Действия При Клике:

let on_click = move |_| set_count.update(|count| *count += 1);
CopyInsert
on_click — это замыкание, которое вызывается при клике на кнопку.
set_count.update(|count| *count += 1) обновляет значение сигнала, увеличивая его на 1.
Рендеринг HTML:

view! {
    <h1>"Welcome to Leptos!"</h1> 
    <button on:click=on_click>"Click Me: " {count}</button>
}
CopyInsert
view! — это макрос Leptos для создания HTML-подобной структуры.
<h1>"Welcome to Leptos!"</h1> — это просто заголовок.
<button on:click=on_click>"Click Me: " {count}</button> — это кнопка, которая:
Привязывает событие click к замыканию on_click.
Отображает текущее значение сигнала count внутри кнопки.
Подробное Объяснение
Реактивные Сигналы:

Сигналы в Leptos позволяют создавать реактивные значения, которые могут автоматически обновлять DOM при изменении.
create_signal(0) создает пару (count, set_count), где count — это геттер, а set_count — сеттер.
Замыкания и Действия:

Замыкание on_click определяет, что должно произойти при клике на кнопку.
set_count.update(|count| *count += 1) обновляет значение сигнала, увеличивая его на 1.
HTML и Leptos Макрос:

Макрос view! позволяет вам создавать HTML-подобную структуру, встраивая Rust-код прямо в HTML.
on:click=on_click привязывает событие клика к замыканию on_click.
{count} вставляет текущее значение сигнала count в текст кнопки.
Итог
Ваше объяснение верно и достаточно подробно. Основная идея заключается в том, что Leptos позволяет вам создавать реактивные интерфейсы, где изменение сигналов автоматически обновляет DOM. Вы используете сигналы для хранения состояния, замыкания для определения действий и макрос view! для создания HTML-структуры.





#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { // view! - это макрос leptos для создания html подобной структуры 
        <h1>"Welcome to Leptos!"</h1> 
        <button on:click=on_click>"Click Me: " {count}</button>
    } // здесь дается кнопка, щелкнув на которую, выполняется код выше -- let on_click = move |_| и так далее
} // то есть компилятор leptos понимает, что при клике на button нужно выполнить код выше
// так как в html разметке on:click выполняет действие по нажатию, 
// но в данном случае мы приравниваем on:click из html к on_click переменной, прописанной выше 
// а в on_click переменной прописано необходимое нам действие на языке rust по версии leptos
// в данном случае, это прибавление единицы к начальному значению кнопки count, 
// которая была создана выше как сигнал, доступный для изменения посредством set_count

// и общий смысл в том, что мы сначала создаем сигналы методом create_signal() из библиотеки leptos, чтобы leptos вообще работал на странице - мы так его вносим в html, по сути
// и далее обычно или всегда при помощи замыкания мы манипулируем сигналами в виде переменных count и set_count - for example 
// для достижения нужной нам функциональности и далее просто, 
// например, помещаем функцию прибавления в on_click 
// и здесь фундаментальный момент - мы вкрапляем leptos в html - в данном случае изменяем действие клика
// по кнопке из html привычного поведения в leptos - то есть мы как бы перепрограммируем стандартную функциональность html элементов в реактивные сигналы Leptos
// и так же мы может использовать сигналы отдельно, то есть 
// <button on:click=on_click>А здесь внутри кнопки html мы используем count как состояние нужного нам сигнала и его отображения - в данно случае - это 0
// потому что вверху мы указали 0 в let (count, set_count) = create_signal(0)
// и значит - тут используем {count} в таком виде и в web будет отображаться 0, 
// но фокус в том, что мы изменяет 0 {count} при нажатии на кнопку, действие которого при нажатии 
// перепрограммированно нами на изменение состояния count то есть 0 на +1, 
// что реально выполняется благодаря прописанному нами замыканию из rust, используя сигналы leptos 
// ну это это - let on_click = move |_| set_count.update(|count| *count +=1)
// просто по сути итерируем set_count сигнал, и каждую итерацию замыкаем значение count из set_count
// и возвращаем в set_count обновленное значение каждого count



*/