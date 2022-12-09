use bevy::prelude::*;


fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.02,0.0,0.02)))
		.add_startup_system(spawn_basic_scene)
		.add_startup_system(spawn_camera)
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Better than minecraft".to_string(),
				width: 1280.0,
				height: 720.0,
				resizable: false,
				..default()
			},
			..default()
		}))
		.run();
}


fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 1.8, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}

fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>, 
	asset_server: Res<AssetServer>
){
	commands.spawn((
		TextBundle::from_section(
			"BMC.rs",
			TextStyle {
				font: asset_server.load(r"fonts\MyFont.ttf"),
				font_size: 50.0,
				color: Color::CYAN,
			},
		)
		.with_text_alignment(TextAlignment::BOTTOM_CENTER)
		.with_style(Style {
            position_type: PositionType::Relative,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
	));

	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3,0.5,0.3).into()),
		..default()
	});

		commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
}