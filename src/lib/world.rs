use bevy::prelude::*;


struct Solid;
struct Liquid;
struct Gas;

struct Grass {
	solid_state: Solid,
}

struct Stone {
	solid_state: Solid,
}

struct Air {
	solid_state: Gas,
}


enum BlockTypes{
	Grass,
	Stone,
	Air,
}
enum SolidStates{
	Solid,
	Liquid,
	Gas,
}

struct Block{
	chunk_position: Vec3,
	block_type: BlockTypes,
	solid_state: SolidStates,
}
impl Block{
	fn new(
		chunk_position: Vec3,
		Block_type: BlockTypes,
	) -> Block{
		return Block{
			chunk_position: chunk_position,
			block_type: Block_type,
			solid_state: SolidStates::Solid,
		}
	}
}

struct Chunk{
	position: Vec3,
	blocks: Vec<Block>,
}

impl Chunk{
	fn new(
		position: Vec3,
	) -> Chunk{
		let mut blocks = Vec::<Block>::new();
		for x in 0..8{
			for y in 0..8{
				for z in 0..8{
					blocks.push(
						Block::new(
							Vec3::new(x as f32,y as f32,z as f32),
							BlockTypes::Grass,
						)
					)
				}
			}
		}
		return Chunk{
			position: position,
			blocks: blocks,
		}
	}
}

pub fn test(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
){
	let mut spawn_chunck = Chunk::new(
		Vec3::new(0.0,0.0,0.0)
	);
	
	for block in &mut spawn_chunck.blocks {
		commands.spawn(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgb(0.7,0.7,0.7).into()),
			transform: Transform::from_xyz(
				(spawn_chunck.position.x*8.) + block.chunk_position.x,
				(spawn_chunck.position.y*8.) + block.chunk_position.y,
				(spawn_chunck.position.z*8.) + block.chunk_position.z,
			),
			..default()
		});
	}

}

