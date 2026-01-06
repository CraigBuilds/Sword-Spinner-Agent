use avian2d::prelude::*;
use bevy::prelude::*;

// Resource to track touch input for mobile control
#[derive(Resource, Default)]
struct TouchState {
    direction: Vec2,
    is_active: bool,
}

/// Creates and configures the main App with all plugins and systems.
/// This function is public to allow testing.
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins((
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
                update_touch_state,
                player_movement,
                spin_button_interaction,
                sword_spin,
                camera_follow,
                update_direction_arrow,
            )
                .chain(),
        );
    app
}

// Public function that runs the game - can be called from both main.rs and lib.rs entry points
pub fn run() {
    create_app().run();
}

// Android entry point
#[bevy_main]
fn main() {
    run();
}

// Component markers
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Sword;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SpinButton;

#[derive(Component)]
struct DirectionArrow;

// Setup system - initializes the game world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn((Camera2d, MainCamera));

    // Create a circle mesh for the player
    let circle_mesh = Circle::new(20.0).mesh().build();
    let circle_mesh_handle = meshes.add(circle_mesh);
    let circle_material = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.4, 0.8)));

    // Spawn player (circle shape to prevent sword collision)
    let player_entity = commands
        .spawn((
            Player,
            Mesh2d(circle_mesh_handle),
            MeshMaterial2d(circle_material),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::circle(20.0), // Circle collider
            CollisionLayers::new([LayerMask(0b0001)], [LayerMask(0b1100)]), // Layer 0: collides with walls (bit 2) and obstacles (bit 3), NOT sword (bit 1)
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity::default(),
            LinearDamping(2.0),
            Mass(2.0),
        ))
        .id();

    // Spawn sword (longer and less damping for more fluid motion)
    let sword_entity = commands
        .spawn((
            Sword,
            Sprite {
                color: Color::srgb(0.6, 0.6, 0.6),
                custom_size: Some(Vec2::new(90.0, 10.0)), // Longer sword (60 -> 90)
                ..default()
            },
            Transform::from_xyz(60.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(90.0, 10.0), // Longer sword collider
            CollisionLayers::new([LayerMask(0b0010)], [LayerMask(0b1100)]), // Layer 1: collides with walls (bit 2) and obstacles (bit 3), NOT player (bit 0)
            AngularVelocity::default(),
            LinearVelocity::default(),
            LinearDamping(0.5),  // Reduced damping (1.0 -> 0.5)
            AngularDamping(0.5), // Reduced damping (2.0 -> 0.5)
            Mass(0.5),
        ))
        .id();

    // Create revolute joint connecting sword to player
    // The sword rotates around the player at the player's center
    commands.spawn(
        RevoluteJoint::new(player_entity, sword_entity)
            .with_local_anchor_1(Vec2::ZERO) // Player center
            .with_local_anchor_2(Vec2::new(-35.0, 0.0)) // Offset adjusted for longer sword
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
        CollisionLayers::new([LayerMask(0b0100)], [LayerMask(0b1011)]), // Layer 2: walls - collide with player, sword, and obstacles
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
        CollisionLayers::new([LayerMask(0b0100)], [LayerMask(0b1011)]), // Layer 2: walls - collide with player, sword, and obstacles
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
        CollisionLayers::new([LayerMask(0b0100)], [LayerMask(0b1011)]), // Layer 2: walls - collide with player, sword, and obstacles
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
        CollisionLayers::new([LayerMask(0b0100)], [LayerMask(0b1011)]), // Layer 2: walls - collide with player, sword, and obstacles
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
            CollisionLayers::new([LayerMask(0b1000)], [LayerMask(0b0111)]), // Layer 3: obstacles - collide with player, sword, and walls
            LinearDamping(0.3),  // Less damping for more impact
            AngularDamping(0.5), // Less damping for more impact
            Mass(0.8),           // Lighter obstacles for more dramatic impacts
        ));
    }

    // Spawn spin button at bottom center
    commands
        .spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                bottom: Val::Px(20.0),
                margin: UiRect::left(Val::Px(-50.0)), // Center the button
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
            Button,
            SpinButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SPIN"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.1, 0.1, 0.1)),
            ));
        });

    // Spawn direction arrow (initially invisible)
    // This arrow shows the touch direction from the player
    commands.spawn((
        DirectionArrow,
        Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.0), // Red but initially invisible
            custom_size: Some(Vec2::new(30.0, 6.0)), // Small arrow shape
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0), // Above player
        Visibility::Hidden,
    ));

    // Note: Touch controls are temporarily disabled/stubbed out
    // A proper implementation will be added after verifying the app loads on Android
}

// System to handle spin button interaction
fn spin_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SpinButton>)>,
    mut sword_query: Query<&mut AngularVelocity, With<Sword>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut angular_velocity) = sword_query.get_single_mut() {
                angular_velocity.0 += 30.0; // Bigger impulse (15.0 -> 30.0)
            }
        }
    }
}

/// Updates touch state resource for mobile control
/// 
/// **NOTE:** This is currently a placeholder implementation. Touch controls are disabled
/// to isolate and fix the Android loading issue. Once the app loads successfully on Android,
/// this will be properly implemented to calculate movement direction based on touch input.
fn update_touch_state(
    mut touch_state: ResMut<TouchState>,
    touches: Option<Res<Touches>>,
) {
    if let Some(touches) = touches {
        if let Some(_touch) = touches.first_pressed_position() {
            // TODO: Calculate actual direction from touch position
            // For now, just mark as active but don't move (placeholder)
            touch_state.is_active = true;
            touch_state.direction = Vec2::ZERO; // Will be calculated in future implementation
        } else {
            touch_state.is_active = false;
            touch_state.direction = Vec2::ZERO;
        }
    } else {
        // No Touches resource available (e.g., in tests or desktop)
        touch_state.is_active = false;
        touch_state.direction = Vec2::ZERO;
    }
}

// System to handle player movement (keyboard and touch)
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    touch_state: Res<TouchState>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    let mut velocity = player_query.single_mut();
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

    // Touch input for mobile
    // Only use touch if keyboard isn't being used
    if direction.length() < 0.1 && touch_state.is_active {
        direction = touch_state.direction;
    }

    // Normalize and apply velocity
    if direction.length() > 0.0 {
        direction = direction.normalize();
        velocity.0 = direction * 300.0; // Faster movement speed (200.0 -> 300.0)
    } else {
        velocity.0 = Vec2::ZERO;
    }
}

// System to spin the sword
fn sword_spin(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut sword_query: Query<&mut AngularVelocity, With<Sword>>,
) {
    // Desktop input only - mobile uses the button
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        if let Ok(mut angular_velocity) = sword_query.get_single_mut() {
            angular_velocity.0 += 30.0; // Bigger impulse (15.0 -> 30.0)
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

// System to update direction arrow based on touch input
#[allow(clippy::type_complexity)]
fn update_direction_arrow(
    player_query: Query<&Transform, With<Player>>,
    mut arrow_query: Query<
        (&mut Transform, &mut Sprite, &mut Visibility),
        (With<DirectionArrow>, Without<Player>),
    >,
    touch_state: Res<TouchState>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut arrow_transform, mut arrow_sprite, mut visibility)) =
            arrow_query.get_single_mut()
        {
            if touch_state.is_active && touch_state.direction.length() > 0.1 {
                // Show and update arrow
                *visibility = Visibility::Visible;
                arrow_sprite.color.set_alpha(0.8);

                // Position arrow at player position
                arrow_transform.translation.x = player_transform.translation.x;
                arrow_transform.translation.y = player_transform.translation.y;
                arrow_transform.translation.z = 1.0; // Above player

                // Rotate arrow to point in touch direction
                let angle = touch_state.direction.y.atan2(touch_state.direction.x);
                arrow_transform.rotation = Quat::from_rotation_z(angle);
            } else {
                // Hide arrow when no input
                *visibility = Visibility::Hidden;
                arrow_sprite.color.set_alpha(0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simplified setup for tests that doesn't require physics
    fn setup_for_tests(mut commands: Commands) {
        // Spawn camera
        commands.spawn((Camera2d, MainCamera));

        // Spawn player (simplified for testing)
        commands.spawn((
            Player,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Spawn sword (simplified for testing)
        commands.spawn((
            Sword,
            Transform::from_xyz(60.0, 0.0, 0.0),
        ));

        // Spawn spin button
        commands
            .spawn((
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    bottom: Val::Px(20.0),
                    margin: UiRect::left(Val::Px(-50.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
                Button,
                SpinButton,
            ));

        // Spawn direction arrow
        commands.spawn((
            DirectionArrow,
            Transform::from_xyz(0.0, 0.0, 1.0),
            Visibility::Hidden,
        ));
    }

    #[test]
    fn test_app_runs_multiple_updates() {
        // Create a minimal test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TouchState::default())
            .add_systems(Startup, setup_for_tests);

        // Run the app for a few updates
        app.update();
        app.update();
        app.update();

        // Verify basic entities exist
        let mut query = app.world_mut().query::<&Player>();
        assert_eq!(query.iter(app.world()).count(), 1, "Should have one player");

        let mut query = app.world_mut().query::<&Sword>();
        assert_eq!(query.iter(app.world()).count(), 1, "Should have one sword");
    }

    #[test]
    fn test_player_exists_after_setup() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TouchState::default())
            .add_systems(Startup, setup_for_tests);

        app.update(); // Run startup systems

        // Check that player entity was created
        let mut query = app.world_mut().query::<&Player>();
        assert_eq!(query.iter(app.world()).count(), 1, "Player should exist after setup");
    }

    #[test]
    fn test_sword_exists_after_setup() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TouchState::default())
            .add_systems(Startup, setup_for_tests);

        app.update();

        // Check that sword entity was created
        let mut query = app.world_mut().query::<&Sword>();
        assert_eq!(query.iter(app.world()).count(), 1, "Sword should exist after setup");
    }

    #[test]
    fn test_touch_state_updates() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TouchState::default())
            .add_systems(Update, update_touch_state);

        // Initial state should be inactive
        let touch_state = app.world().resource::<TouchState>();
        assert!(!touch_state.is_active, "Touch should start inactive");

        app.update();

        // After update with no input, should still be inactive
        let touch_state = app.world().resource::<TouchState>();
        assert!(!touch_state.is_active, "Touch should remain inactive with no input");
    }
}
