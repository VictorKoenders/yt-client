use http_types::mime;
use tide::{Request, Response, Result, Route};

pub fn mount(mut route: Route<crate::State>) {
    route.at("refresh-token").post(refresh_token);
    route.at("redirect-login").get(redirect_login);
    route.at("confirm_code").post(confirm_code);
    route.at("subscription/list").post(subscription_list);
}

pub async fn refresh_token(mut req: Request<crate::State>) -> Result {
    #[derive(serde::Deserialize)]
    struct Login {
        refresh_token: String,
    }
    let body = req.body_json::<Login>().await?;
    match crate::yt::refresh_token(&body.refresh_token).await {
        Ok(response) => json(response),
        Err(e) => Err(tide::Error::from_debug(e)),
    }
}

#[derive(serde::Serialize)]
struct LoginStatus {
    logged_in: bool,
}

pub async fn redirect_login(_: Request<crate::State>) -> Result {
    let url = crate::yt::login_url();
    Ok(tide::Redirect::new(url).into())
}

fn json<T: serde::Serialize>(t: T) -> Result {
    let str = serde_json::to_string(&t).unwrap();
    Ok(Response::builder(200)
        .content_type(mime::JSON)
        .body(str)
        .build())
}

pub async fn confirm_code(mut req: Request<crate::State>) -> Result {
    #[derive(serde::Deserialize)]
    struct Body {
        code: String,
    }
    let code = req.body_json::<Body>().await?;
    match crate::yt::confirm_login(code.code).await {
        Ok(response) => json(response),
        Err(e) => Err(tide::Error::from_debug(e)),
    }
}

pub async fn subscription_list(mut req: Request<crate::State>) -> Result {
    let token = req.body_string().await?;
    match crate::yt::list(token).await {
        Ok(response) => json(response),
        Err(e) => Err(tide::Error::from_debug(e)),
    }
}
