use crate::state::State;
use actix_web::{get, web::Data, HttpResponse, Responder};

#[get("/metrics")]
pub async fn get_metrics(state: Data<State>) -> impl Responder {
    let metric = crate::metric::get_metric(&state.client, &state.usernames, &state.prefix).await;
    HttpResponse::Ok().body(metric)
}
