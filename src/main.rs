use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use serde_json::json;

// // Wrapper for our core function
// // Its role is to extract the relevant info from the incoming event, and convert the
// // response to json.
// #[tracing::instrument()]
// async fn run_lambda(event: LambdaEvent<Value>) -> Result<Value, Error> {
//     let (event, context) = event.into_parts();
//     tracing::info!(event = ?event, context = ?context);

//     let name = event["name"].as_str();
//     let result = say_hello(name);

//     Ok(json!(result))
// }
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful information from the request

    let name = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("stranger")
        .to_string();

    // Represents an HTTP response
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("Hello, {}!", name),
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_current_span(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use lambda_runtime::{Context, LambdaEvent};

//     #[test]
//     fn test_name_provided() {
//         let name = "world";
//         let result = say_hello(Some(name));
//         assert_eq!(
//             HelloResponse {
//                 message: format!("Hello, {name}!")
//             },
//             result
//         );
//     }

//     #[test]
//     fn test_no_name_provided() {
//         let result = say_hello(None);
//         assert_eq!(
//             HelloResponse {
//                 message: "Hello, stranger!".into()
//             },
//             result
//         );
//     }

//     #[tokio::test]
//     async fn test_wrapper_name_provided() {
//         let name = "world";
//         let event = LambdaEvent::new(json!({ "name": name }), Context::default());
//         let expected_result = json!({ "message": format!("Hello, {name}!") });

//         let result = run_lambda(event).await;

//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert_eq!(result, expected_result);
//     }

//     #[tokio::test]
//     async fn test_wrapper_no_name_provided() {
//         let event = LambdaEvent::new(
//             json!({ "meaningless_key": "meaningless_value" }),
//             Context::default(),
//         );
//         let expected_result = json!({ "message": format!("Hello, stranger!") });

//         let result = run_lambda(event).await;

//         assert!(result.is_ok());
//         let result = result.unwrap();
//         assert_eq!(result, expected_result);
//     }
// }
