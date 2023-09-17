use bevy::prelude::*;
use ecsmos::systems::*;

fn main() {
    App::new()

    .add_plugins(DefaultPlugins)
    .configure_set(Update, Stage::Rendering)
    .configure_set(Update, Stage::Simulation.after(Stage::Rendering))

    .add_systems(Startup, add_resources)
    .add_systems(Startup, add_vehicles)

    .add_systems(Update, movement_system.in_set(Stage::Simulation))
    .add_systems(Update, confine_system.in_set(Stage::Simulation).after(movement_system))
    .add_systems(Update, route_movement_system.in_set(Stage::Simulation))
    .add_systems(Update, transform_update_system.in_set(Stage::Simulation).after(route_movement_system))

    .add_systems(Update, draw_paths)
    // .add_systems(drawing_system)
    .run();
}

// use std::f32::consts::PI;

// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(Update, (system, update_config))
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     // text
//     commands.spawn(TextBundle::from_section(
//         "Hold 'Left' or 'Right' to change the line width",
//         TextStyle {
//             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//             font_size: 24.,
//             color: Color::WHITE,
//         },
//     ));
// }

// fn system(mut gizmos: Gizmos, time: Res<Time>) {
//     let sin = time.elapsed_seconds().sin() * 50.;
//     gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
//     gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

//     // Triangle
//     gizmos.linestrip_gradient_2d([
//         (Vec2::Y * 300., Color::BLUE),
//         (Vec2::new(-255., -155.), Color::RED),
//         (Vec2::new(255., -155.), Color::GREEN),
//         (Vec2::Y * 300., Color::BLUE),
//     ]);

//     gizmos.rect_2d(
//         Vec2::ZERO,
//         time.elapsed_seconds() / 3.,
//         Vec2::splat(300.),
//         Color::BLACK,
//     );

//     // The circles have 32 line-segments by default.
//     gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
//     // You may want to increase this for larger circles.
//     gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);

//     // Arcs default amount of segments is linerarly interpolated between
//     // 1 and 32, using the arc length as scalar.
//     gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
// }

// fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
//     if keyboard.pressed(KeyCode::Right) {
//         config.line_width += 5. * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::Left) {
//         config.line_width -= 5. * time.delta_seconds();
//     }
// }