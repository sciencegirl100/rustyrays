#[macro_use]

extern crate bmp;
extern crate nalgebra;
use bmp::Image;
use bmp::Pixel;

struct Camera {
  x: f64,
  y: f64,
  z: f64,
  pitch: f64,
  yaw: f64,
  plane: bmp::Image
}

struct OrthoCamera {
  x: f64,
  y: f64,
  z: f64,
  plane: bmp::Image
}

impl Camera {
  fn new(x: f64, y: f64, z: f64, pitch: f64, yaw: f64) -> Camera {
    Camera {
      x: x,
      y: y,
      z: z,
      pitch: pitch,
      yaw: yaw,
      plane: Image::new(256,256)
    }
  }
}

impl OrthoCamera {
  fn new(x: f64, y: f64, z: f64) -> OrthoCamera {
    OrthoCamera {
      x: x,
      y: y,
      z: z,
      plane: Image::new(256,256)
    }
  }
}

struct Sphere {
  x: f64,
  y: f64,
  z: f64,
  radius: f64
}

fn main() {
  let mut camera = OrthoCamera::new(0.0, 0.0, 0.0);
  let mut spheres = Vec::new();
  spheres.push(Sphere {
    x: 0.0,
    y: 0.0,
    z: 100.0,
    radius: 5.0
  });

  for (x, y) in camera.plane.coordinates() {
    camera.plane.set_pixel(x, y, px!(x, y, 200));
  }

  let _ = camera.plane.save("img.bmp");
//  let mut img = Image::new(256,256);
//
//  for (x, y) in img.coordinates() {
//    img.set_pixel(x, y, px!(x, y, 20));
//  }
//
//  let _ = img.save("img.bmp");
}
