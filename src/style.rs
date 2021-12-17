use rand::distributions::{Distribution, Uniform};
use rand::{rngs::SmallRng, SeedableRng};
use rocket::http::{ContentType, Status};
use rocket::Response;
use sailfish::{runtime, RenderError, TemplateOnce};
use std::io::Cursor;
use std::time::SystemTime;

#[derive(TemplateOnce)]
#[template(path = "style/style.css.stpl")]
struct StyleTemplate {
    palette: ColorPalette,
}

#[allow(dead_code)]
enum Color {
    Rgb(u8, u8, u8),
    Hsl(u16, u8, u8),
}

impl runtime::Render for Color {
    fn render(&self, buf: &mut runtime::Buffer) -> Result<(), RenderError> {
        use Color::*;
        match *self {
            Rgb(r, g, b) => format!("Rgb({}, {}, {})", r, g, b).render(buf),
            Hsl(h, s, l) => format!("Hsl({}, {}%, {}%)", h, s, l).render(buf),
        }
    }
}

struct ColorPalette {
    primary: Color,
    secondary: Color,
    background: Color,
}

impl ColorPalette {
    fn random() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut rng = SmallRng::seed_from_u64(seed);

        let hue = Uniform::from(0..360).sample(&mut rng);
        let saturation_deviation = Uniform::from(0..=10).sample(&mut rng);
        let lightness_deviation = Uniform::from(0..=10).sample(&mut rng);

        Self {
            primary: Color::Hsl(hue, 60 + saturation_deviation, 80 + lightness_deviation),
            secondary: Color::Hsl(hue, 50 + saturation_deviation, 70 + lightness_deviation),
            background: Color::Hsl(hue, 40 + saturation_deviation, 30 + lightness_deviation),
        }
    }
}

#[get("/style")]
pub fn route<'a>() -> Result<Response<'a>, RenderError> {
    let ctx = StyleTemplate {
        palette: ColorPalette::random(),
    };

    let body = ctx.render_once()?;

    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::CSS)
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}
