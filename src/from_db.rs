use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Json},
    Result,
};
use serde::Serialize;
use stargate_grpc::{Query, ResultSet, StargateClient, TryFromRow};

use crate::custom_error::AppError;

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
    let data: ResultSet = (**pool)
        .clone()
        .execute_query(query)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Some error occured 1"))?
        .try_into()
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Some error occured 2"))?;
    let mapper = data
        .mapper()
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Some error occured 3"))?;
    let rows = data
        .rows
        .iter()
        .map(|row| {
            mapper
                .try_unpack(row.to_owned())
                .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Some error occured 4"))
                .unwrap()
        })
        .collect::<Vec<GeneralStat>>();
    let response = FromDBResponse { rows };
    Ok(Json(response))
}
