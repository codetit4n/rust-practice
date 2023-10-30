use leptos::*;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String,ServerFnError>{
}
