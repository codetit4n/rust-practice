use axum::{
    extract::{FromRef, Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    ctx::Ctx,
    model::{ModelController, Ticket, TicketForCreate},
    Result,
};

// NOTE: If you want to pass multiple states add them to this struct and pass this
//#[derive(Clone, FromRef)]
//struct AppState {
//    mc: ModelController,
//}

pub fn routes(mc: ModelController) -> Router {
    //let app_state = AppState { mc }; // for multiple states, and pass this
    // in place of mc below
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// REST handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
