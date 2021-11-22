use lambda_http::{
    handler,
    lambda_runtime::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};
use serde::{Serialize, Deserialize};
// use serde_json::Result;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Serialize, Deserialize)]
struct JsonResponse {
    name: String,
    path_param: String
}

fn create_json_resp(name: String, path_param: String) -> serde_json::Result<String> {
    let resp = JsonResponse {
        name,
        path_param
    };

    let json_resp = serde_json::to_string(&resp)?;
    Ok(json_resp)
}

async fn get_name(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(match event.query_string_parameters().get("name") {
        Some(name) => {
            match event.path_parameters().get("id") {
                Some(id) => {
                    // let json = format!(r#"{{"name": "{}", "pathParam": "{}"}}"#, name, id);
                    let json = create_json_resp(name.to_string(), id.to_string())?;
                    Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
                        .header("Access-Control-Allow-Credential", "true")
                        .header("Access-Control-Allow-Origin", "*")
                        .body(json)
                        .expect("failed to render response")
                }
                _ => {
                    let path_param = "nothing";
                    // let json = format!(r#"{{"name": "{}", "pathParam": "{}"}}"#, name, path_param);
                    let json = create_json_resp(name.to_string(), path_param.to_string())?;
                    Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
                        .header("Access-Control-Allow-Credential", "true")
                        .header("Access-Control-Allow-Origin", "*")
                        .body(json)
                        .expect("failed to render response")
                }
            }
        },
        _ => Response::builder()
            .status(400)
            .body("error".into())
            .expect("failed to render response"),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(get_name)).await?;
    Ok(())
}
