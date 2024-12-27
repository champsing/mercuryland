mod bot;

use crate::{config::CONFIG, error::ServerError};
use google_youtube3::{hyper_rustls, hyper_util, yup_oauth2, YouTube};

pub async fn run() -> Result<(), ServerError> {
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        CONFIG.youtube.clone(),
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("data/youtube.conf")
    .build()
    .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()?
                .https_only()
                .enable_all_versions()
                .build(),
        );

    bot::YoutubeBot::new(YouTube::new(client, auth))
        .start()
        .await?;

    Ok(())
}
