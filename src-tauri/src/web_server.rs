use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Deserialize;
use chrono::{Duration, NaiveDate};
use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::commands::aggregate_week_activity_logs;

// Define a struct to hold the query parameters
#[derive(Deserialize, Debug)]
struct DateRangeQuery {
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
}

// Custom error type for the Axum handler
enum AppError {
    BadRequest(String),
    InternalServerError(String),
}

// Implement IntoResponse for AppError to convert errors into HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// The main handler function for the /aggregate route
async fn aggregate_handler(
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<String>>, AppError> {
    println!("Received aggregate request with params: {:?}", params);
    println!("Processing request for date range: {} to {}", params.start_date, params.end_date);

    // 1. Parse dates
    let start_date = NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
        .map_err(|e| {
            eprintln!("Failed to parse start date '{}': {}", params.start_date, e);
            AppError::BadRequest("Invalid startDate format. Use YYYY-MM-DD.".to_string())
        })?;

    let end_date = NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d")
        .map_err(|e| {
            eprintln!("Failed to parse end date '{}': {}", params.end_date, e);
            AppError::BadRequest("Invalid endDate format. Use YYYY-MM-DD.".to_string())
        })?;

    println!("Successfully parsed dates: {} to {}", start_date, end_date);

    if start_date > end_date {
        return Err(AppError::BadRequest("startDate cannot be after endDate.".to_string()));
    }

    // 2. Generate list of dates in the range (inclusive)
    let mut dates_to_process = Vec::new();
    let mut current_date = start_date;
    while current_date <= end_date {
        dates_to_process.push(current_date);
        match current_date.checked_add_signed(Duration::days(1)) {
            Some(next_date) => current_date = next_date,
            _ => {
                eprintln!("Date range too large, overflowed.");
                return Err(AppError::InternalServerError("Date range caused an overflow.".to_string()));
            }
        }
    }

    // 3. Format filenames and call the aggregation logic
    let filenames: Vec<String> = dates_to_process
        .into_iter()
        .map(|date| format!("{}", date.format("%Y-%m-%d")))
        .collect();

    let results = aggregate_week_activity_logs(filenames);

    Ok(Json(results))
}

pub async fn start_web_server() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/aggregate", get(aggregate_handler))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 7930));
    println!("Starting server on {}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Successfully bound to address {}", addr);
            listener
        },
        Err(e) => {
            eprintln!("Failed to bind server address {}: {}", addr, e);
            return;
        }
    };

    println!("Server is now listening for connections...");
    if let Err(e) = serve(listener, app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}
