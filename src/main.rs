use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Body, Error, Request, RequestExt,
};
use og_image_writer::{style, writer::OGImageWriter};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let name = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("text"))
        .unwrap_or("stranger")
        .to_string();

    let bg = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("bg"))
        .unwrap_or("purple");

    let justify = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("justify"))
        .unwrap_or("center");

    let img = generate_image(&name, bg, justify).expect("image created");

    // Represents an HTTP response
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(img.into())
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

fn generate_image(text: &str, background: &str, justify: &str) -> Result<Vec<u8>, Error> {
    let bg = match background {
        "purple" => [70, 40, 90, 255],
        "one" => [20, 40, 40, 200],
        "two" => [60, 50, 30, 155],
        _ => [55, 40, 90, 255],
    };

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba(bg)),
        align_items: style::AlignItems::Center,
        justify_content: match justify {
            "start" => style::JustifyContent::Start,
            "end" => style::JustifyContent::End,
            _ => style::JustifyContent::Center,
        },
        ..style::WindowStyle::default()
    })
    .expect("intialize writer");

    let font = Vec::from(include_bytes!("../assets/SKRAPPA.ttf") as &[u8]);

    writer
        .set_text(
            text,
            style::Style {
                margin: style::Margin(0, 20, 0, 20),
                line_height: 1.8,
                font_size: 100.,
                word_break: style::WordBreak::Normal,
                color: style::Rgba([255, 255, 255, 255]),
                text_align: style::TextAlign::Start,
                ..style::Style::default()
            },
            Some(font.clone()),
        )
        .expect("set text");

    writer.paint().expect("paint img");

    let img = writer
        .encode(og_image_writer::ImageOutputFormat::Png)
        .expect("encode png");

    Ok(img)
}
