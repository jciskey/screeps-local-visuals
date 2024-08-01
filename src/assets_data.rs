use lazy_static::lazy_static;
use std::io::Cursor;
use std::io;
use image::io::Reader;
use image::RgbaImage;

#[cfg(target_family = "unix")]
macro_rules! include_asset {($folder:literal, $filename:literal) => (
  include_bytes!(concat!("assets/", $folder, "/", $filename))
)}

#[cfg(target_family = "windows")]
macro_rules! include_asset {(filename:str) => (
  include_bytes!(concat!(r"assets\", $folder, r"\", $filename))
)}

fn decode_image_data(image_data: &[u8]) -> Result<RgbaImage, io::Error> {
  let tile_img_reader = Reader::new(Cursor::new(image_data))
    .with_guessed_format()?;
  let tile_img_dynamic = tile_img_reader.decode()
    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
  Ok(tile_img_dynamic.to_rgba8())
}

macro_rules! include_image {($folder:literal, $filename:literal) => (
  decode_image_data(include_asset!($folder, $filename))
    .expect(&format!("Could not decode image {}/{}", $folder, $filename))
)}

// To use these resources, you need to do `&NAME` or `&*NAME`.
//
// The way that this works is that under the hood, `lazy_static` creates a unique
// type that has a dereference operator `*` that initializes the value if needed
// and otherwise just returns the stored value.
//
// In a match statement where multiple of these are returned from different branches,
// just a `&NAME` can cause problems for type inference. Using either a type
// annotation of `&OutputImage` or `&RgbaImage` on the resulting variable can
// fix this, or you can use `&*NAME`.
lazy_static! {
  pub static ref FREE_MONO_FONT: rusttype::Font<'static> =
    rusttype::Font::try_from_bytes(include_asset!("fonts", "FreeMono.ttf"))
    .expect("Could not load FreeMono font");

  pub static ref TERRAIN_PLAIN_IMG: RgbaImage = include_image!("terrains", "plain.png");
  pub static ref TERRAIN_SWAMP_IMG: RgbaImage = include_image!("terrains", "swamp.png");
  pub static ref TERRAIN_WALL_IMG: RgbaImage = include_image!("terrains", "wall.png");

  pub static ref RESOURCE_SOURCE_IMG: RgbaImage = include_image!("resources", "source.png");
  pub static ref RESOURCE_HYDROGEN_IMG: RgbaImage = include_image!("resources", "H.png");
  pub static ref RESOURCE_OXYGEN_IMG: RgbaImage = include_image!("resources", "O.png");
  pub static ref RESOURCE_KEANIUM_IMG: RgbaImage = include_image!("resources", "K.png");
  pub static ref RESOURCE_LEMERGIUM_IMG: RgbaImage = include_image!("resources", "L.png");
  pub static ref RESOURCE_UTRIUM_IMG: RgbaImage = include_image!("resources", "U.png");
  pub static ref RESOURCE_ZYNTHIUM_IMG: RgbaImage = include_image!("resources", "Z.png");
  pub static ref RESOURCE_CATALYST_IMG: RgbaImage = include_image!("resources", "X.png");
  pub static ref RESOURCE_UNKNOWN_IMG: RgbaImage = include_image!("resources", "unknown.png");

  pub static ref STRUCTURE_CONSTRUCTEDWALL_IMG: RgbaImage = include_image!("structures", "constructedWall.png");
  pub static ref STRUCTURE_CONTAINER_IMG: RgbaImage = include_image!("structures", "container.png");
  pub static ref STRUCTURE_CONTROLLER_IMG: RgbaImage = include_image!("structures", "controller.png");
  pub static ref STRUCTURE_EXTENSION_IMG: RgbaImage = include_image!("structures", "extension.png");
  pub static ref STRUCTURE_EXTRACTOR_IMG: RgbaImage = include_image!("structures", "extractor.png");
  pub static ref STRUCTURE_FACTORY_IMG: RgbaImage = include_image!("structures", "factory.png");
  pub static ref STRUCTURE_LAB_IMG: RgbaImage = include_image!("structures", "lab.png");
  pub static ref STRUCTURE_LINK_IMG: RgbaImage = include_image!("structures", "link.png");
  pub static ref STRUCTURE_NUKER_IMG: RgbaImage = include_image!("structures", "nuker.png");
  pub static ref STRUCTURE_OBSERVER_IMG: RgbaImage = include_image!("structures", "observer.png");
  pub static ref STRUCTURE_POWERSPAWN_IMG: RgbaImage = include_image!("structures", "powerSpawn.png");
  pub static ref STRUCTURE_RAMPART_IMG: RgbaImage = include_image!("structures", "rampart.png");
  pub static ref STRUCTURE_ROAD_IMG: RgbaImage = include_image!("structures", "road.png");
  pub static ref STRUCTURE_SPAWN_IMG: RgbaImage = include_image!("structures", "spawn.png");
  pub static ref STRUCTURE_STORAGE_IMG: RgbaImage = include_image!("structures", "storage.png");
  pub static ref STRUCTURE_TERMINAL_IMG: RgbaImage = include_image!("structures", "terminal.png");
  pub static ref STRUCTURE_TOWER_IMG: RgbaImage = include_image!("structures", "tower.png");
  pub static ref STRUCTURE_UNKNOWN_IMG: RgbaImage = include_image!("structures", "icon.png");
}
