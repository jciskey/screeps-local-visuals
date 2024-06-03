
use std::io::Cursor;
use image::io::Reader;

use crate::assets_data;

pub use screeps::constants::Terrain;

use screeps::constants::{
  ResourceType,
  structure::StructureType
};

use screeps::local::LocalCostMatrix;
use screeps::objects::Source;

use screeps_utils::offline_map::OfflineObject;

/// A helpful type alias for the type of images this library operates on
pub type OutputImage = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

/// The default number of columns in a room (x-coordinate)
pub const DEFAULT_ROOM_MAX_COLUMNS: u32 = 50;

/// The default number of rows in a room (y-coordinate)
pub const DEFAULT_ROOM_MAX_ROWS: u32 = 50;

/// The default scaling factor for the final image, meaning the number of pixels allocated for each room cell
pub const DEFAULT_SCALE_FACTOR: u32 = 50;

/// Represents the various different types of resources that can exist in a room
#[derive(Debug)]
pub enum Resource {
  Source,
  Hydrogen,
  Oxygen,
  Keanium,
  Lemergium,
  Utrium,
  Zynthium,
  Catalyst,
  Unknown,
}

/// Represents the various types of player-buildable structures
#[derive(Debug)]
pub enum BuildableStructure {
  ConstructedWall,
  Container,
  Controller,
  Extension,
  Extractor,
  Factory,
  Lab,
  Link,
  Nuker,
  Observer,
  PowerSpawn,
  Rampart,
  Road,
  Spawn,
  Storage,
  Terminal,
  Tower,
  Unknown,
}

impl From<Source> for Resource {
    #[inline]
    fn from(_source: Source) -> Resource {
      Resource::Source
    }
}

impl TryFrom<&ResourceType> for Resource {
    type Error = ();

    #[inline]
    fn try_from(resource_type: &ResourceType) -> Result<Resource, Self::Error> {
        Ok(match resource_type {
            ResourceType::Hydrogen   => Resource::Hydrogen,
            ResourceType::Oxygen     => Resource::Oxygen,
            ResourceType::Keanium    => Resource::Keanium,
            ResourceType::Lemergium  => Resource::Lemergium,
            ResourceType::Utrium     => Resource::Utrium,
            ResourceType::Zynthium   => Resource::Zynthium,
            ResourceType::Catalyst   => Resource::Catalyst,
            _                        => Resource::Unknown,
        })
    }
}

impl TryFrom<ResourceType> for Resource {
    type Error = ();

    #[inline]
    fn try_from(resource_type: ResourceType) -> Result<Resource, Self::Error> {
        (&resource_type).try_into()
    }
}

impl TryFrom<&OfflineObject> for Resource {
    type Error = ();

    #[inline]
    fn try_from(obj: &OfflineObject) -> Result<Resource, Self::Error> {
      match obj {
        OfflineObject::Source { .. } => Ok(Resource::Source),
        OfflineObject::Mineral { mineral_type, .. } => {
          Ok(match mineral_type {
              ResourceType::Hydrogen   => Resource::Hydrogen,
              ResourceType::Oxygen     => Resource::Oxygen,
              ResourceType::Keanium    => Resource::Keanium,
              ResourceType::Lemergium  => Resource::Lemergium,
              ResourceType::Utrium     => Resource::Utrium,
              ResourceType::Zynthium   => Resource::Zynthium,
              ResourceType::Catalyst   => Resource::Catalyst,
              _                        => Resource::Unknown,
          })
        },
        _ => Ok(Resource::Unknown)
      }
    }
}

impl TryFrom<OfflineObject> for Resource {
    type Error = ();

    #[inline]
    fn try_from(obj: OfflineObject) -> Result<Resource, Self::Error> {
        (&obj).try_into()
    }
}

impl TryFrom<StructureType> for BuildableStructure {
    type Error = ();

    #[inline]
    fn try_from(structure_type: StructureType) -> Result<BuildableStructure, Self::Error> {
        Ok(match structure_type {
            StructureType::Wall        => BuildableStructure::ConstructedWall,
            StructureType::Container   => BuildableStructure::Container,
            StructureType::Controller  => BuildableStructure::Controller,
            StructureType::Extension   => BuildableStructure::Extension,
            StructureType::Extractor   => BuildableStructure::Extractor,
            StructureType::Factory     => BuildableStructure::Factory,
            StructureType::Lab         => BuildableStructure::Lab,
            StructureType::Link        => BuildableStructure::Link,
            StructureType::Nuker       => BuildableStructure::Nuker,
            StructureType::Observer    => BuildableStructure::Observer,
            StructureType::PowerSpawn  => BuildableStructure::PowerSpawn,
            StructureType::Rampart     => BuildableStructure::Rampart,
            StructureType::Road        => BuildableStructure::Road,
            StructureType::Spawn       => BuildableStructure::Spawn,
            StructureType::Storage     => BuildableStructure::Storage,
            StructureType::Terminal    => BuildableStructure::Terminal,
            StructureType::Tower       => BuildableStructure::Tower,
            _                          => BuildableStructure::Unknown,
        })
    }
}

/// Creates an image with default size parameters
pub fn create_image() -> OutputImage {
  create_image_with_size_params(DEFAULT_ROOM_MAX_COLUMNS, DEFAULT_ROOM_MAX_ROWS, DEFAULT_SCALE_FACTOR)
}

/// Creates an image with user-supplied size parameters
pub fn create_image_with_size_params(room_max_cols: u32, room_max_rows: u32, scale_factor: u32) -> OutputImage {
  let mut imgbuf = image::ImageBuffer::new((room_max_cols * scale_factor) + 1 as u32, (room_max_rows * scale_factor) + 1 as u32);

  for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
    let r: u8 = 0;
    let g: u8 = 0;
    let b: u8 = 0;
    *pixel = image::Rgba([r, g, b, 255]);
  }

  imgbuf
}

/// Draws a grid on a default-sized image
pub fn draw_grid(imgbuf: &mut OutputImage)  {
  draw_grid_with_scale_factor(imgbuf, DEFAULT_SCALE_FACTOR)
}

/// Draws a grid on an image with user-supplied scaling
pub fn draw_grid_with_scale_factor(imgbuf: &mut OutputImage, scale_factor: u32) {
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    if (x % scale_factor == 0) | (y % scale_factor == 0)  {
      let r: u8 = 255;
      let g: u8 = 255;
      let b: u8 = 255;
      *pixel = image::Rgba([r, g, b, 128]);
    }
  }
}

/// Draws a text number on a default-sized image at a specific cell location
pub fn draw_text_number_xy(imgbuf: &mut OutputImage, col: u32, row: u32, text: &str) {
  draw_text_number_xy_with_scale_factor(imgbuf, col, row, text, DEFAULT_SCALE_FACTOR, DEFAULT_SCALE_FACTOR)
}

/// Draws a text number on a user-sized image at a specific cell location
pub fn draw_text_number_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, text: &str, scale_factor: u32, text_scale_factor: u32) {
  let x = (col * scale_factor + 2).try_into().unwrap();
  let y = (row * scale_factor + 2).try_into().unwrap();
  draw_text_number_raw(imgbuf, x, y, text, text_scale_factor as f32);
}

/// Underlying function for drawing numbers on an image at a specific cell location
fn draw_text_number_raw(imgbuf: &mut OutputImage, x: i32, y: i32, text: &str, text_scale_factor: f32) {
  // let scale = rusttype::Scale::uniform(15.0);
  let scale = rusttype::Scale::uniform(text_scale_factor);
  let font = rusttype::Font::try_from_bytes(assets_data::FREE_MONO_FONT_DATA).expect("Could not load FreeMono font");
  imageproc::drawing::draw_text_mut(imgbuf, image::Rgba([255,255,255,255]), x, y, scale, &font, text);
}

pub fn draw_cost_matrix(imgbuf: &mut OutputImage, cm: LocalCostMatrix, v_min: u8, v_max: u8, b_max: u8, a: u8, skip_out_of_bounds_values: bool) {
  draw_cost_matrix_with_scale_factor(imgbuf, cm, v_min, v_max, b_max, a, DEFAULT_SCALE_FACTOR, skip_out_of_bounds_values)
}

fn draw_cost_matrix_with_scale_factor(imgbuf: &mut OutputImage, cm: LocalCostMatrix, v_min: u8, v_max: u8, b_max: u8, a: u8, scale_factor: u32, skip_out_of_bounds_values: bool) {
  let alpha_overlay = get_cost_matrix_alpha_overlay(&cm, imgbuf.width(), imgbuf.height(), scale_factor, v_min, v_max, b_max, a, skip_out_of_bounds_values);
  image::imageops::overlay(imgbuf, &alpha_overlay, 0, 0);

  for (position, value) in cm.iter() {
    if value == 0 {
      continue;
    }

    if skip_out_of_bounds_values {
      if value > v_max {
        continue;
      }

      if value < v_min {
        continue;
      }
    }

    let col = position.x.u8();
    let row = position.y.u8();

    let text = value.to_string();

    let text_scale_factor = if value > 9 {
      ((scale_factor as f32) * 0.75) as u32
    }
    else {
      scale_factor
    };

    draw_text_number_xy_with_scale_factor(imgbuf, col.into(), row.into(), &text, scale_factor, text_scale_factor);
  }
}

fn get_cost_matrix_alpha_overlay(cm: &LocalCostMatrix, overlay_width: u32, overlay_height: u32, scale_factor: u32, v_min: u8, v_max: u8, b_max: u8, a: u8, skip_out_of_bounds_values: bool) -> OutputImage {
  let mut alpha_overlay = image::ImageBuffer::new(overlay_width, overlay_height);

  for (position, value) in cm.iter() {
    let clamped_value = if value > v_max {
      if skip_out_of_bounds_values {
        continue;
      }
      v_max
    }
    else {
      if value < v_min {
        if skip_out_of_bounds_values {
          continue;
        }
        v_min
      }
      else {
        value
      }
    };

    let range = (v_max - v_min) as f32;

    let b = b_max - lerp(0.0, b_max as f32, ((clamped_value - v_min) as f32)/range) as u8;

    let others = lerp(b_max as f32, 0.0, (b as f32)/(b_max as f32)) as u8;

    let alpha = if value == 0 {
      0
    }
    else {
      a
    };

    let rgba = image::Rgba([others, others, b, alpha]);
    // let rgba = image::Rgba([0, 0, 255, a]);

    let x = position.x.u8();
    let y = position.y.u8();

    let x_start = (x as u32) * scale_factor + 1;
    let y_start = (y as u32) * scale_factor + 1;
    let x_end = x_start + scale_factor;
    let y_end = y_start + scale_factor;

    for draw_x in x_start..x_end {
      for draw_y in y_start..y_end {
        alpha_overlay.put_pixel(draw_x, draw_y, rgba);
      }
    }
  }

  alpha_overlay
}

/// Draws a [Terrain] tile at a specific cell location
pub fn draw_terrain_tile_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Terrain) {
  draw_terrain_tile_xy_with_scale_factor(imgbuf, col, row, tile, DEFAULT_SCALE_FACTOR)
}

/// Draws a [Terrain] tile at a specific cell location with a user-supplied scaling factor
pub fn draw_terrain_tile_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Terrain, scale_factor: u32) {
  let tile_img_reader_result = match tile {
    Terrain::Plain => Reader::new(Cursor::new(assets_data::TERRAIN_PLAIN_IMG_DATA)).with_guessed_format(),
    Terrain::Swamp => Reader::new(Cursor::new(assets_data::TERRAIN_SWAMP_IMG_DATA)).with_guessed_format(),
    Terrain::Wall  => Reader::new(Cursor::new(assets_data::TERRAIN_WALL_IMG_DATA)).with_guessed_format(),
  };

  draw_tile_img_xy(imgbuf, col, row, tile_img_reader_result, scale_factor);
}

/// Draws a [Resource] tile at a specific cell location
pub fn draw_resource_tile_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Resource) {
  draw_resource_tile_xy_with_scale_factor(imgbuf, col, row, tile, DEFAULT_SCALE_FACTOR)
}

/// Draws a [Resource] tile at a specific cell location with a user-supplied scaling factor
pub fn draw_resource_tile_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Resource, scale_factor: u32) {
  let tile_img_reader_result = match tile {
    Resource::Source    => Reader::new(Cursor::new(assets_data::RESOURCE_SOURCE_IMG_DATA)).with_guessed_format(),
    Resource::Hydrogen  => Reader::new(Cursor::new(assets_data::RESOURCE_HYDROGEN_IMG_DATA)).with_guessed_format(),
    Resource::Oxygen    => Reader::new(Cursor::new(assets_data::RESOURCE_OXYGEN_IMG_DATA)).with_guessed_format(),
    Resource::Keanium   => Reader::new(Cursor::new(assets_data::RESOURCE_KEANIUM_IMG_DATA)).with_guessed_format(),
    Resource::Lemergium => Reader::new(Cursor::new(assets_data::RESOURCE_LEMERGIUM_IMG_DATA)).with_guessed_format(),
    Resource::Utrium    => Reader::new(Cursor::new(assets_data::RESOURCE_UTRIUM_IMG_DATA)).with_guessed_format(),
    Resource::Zynthium  => Reader::new(Cursor::new(assets_data::RESOURCE_ZYNTHIUM_IMG_DATA)).with_guessed_format(),
    Resource::Catalyst  => Reader::new(Cursor::new(assets_data::RESOURCE_CATALYST_IMG_DATA)).with_guessed_format(),
    Resource::Unknown   => Reader::new(Cursor::new(assets_data::RESOURCE_UNKNOWN_IMG_DATA)).with_guessed_format(),
  };

  draw_tile_img_xy(imgbuf, col, row, tile_img_reader_result, scale_factor);
}

/// Draws a [BuildableStructure] tile at a specific cell location
pub fn draw_buildablestructure_tile_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &BuildableStructure) {
  draw_buildablestructure_tile_xy_with_scale_factor(imgbuf, col, row, tile, DEFAULT_SCALE_FACTOR)
}

/// Draws a [BuildableStructure] tile at a specific cell location with a user-supplied scaling factor
pub fn draw_buildablestructure_tile_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &BuildableStructure, scale_factor: u32) {
  let tile_img_reader_result = match tile {
    BuildableStructure::ConstructedWall => Reader::new(Cursor::new(assets_data::STRUCTURE_CONSTRUCTEDWALL_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Container       => Reader::new(Cursor::new(assets_data::STRUCTURE_CONTAINER_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Controller      => Reader::new(Cursor::new(assets_data::STRUCTURE_CONTROLLER_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Extension       => Reader::new(Cursor::new(assets_data::STRUCTURE_EXTENSION_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Extractor       => Reader::new(Cursor::new(assets_data::STRUCTURE_EXTRACTOR_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Factory         => Reader::new(Cursor::new(assets_data::STRUCTURE_FACTORY_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Lab             => Reader::new(Cursor::new(assets_data::STRUCTURE_LAB_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Link            => Reader::new(Cursor::new(assets_data::STRUCTURE_LINK_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Nuker           => Reader::new(Cursor::new(assets_data::STRUCTURE_NUKER_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Observer        => Reader::new(Cursor::new(assets_data::STRUCTURE_OBSERVER_IMG_DATA)).with_guessed_format(),
    BuildableStructure::PowerSpawn      => Reader::new(Cursor::new(assets_data::STRUCTURE_POWERSPAWN_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Rampart         => Reader::new(Cursor::new(assets_data::STRUCTURE_RAMPART_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Road            => Reader::new(Cursor::new(assets_data::STRUCTURE_ROAD_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Spawn           => Reader::new(Cursor::new(assets_data::STRUCTURE_SPAWN_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Storage         => Reader::new(Cursor::new(assets_data::STRUCTURE_STORAGE_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Terminal        => Reader::new(Cursor::new(assets_data::STRUCTURE_TERMINAL_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Tower           => Reader::new(Cursor::new(assets_data::STRUCTURE_TOWER_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Unknown         => Reader::new(Cursor::new(assets_data::STRUCTURE_UNKNOWN_IMG_DATA)).with_guessed_format(),
  };

  draw_tile_img_xy(imgbuf, col, row, tile_img_reader_result, scale_factor);
}

/// Underlying helper function to draw a tile image at a specific cell location
fn draw_tile_img_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile_img_reader_result: Result<Reader<Cursor<&[u8]>>, std::io::Error>, scale_factor: u32) {

  if let Ok(tile_img_reader) = tile_img_reader_result {
    let tile_img_result = tile_img_reader.decode();
    if let Ok(tile_img_dynamic) = tile_img_result {
      let mut tile_img = tile_img_dynamic.to_rgba8();

      let new_width = scale_factor;
      let new_height = scale_factor;
      if (new_width != tile_img.width()) | (new_height != tile_img.height())  {
        tile_img = image::imageops::resize(&tile_img, new_width, new_height, image::imageops::FilterType::Nearest);
      }

      let x = (col * scale_factor + 1).try_into().unwrap();
      let y = (row * scale_factor + 1).try_into().unwrap();

      image::imageops::overlay(imgbuf, &tile_img, x, y);
    }
  };
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
  return (1.0 - t) * v0 + t * v1;
}
