use std::mem;

#[macro_use]
extern crate bmp;
extern crate nalgebra;

use nalgebra::*;
use bmp::Image;
use bmp::Pixel;

struct Ray {
  pos: Vec3<f64>,
  dir: Vec3<f64>
}

impl Ray {
  fn new(pos: Vec3<f64>, dir: Vec3<f64>) -> Ray {
    Ray {
      pos: pos,
      dir: dir
    }
  }

  fn at(&self, t: f64) -> Vec3<f64> {
    self.pos + t * self.dir
  }
}

struct OrthoCamera {
  pos: Vec3<f64>,
  plane: bmp::Image
}

impl OrthoCamera {
  fn new(pos: Vec3<f64>) -> OrthoCamera {
    OrthoCamera {
      pos: pos,
      plane: Image::new(256,256)
    }
  }
}

struct Sphere {
  pos: Vec3<f64>,
  radius: f64
}

impl Sphere {
  fn new(pos: Vec3<f64>, radius: f64) -> Sphere {
    Sphere {
      pos: pos,
      radius: radius
    }
  }

  // Implemented from
  // http://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
  fn intersection(&self, primary_ray: Ray, t1: &mut f64, t2: &mut f64) -> bool {
    let l = self.pos - primary_ray.pos;
    let tc = dot(&l, &primary_ray.dir);

    if tc < 0.0 { 
      return false;
    }

    let d = f64::sqrt((l.norm() * l.norm()) as f64 - (tc*tc));

    if d > self.radius { return false };

    let t1c = f64::sqrt(f64::powf(self.radius, 2.0) - f64::powf(d, 2.0));
    *t1 = tc - t1c;
    *t2 = tc + t1c;

    return true;
  }
}

fn main() {
  let mut camera = OrthoCamera::new(Vec3::new(0.0, 0.0, 0.0));
  let mut spheres = Vec::new();

  spheres.push(Sphere::new(Vec3::new(100.0, 100.0, 50.0), 25.0));

  for (x, y) in camera.plane.coordinates() {
    println!("Camera coords: {}. {}", x, y);
    for sphere in &spheres {
      let ray = Ray::new(Vec3::new(x as f64, y as f64, camera.pos.z as f64), Vec3::new(0.0, 0.0, 1.0));
      let mut t1 = 0.0;
      let mut t2 = 0.0;
      let result = sphere.intersection(ray, &mut t1, &mut t2);
      if result == true {
        camera.plane.set_pixel(x, y, px!(255, 255, 255));
      } else {
        camera.plane.set_pixel(x, y, px!(0, 0, 0));
      }
    }
  }

  let _ = camera.plane.save("img.bmp");


  // Testing rays
  let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.8));

  let result = ray.at(5.0);

  println!("Result: {}", result);
}
