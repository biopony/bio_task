use crate::backend;
use crate::errors::FinError;
use crate::global::ROOT;
use crate::server;
use std::collections::HashMap;

lazy_static! {
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "portfolio_server"));
}

pub fn get_incomplete_by_proj_id(
    proj_id: i64,
    res_tasks_backend: Result<impl backend::TasksBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!("1");
    let task_backend = res_tasks_backend?;
    dbg!("2");
    let resp =
        task_backend
            .get_incomplete_by_proj_id(proj_id)
            .map_err(|err| {
                lineError!(LOGGER, err);
                warp::reject::custom(FinError::ServerErr)
            })?;

    dbg!(proj_id);
    let reply = serde_json::to_string(&resp).map_err(|err| {
        lineError!(LOGGER, err);
        warp::reject::custom(err)
    })?;
    Ok(reply)
}

pub fn get_all_tasks(
    res_tasks_backend: Result<impl backend::TasksBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let task_backend = res_tasks_backend?;
    let resp = task_backend.get_all_tasks().map_err(|err| {
        lineError!(LOGGER, err);
        warp::reject::custom(FinError::ServerErr)
    })?;

    let reply = serde_json::to_string(&resp).map_err(|err| {
        lineError!(LOGGER, err);
        warp::reject::custom(err)
    })?;
    Ok(reply)
}

pub fn create_task(
    res_tasks_backend: Result<impl backend::TasksBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let task_backend = res_tasks_backend?;
    // let resp = task_backend
    //     .create_port_a(&user_id, &data.goal_id, &data.stock_percent)
    //     .map_err(|err| {
    //         error!(LOGGER, "{}: {}", line!(), err);
    //         warp::reject::custom(FinError::ServerErr)
    //     })?;

    // let reply = serde_json::to_string(&resp).map_err(|err| {
    //     error!(LOGGER, "{}: {}", line!(), err);
    //     warp::reject::custom(err)
    // })?;
    // Ok(warp::reply::with_status(
    //     reply,
    //     warp::http::StatusCode::CREATED,
    // ))
    // unimplemented!()
    Ok("")
}
