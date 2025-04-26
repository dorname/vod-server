use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Error};
use futures_util::StreamExt as _;
use std::io::Write;
use crate::services::transcoder;
use sanitize_filename::sanitize;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("uploads/{}", sanitize(&filename));
        let filepath_clone = filepath.clone();

        let mut f = web::block(move || std::fs::File::create(&filepath_clone)).await??;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }

        // Start transcoding to HLS
        transcoder::transcode_to_hls(filepath.clone()).await?;
    }

    Ok(HttpResponse::Ok().body("File uploaded and transcoding started."))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload);
}