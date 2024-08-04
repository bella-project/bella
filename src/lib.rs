pub mod prelude {

    use bevy::prelude::*;
    pub use bevy_vello::{
        prelude::*,
        vello::{kurbo, peniko},
        VelloPlugin,
    };
    
    use bevy_vello::prelude::kurbo::{Stroke, Line, Affine};
    use bevy_vello::prelude::peniko::{Fill, BlendMode, Blob, ColorStop};
    use crate::prelude::skrifa::{FontRef, MetadataProvider};
    use crate::prelude::vello::glyph::{Glyph};

    use vello_svg::usvg;
    use vello_svg::util;
    use std::{fs, sync::Arc};
    
    use std::option::Option;

    use std::collections::HashMap;

    pub use smallvec::SmallVec;
    
    #[derive(Component)]
    pub struct BellaInstance {
        vello_root: Option<Entity>,
    }
    
    #[derive(Component)]
    pub struct BellaShape {
        root: Entity,
        scene: vello::Scene,
        get_type: BellaType,
    }

    #[derive(Component)]
    pub struct BellaKurboTransform(Affine);

    #[derive(Bundle)]
    pub struct BellaBundle {
        shape: BellaShape,
        bkt: BellaKurboTransform,
        transform: TransformBundle,
    }
    
    #[derive(Clone)]
    pub enum BellaType {
        Line {
            style: BellaStyle,
            begin: Vec2,
            end: Vec2,
        },
        Rect {
            style: BellaStyle,
            size: Vec2,
            radius: Vec4,
        },
        SubScene {
            data: vello::Scene,
            size: Vec2,
        },
        Text {
            style: BellaStyle,
            text: String,
            font: peniko::Font,
            font_size: f32,
        }
    }

    #[derive(Clone)]
    pub enum BellaGradientKind {
        Linear {
            start: Vec2,
            end: Vec2,
        },
        Radial {
            start_center: Vec2,
            start_radius: f32,
            end_center: Vec2,
            end_radius: f32,
        },
        Sweep {
            center: Vec2,
            start_angle: f32,
            end_angle: f32,
        }
    }

    pub type BellaExtend = peniko::Extend;

    #[derive(Clone)]
    pub enum BellaBrush {
        Solid(bevy::prelude::Color),
        Gradient {
            kind: BellaGradientKind,
            extend: peniko::Extend,
            color_stops: Vec<(bevy::prelude::Color, f32)>,
        }
    }

    pub fn bevy_color_to_peniko_color(c: bevy::prelude::Color) -> peniko::Color {

        let linear_bevy_color: bevy::color::LinearRgba = c.into();

        let peniko_color = peniko::Color::rgba(
                linear_bevy_color.red as f64,
                linear_bevy_color.green as f64,
                linear_bevy_color.blue as f64,
                linear_bevy_color.alpha as f64);

        return peniko_color;
    }

    pub fn bevy_vec2_to_kurbo_point(v: Vec2) -> kurbo::Point {
        kurbo::Point { x: v.x as f64, y: v.y as f64 }
    }

    pub fn bella_gradient_kind_to_peniko_gradient_kind(gk: BellaGradientKind) -> peniko::GradientKind {
        match gk {
            BellaGradientKind::Linear { start, end } => {
                peniko::GradientKind::Linear {
                    start: bevy_vec2_to_kurbo_point(start),
                    end: bevy_vec2_to_kurbo_point(end)
                }
            },
            BellaGradientKind::Radial { start_center, start_radius, end_center, end_radius } => {
                peniko::GradientKind::Radial {
                    start_center: bevy_vec2_to_kurbo_point(start_center),
                    start_radius: start_radius,
                    end_center: bevy_vec2_to_kurbo_point(end_center),
                    end_radius: end_radius,
                }
            },
            BellaGradientKind::Sweep { center, start_angle, end_angle } => {
                peniko::GradientKind::Sweep {
                    center: bevy_vec2_to_kurbo_point(center),
                    start_angle: start_angle,
                    end_angle: end_angle,
                }
            }
        }
    }

    pub fn bella_color_stops_to_peniko_color_stops(v: &Vec<(bevy::prelude::Color, f32)>) -> peniko::ColorStops {
        let mut cs: peniko::ColorStops = peniko::ColorStops::with_capacity(4);

        for (col, off) in v {
            cs.push(ColorStop { offset: *off, color: bevy_color_to_peniko_color(*col) });
        }

        return cs;
    }

    impl BellaBrush {
        pub fn to_peniko_brush(&self) -> peniko::Brush {
            match self {
                BellaBrush::Solid(color) => { peniko::Brush::Solid(bevy_color_to_peniko_color(*color)) },
                BellaBrush::Gradient { kind, extend, color_stops } => {
                    peniko::Brush::Gradient(
                        peniko::Gradient {
                            kind: bella_gradient_kind_to_peniko_gradient_kind(kind.clone()),
                            extend: *extend,
                            stops: bella_color_stops_to_peniko_color_stops(color_stops),
                        }
                    )
                },
            }
        }
    }

    #[derive(Clone)]
    pub struct BellaStyle {
        stroke: kurbo::Stroke,
        stroke_brush: BellaBrush,
        fill_brush: BellaBrush,
    }

    impl BellaStyle {
        pub fn new() -> Self {
            Self {
                stroke: Stroke::new(1.0),
                stroke_brush: BellaBrush::Solid(bevy::prelude::Color::Srgba(Srgba::WHITE)),
                fill_brush: BellaBrush::Solid(bevy::prelude::Color::Srgba(Srgba::WHITE)),
            }
        }

        pub fn with_stroke(&self, s: kurbo::Stroke) -> BellaStyle {
            BellaStyle {
                stroke: s.clone(),
                stroke_brush: self.stroke_brush.clone(),
                fill_brush: self.fill_brush.clone(),
            }
        }

        pub fn with_stroke_brush(&self, sb: BellaBrush) -> BellaStyle {
            BellaStyle {
                stroke: self.stroke.clone(),
                stroke_brush: sb.clone(),
                fill_brush: self.fill_brush.clone(),
            }
        }

        pub fn with_fill_brush(&self, fb: BellaBrush) -> BellaStyle {
            BellaStyle {
                stroke: self.stroke.clone(),
                stroke_brush: self.stroke_brush.clone(),
                fill_brush: fb.clone(),
            }
        }
    }

    pub enum BellaExportOptions {
        Original,
        ScaleToUnit(f32),
        ScaleToUnitKeepAspect(f32),
    }

    pub fn bella_text(t: String, font_path: &str) -> BellaType {
        let bytes = fs::read(font_path).unwrap();

        let create_font = peniko::Font::new(Blob::new(Arc::new(bytes)), 0);

        BellaType::Text {
            style: BellaStyle::new(),
            text: t.to_string(),
            font: create_font,
            font_size: 2.0,
        }
    }

    pub fn bella_svg(path: &str, opt: BellaExportOptions) -> BellaType {
        let contents = fs::read_to_string(path).unwrap();

        let svg = usvg::Tree::from_str(&contents, &usvg::Options::default())
            .unwrap_or_else(|e| panic!("failed to parse svg file {path}: {e}"));

        let mut final_scene = vello::Scene::new();

        let get_size = Vec2::new(svg.size().width(), svg.size().height());
        let mut scale_vec = Vec2::new(1.0, 1.0);

        match opt {
            BellaExportOptions::ScaleToUnit(u) => {
                scale_vec.x = (u / get_size.x) * 2.0;
                scale_vec.y = (u / get_size.y) * 2.0;
            },
            BellaExportOptions::ScaleToUnitKeepAspect(u) => {

                if get_size.x >= get_size.y {

                    let aspect_ratio: f32 = get_size.x / get_size.y;

                    let scale_x = (u / get_size.x) * aspect_ratio;
                    let scale_down = scale_x - (u / get_size.x);
    
                    scale_vec.x = (scale_x - scale_down) * 2.0;
                    scale_vec.y = ((u / get_size.y) - scale_down) * 2.0;
                }
                else {

                    let aspect_ratio: f32 = get_size.y / get_size.x;

                    let scale_y = (u / get_size.y) * aspect_ratio;
                    let scale_down = scale_y - (u / get_size.y);

                    scale_vec.x = ((u / get_size.x) - scale_down) * 2.0;
                    scale_vec.y = (scale_y - scale_down) * 2.0;
                }
            },
            _ => {}
        }

        let svg_scene = vello_svg::render_tree(&svg);
        final_scene.append(&svg_scene, Some(Affine::translate(kurbo::Vec2 { x: -(get_size.x / 2.0) as f64, y: -(get_size.y / 2.0) as f64 } )
                .then_scale_non_uniform(scale_vec.x as f64, scale_vec.y as f64)));

        BellaType::SubScene { 
            data: final_scene,
            size: get_size,
        }
    }
    
    pub fn bella_line() -> BellaType {
        BellaType::Line {
            style: BellaStyle::new(),
            begin: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 1.0, y: 1.0 },
        }
    }

    pub fn bella_rect() -> BellaType {
        BellaType::Rect {
            style: BellaStyle::new(),
            size: Vec2 { x: 1.0, y: 1.0 },
            radius: Vec4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    
    impl BellaType {
        pub fn with_stroke(&self, s: BellaStroke) -> BellaType {
            match self {
                BellaType::Line { style, begin, end, .. } => {
                    BellaType::Line {
                        style: style.with_stroke(s),
                        begin: *begin,
                        end: *end,
                    }
                },
                BellaType::Rect { style, size, radius, .. } => {
                    BellaType::Rect {
                        style: style.with_stroke(s),
                        size: *size,
                        radius: *radius,
                    }
                },
                BellaType::Text { style, text, font, font_size, .. } => {
                    BellaType::Text {
                        style: style.with_stroke(s),
                        text: text.to_string(),
                        font: font.clone(),
                        font_size: *font_size,
                    }
                }
                _ => { self.clone() }
            }
        }

        pub fn with_stroke_brush(&self, sb: BellaBrush) -> BellaType {
            match self {
                BellaType::Line { style, begin, end, .. } => {
                    BellaType::Line {
                        style: style.with_stroke_brush(sb),
                        begin: *begin,
                        end: *end,
                    }
                },
                BellaType::Rect { style, size, radius, .. } => {
                    BellaType::Rect {
                        style: style.with_stroke_brush(sb),
                        size: *size,
                        radius: *radius,
                    }
                },
                BellaType::Text { style, text, font, font_size, .. } => {
                    BellaType::Text {
                        style: style.with_stroke_brush(sb),
                        text: text.to_string(),
                        font: font.clone(),
                        font_size: *font_size,
                    }
                }
                _ => { self.clone() }
            }
        }

        pub fn with_fill_brush(&self, fb: BellaBrush) -> BellaType {
            match self {
                BellaType::Line { style, begin, end, .. } => {
                    BellaType::Line {
                        style: style.with_fill_brush(fb),
                        begin: *begin,
                        end: *end,
                    }
                },
                BellaType::Rect { style, size, radius, .. } => {
                    BellaType::Rect {
                        style: style.with_fill_brush(fb),
                        size: *size,
                        radius: *radius,
                    }
                },
                BellaType::Text { style, text, font, font_size, .. } => {
                    BellaType::Text {
                        style: style.with_fill_brush(fb),
                        text: text.to_string(),
                        font: font.clone(),
                        font_size: *font_size,
                    }
                }
                _ => { self.clone() }
            }
        }

        pub fn with_begin(&self, b: Vec2) -> BellaType {
            match self {
                BellaType::Line { style, end, .. } => {
                    BellaType::Line {
                        style: style.clone(),
                        begin: b,
                        end: *end,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_end(&self, e: Vec2) -> BellaType {
            match self {
                BellaType::Line { style, begin, .. } => {
                    BellaType::Line {
                        style: style.clone(),
                        begin: *begin,
                        end: e,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_size(&self, s: Vec2) -> BellaType {
            match self {
                BellaType::Rect { style, radius, .. } => {
                    BellaType::Rect {
                        style: style.clone(),
                        size: s,
                        radius: *radius,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius(&self, r: f32) -> BellaType {
            match self {
                BellaType::Rect { style, size, .. } => {
                    BellaType::Rect {
                        style: style.clone(),
                        size: *size,
                        radius: Vec4::new(r, r, r, r),
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius_non_uniform(&self, tl: f32, tr: f32, br: f32, bl: f32) -> BellaType {
            match self {
                BellaType::Rect { style, size, .. } => {
                    BellaType::Rect {
                        style: style.clone(),
                        size: *size,
                        radius: Vec4::new(tl, tr, br, bl),
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius_vec(&self, v: Vec4) -> BellaType {
            match self {
                BellaType::Rect { style, size, .. } => {
                    BellaType::Rect {
                        style: style.clone(),
                        size: *size,
                        radius: v,
                    }
                },
                _ => { self.clone() },
            }
        }
    }

    fn to_font_ref(font: &peniko::Font) -> Option<FontRef<'_>> {
        use vello::skrifa::raw::FileRef;
        let file_ref = FileRef::new(font.data.as_ref()).ok()?;
        match file_ref {
            FileRef::Font(font) => Some(font),
            FileRef::Collection(collection) => collection.get(font.index).ok(),
        }
    }
    
    pub type BellaStroke = kurbo::Stroke;

    pub fn scale_test(
        mut vello_scene_query: Query<&mut Transform, With<VelloScene>>) {

        for mut t in &mut vello_scene_query {
            t.scale.x = 3.0;
            t.scale.y = 3.0;
        }
    }

    pub fn transform_impl(mut bella_shape_query: Query<(&mut BellaShape, &mut BellaKurboTransform, &GlobalTransform), Changed<GlobalTransform>>) {

        for (mut s, mut bkt, gt) in &mut bella_shape_query {

            let gt_matrix = gt.affine().matrix3;
            let gt_translation: Vec3 = gt.affine().translation.into();

            let gt_x: [f32; 3] = gt_matrix.x_axis.to_array();
            let gt_y: [f32; 3] = gt_matrix.y_axis.to_array();
            //let gt_z: [f32; 3] = gt_matrix.z_axis.to_array();

            let position_point = kurbo::Point { x: gt_translation.x as f64, y: gt_translation.y as f64 };

            bkt.0 = Affine::new([gt_x[0] as f64, gt_x[1] as f64, gt_y[0] as f64, gt_y[1] as f64, 0.0, 0.0])
                .then_translate(position_point.to_vec2());
        }
    }
    
    pub fn draw_impl(
        mut bella_shape_query: Query<&mut BellaShape, (With<GlobalTransform>, Changed<BellaShape>)>,
        mut vello_scene_query: Query<&mut VelloScene>) {
    
        for mut s in &mut bella_shape_query {

            match &s.get_type {
                BellaType::Line { style, begin, end } => {
    
                    let get_style = style.clone();
            
                    let begin_tuple: (f64, f64) = (begin.x as f64, begin.y as f64);
                    let end_tuple: (f64, f64) = (end.x as f64, end.y as f64);
        
                    let line = Line::new(begin_tuple, end_tuple);
    
                    s.scene.reset();
            
                    s.scene.stroke(&get_style.stroke, Affine::IDENTITY, &get_style.stroke_brush.to_peniko_brush(), None, &line);
                },
                BellaType::Rect { style, size, radius } => {
    
                    let get_style = style.clone();
    
                    let begin_tuple: (f64, f64) = (-(size.x / 2.0) as f64, -(size.y / 2.0) as f64);
                    let end_tuple: (f64, f64) = ((size.x / 2.0) as f64, (size.y / 2.0) as f64);
        
                    let rect = kurbo::Rect::new(begin_tuple.0, begin_tuple.1, end_tuple.0, end_tuple.1);
                    let radii = kurbo::RoundedRectRadii::new(radius.x as f64, radius.y as f64, radius.z as f64, radius.w as f64);
                    let final_rect = kurbo::RoundedRect::from_rect(rect, radii);
    
                    s.scene.reset();
                    
                    s.scene.fill(Fill::NonZero, Affine::IDENTITY, &get_style.fill_brush.to_peniko_brush(), None, &final_rect);
                    s.scene.stroke(&get_style.stroke, Affine::IDENTITY, &get_style.stroke_brush.to_peniko_brush(), None, &final_rect);
                },
                BellaType::Text { style, text, font, font_size } => {

                    let get_style = style.clone();
                    let get_text = text.clone();
                    let get_font = font.clone();

                    let font_ref = to_font_ref(&get_font).unwrap();
                    //let brush = brush.into();
                    //let style = style.into();
                    let axes = font_ref.axes();
                    let charmap = font_ref.charmap();
                    let final_font_size = vello::skrifa::instance::Size::new(*font_size);
                    let variations: &[(&str, f32)] = &[];
                    let var_loc = axes.location(variations.iter().copied());
                    let metrics = font_ref.metrics(final_font_size, &var_loc);
                    let line_height = metrics.ascent - metrics.descent + metrics.leading;
                    let glyph_metrics = font_ref.glyph_metrics(final_font_size, &var_loc);
                    let mut pen_x = 0f32;
                    let mut pen_y = 0f32;

                    s.scene
                        .draw_glyphs(&get_font)
                        .font_size(2.0)
                        .transform(Affine::IDENTITY)
                        .glyph_transform(None)
                        .normalized_coords(var_loc.coords())
                        .brush(&get_style.stroke_brush.to_peniko_brush())
                        .hint(false)
                        .draw(
                            &peniko::Style::Stroke(get_style.stroke),
                            get_text.chars().filter_map(|ch| {
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
                                id: gid.to_u16() as u32,
                                x,
                                y: pen_y,
                            })
                        }));
                },
                BellaType::SubScene { .. } => {}
            }
        }
    }

    pub fn finish_impl(
        mut bella_shape_query: Query<(&mut BellaShape, &mut BellaKurboTransform, &GlobalTransform)/*, Changed<GlobalTransform>*/>,
        mut vello_scene_query: Query<&mut VelloScene>) {

        let mut reset_vello_scenes: HashMap<Entity, bool> = HashMap::new();

        for (mut s, mut bkt, gt) in &mut bella_shape_query {

            let mut scene = vello_scene_query.get_mut(s.root).unwrap();

            let is_reset: &mut bool = reset_vello_scenes.entry(s.root).or_insert(false);

            if *is_reset != true {
                *scene = VelloScene::default();
                *is_reset = true;
            }

            match &s.get_type {
                BellaType::SubScene { data, .. } => scene.append(data, Some(bkt.0)),
                _ => scene.append(&s.scene, Some(bkt.0)),
            }
        }
    }
    
    impl BellaInstance {
        pub fn new(commands: &mut Commands) -> Self {
    
            let mut s = Self {
                vello_root: None,
            };
    
            let v = commands.spawn(VelloSceneBundle::default());
            s.vello_root = Some(v.id());
    
            s
        }
    
        pub fn shape(&self, bt: BellaType, tr: Transform) -> BellaBundle {
            BellaBundle {
                shape: BellaShape {
                    root: self.vello_root.unwrap(),
                    scene: vello::Scene::new(),
                    get_type: bt,
                },
                bkt: BellaKurboTransform(Affine::scale(1.0)),
                transform: TransformBundle::from_transform(tr),
            }
        }
    }
    
    pub struct BellaPlugin;
    
    impl Plugin for BellaPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_plugins(VelloPlugin)
                .add_systems(Update, transform_impl)
                .add_systems(Update, draw_impl.after(transform_impl))
                .add_systems(Update, finish_impl.after(draw_impl));
        }
    }
}