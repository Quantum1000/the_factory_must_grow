use bevy::{prelude::*, math::*, utils::HashMap};
use std::f32::consts::PI;
use rand::Rng;

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
    Grid::generate(&mut commands, asset_server);
}
const GRIDSIZE: usize = 512;
const ORE_SPACING: usize = 32;
const TILESIZE: f32 = 16.;


#[derive(Component)]
struct Grid {
    tiles: Vec<Tile>
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
    tile_type: TileType,
    rot: u8
}

fn index_to_pos(i: usize, j: usize) -> (i32, i32) {
    (i as i32 - GRIDSIZE as i32/2, j as i32 -  GRIDSIZE as i32/2)
}
impl Tile {
    fn push(mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, tile_type: TileType, rot: u8) -> Self {
        let empt_map = HashMap::from([
            (TileType::Empty, true),
        ]);
        if let Some(_) = empt_map.get(&tile_type) {
            if self.stuff > 0 {
                self.contents[self.stuff-1] = None;
                self.stuff -= 1;
            }
            return self
        };
        let world_pos = vec2(
            self.pos.0 as f32 * TILESIZE,
            self.pos.1 as f32 * TILESIZE,
        );
        let transform = Transform { 
            translation: world_pos.extend(0.),
            rotation: Quat::from_axis_angle(Vec3::Z, PI / 2. * rot as f32),
            ..default()
        };
        let sprite = Sprite {
            custom_size: Some(vec2(TILESIZE, TILESIZE)),
            ..default()
        };
        let mut prev_tile = TileType::Empty;
        if self.stuff > 0 {
            if let Some(thing) = self.contents[self.stuff-1] {
                prev_tile = thing.tile_type;
            }
        }
        let build_map = HashMap::from([
            (TileType::Printer3D, true),
            (TileType::WireExtruder, true),
            (TileType::Worker, true),
        ]);
        let ore_map = HashMap::from([
            (TileType::Iron, true),
            (TileType::Silicon, true),
            (TileType::Copper, true),
        ]);
        let is_empt = if let Some(b) = empt_map.get(&prev_tile) {*b} else {false};
        let is_ore = if let Some(b) = ore_map.get(&prev_tile) {*b} else {false};
        let is_build = if let Some(b) = build_map.get(&prev_tile) {*b} else {false};

        let ent = match tile_type {
            TileType::Empty => {None},
            TileType::Copper => {
                if is_empt
                {
                    let texture = asset_server.load("textures/Copper.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id())
                }
                else {None}
            },
            TileType::Iron => {
                if is_empt
                {
                    let texture = asset_server.load("textures/Iron.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id())
                }
                else {None}
            },
            TileType::Silicon => {
                if is_empt
                {
                    let texture = asset_server.load("textures/Silicon.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id())
                }
                else {None}
            },
            TileType::Printer3D => {
                let texture = asset_server.load("textures/3D_Printer.png");
                Some(commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id())
            },
            TileType::WireExtruder => {
                let texture = asset_server.load("textures/Wire_Extruder.png");
                Some(commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id())
            },
            TileType::Worker => {
                let texture = asset_server.load("textures/Worker_Base.png");
                let base = 
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id();
                let transform = Transform::from_translation(Vec3::Y * TILESIZE / 2.);
                let sprite = Sprite {
                    custom_size: Some(vec2(TILESIZE, TILESIZE*2.)),
                    ..default()
                };
                let texture = asset_server.load("textures/Worker_Arm.png");
                let arm = 
                commands.spawn((
                    SpriteBundle {transform, sprite, texture, ..default()},
                    tile_type
                )).id();
                commands.entity(base).push_children(&[arm]);
                Some(base)
            }
        };
        if let Some(new_ent) = ent {
            let new_content = TileContent {this: new_ent, tile_type, rot};
            self.contents[self.stuff] = Some(new_content);
            self.stuff += 1;
        }
        self
    }
}

impl Grid {
    pub fn generate(commands: &mut Commands, asset_server:Res<AssetServer>) {
        let mut grid = Grid {tiles: vec![Tile::default(); GRIDSIZE.pow(2)]};
        let mut tile_ents: Vec<Option<Entity>> = vec![None; (GRIDSIZE+2).pow(2)];
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                let this_tile = &mut grid.tiles[i * GRIDSIZE + j];
                let pos = index_to_pos(i,j);
                this_tile.pos = pos;
                tile_ents[(i+1) * (GRIDSIZE + 2) + j+1] = Some(commands.spawn(*this_tile).id());
            }
        }
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                let this_tile = &mut grid.tiles[i * GRIDSIZE + j];
                this_tile.neighbors[0] = tile_ents[(i+1) * (GRIDSIZE + 2) + j+2];
                this_tile.neighbors[1] = tile_ents[(i+2) * (GRIDSIZE + 2) + j+1];
                this_tile.neighbors[2] = tile_ents[(i+1) * (GRIDSIZE + 2) + j];
                this_tile.neighbors[3] = tile_ents[i * (GRIDSIZE + 2) + j+1];
            }
        }
        let c = GRIDSIZE/2;
        grid.tiles[c * GRIDSIZE + c].push(commands, &asset_server, TileType::Printer3D, 0);
        grid.tiles[(c+1) * GRIDSIZE + c].push(commands, &asset_server, TileType::Iron, 0);
        grid.tiles[(c-1) * GRIDSIZE + c].push(commands, &asset_server, TileType::Copper, 0);
        grid.tiles[c * GRIDSIZE + (c-1)].push(commands, &asset_server, TileType::Silicon, 0);
        grid.tiles[(c+1) * GRIDSIZE + c].push(commands, &asset_server, TileType::WireExtruder, 0);
        grid.tiles[(c-1) * GRIDSIZE + c].push(commands, &asset_server, TileType::WireExtruder, 2);
        grid.tiles[c * GRIDSIZE + (c-1)].push(commands, &asset_server, TileType::WireExtruder, 3);
        grid.tiles[c * GRIDSIZE + (c+1)].push(commands, &asset_server, TileType::Worker, 1);
        let mut rng = rand::thread_rng();
        for i in 0..(GRIDSIZE/ORE_SPACING) {
            for j in 0..(GRIDSIZE/ORE_SPACING) {
                let column_off = rng.gen_range(0..ORE_SPACING);
                let row_off = rng.gen_range(0..ORE_SPACING);
                let column: usize = i * ORE_SPACING + column_off;
                let row: usize = j * ORE_SPACING + row_off;
                let ore = ORES[rng.gen_range(0..3)];
                let valid_moves = [I64Vec2::new(1, 0), I64Vec2::new(-1, 0), I64Vec2::new(0, 1), I64Vec2::new(0, -1)];
                let mut this_tile = grid.tiles[column * GRIDSIZE + row];
                this_tile.push(commands, &asset_server, ore, 0);
                let mut pos = I64Vec2::new(column as i64, row as i64);
                for _ in 0..32 {
                    pos = pos + valid_moves[rng.gen_range(0..4)];
                    if pos.x < 0 || pos.x >= GRIDSIZE as i64 || pos.y < 0 || pos.y >= GRIDSIZE as i64 {
                        break
                    }
                    this_tile = grid.tiles[pos.x as usize * GRIDSIZE + pos.y as usize];
                    this_tile.push(commands, &asset_server, ore, 0);
                }
            }
        }
        commands.spawn(grid);
    }
}

#[derive(Component, Copy, Clone, Default, PartialEq, Eq, Hash)]
enum TileType {
    #[default] Empty,
    Copper,
    Iron,
    Silicon,
    Printer3D,
    WireExtruder,
    Worker,
}

const ORES: [TileType; 3] = [TileType::Copper, TileType::Iron, TileType::Silicon];