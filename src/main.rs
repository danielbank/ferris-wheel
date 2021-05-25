use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use core::f32::consts::FRAC_PI_2;

const ROTATION_SPEED: f32 = 200.;

struct Particle {}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_system(rotation_system.system())
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
        .insert(Particle {});
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
        .insert(Particle {});
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
        .insert(Particle {});
}

fn rotation_system(mut particles_query: Query<(&Particle, &mut Transform)>) {
    particles_query.iter_mut().for_each(|(_, mut transform)| {
        transform.rotate(Quat::from_rotation_z(FRAC_PI_2 / ROTATION_SPEED));
    });
}
