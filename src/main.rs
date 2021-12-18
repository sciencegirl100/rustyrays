use std::mem;

#[macro_use]
extern crate bmp;
extern crate rand;
extern crate nalgebra;

use rand::Rng;
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
  fn intersection(&self, primary_ray: &Ray, t1: &mut f64, t2: &mut f64) -> bool {
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

  for i in 0..15 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen::<f64>() * 250.0;
    let y: f64 = rng.gen::<f64>() * 250.0;
    let z: f64 = rng.gen::<f64>() * 250.0;
    let radius: f64 = rng.gen::<f64>() * 40.0;
    spheres.push(Sphere::new(Vec3::new(x, y, 1000.0), radius));
  }

  for (x, y) in camera.plane.coordinates() {
    camera.plane.set_pixel(x, y, px!(0, 0, 0));
  }

  for (x, y) in camera.plane.coordinates() {
    for sphere in &spheres {
      let ray = Ray::new(Vec3::new(x as f64, y as f64, camera.pos.z as f64), Vec3::new(0.0, 0.0, 1.0));
      let mut t1 = 0.0;
      let mut t2 = 0.0;
      let mut tfront = 0.0;
      let result = sphere.intersection(&ray, &mut t1, &mut t2);

      tfront = if t2 > t1 { t1 } else { t2 };

      let hit = ray.at(tfront);
      let normal = hit - sphere.pos;
      //let angle = abs(&hit1.normalize().dot(&normal.normalize()));
      let angle = f64::abs(f64::acos(hit.dot(&normal)/(hit.norm()*normal.norm())));
      if result == true {
        camera.plane.set_pixel(x, y, px!(80.0 * angle, 80.0 * angle, 80.0 * angle));
      }
    }
  }

  let _ = camera.plane.save("img.bmp");
}
