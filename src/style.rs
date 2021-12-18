use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use sailfish::{runtime, RenderError, TemplateOnce};

#[derive(TemplateOnce)]
#[template(path = "style/style.css.stpl")]
pub struct StyleTemplate {
    pub palette: ColorPalette,
}

#[allow(dead_code)]
pub enum Color {
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

pub struct ColorPalette {
    primary: Color,
    secondary: Color,
    background: Color,
}

impl ColorPalette {
    pub fn random<R: Rng>(mut rng: R) -> Self {
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
