use bevy::{prelude::*, math::*};
use std::collections::HashMap;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server:Res<AssetServer>) {
    //camera
    commands.spawn(Camera2dBundle::default());
    let grid = Grid::generate(&mut commands, asset_server);
    commands.spawn(grid);
}
const GRIDSIZE: usize = 32;
const TILESIZE: f32 = 16.;


#[derive(Component)]
struct Grid {
    tiles: [[Tile; GRIDSIZE]; GRIDSIZE]
}


#[derive(Component, Copy, Clone, Default)]
struct Tile {
    pos: (i32, i32),
    contents: [Option<TileContent>; 16],
    stuff: usize,
    neighbors: [Option<Entity>; 4]
}
#[derive(Copy, Clone)]
struct TileContent {
    this: Entity,
    tile_type: TileType
}

fn index_to_pos(i: usize, j: usize) -> (i32, i32) {
    (i as i32 - GRIDSIZE as i32/2, j as i32 -  GRIDSIZE as i32/2)
}
impl Tile {
    fn push(mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, tile_type: TileType) -> Self {
        let world_pos = vec2(
            self.pos.0 as f32 * TILESIZE,
            self.pos.1 as f32 * TILESIZE,
        );
        let transform = Transform { 
            translation: world_pos.extend(0.), 
            ..default()
        };
        let sprite = Sprite {
            custom_size: Some(vec2(TILESIZE, TILESIZE)),
            ..default()
        };
        let ent = match tile_type {
            TileType::Copper => {
                let texture = asset_server.load("textures/Copper.png");
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id()
            },
            TileType::Iron => {
                let texture = asset_server.load("textures/Iron.png");
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id()
            },
            TileType::Silicon => {
                let texture = asset_server.load("textures/Silicon.png");
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id()
            },
            TileType::Printer3D => {
                let texture = asset_server.load("textures/3D_Printer.png");
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id()
            },
            TileType::WireExtruder => {
                let texture = asset_server.load("textures/Wire_Extruder.png");
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id()
            },
            TileType::Worker => {
                let texture = asset_server.load("textures/Worker_Arm.png");
                let arm = 
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id();
                let texture = asset_server.load("textures/Worker_Base.png");
                let sprite = Sprite {
                    custom_size: Some(vec2(TILESIZE, TILESIZE)),
                    ..default()
                };
                let base = 
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id();
                commands.entity(base).push_children(&[arm]);
                base
            }
        };
        let new_content = TileContent {this: ent, tile_type};
        self.contents[self.stuff] = Some(new_content);
        self.stuff += 1;
        self
    }
}

impl Grid {
    pub fn generate(commands: &mut Commands, asset_server:Res<AssetServer>) -> Grid {
        let mut tiles: [[Tile; GRIDSIZE]; GRIDSIZE] = Default::default();
        let mut tile_ents: [[Option<Entity>; GRIDSIZE + 2]; GRIDSIZE + 2] = [[None; 34]; 34];
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                let pos = index_to_pos(i,j);
                tiles[i][j].pos = pos;
                tile_ents[i+1][j+1] = Some(commands.spawn(tiles[i][j]).id());
            }
        }
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                tiles[i][j].neighbors[0] = tile_ents[i+1][j+2];
                tiles[i][j].neighbors[1] = tile_ents[i+2][j+1];
                tiles[i][j].neighbors[2] = tile_ents[i+1][j];
                tiles[i][j].neighbors[3] = tile_ents[i][j+1];
            }
        }
        let center = GRIDSIZE/2;
        tiles[center][center].push(commands, &asset_server, TileType::Printer3D);
        tiles[center+1][center].push(commands, &asset_server, TileType::Iron);
        tiles[center-1][center].push(commands, &asset_server, TileType::Copper);
        tiles[center][center-1].push(commands, &asset_server, TileType::Silicon);
        tiles[center+1][center].push(commands, &asset_server, TileType::WireExtruder);
        tiles[center-1][center].push(commands, &asset_server, TileType::WireExtruder);
        tiles[center][center-1].push(commands, &asset_server, TileType::WireExtruder);
        tiles[center][center+1].push(commands, &asset_server, TileType::Worker);
        Grid {tiles: tiles}
    }
}

#[derive(Component, Copy, Clone)]
enum TileType {
    Copper,
    Iron,
    Silicon,
    Printer3D,
    WireExtruder,
    Worker,
}
