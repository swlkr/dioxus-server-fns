#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_server::prelude::*;

fn main() {
    #[cfg(feature = "front")]
    dioxus_web::launch_cfg(app, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "ssr")]
    {
        GetMeaning::register().unwrap();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9001));
                axum::Server::bind(&addr)
                    .serve(
                        axum::Router::new()
                            .serve_dioxus_application("", ServeConfigBuilder::new(app, ()))
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            });
    }
}

fn app(cx: Scope) -> Element {
    let meaning: &UseState<Option<u32>> = use_state(cx, || None);
    let onclick = move |_| {
        to_owned![meaning];
        async move {
            if let Ok(data) = get_meaning("life the universe and everything".into()).await {
                meaning.set(data);
            }
        }
    };

    cx.render(rsx! {
        button {
            onclick: onclick,
            "Run a server function"
        }
        "Server said: {meaning:?}"
    })
}

// This code will only run on the server
#[server(GetMeaning)]
async fn get_meaning(of: String) -> Result<Option<u32>, ServerFnError> {
    Ok(of.contains("life").then(|| 42))
}
