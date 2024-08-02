pub mod prelude {

    use bevy::prelude::*;
    pub use bevy_vello::{
        prelude::*,
        vello::{kurbo, peniko},
        VelloPlugin,
    };
    
    use bevy_vello::prelude::kurbo::{Stroke, Line, Affine};
    use bevy_vello::prelude::peniko::{Fill, BlendMode};

    use vello_svg::usvg;
    use vello_svg::util;
    use std::fs;
    
    use std::option::Option;

    use std::collections::HashMap;
    
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
            stroke: Stroke,
            color: bevy::prelude::Color,
            begin: Vec2,
            end: Vec2,
        },
        Rect {
            stroke: Stroke,
            color: bevy::prelude::Color,
            fill_color: bevy::prelude::Color,
            size: Vec2,
            radius: Vec4,
        },
        SubScene {
            data: vello::Scene,
            size: Vec2,
        }
    }

    pub enum BellaExportOptions {
        Original,
        ScaleToUnit(f32),
        ScaleToUnitKeepAspect(f32),
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
            stroke: Stroke::new(1.0),
            color: bevy::prelude::Color::Srgba(Srgba::WHITE),
            begin: Vec2 { x: 0.0, y: 0.0 },
            end: Vec2 { x: 1.0, y: 1.0 },
        }
    }

    pub fn bella_rect() -> BellaType {
        BellaType::Rect {
            stroke: Stroke::new(1.0),
            color: bevy::prelude::Color::Srgba(Srgba::WHITE),
            fill_color: bevy::prelude::Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0)),
            size: Vec2 { x: 1.0, y: 1.0 },
            radius: Vec4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    
    impl BellaType {
        pub fn with_stroke(&self, s: BellaStroke) -> BellaType {
            match self {
                BellaType::Line { color, begin, end, .. } => {
                    BellaType::Line {
                        stroke: s,
                        color: *color,
                        begin: *begin,
                        end: *end,
                    }
                },
                BellaType::Rect { color, fill_color, size, radius, .. } => {
                    BellaType::Rect {
                        stroke: s,
                        color: *color,
                        fill_color: *fill_color,
                        size: *size,
                        radius: *radius,
                    }
                },
                _ => { self.clone() }
            }
        }

        pub fn with_color(&self, c: bevy::prelude::Color) -> BellaType {
            match self {
                BellaType::Line { stroke, begin, end, .. } => {
                    BellaType::Line {
                        stroke: stroke.clone(),
                        color: c,
                        begin: *begin,
                        end: *end,
                    }
                },
                BellaType::Rect { stroke, fill_color, size, radius, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: c,
                        fill_color: *fill_color,
                        size: *size,
                        radius: *radius,
                    }
                },
                _ => { self.clone() }
            }
        }

        pub fn with_fill_color(&self, fc: bevy::prelude::Color) -> BellaType {
            match self {
                BellaType::Rect { stroke, color, radius, size, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: *color,
                        fill_color: fc,
                        size: *size,
                        radius: *radius,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_begin(&self, b: Vec2) -> BellaType {
            match self {
                BellaType::Line { stroke, color, end, .. } => {
                    BellaType::Line {
                        stroke: stroke.clone(),
                        color: *color,
                        begin: b,
                        end: *end,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_end(&self, e: Vec2) -> BellaType {
            match self {
                BellaType::Line { stroke, begin, color, .. } => {
                    BellaType::Line {
                        stroke: stroke.clone(),
                        color: *color,
                        begin: *begin,
                        end: e,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_size(&self, s: Vec2) -> BellaType {
            match self {
                BellaType::Rect { stroke, color, fill_color, radius, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: *color,
                        fill_color: *fill_color,
                        size: s,
                        radius: *radius,
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius(&self, r: f32) -> BellaType {
            match self {
                BellaType::Rect { stroke, color, fill_color, size, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: *color,
                        fill_color: *fill_color,
                        size: *size,
                        radius: Vec4::new(r, r, r, r),
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius_non_uniform(&self, tl: f32, tr: f32, br: f32, bl: f32) -> BellaType {
            match self {
                BellaType::Rect { stroke, color, fill_color, size, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: *color,
                        fill_color: *fill_color,
                        size: *size,
                        radius: Vec4::new(tl, tr, br, bl),
                    }
                },
                _ => { self.clone() },
            }
        }

        pub fn with_radius_vec(&self, v: Vec4) -> BellaType {
            match self {
                BellaType::Rect { stroke, color, fill_color, size, .. } => {
                    BellaType::Rect {
                        stroke: stroke.clone(),
                        color: *color,
                        fill_color: *fill_color,
                        size: *size,
                        radius: v,
                    }
                },
                _ => { self.clone() },
            }
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
    
        for (mut s) in &mut bella_shape_query {

            match &s.get_type {
                BellaType::Line { stroke, begin, end, color } => {
    
                    let get_stroke = stroke.clone();
            
                    let begin_tuple: (f64, f64) = (begin.x as f64, begin.y as f64);
                    let end_tuple: (f64, f64) = (end.x as f64, end.y as f64);
        
                    let line = Line::new(begin_tuple, end_tuple);
            
                    let linear_bevy_color: bevy::color::LinearRgba = (*color).into();
                    
                    let line_stroke_color = peniko::Color::rgba(
                        linear_bevy_color.red as f64,
                        linear_bevy_color.green as f64,
                        linear_bevy_color.blue as f64,
                        linear_bevy_color.alpha as f64);
    
                    s.scene.reset();
            
                    s.scene.stroke(&get_stroke, Affine::IDENTITY, line_stroke_color, None, &line);
                },
                BellaType::Rect { stroke, color, fill_color, size, radius } => {
    
                    let get_stroke = stroke.clone();
    
                    let begin_tuple: (f64, f64) = (-(size.x / 2.0) as f64, -(size.y / 2.0) as f64);
                    let end_tuple: (f64, f64) = ((size.x / 2.0) as f64, (size.y / 2.0) as f64);
        
                    let rect = kurbo::Rect::new(begin_tuple.0, begin_tuple.1, end_tuple.0, end_tuple.1);
                    let radii = kurbo::RoundedRectRadii::new(radius.x as f64, radius.y as f64, radius.z as f64, radius.w as f64);
                    let final_rect = kurbo::RoundedRect::from_rect(rect, radii);
            
                    let linear_bevy_color: bevy::color::LinearRgba = (*color).into();
                    let linear_bevy_fill_color: bevy::color::LinearRgba = (*fill_color).into();
            
                    let line_stroke_color = peniko::Color::rgba(
                        linear_bevy_color.red as f64,
                        linear_bevy_color.green as f64,
                        linear_bevy_color.blue as f64,
                        linear_bevy_color.alpha as f64);
    
                    let fill_color = peniko::Color::rgba(
                        linear_bevy_fill_color.red as f64,
                        linear_bevy_fill_color.green as f64,
                        linear_bevy_fill_color.blue as f64,
                        linear_bevy_fill_color.alpha as f64);
    
                    s.scene.reset();
                    
                    s.scene.fill(Fill::NonZero, Affine::IDENTITY, fill_color, None, &final_rect);
                    s.scene.stroke(&get_stroke, Affine::IDENTITY, line_stroke_color, None, &final_rect);
                },
                BellaType::SubScene { .. } => {}
            }
        }
    }

    pub fn finish_impl(
        mut bella_shape_query: Query<(&mut BellaShape, &mut BellaKurboTransform, &GlobalTransform), Changed<GlobalTransform>>,
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