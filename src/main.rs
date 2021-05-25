use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

struct Ball {
    velocity: Vec3,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(ball_movement_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    let wheel = shapes::Circle {
        radius: 200.0,
        ..shapes::Circle::default()
    };

    let line1 = shapes::Line(Vec2::new(0., -200.), Vec2::new(0., 200.));
    let line2 = shapes::Line(Vec2::new(-200., 0.), Vec2::new(200., 0.));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &wheel,
            ShapeColors::outlined(Color::TOMATO, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            Transform::default(),
        ))
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line1,
            ShapeColors::outlined(Color::CRIMSON, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            Transform::default(),
        ))
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line2,
            ShapeColors::outlined(Color::CRIMSON, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            Transform::default(),
        ))
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

fn ball_movement_system(time: Res<Time>, mut balls_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    balls_query.iter_mut().for_each(|(ball, mut transform)| {
        transform.translation += ball.velocity * delta_seconds;
    });
}
