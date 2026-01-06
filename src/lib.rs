use avian2d::prelude::*;
use bevy::prelude::*;
use virtual_joystick::*;

// ID for the virtual joystick
#[derive(Default, Debug, Reflect, Hash, Clone, PartialEq, Eq)]
enum JoystickId {
    #[default]
    Movement,
}

#[bevy_main]
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
            VirtualJoystickPlugin::<JoystickId>::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO)) // Top-down game, no gravity
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                spin_button_interaction,
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

#[derive(Component)]
struct SpinButton;

// Setup system - initializes the game world
fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn((Camera2d, MainCamera));

    // Spawn player (circle shape to prevent sword collision)
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
            LinearDamping(0.5), // Reduced damping (1.0 -> 0.5)
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
            LinearDamping(0.3), // Less damping for more impact
            AngularDamping(0.5), // Less damping for more impact
            Mass(0.8), // Lighter obstacles for more dramatic impacts
        ));
    }

    // Spawn virtual joystick (floating type that appears where touched)
    create_joystick(
        &mut commands,
        JoystickId::Movement,
        Handle::default(), // No knob image
        Handle::default(), // No background image
        Some(Color::srgba(0.2, 0.4, 0.8, 0.8)), // Knob color (blue to match player)
        Some(Color::srgba(0.3, 0.3, 0.3, 0.5)), // Background color (semi-transparent gray)
        Some(Color::srgba(0.1, 0.1, 0.1, 0.3)), // Interactable area color
        Vec2::new(75.0, 75.0),           // Knob size
        Vec2::new(150.0, 150.0),         // Background size
        Node {
            width: Val::Percent(100.0),  // Whole screen
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.0),
            bottom: Val::Percent(0.0),
            ..default()
        },
        JoystickFloating, // Appears where user touches
        NoAction,
    );

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

// System to handle player movement (keyboard and virtual joystick)
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
    mut joystick: EventReader<VirtualJoystickEvent<JoystickId>>,
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

    // Virtual joystick input for mobile
    for event in joystick.read() {
        let axis = event.axis();
        // Only use joystick if keyboard isn't being used
        if direction.length() < 0.1 {
            direction = *axis;
        }
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
