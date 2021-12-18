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

struct LightSrc {
  pos: Vec3<f64>,
  intensity: f64
}

impl LightSrc {
  fn new(pos: Vec3<f64>, intensity: f64) -> LightSrc {
    LightSrc {
      pos: pos,
      intensity: intensity
    }
  }
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
  fn intersection(&self, ray: &Ray) -> Option<f64> {
    let l = self.pos - ray.pos;
    let adj = l.dot(&ray.dir);
    let d2 = l.dot(&l) - (adj * adj);
    let radius2 = self.radius * self.radius;

    if d2 > radius2 {
      return None;
    }

    let thc = (radius2 - d2).sqrt();
    let t0 = adj - thc;
    let t1 = adj + thc;

    if t0 < 0.0 && t1 < 0.0 {
      return None;
    }

    let distance = if t0 < t1 { t0 } else { t1 };
    Some(distance)
  }
}

fn main() {
  let mut camera = OrthoCamera::new(Vec3::new(0.0, 0.0, 0.0));
  let mut spheres = Vec::new();
  let light = LightSrc::new(Vec3::new(120.0, 0.0, 100.0), 500.0);

  spheres.push(Sphere::new(Vec3::new(125.0, 50.0, 100.0), 20.0));
  spheres.push(Sphere::new(Vec3::new(150.0, 150.0, 100.0), 40.0));
//  for i in 0..15 {
//    let mut rng = rand::thread_rng();
//    let x: f64 = rng.gen::<f64>() * 250.0;
//    let y: f64 = rng.gen::<f64>() * 250.0;
//    let z: f64 = rng.gen::<f64>() * 250.0;
//    let radius: f64 = rng.gen::<f64>() * 40.0;
//    spheres.push(Sphere::new(Vec3::new(x, y, 1000.0), radius));
//  }

  for (x, y) in camera.plane.coordinates() {
    camera.plane.set_pixel(x, y, px!(0, y, 0));
  }

  for (x, y) in camera.plane.coordinates() {
    for sphere in &spheres {
      let ray = Ray::new(Vec3::new(x as f64, y as f64, camera.pos.z as f64), Vec3::new(0.0, 0.0, 1.0));
      let result = sphere.intersection(&ray);


      match result {
        Some(distance) => {
          let hit = ray.at(distance);
          let normal = sphere.pos - hit;
          let light_vec = hit - light.pos;
          let angle = f64::abs(f64::acos(hit.dot(&normal)/(hit.norm()*normal.norm())));

          let light_power = (normal.normalize().dot(&light_vec.normalize()) as f64).max(0.0) * light.intensity;
          let light_reflected = 2.0 / std::f64::consts::PI;
          let light_calc = light_power * light_reflected;
          println!("{}", light_calc);
          camera.plane.set_pixel(x, y, px!(light_calc, light_calc, light_calc));
        }
        None => { },
      }
    }
  }

  let _ = camera.plane.save("img.bmp");
}
