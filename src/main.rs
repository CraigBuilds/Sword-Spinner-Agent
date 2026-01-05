use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sword Spinner".to_string(),
                    resolution: (800.0, 600.0).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO)) // Top-down game, no gravity
        .insert_resource(TouchState::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                detect_double_tap,
                player_movement,
                sword_spin,
                camera_follow,
            )
                .chain(),
        )
        .run();
}

// Component markers
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Sword;

#[derive(Component)]
struct MainCamera;

// Touch state resource for double-tap detection
#[derive(Resource)]
struct TouchState {
    last_tap_time: Option<Instant>,
    last_tap_position: Option<Vec2>,
    double_tap_detected: bool,
    double_tap_window: Duration,
    tap_distance_threshold: f32,
    touch_start_position: Option<Vec2>,
    is_dragging: bool,
    current_touch_position: Option<Vec2>, // Track current touch for movement
}

impl Default for TouchState {
    fn default() -> Self {
        Self {
            last_tap_time: None,
            last_tap_position: None,
            double_tap_detected: false,
            double_tap_window: Duration::from_millis(300),
            tap_distance_threshold: 50.0,
            touch_start_position: None,
            is_dragging: false,
            current_touch_position: None,
        }
    }
}

impl TouchState {
    fn register_tap(&mut self, position: Vec2) {
        let now = Instant::now();
        
        // Check if this is a double-tap
        if let (Some(last_time), Some(last_pos)) = (self.last_tap_time, self.last_tap_position) {
            let time_diff = now.duration_since(last_time);
            let distance = position.distance(last_pos);
            
            if time_diff <= self.double_tap_window && distance <= self.tap_distance_threshold {
                self.double_tap_detected = true;
                // Reset to prevent triple-tap
                self.last_tap_time = None;
                self.last_tap_position = None;
                return;
            }
        }
        
        self.last_tap_time = Some(now);
        self.last_tap_position = Some(position);
    }
    
    fn consume_double_tap(&mut self) -> bool {
        let detected = self.double_tap_detected;
        self.double_tap_detected = false;
        detected
    }
}

// Setup system - initializes the game world
fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn((Camera2d, MainCamera));

    // Spawn player
    let player_entity = commands
        .spawn((
            Player,
            Sprite {
                color: Color::srgb(0.2, 0.4, 0.8),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(40.0, 40.0),
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::default(),
            LinearDamping(2.0),
            Mass(2.0),
        ))
        .id();

    // Spawn sword
    let sword_entity = commands
        .spawn((
            Sword,
            Sprite {
                color: Color::srgb(0.6, 0.6, 0.6),
                custom_size: Some(Vec2::new(60.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(50.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(60.0, 10.0),
            AngularVelocity::default(),
            LinearVelocity::default(),
            LinearDamping(1.0),
            AngularDamping(2.0),
            Mass(0.5),
        ))
        .id();

    // Create revolute joint connecting sword to player
    // The sword rotates around the player at the player's center
    commands.spawn(
        RevoluteJoint::new(player_entity, sword_entity)
            .with_local_anchor_1(Vec2::ZERO) // Player center
            .with_local_anchor_2(Vec2::new(-25.0, 0.0)) // Offset from sword center
            .with_compliance(0.00001), // Very stiff connection
    );

    // Spawn arena boundaries
    let wall_thickness = 20.0;
    let arena_width = 800.0;
    let arena_height = 600.0;

    // Top wall
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(arena_width, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, arena_height / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(arena_width, wall_thickness),
    ));

    // Bottom wall
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(arena_width, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -arena_height / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(arena_width, wall_thickness),
    ));

    // Left wall
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(wall_thickness, arena_height)),
            ..default()
        },
        Transform::from_xyz(-arena_width / 2.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_thickness, arena_height),
    ));

    // Right wall
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(wall_thickness, arena_height)),
            ..default()
        },
        Transform::from_xyz(arena_width / 2.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_thickness, arena_height),
    ));

    // Spawn some dynamic obstacles
    let obstacle_positions = [
        Vec2::new(150.0, 100.0),
        Vec2::new(-150.0, -100.0),
        Vec2::new(200.0, -150.0),
        Vec2::new(-200.0, 150.0),
        Vec2::new(0.0, 200.0),
    ];

    for pos in obstacle_positions.iter() {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.5, 0.2),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(30.0, 30.0),
            LinearDamping(0.5),
            AngularDamping(1.0),
            Mass(1.0),
        ));
    }
}

// System to detect double-tap gestures
fn detect_double_tap(
    mut touch_events: EventReader<bevy::input::touch::TouchInput>,
    mut touch_state: ResMut<TouchState>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();

    for touch in touch_events.read() {
        if touch.phase == bevy::input::touch::TouchPhase::Started {
            // Convert touch position to world coordinates
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position)
            {
                touch_state.touch_start_position = Some(world_pos);
                touch_state.current_touch_position = Some(world_pos);
                touch_state.is_dragging = false;
            }
        } else if touch.phase == bevy::input::touch::TouchPhase::Moved {
            // Update current touch position for movement
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position)
            {
                touch_state.current_touch_position = Some(world_pos);
                
                // Check if this is a drag (moved more than threshold)
                if let Some(start_pos) = touch_state.touch_start_position {
                    let distance = world_pos.distance(start_pos);
                    if distance > 10.0 {
                        // 10px drag threshold
                        touch_state.is_dragging = true;
                    }
                }
            }
        } else if touch.phase == bevy::input::touch::TouchPhase::Ended {
            // Only register tap if it wasn't a drag
            if !touch_state.is_dragging {
                if let Ok(world_pos) =
                    camera.viewport_to_world_2d(camera_transform, touch.position)
                {
                    touch_state.register_tap(world_pos);
                }
            }
            touch_state.touch_start_position = None;
            touch_state.current_touch_position = None;
            touch_state.is_dragging = false;
        }
    }
}

// System to handle player movement (keyboard and touch)
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut LinearVelocity), With<Player>>,
    touch_state: Res<TouchState>,
) {
    let (player_transform, mut velocity) = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    // Keyboard input for desktop
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    // Touch input for mobile (drag to move)
    if touch_state.is_dragging {
        if let Some(world_pos) = touch_state.current_touch_position {
            // Calculate direction from player to touch position
            let target_direction = world_pos - player_transform.translation.truncate();
            
            // Only move if touch is reasonably far from player
            if target_direction.length() > 20.0 {
                direction = target_direction;
            }
        }
    }

    // Normalize and apply velocity
    if direction.length() > 0.0 {
        direction = direction.normalize();
        velocity.0 = direction * 200.0; // Movement speed
    } else {
        velocity.0 = Vec2::ZERO;
    }
}

// System to spin the sword
fn sword_spin(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut touch_state: ResMut<TouchState>,
    mut sword_query: Query<&mut AngularVelocity, With<Sword>>,
) {
    let mut should_spin = false;

    // Desktop input
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        should_spin = true;
    }

    // Mobile input - double-tap
    if touch_state.consume_double_tap() {
        should_spin = true;
    }

    if should_spin {
        if let Ok(mut angular_velocity) = sword_query.get_single_mut() {
            angular_velocity.0 += 15.0; // Apply spin force
        }
    }
}

// System to make camera follow the player
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
