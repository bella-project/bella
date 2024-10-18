use bella::prelude::*;
use interpoli::{Framerate, Keyframe, Sequence, StaticTimeline, Timecode};
use kurbo::Vec2;

#[derive(Component)]
struct AnimatedObject {
    timeline: StaticTimeline<Vec2>,
    color: peniko::Color,
    animation_started: bool,
}

fn animated_fixed_timeline() -> StaticTimeline<Vec2> {
    let mut t = StaticTimeline::<Vec2>::new(Framerate::Fixed(12.0));
    let s: &mut Sequence<Vec2> = t.new_sequence("sequence").unwrap();

    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(100.0, 100.0),
        },
        &tcode_hms!(00:00:00),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(200.0, 100.0),
        },
        &tcode_hms!(00:00:01),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(700.0, 300.0),
        },
        &tcode_hms!(00:00:02),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(600.0, 400.0),
        },
        &tcode_hms!(00:00:03),
    );

    t
}

fn animated_inter_timeline() -> StaticTimeline<Vec2> {
    let mut t = StaticTimeline::<Vec2>::new(Framerate::Interpolated(12.0));
    let s: &mut Sequence<Vec2> = t.new_sequence("sequence").unwrap();

    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(200.0, 200.0),
        },
        &tcode_hms!(00:00:00),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(300.0, 200.0),
        },
        &tcode_hms!(00:00:01),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(800.0, 400.0),
        },
        &tcode_hms!(00:00:02),
    );
    s.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(700.0, 500.0),
        },
        &tcode_hms!(00:00:03),
    );

    t
}

fn start(mut commands: Commands, mut instance: ResMut<Instance>) {
    instance.new_scene("scene1");

    commands.spawn((
        AnimatedObject {
            timeline: animated_fixed_timeline(),
            color: peniko::Color::RED,
            animation_started: false,
        },
        Transform::from_xy(100.0, 100.0),
    ));
    commands.spawn((
        AnimatedObject {
            timeline: animated_inter_timeline(),
            color: peniko::Color::YELLOW,
            animation_started: false,
        },
        Transform::from_xy(200.0, 200.0),
    ));
}

fn draw(query: Query<(&Transform, &AnimatedObject)>, mut instance: ResMut<Instance>) {
    let scene = instance.get_scene("scene1").unwrap();

    for (t, a) in &query {
        scene.stroke(
            &kurbo::Stroke::new(6.0),
            t.affine,
            a.color,
            None,
            &kurbo::RoundedRect::new(0.0, 0.0, 50.0, 50.0, 20.0),
        );
    }
}

fn update(
    mut query: Query<(&mut AnimatedObject, &mut Transform)>,
    time: Res<Time>,
    input: Res<Input>,
) {
    for (mut a, mut t) in &mut query {
        if input.is_key_pressed(KeyCode::KeyW) {
            a.animation_started = true;
        }

        if a.animation_started {
            a.timeline.add_by_duration(time.delta());
            let vec = a.timeline.tween_by_name("sequence");
            t.affine = kurbo::Affine::translate(vec);
        }
    }
}

fn main() {
    App::new("Bella: Simple Animation", 1280, 720)
        .new_world()
        .on_start(start)
        .on_draw(draw)
        .on_update(update)
        .run()
}
