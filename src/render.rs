
use std::io::Cursor;
use image::io::Reader;

use crate::assets_data;

/// A helpful type alias for the type of images this library operates on
pub type OutputImage = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

/// The default number of columns in a room (x-coordinate)
pub const DEFAULT_ROOM_MAX_COLUMNS: u32 = 50;

/// The default number of rows in a room (y-coordinate)
pub const DEFAULT_ROOM_MAX_ROWS: u32 = 50;

/// The default scaling factor for the final image, meaning the number of pixels allocated for each room cell
pub const DEFAULT_SCALE_FACTOR: u32 = 50;

/// Represents the 3 terrain types: Plains, Swamps, and Walls
#[derive(Debug)]
pub enum Terrain {
  Plain,
  Swamp,
  Wall,
}

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
  Powerspawn,
  Rampart,
  Road,
  Spawn,
  Storage,
  Terminal,
  Tower,
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
  draw_text_number_xy_with_scale_factor(imgbuf, col, row, text, DEFAULT_SCALE_FACTOR)
}

/// Draws a text number on a user-sized image at a specific cell location
pub fn draw_text_number_xy_with_scale_factor(imgbuf: &mut OutputImage, col: u32, row: u32, text: &str, scale_factor: u32) {
  let x = (col * scale_factor + 2).try_into().unwrap();
  let y = (row * scale_factor + 2).try_into().unwrap();
  draw_text_number_raw(imgbuf, x, y, text);
}

/// Underlying function for drawing numbers on an image at a specific cell location
fn draw_text_number_raw(imgbuf: &mut OutputImage, x: i32, y: i32, text: &str) {
  let scale = rusttype::Scale::uniform(15.0);
  let font = rusttype::Font::try_from_bytes(assets_data::FREE_MONO_FONT_DATA).expect("Could not load FreeMono font");
  imageproc::drawing::draw_text_mut(imgbuf, image::Rgba([255,255,255,255]), x, y, scale, &font, text);
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
    BuildableStructure::Powerspawn      => Reader::new(Cursor::new(assets_data::STRUCTURE_POWERSPAWN_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Rampart         => Reader::new(Cursor::new(assets_data::STRUCTURE_RAMPART_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Road            => Reader::new(Cursor::new(assets_data::STRUCTURE_ROAD_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Spawn           => Reader::new(Cursor::new(assets_data::STRUCTURE_SPAWN_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Storage         => Reader::new(Cursor::new(assets_data::STRUCTURE_STORAGE_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Terminal        => Reader::new(Cursor::new(assets_data::STRUCTURE_TERMINAL_IMG_DATA)).with_guessed_format(),
    BuildableStructure::Tower           => Reader::new(Cursor::new(assets_data::STRUCTURE_TOWER_IMG_DATA)).with_guessed_format(),
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
