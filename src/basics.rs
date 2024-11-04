use crate::assets::ToFontRef;
use vello::kurbo::{Affine, Circle, Point, RoundedRect, Stroke, Vec2};
use vello::peniko::{BrushRef, Fill, Font, Style};
use vello::skrifa::MetadataProvider;
use vello::{Glyph, Scene};

pub trait SceneBasics {
    fn fill_circle<'b>(&mut self, f: Fill, t: Affine, b: impl Into<BrushRef<'b>>, radius: f64);
    fn fill_rounded_rect<'b>(
        &mut self,
        f: Fill,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        size: Vec2,
        corner: f64,
    );

    fn stroke_circle<'b>(&mut self, s: Stroke, t: Affine, b: impl Into<BrushRef<'b>>, radius: f64);
    fn stroke_rounded_rect<'b>(
        &mut self,
        s: Stroke,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        size: Vec2,
        corner: f64,
    );

    fn fill_text<'b>(
        &mut self,
        text: &str,
        fill: Fill,
        font: &Font,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        font_size: f64,
    );
}

impl SceneBasics for Scene {
    fn fill_circle<'b>(&mut self, f: Fill, t: Affine, b: impl Into<BrushRef<'b>>, radius: f64) {
        self.fill(f, t, b, None, &Circle::new(Point::new(0.0, 0.0), radius));
    }

    fn fill_rounded_rect<'b>(
        &mut self,
        f: Fill,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        size: Vec2,
        corner: f64,
    ) {
        self.fill(
            f,
            t,
            b,
            None,
            &RoundedRect::new(
                -(size.x / 2.0),
                -(size.y / 2.0),
                size.x / 2.0,
                size.y / 2.0,
                corner,
            ),
        );
    }

    fn fill_text<'b>(
        &mut self,
        text: &str,
        fill: Fill,
        font: &Font,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        font_size: f64,
    ) {
        let font_ref = font.to_font_ref().unwrap();
        let axes = font_ref.axes();
        let charmap = font_ref.charmap();
        let final_font_size = vello::skrifa::instance::Size::new(font_size as f32);
        let variations: &[(&str, f32)] = &[];
        let var_loc = axes.location(variations.iter().copied());
        let metrics = font_ref.metrics(final_font_size, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(final_font_size, &var_loc);
        let mut pen_x = 0f32;
        let mut pen_y = 0f32;

        let gly: Vec<Glyph> = text
            .to_string()
            .chars()
            .filter_map(|ch| {
                if ch == '\n' {
                    pen_y += line_height;
                    pen_x = 0.0;
                    return None;
                }
                let gid = charmap.map(ch).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                let x = pen_x;
                pen_x += advance;
                Some(Glyph {
                    id: gid.to_u32(),
                    x,
                    y: pen_y,
                })
            })
            .collect();

        // scene
        //     .draw_glyphs(&get_font)
        //     .font_size(get_font_size)
        //     .transform(Affine::IDENTITY)
        //     .glyph_transform(None)
        //     .normalized_coords(var_loc.coords())
        //     .brush(&get_style.stroke_brush.to_peniko_brush())
        //     .hint(false)
        //     .draw(&peniko::Style::Stroke(get_style.stroke), gly.clone().into_iter());

        self.draw_glyphs(font)
            .font_size(font_size as f32)
            .transform(t.then_translate(Vec2::new(0.0, font_size)))
            .glyph_transform(None)
            .normalized_coords(var_loc.coords())
            .brush(b.into())
            .hint(false)
            .draw(&Style::Fill(fill), gly.into_iter());
    }

    fn stroke_circle<'b>(&mut self, s: Stroke, t: Affine, b: impl Into<BrushRef<'b>>, radius: f64) {
        self.stroke(&s, t, b, None, &Circle::new(Point::new(0.0, 0.0), radius));
    }

    fn stroke_rounded_rect<'b>(
        &mut self,
        s: Stroke,
        t: Affine,
        b: impl Into<BrushRef<'b>>,
        size: Vec2,
        corner: f64,
    ) {
        self.stroke(
            &s,
            t,
            b,
            None,
            &RoundedRect::new(
                -(size.x / 2.0),
                -(size.y / 2.0),
                size.x / 2.0,
                size.y / 2.0,
                corner,
            ),
        );
    }
}
