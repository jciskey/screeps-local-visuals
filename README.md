# Screeps Local Visuals

This library provides helper methods for creating room images for the programmable MMO [Screeps](https://screeps.com/), without relying on the in-game renderer.


## Example Usage

The following example code produces an image `simple.png` that represents an empty room with an Extension, a Source, a Catalyst Mineral, a Swamp, and 3 Walls.

```
use screeps_local_visuals::render;
use render::{ Terrain, Resource, BuildableStructure, OutputImage, DEFAULT_ROOM_MAX_ROWS, DEFAULT_ROOM_MAX_COLUMNS };


fn main()   {
  make_map_image("simple.png");
}


fn make_map_image(filename: &str) {

  let mut imgbuf = render::create_image();

  draw_terrain(&mut imgbuf);

  draw_test_terrain(&mut imgbuf);
  draw_test_resource(&mut imgbuf);
  draw_test_structure(&mut imgbuf);

  render::draw_grid(&mut imgbuf);

  imgbuf.save(filename).unwrap();
}

fn draw_terrain(imgbuf: &mut OutputImage) {
  for col in 0..DEFAULT_ROOM_MAX_COLUMNS {
    for row in 0..DEFAULT_ROOM_MAX_ROWS {
      render::draw_terrain_tile_xy(imgbuf, col, row, &Terrain::Plain);
    }
  }
}

fn draw_test_terrain(imgbuf: &mut OutputImage)  {
  render::draw_terrain_tile_xy(imgbuf, 20, 20, &Terrain::Swamp);
  render::draw_terrain_tile_xy(imgbuf, 21, 20, &Terrain::Wall);
  render::draw_terrain_tile_xy(imgbuf, 21, 21, &Terrain::Wall);
  render::draw_terrain_tile_xy(imgbuf, 20, 21, &Terrain::Wall);
}

fn draw_test_resource(imgbuf: &mut OutputImage)  {
  render::draw_resource_tile_xy(imgbuf, 10, 20, &Resource::Source);
  render::draw_resource_tile_xy(imgbuf, 20, 10, &Resource::Catalyst);
}

fn draw_test_structure(imgbuf: &mut OutputImage)  {
  render::draw_buildablestructure_tile_xy(imgbuf, 10, 10, &BuildableStructure::Extension);
}
```