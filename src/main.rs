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
	mut materials: ResMut<Assets<StandardMaterial>>
){
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3,0.5,0.3).into()),
		..default()
	});
}