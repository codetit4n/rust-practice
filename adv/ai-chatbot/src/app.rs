use crate::model::conversation::{Conversation, Message};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |msg: &String| {
        let user_msg = Message {
            is_user: true,
            text: msg.clone(),
        };
        set_conversation.update(move |c| {
            c.message.push(user_msg);
        })

        // TODO: converse
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="AI Chatbot in rust"/>
        <ChatArea conversation/>
        <TypeAres send/>
    }
}

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
