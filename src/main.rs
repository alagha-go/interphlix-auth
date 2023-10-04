use lambda_http::{service_fn, Response, Request, RequestExt};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() -> Result<()> {
    lambda_http::run(service_fn(handle)).await?;
    Ok(())
}


async fn handle(req: Request) -> Result<Response<String>> {
    let path = req.raw_http_path();
    match path {
        "/login" => login(req).await,
        "/token" => token(req).await,
        "/redirect" => redirect(req).await,
        _ => Ok(Response::builder().status(404).body(String::from("Not Found"))?)
    }
}

async fn login(_req: Request) -> Result<Response<String>> {
    Ok(Response::builder().status(200).header("Content-Type", "text/html").body(String::from("<h1>You Seriously want to login?"))?)
}

async fn token(_req: Request) -> Result<Response<String>> {
    Ok(Response::builder().status(200).header("Content-Type", "text/html").body(String::from("<h1>You Seriously want to a Token?"))?)
}

async fn redirect(_req: Request) -> Result<Response<String>> {
    Ok(Response::builder().status(200).header("Content-Type", "text/html").body(String::from("<h1>You've Successfully been redirected."))?)
}