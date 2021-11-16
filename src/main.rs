use lambda_http::{
    handler,
    lambda_runtime::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn get_name(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(match event.query_string_parameters().get("name") {
        Some(name) => {
            match event.path_parameters().get("id") {
                Some(id) => {
                    let json = format!(r#"{{"name": "{}", "pathParam": "{}"}}"#, name, id);
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
                    let json = format!(r#"{{"name": "{}", "pathParam": "{}"}}"#, name, path_param);
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
