use actix_web::{get, web, HttpResponse, Result};

#[get("/play/{video_id}")]
async fn play(video_id: web::Path<String>) -> Result<HttpResponse> {
    let video_path = format!("/videos/{}/index.m3u8", video_id.into_inner());
    let html = format!(r#"
    <html>
    <body>
        <video id="video" width="800" controls>
            <source src="{}" type="application/vnd.apple.mpegurl">
        </video>
    </body>
    </html>
    "#, video_path);

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(play);
}
