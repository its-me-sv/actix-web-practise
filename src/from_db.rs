use actix_web::{
    get,
    web::{Data, Json},
    Result,
};
use serde::Serialize;
use stargate_grpc::{Query, StargateClient, TryFromRow};

use crate::{custom_error::AppError, data_fetcher::cql_query};

#[derive(TryFromRow, Serialize)]
struct GeneralStat {
    year: i32,
    total_session_time: i32,
    total_views: i32,
}

#[derive(Serialize)]
struct FromDBResponse {
    rows: Vec<GeneralStat>,
}

#[get("/from-db")]
async fn fetch_from_db(pool: Data<StargateClient>) -> Result<Json<FromDBResponse>, AppError> {
    let query = Query::builder()
        .keyspace("portfolio")
        .query("SELECT * FROM general_stat;")
        .build();
    let data = cql_query::<GeneralStat>(&pool, query.clone()).await?;
    let response = FromDBResponse { rows: data.rows };
    Ok(Json(response))
}
