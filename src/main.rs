use bevy::{prelude::*, math::*};
use std::f32::consts::PI;
use rand::Rng;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_systems(Startup, setup)
    .add_systems(Update, camera_control)
    .run();
}

fn setup(mut commands: Commands, asset_server:Res<AssetServer>) {
    //camera
    commands.spawn(Camera2dBundle::default());
    // ui
    let texture_3d_printer = asset_server.load("textures/3D_Printer.png");
    let texture_wire_extruder = asset_server.load("textures/Wire_Extruder.png");
    let texture_worker = asset_server.load("textures/Worker_Base.png");
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::End,
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| { 
        parent.spawn(NodeBundle {
            style: Style {
                column_gap: Val::Px(8.),
                padding: UiRect::all(Val::Px(4.)),
                border: UiRect::all(Val::Px(1.)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::End,
                ..default()
            },
            border_color: BorderColor {
                0: Color::rgb(0.5, 0.5, 0.5)
            },
            background_color: BackgroundColor {
                0: Color::rgb(0.2, 0.2, 0.2)
            },
            ..default()
        })
        .with_children(|parent| { 
            parent.spawn(ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(4.)),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: BorderColor {
                    0: Color::rgb(0.9, 0.9, 0.9)
                },
                background_color: BackgroundColor {
                    0: Color::rgb(0.1, 0.1, 0.1)
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(32.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    image: UiImage {texture: texture_3d_printer, ..default()},
                    ..default()
                });
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(4.)),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: BorderColor {
                    0: Color::rgb(0.9, 0.9, 0.9)
                },
                background_color: BackgroundColor {
                    0: Color::rgb(0.1, 0.1, 0.1)
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(32.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    image: UiImage {texture: texture_wire_extruder, ..default()},
                    ..default()
                });
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(4.)),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                border_color: BorderColor {
                    0: Color::rgb(0.9, 0.9, 0.9)
                },
                background_color: BackgroundColor {
                    0: Color::rgb(0.1, 0.1, 0.1)
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(32.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    image: UiImage {texture: texture_worker, ..default()},
                    ..default()
                });
            });
        });
    });
    //terrain
    Grid::generate(&mut commands, asset_server);
}

const CAMERA_SPEED: f32 = 256.;
fn camera_control(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut commanded_pan = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        commanded_pan += Vec3::Y
    }
    if keys.pressed(KeyCode::A) {
        commanded_pan += Vec3::NEG_X
    }
    if keys.pressed(KeyCode::S) {
        commanded_pan += Vec3::NEG_Y
    }
    if keys.pressed(KeyCode::D) {
        commanded_pan += Vec3::X
    }
    let pan_amount = commanded_pan.normalize_or_zero() * CAMERA_SPEED * time.delta().as_secs_f32();
    let mut camera_t = camera.single_mut();
    camera_t.translation += pan_amount;
}
const GRIDSIZE: usize = 512;
const ORE_SPACING: usize = 32;
const TILESIZE: f32 = 16.;


#[derive(Component)]
struct Grid {
    tiles: Vec<Tile>,
}


#[derive(Component, Copy, Clone, Default)]
struct Tile {
    pos: (i32, i32),
    contents: [Option<TileContent>; 16],
    stuff: usize,
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
    fn push(mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, tile_type: TileType, rot: u8) -> (Self, bool) {
        let mut success = false;
        // the position in world space
        let world_pos = vec2(
            self.pos.0 as f32 * TILESIZE,
            self.pos.1 as f32 * TILESIZE,
        );
        // a transform of the tile in world space
        let transform = Transform { 
            translation: world_pos.extend(0.),
            rotation: Quat::from_axis_angle(Vec3::Z, PI / 2. * rot as f32),
            ..default()
        };
        // information on the tile size
        let sprite = Sprite {
            custom_size: Some(vec2(TILESIZE, TILESIZE)),
            ..default()
        };
        // fetch the previous tile for blocking logic
        let mut prev_tile = TileType::Empty;
        let is_empt = self.stuff == 0;
        if !is_empt {
            prev_tile = self.contents[self.stuff - 1].unwrap().tile_type;
        }
        // Handle the different types of tiles that can be added
        let ent = match tile_type {
            // If an empty tile is pushed, this function acts like a pop!
            TileType::Empty => {
                // can't pop an object from the tile if the tile is empty
                if !is_empt {
                    self.contents[self.stuff-1] = None; // this line is technically unnecessary
                    self.stuff -= 1;
                    success = true;
                }
                // early stopping; None would also work here
                return (self, success)
            },
            // ore handling
            TileType::Copper => {
                // is the bottom layer, and can only be placed if the tile is empty
                if is_empt
                {
                    success = true;
                    // load the correct texture for the tile
                    let texture = asset_server.load("textures/Copper.png");
                    // spawn the tile and return the entity
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type,
                        Ore
                    )).id())
                }
                else {None}
            },
            TileType::Iron => {
                if is_empt
                {
                    success = true;
                    let texture = asset_server.load("textures/Iron.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type,
                        Ore
                    )).id())
                }
                else {None}
            },
            TileType::Silicon => {
                if is_empt
                {
                    success = true;
                    let texture = asset_server.load("textures/Silicon.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type,
                        Ore
                    )).id())
                }
                else {None}
            },
            // building handling
            TileType::Printer3D => {
                if prev_tile.layer() < TileType::Printer3D.layer() {
                    success = true;
                    let texture = asset_server.load("textures/3D_Printer.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id())
                }
                // since this is something player machines might place, there's a need to return failure
                else {None}
            },
            TileType::WireExtruder => {
                if prev_tile.layer() < TileType::Printer3D.layer() {
                    success = true;
                    let texture = asset_server.load("textures/Wire_Extruder.png");
                    Some(commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id())
                }
                else {None}
            },
            TileType::Worker => {
                if prev_tile.layer() < TileType::Printer3D.layer() {
                    success = true;
                    // the worker is a composite tiles, made of multiple entities. First, spawn the base.
                    let texture = asset_server.load("textures/Worker_Base.png");
                    let base = 
                    commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id();
                    // transform and sprite don't implement copy, so remake them
                    let transform = Transform::from_translation(Vec3::Y * TILESIZE / 2.);
                    let sprite = Sprite {
                        custom_size: Some(vec2(TILESIZE, TILESIZE*2.)),
                        ..default()
                    };
                    let texture = asset_server.load("textures/Worker_Arm.png");
                    // spawn the arm
                    let arm = 
                    commands.spawn((
                        SpriteBundle {transform, sprite, texture, ..default()},
                        tile_type
                    )).id();
                    // make the arm the child of the base
                    commands.entity(base).push_children(&[arm]);
                    // return the base entity
                    Some(base)
                }
                else {None}
            }
            TileType::ResourceTile(contents) => {
                // sanity check
                if contents.1 > 0 {
                    // resource handling
                    // resources can be stacked, so try that
                    if prev_tile.layer() == TileType::ResourceTile(contents).layer() {
                        // if the stack would be overfilled, just fail. There should probably be a way to report back that it failed.
                        if let TileType::ResourceTile(mut prev_contents) = prev_tile {
                            if prev_contents.1 + contents.1 < MAX_RESOURCE_PER_TILE {
                                success = true;
                                for resource in contents.0 {
                                    prev_contents.0[prev_contents.1] = resource;
                                    prev_contents.1 += 1;
                                }
                            }
                        }
                        None
                    }
                    // if the resources can't be stacked, add the resource tile
                    else {
                        success = true;
                        // texture is WIP
                        let texture = asset_server.load("textures/Resource_Tile.png");
                        Some(commands.spawn((
                            SpriteBundle {transform, sprite, texture, ..default()},
                            tile_type
                        )).id())
                    }
                }
                else {None}
            }
        };
        if let Some(new_ent) = ent {
            let new_content = TileContent {this: new_ent, tile_type, rot};
            self.contents[self.stuff] = Some(new_content);
            self.stuff += 1;
        }
        (self, success)
    }
}

impl Grid {
    pub fn generate(commands: &mut Commands, asset_server:Res<AssetServer>) {
        let mut tiles = vec![Tile::default(); GRIDSIZE.pow(2)];
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                tiles[i * GRIDSIZE + j].pos = index_to_pos(i,j);
            }
        }
        let c = GRIDSIZE/2;
        tiles[c * GRIDSIZE + c] = tiles[c * GRIDSIZE + c].push(commands, &asset_server, TileType::Printer3D, 0).0;
        tiles[(c+1) * GRIDSIZE + c] = tiles[(c+1) * GRIDSIZE + c].push(commands, &asset_server, TileType::Iron, 0).0;
        tiles[(c-1) * GRIDSIZE + c] = tiles[(c-1) * GRIDSIZE + c].push(commands, &asset_server, TileType::Copper, 0).0;
        tiles[c * GRIDSIZE + (c-1)] = tiles[c * GRIDSIZE + (c-1)].push(commands, &asset_server, TileType::Silicon, 0).0;
        tiles[(c+1) * GRIDSIZE + c] = tiles[(c+1) * GRIDSIZE + c].push(commands, &asset_server, TileType::WireExtruder, 0).0;
        tiles[(c-1) * GRIDSIZE + c] = tiles[(c-1) * GRIDSIZE + c].push(commands, &asset_server, TileType::WireExtruder, 2).0;
        tiles[c * GRIDSIZE + (c-1)] = tiles[c * GRIDSIZE + (c-1)].push(commands, &asset_server, TileType::WireExtruder, 3).0;
        tiles[c * GRIDSIZE + (c+1)] = tiles[c * GRIDSIZE + (c+1)].push(commands, &asset_server, TileType::Worker, 1).0;
        let mut rng = rand::thread_rng();
        for i in 0..(GRIDSIZE/ORE_SPACING) {
            for j in 0..(GRIDSIZE/ORE_SPACING) {
                let column_off = rng.gen_range(0..ORE_SPACING);
                let row_off = rng.gen_range(0..ORE_SPACING);
                let column: usize = i * ORE_SPACING + column_off;
                let row: usize = j * ORE_SPACING + row_off;
                let ore = ORES[rng.gen_range(0..3)];
                let valid_moves = [I64Vec2::new(1, 0), I64Vec2::new(-1, 0), I64Vec2::new(0, 1), I64Vec2::new(0, -1)];
                let mut this_tile = &mut tiles[column * GRIDSIZE + row];
                *this_tile = this_tile.push(commands, &asset_server, ore, 0).0;
                let mut pos = I64Vec2::new(column as i64, row as i64);
                for _ in 0..32 {
                    pos = pos + valid_moves[rng.gen_range(0..4)];
                    if pos.x < 0 || pos.x >= GRIDSIZE as i64 || pos.y < 0 || pos.y >= GRIDSIZE as i64 {
                        break
                    }
                    this_tile = &mut tiles[pos.x as usize * GRIDSIZE + pos.y as usize];
                    *this_tile = this_tile.push(commands, &asset_server, ore, 0).0;
                }
            }
        }
        let ent = commands.spawn(Grid {tiles}).id();
    }
}

#[derive(Component, Copy, Clone, Default)]
enum TileType {
    #[default] Empty,
    Copper,
    Iron,
    Silicon,
    Printer3D,
    WireExtruder,
    Worker,
    ResourceTile(([Resource; MAX_RESOURCE_PER_TILE], usize)),
}

#[derive(Component)]
struct Ore;

const MAX_RESOURCE_PER_TILE: usize = 32;

impl TileType {
    pub fn layer(self) -> usize {
        match self {
            TileType::Empty => 0,
            TileType::Copper => 1,
            TileType::Iron => 1,
            TileType::Silicon => 1,
            TileType::Printer3D => 2,
            TileType::WireExtruder => 2,
            TileType::Worker => 2,
            TileType::ResourceTile(_) => 3,
        }
    }

    pub fn ore(self) -> Option<Resource> {
        match self {
            TileType::Empty => None,
            TileType::Copper => Some(Resource::Copper),
            TileType::Iron => Some(Resource::Iron),
            TileType::Silicon => Some(Resource::Silicon),
            TileType::Printer3D => None,
            TileType::WireExtruder => None,
            TileType::Worker => None,
            TileType::ResourceTile(_) => None,
        }
    }
}

#[derive(Copy, Clone, Default)]
enum Resource {
    #[default] Copper, // used as a default for values made empty by the stack not having filled yet
    Iron,
    Silicon
}

const ORES: [TileType; 3] = [TileType::Copper, TileType::Iron, TileType::Silicon];