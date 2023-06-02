use actix_web::{http::StatusCode, web::Data};
use stargate_grpc::{
    result::{ColumnPositions, TryFromRow},
    Query, ResultSet, StargateClient,
};

use crate::custom_error::AppError;

pub struct QueryResult<T> {
    pub rows: Vec<T>,
    pub paging_state: Option<Vec<u8>>,
}

pub async fn cql_query<T>(
    pool: &Data<StargateClient>,
    query: Query,
) -> Result<QueryResult<T>, AppError>
where
    T: TryFromRow + ColumnPositions,
{
    let data: ResultSet = (**pool.clone())
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
        .collect::<Vec<T>>();
    Ok(QueryResult {
        rows,
        paging_state: data.paging_state,
    })
}
