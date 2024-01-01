use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "First Game Ever".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
        .build(),

    )
    .add_systems(Startup, setup)
    .add_systems(Update, character_movement)
    .run();
}
        
    

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());


    let texture= asset_server.load("dogpsi.png");

    // use a png file in the assets folder

commands.spawn(SpriteBundle {
    sprite: Sprite {
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()

    },
    texture,
    ..default()
});

}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

   for (mut transform, _) in &mut characters {
    if input.pressed(KeyCode::W) {
        transform.translation.y += 100. * time.delta_seconds();

    }

    if input.pressed(KeyCode::A) {
        transform.translation.x -= 100. * time.delta_seconds();


    }

    if input.pressed(KeyCode::S) {
        transform.translation.y -= 100. * time.delta_seconds();

    }


    if input.pressed(KeyCode::D) {
        transform.translation.x += 100. * time.delta_seconds();


        }
    

    // create functions for Moving left and use the keyboard input to control the direction

        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 100. * time.delta_seconds();

            
        }    

        
        
        
        // create functions for Moving right and use the keyboard input to control the direction
        
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 100. * time.delta_seconds();

            
        }


        // create functions for Moving up and use the keyboard input to control the direction
        
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 100. * time.delta_seconds();

            
        }

        // create functions for Moving down and use the keyboard input to control the direction
        
        

        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 100. * time.delta_seconds();

            
        }
        // // create functions for Jumping and use the keyboard input to control the direction

        // if input.pressed(KeyCode::Space) {
        //     transform.translation.y += 100. * time.delta_seconds();

            
        // }

        // // create functions for casting spells and use the keyboard input to control the direction

        // if input.pressed(KeyCode::E) {
        //     transform.translation.y += 100. * time.delta_seconds();

            
        // }

    }
}


