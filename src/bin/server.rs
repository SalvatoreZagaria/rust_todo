use warp::Filter;
use tasks_lib::*;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Shared, thread-safe
    let tasks = Arc::new(Mutex::new(load_tasks(".tasks.json").unwrap_or_default()));

    let list = {
        let tasks = tasks.clone();
        warp::path("tasks")
            .and(warp::get())
            .map(move || {
                let tasks = tasks.lock().unwrap();
                warp::reply::json(&*tasks)
            })
    };

    let add = {
        let tasks = tasks.clone();
        warp::path("tasks")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |body: serde_json::Value| {
                let mut tasks = tasks.lock().unwrap();
                if let Some(desc) = body.get("description").and_then(|d| d.as_str()) {
                    add_task(&mut tasks, desc);
                    save_tasks(&tasks, ".tasks.json").unwrap();
                    warp::reply::with_status("Created", warp::http::StatusCode::CREATED)
                } else {
                    warp::reply::with_status("Bad Request", warp::http::StatusCode::BAD_REQUEST)
                }
            })
    };

    let done = {
        let tasks = tasks.clone();
        warp::path!("tasks" / String / "done")
            .and(warp::put())
            .map(move |id: String| {
                let mut tasks = tasks.lock().unwrap();
                if mark_done(&id, &mut tasks) {
                    save_tasks(&tasks, ".tasks.json").unwrap();
                    warp::reply::with_status("OK", warp::http::StatusCode::OK)
                } else {
                    warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND)
                }
            })
    };

    let remove = {
        let tasks = tasks.clone();
        warp::path!("tasks" / String / "delete")
            .and(warp::delete())
            .map(move |id: String| {
                let mut tasks = tasks.lock().unwrap();
                if remove_task(&mut tasks, &id) {
                    save_tasks(&tasks, ".tasks.json").unwrap();
                    warp::reply::with_status("OK", warp::http::StatusCode::OK)
                } else {
                    warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND)
                }
            })
    };

    let clear_done = {
        let tasks = tasks.clone();
        warp::path("tasks")
            .and(warp::path("done"))
            .and(warp::delete())
            .map(move || {
                let mut tasks = tasks.lock().unwrap();
                clear_completed(&mut tasks);
                save_tasks(&tasks, ".tasks.json").unwrap();
                warp::reply::with_status("OK", warp::http::StatusCode::OK)
            })
    };

    let routes = list.or(add).or(done).or(remove).or(clear_done);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
