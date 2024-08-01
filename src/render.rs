use crate::assets_data;

pub use screeps::constants::Terrain;

use screeps::constants::{
  ResourceType,
  structure::StructureType
};

use screeps::local::LocalCostMatrix;
use screeps::objects::Source;
use screeps::constants::ROOM_SIZE;

use screeps_utils::offline_map::OfflineObject;

/// A helpful type alias for the type of images this library operates on
///
/// This is the same as RgbaImage.
pub type OutputImage = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

/// The default number of columns in a room (x-coordinate)
pub const DEFAULT_ROOM_MAX_COLUMNS: u32 = ROOM_SIZE as u32;

/// The default number of rows in a room (y-coordinate)
pub const DEFAULT_ROOM_MAX_ROWS: u32 = ROOM_SIZE as u32;

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

// TODO: I'm pretty sure you only need to implement for the reference.
// once compiling, try taking it out and seeing what happens.
// 
// Also, like none of these fail. What are they being used with?
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

impl TryFrom<&StructureType> for BuildableStructure {
    type Error = ();

    #[inline]
    fn try_from(structure_type: &StructureType) -> Result<BuildableStructure, Self::Error> {
        (*structure_type).try_into()
    }
}

impl TryFrom<&OfflineObject> for BuildableStructure {
  type Error = ();

  #[inline]
    fn try_from(obj: &OfflineObject) -> Result<BuildableStructure, Self::Error> {
        Ok(match obj {
            OfflineObject::ConstructedWall { .. } => BuildableStructure::ConstructedWall,
            OfflineObject::Controller { .. } => BuildableStructure::Controller,
            OfflineObject::Extractor { .. } => BuildableStructure::Extractor,
            OfflineObject::Terminal { .. } => BuildableStructure::Terminal,
            _                          => BuildableStructure::Unknown,
        })
    }
}

impl TryFrom<OfflineObject> for BuildableStructure {
  type Error = ();

  #[inline]
    fn try_from(obj: OfflineObject) -> Result<BuildableStructure, Self::Error> {
        (&obj).try_into()
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

/// Calculates a scale that fits the given text within the given
/// square area.
///
/// A new scale will only be calculated if it needs to be smaller to
/// fit within the cell. (Treating the given cell size as the scale
/// factor.)
///
/// Also returns the width and height of the text with the new scale.
fn calculate_centered_text_scale(font: &rusttype::Font, area: u32, text: &str) -> (rusttype::Scale, u32, u32) {
  let default_scale = rusttype::Scale::uniform(area as f32);
  let (x,y) = imageproc::drawing::text_size(default_scale, font, text);
  if x > area as i32 {
    let ratio = (area as f32) / (x as f32);
    let new_scale_factor = area as f32 * ratio;
    let new_scale = rusttype::Scale::uniform(new_scale_factor);
    let (x,y) = imageproc::drawing::text_size(new_scale, font, text);
    (new_scale, x as u32, y as u32)
  } else {
    (default_scale, x as u32, y as u32)
  }
}

/// Draws a centered text number on a default-sized image at a specific cell location.
///
/// Will scale the text down to fit.
pub fn draw_centered_text_number_xy(imgbuf: &mut OutputImage, col: u32, row: u32, text: &str) {
  let font = &assets_data::FREE_MONO_FONT;
  let cell_size = DEFAULT_SCALE_FACTOR; // cell_size in pixels.
  // we want some borders between text, so we need to define an area
  // we'll draw the text within.
  let border_size = 2;
  let text_area = cell_size - 2*border_size;
  let (scale, width, height) = calculate_centered_text_scale(font, text_area, text);
  let x_offset = (cell_size - width)/2;
  let y_offset = (cell_size - height)/2;
  let x = (col * cell_size + border_size + x_offset).try_into().unwrap();
  let y = (row * cell_size + border_size + y_offset).try_into().unwrap();
  imageproc::drawing::draw_text_mut(imgbuf, image::Rgba([255,255,255,255]), x, y, scale, &font, text);
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
  let font = &assets_data::FREE_MONO_FONT;
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
  let tile_img: &OutputImage = match tile {
    Terrain::Plain => &assets_data::TERRAIN_PLAIN_IMG,
    Terrain::Swamp => &assets_data::TERRAIN_SWAMP_IMG,
    Terrain::Wall  => &assets_data::TERRAIN_WALL_IMG,
  };

  draw_tile_img_xy(imgbuf, col, row, tile_img, scale_factor);
}

/// Draws a [Resource] tile at a specific cell location
pub fn draw_resource_tile_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Resource) {
  draw_resource_tile_xy_with_scale_factor(imgbuf, col, row, tile, DEFAULT_SCALE_FACTOR)
}

/// Draws a [Resource] tile at a specific cell location with a user-supplied scaling factor
pub fn draw_resource_tile_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &Resource, scale_factor: u32) {
  let tile_img = match tile {
    Resource::Source    => &*assets_data::RESOURCE_SOURCE_IMG,
    Resource::Hydrogen  => &*assets_data::RESOURCE_HYDROGEN_IMG,
    Resource::Oxygen    => &*assets_data::RESOURCE_OXYGEN_IMG,
    Resource::Keanium   => &*assets_data::RESOURCE_KEANIUM_IMG,
    Resource::Lemergium => &*assets_data::RESOURCE_LEMERGIUM_IMG,
    Resource::Utrium    => &*assets_data::RESOURCE_UTRIUM_IMG,
    Resource::Zynthium  => &*assets_data::RESOURCE_ZYNTHIUM_IMG,
    Resource::Catalyst  => &*assets_data::RESOURCE_CATALYST_IMG,
    Resource::Unknown   => &*assets_data::RESOURCE_UNKNOWN_IMG,
  };

  draw_tile_img_xy(imgbuf, col, row, &tile_img, scale_factor);
}

/// Draws a [BuildableStructure] tile at a specific cell location
pub fn draw_buildablestructure_tile_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &BuildableStructure) {
  draw_buildablestructure_tile_xy_with_scale_factor(imgbuf, col, row, tile, DEFAULT_SCALE_FACTOR)
}

/// Draws a [BuildableStructure] tile at a specific cell location with a user-supplied scaling factor
pub fn draw_buildablestructure_tile_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, tile: &BuildableStructure, scale_factor: u32) {
  use BuildableStructure::*;
  use assets_data::*;

  let tile_image = match tile {
    ConstructedWall => &*STRUCTURE_CONSTRUCTEDWALL_IMG,
    Container       => &*STRUCTURE_CONTAINER_IMG,
    Controller      => &*STRUCTURE_CONTROLLER_IMG,
    Extension       => &*STRUCTURE_EXTENSION_IMG,
    Extractor       => &*STRUCTURE_EXTRACTOR_IMG,
    Factory         => &*STRUCTURE_FACTORY_IMG,
    Lab             => &*STRUCTURE_LAB_IMG,
    Link            => &*STRUCTURE_LINK_IMG,
    Nuker           => &*STRUCTURE_NUKER_IMG,
    Observer        => &*STRUCTURE_OBSERVER_IMG,
    PowerSpawn      => &*STRUCTURE_POWERSPAWN_IMG,
    Rampart         => &*STRUCTURE_RAMPART_IMG,
    Road            => &*STRUCTURE_ROAD_IMG,
    Spawn           => &*STRUCTURE_SPAWN_IMG,
    Storage         => &*STRUCTURE_STORAGE_IMG,
    Terminal        => &*STRUCTURE_TERMINAL_IMG,
    Tower           => &*STRUCTURE_TOWER_IMG,
    Unknown         => &*STRUCTURE_UNKNOWN_IMG,
  };
  draw_tile_img_xy(imgbuf, col, row, tile_image, scale_factor);
}

/// Underlying helper function to draw a tile image at a specific cell location
fn draw_tile_img_xy(imgbuf: &mut OutputImage, col: u32, row: u32, tile_img: &OutputImage, scale_factor: u32) {
  let new_width = scale_factor;
  let new_height = scale_factor;
  let tile_img = if (new_width != tile_img.width()) | (new_height != tile_img.height()) {
    &image::imageops::resize(tile_img, new_width, new_height, image::imageops::FilterType::Nearest)
  } else {
    tile_img
  };

  let x = (col * scale_factor + 1).try_into().unwrap();
  let y = (row * scale_factor + 1).try_into().unwrap();

  image::imageops::overlay(imgbuf, tile_img, x, y);
}

pub fn get_tile_alpha_overlay(overlay_width: u32, overlay_height: u32, scale_factor: u32, r: u8, g: u8, b: u8, a: u8, x: u8, y: u8) -> OutputImage {
  let mut alpha_overlay = image::ImageBuffer::new(overlay_width, overlay_height);

  let rgba = image::Rgba([r, g, b, a]);

  let x_start = (x as u32) * scale_factor + 1;
  let y_start = (y as u32) * scale_factor + 1;
  let x_end = x_start + scale_factor;
  let y_end = y_start + scale_factor;

  for draw_x in x_start..x_end {
    for draw_y in y_start..y_end {
      alpha_overlay.put_pixel(draw_x, draw_y, rgba);
    }
  }

  alpha_overlay
}

pub fn get_tile_alpha_overlay_multi_tile(overlay_width: u32, overlay_height: u32, scale_factor: u32, r: u8, g: u8, b: u8, a: u8, tiles: &[(u8, u8)]) -> OutputImage {
  let mut alpha_overlay = image::ImageBuffer::new(overlay_width, overlay_height);

  let rgba = image::Rgba([r, g, b, a]);

  for (x, y) in tiles {
    let x_start = (*x as u32) * scale_factor + 1;
    let y_start = (*y as u32) * scale_factor + 1;
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

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
  return (1.0 - t) * v0 + t * v1;
}
