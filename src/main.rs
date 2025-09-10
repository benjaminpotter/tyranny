use bevy::prelude::*;
use tyranny::player::PlayerPlugin;
use tyranny::tools::ToolPlugin;

// Marker so we know which entity to move
#[derive(Component)]
struct MySphere;

#[derive(Component, Default)]
struct SphereCounter {
    value: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ToolPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, sphere_system) // runs every frame
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.0, 0.0))),
    ));

    // sphere with counter + marker
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.8, 1.0))),
        Transform::from_xyz(0.0, 2.0, 2.0),
        MySphere,
        SphereCounter::default(),
    ));

    // lights
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn sphere_system(mut query: Query<(&mut Transform, &mut SphereCounter), With<MySphere>>) {
    if let Ok((mut transform, mut counter)) = query.single_mut() {
        // increment + wrap at 360
        counter.value = (counter.value + 0.5) % 360.0;

        // convert degrees to radians
        let angle = (counter.value as f32).to_radians();

        // move in a circle on the XZ plane (radius = 5.0)
        let radius = 5.0;
        transform.translation.x = radius * angle.cos();
        transform.translation.z = radius * angle.sin();

        // keep Y fixed
        transform.translation.y = 2.0;
    }
}
