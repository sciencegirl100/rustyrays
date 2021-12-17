#[macro_use]

extern crate bmp;
extern crate nalgebra;
use nalgebra::Pnt3;
use nalgebra::Vec3;
use bmp::Image;
use bmp::Pixel;

struct Ray {
  pos: nalgebra::Pnt3<f64>,
  dir: nalgebra::Vec3<f64>
}

impl Ray {
  fn new(pos: nalgebra::Pnt3<f64>, dir: nalgebra::Vec3<f64>) -> Ray {
    Ray {
      pos: pos,
      dir: dir
    }
  }

  fn at(&self, t: f64) -> Pnt3<f64> {
    self.pos + t * self.dir
  }
}

struct OrthoCamera {
  pos: nalgebra::Pnt3<f64>,
  plane: bmp::Image
}

impl OrthoCamera {
  fn new(pos: nalgebra::Pnt3<f64>) -> OrthoCamera {
    OrthoCamera {
      pos: pos,
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
  let mut camera = OrthoCamera::new(Pnt3::new(0.0, 0.0, 0.0));
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


  // Testing rays
  let ray = Ray::new(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.8));

  let result = ray.at(5.0);

  println!("Result: {}", result);
}
