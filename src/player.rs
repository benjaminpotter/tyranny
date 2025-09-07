use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};
use bevy_ecs::component::HookContext;
use bevy_ecs::world::DeferredWorld;
use leafwing_input_manager::prelude::*;
use std::ops::DerefMut;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .register_type::<ActivePlayer>()
            .register_type::<PlayerSettings>()
            .register_type::<CameraSettings>()
            .init_resource::<ActivePlayer>()
            .init_resource::<PlayerSettings>()
            .init_resource::<CameraSettings>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (pan_player, zoom_camera))
            .add_systems(Update, fit_window);
    }
}

#[derive(Component)]
#[require(Transform, InputMap<PlayerAction>)]
#[component(on_add = on_player_add)]
#[component(on_remove = on_player_remove)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Pan,
    #[actionlike(Axis)]
    Zoom,
}

#[derive(Resource, Reflect, Default, Debug, PartialEq)]
#[reflect(Resource)]
pub struct ActivePlayer {
    pub player_id: Option<Entity>,
}

#[derive(Component, Reflect)]
pub struct ScreenCamera;

#[derive(Component, Reflect)]
pub struct WorldCamera;

#[derive(Resource, Reflect, Debug, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub struct CameraSettings {
    /// The extent (a.k.a. size) that is used to create the render target for the world camera.
    pub world_render_width: u32,
    pub world_render_height: u32,

    // The speed that the camera can zoom.
    pub zoom_speed: f32,
}

#[derive(Resource, Reflect, Debug, PartialEq, Clone, Copy)]
#[reflect(Resource)]
pub struct PlayerSettings {
    /// The speed (m/s) of the player in XZ plane.
    pub pan_speed: f32,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Pan, VirtualDPad::wasd())
            .with_axis(Self::Zoom, MouseScrollAxis::Y)
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            world_render_width: 320,
            world_render_height: 180,
            zoom_speed: 1.0,
        }
    }
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self { pan_speed: 2.0 }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    camera_settings: Res<CameraSettings>,
) {
    let size = Extent3d {
        width: camera_settings.world_render_width,
        height: camera_settings.world_render_height,
        ..default()
    };
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            // You need to set these texture usage flags in order to use the image as a render target
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);
    let screen_layer = RenderLayers::layer(1);

    // Camera that only sees the downsampled image created by the world camera.
    commands.spawn((Camera2d, Msaa::Off, screen_layer.clone(), ScreenCamera));

    // Draw the world render image.
    commands.spawn((
        Sprite {
            image: image_handle.clone(),
            ..default()
        },
        screen_layer.clone(),
    ));

    commands.spawn((
        Player,
        PlayerAction::default_input_map(),
        // Rotate the player around the Z (UP) axis, to get isometric view.
        Transform::from_xyz(0.0, 0.0, 0.0).looking_to(
            Dir3::from_xyz(-1.0, 0.0, -1.0).expect("length of xyz is not zero, infinite, or NaN"),
            Dir3::Y,
        ),
        Visibility::default(),
        children![
            // Camera that sees the world in isometric projection.
            (
                WorldCamera,
                Camera3d::default(),
                Camera {
                    // Render this camera to an image rather than directly to the user's screen.
                    target: image_handle.into(),
                    clear_color: Color::BLACK.into(),
                    ..default()
                },
                Projection::from(OrthographicProjection {
                    // 6 world units per pixel of window height.
                    scaling_mode: ScalingMode::FixedVertical {
                        viewport_height: 6.0,
                    },
                    ..OrthographicProjection::default_3d()
                }),
                Msaa::Off,
                // Child transform is with respect to the parent transform.
                // So this camera is positioned at the same location as the parent, Player.
                Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            )
        ],
    ));
}

// Runs when the Player component is added to the world.
fn on_player_add(mut world: DeferredWorld, context: HookContext) {
    info!("player added");

    // Get the ActivePlayer resource from the world.
    let mut active_player = world.resource_mut::<ActivePlayer>();

    // Check if an active player already exists.
    if active_player.player_id.is_none() {
        // Set the entity id of the ActivePlayer to the entity that was added to the world.
        // The entity id from context.entity contains the Player component.
        active_player.player_id = Some(context.entity);
        info!("active_player set to {}", context.entity);
    } else {
        warn!("more than one player exists in the world!");
    }
}

// Runs when the Player component is removed from the world.
fn on_player_remove(mut world: DeferredWorld, _context: HookContext) {
    info!("player removed");

    // Get the ActivePlayer resource from the world.
    let mut active_player = world.resource_mut::<ActivePlayer>();

    // Check if an active player actually exists.
    if active_player.player_id.is_some() {
        // Return the ActivePlayer entity id to None to indicate no player in the world.
        active_player.player_id = None;
    } else {
        warn!("tried to remove a player from the world, but no active player exists!");
    }
}

fn pan_player(
    mut query: Query<(Entity, &ActionState<PlayerAction>, &mut Transform), With<Player>>,
    active_player: Res<ActivePlayer>,
    player_settings: Res<PlayerSettings>,
    time: Res<Time>,
) {
    if let Some(player_id) = active_player.player_id {
        for (id, action_state, mut transform) in query.iter_mut() {
            if id != player_id {
                continue;
            }

            let pan_distance = action_state.axis_pair(&PlayerAction::Pan)
                * player_settings.pan_speed
                * time.delta_secs();

            // Because we are in isometric view, the WASD directions are rotated 45 degrees from the XZ axes.
            // We want to move along the WASD directions rather than the world's XZ axes.
            // Without this rotation WASD would appear to move the camera diagonally, very unintuitive.
            let delta_translation =
                transform.forward() * pan_distance.y + transform.right() * pan_distance.x;

            transform.translation += delta_translation;
        }
    }
}

fn zoom_camera(
    mut player_query: Query<(Entity, &ActionState<PlayerAction>, &Children), With<Player>>,
    mut camera_query: Query<(Entity, &mut Projection), With<Camera3d>>,
    active_player: Res<ActivePlayer>,
    camera_settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    if let Some(player_id) = active_player.player_id {
        for (id, action_state, children) in player_query.iter_mut() {
            if id != player_id {
                continue;
            }

            let zoom_delta = action_state.value(&PlayerAction::Zoom)
                * camera_settings.zoom_speed
                * time.delta_secs();

            // HACK: This can't be very efficient...
            for (camera_id, mut projection) in camera_query.iter_mut() {
                if let Some(_camera_id) = children.iter().find(|child_id| *child_id == camera_id) {
                    match projection.deref_mut() {
                        Projection::Orthographic(orthographic_projection) => {
                            orthographic_projection.scale *= 1. - zoom_delta;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_window(
    mut resize_events: EventReader<WindowResized>,
    mut projection: Single<&mut Projection, With<ScreenCamera>>,
    camera_settings: Res<CameraSettings>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for event in resize_events.read() {
        let h_scale = event.width / camera_settings.world_render_width as f32;
        let v_scale = event.height / camera_settings.world_render_height as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
