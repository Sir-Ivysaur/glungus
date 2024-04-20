use bevy::prelude::Commands;
use bevy::prelude::*;

mod glunger;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(SpriteBundle {
		texture: asset_server.load("assets/glungus.png"),
		..default()
	});
}  