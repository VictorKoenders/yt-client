use http_types::mime;

include!(concat!(env!("OUT_DIR"), "/yt_credentials.rs"));

pub fn login_url() -> String {
    #[derive(serde::Serialize)]
    struct LoginUrlRequest {
        scope: &'static str,
        response_type: &'static str,
        redirect_uri: &'static str,
        client_id: &'static str,
    }
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?{}",
        serde_urlencoded::to_string(LoginUrlRequest {
            scope: "https://www.googleapis.com/auth/youtube.readonly",
            response_type: "code",
            redirect_uri: "urn:ietf:wg:oauth:2.0:oob",
            client_id: yt_client_id(),
        })
        .unwrap()
    )
}

pub async fn confirm_login(code: String) -> surf::Result<LoginResponse> {
    #[derive(serde::Serialize)]
    struct Request {
        code: String,
        client_id: &'static str,
        client_secret: &'static str,
        redirect_uri: &'static str,
        grant_type: &'static str,
    }

    let request = Request {
        code,
        client_id: yt_client_id(),
        client_secret: yt_client_secret(),
        redirect_uri: "urn:ietf:wg:oauth:2.0:oob",
        grant_type: "authorization_code",
    };
    let string = surf::post("https://oauth2.googleapis.com/token")
        .content_type(mime::FORM)
        .body(serde_urlencoded::to_string(request).unwrap())
        .recv_string()
        .await?;
    println!("{}", string);
    Ok(serde_json::from_str(&string).unwrap())
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: String,
}

pub async fn refresh_token(refresh_token: &str) -> surf::Result<Token> {
    #[derive(serde::Serialize)]
    struct Request<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        refresh_token: &'a str,
        grant_type: &'a str,
    }
    surf::post("https://oauth2.googleapis.com/token")
        .content_type(mime::FORM)
        .body_string(
            serde_urlencoded::to_string(Request {
                client_id: yt_client_id(),
                client_secret: yt_client_secret(),
                grant_type: "refresh_token",
                refresh_token,
            })
            .unwrap(),
        )
        .recv_json()
        .await
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Token {
    access_token: String,
    expires_in: u64,
    scope: String,
    token_type: String,
}

pub async fn list(token: String) -> surf::Result<Vec<Channel>> {
    #[derive(serde::Deserialize)]
    struct Response {
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        pub items: Vec<ListItem>,
    }

    #[derive(serde::Deserialize)]
    struct ListItem {
        pub snippet: Snippet,
    }
    #[derive(serde::Deserialize)]
    struct Snippet {
        pub title: String,
        #[serde(rename = "channelId")]
        pub channel_id: String,
        pub thumbnails: SnippetThumbnail,
    }
    #[derive(serde::Deserialize)]
    struct SnippetThumbnail {
        #[serde(default)]
        pub default: Option<SnippetThumbnailUrl>,
        #[serde(default)]
        pub medium: Option<SnippetThumbnailUrl>,
        #[serde(default)]
        pub high: Option<SnippetThumbnailUrl>,
    }
    #[derive(serde::Deserialize)]
    struct SnippetThumbnailUrl {
        pub url: String,
    }

    let mut result = Vec::new();

    let mut next_page_token = Option::<String>::None;
    let mut is_first = true;
    while is_first || next_page_token.is_some() {
        is_first = false;
        let mut url =
            "https://www.googleapis.com/youtube/v3/subscriptions?mine=true&part=snippet&maxResults=50"
                .to_string();
        if let Some(token) = next_page_token {
            url += "&pageToken=";
            url += token.as_str();
        }
        let response: Response = surf::get(url)
            .header("Authorization", format!("Bearer {}", token))
            .recv_json()
            .await?;
        for item in response.items {
            result.push(Channel {
                title: item.snippet.title,
                channel_id: item.snippet.channel_id,
                thumbnail: item
                    .snippet
                    .thumbnails
                    .high
                    .or(item.snippet.thumbnails.medium)
                    .or(item.snippet.thumbnails.default)
                    .map(|t| t.url)
                    .unwrap_or_default(),
            });
        }

        next_page_token = response.next_page_token;
    }

    Ok(result)
}

#[derive(serde::Serialize, Debug)]
pub struct Channel {
    pub title: String,
    pub channel_id: String,
    pub thumbnail: String,
}
