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
  let light = LightSrc::new(Vec3::new(125.0, 0.0, 100.0), 500.0);

  spheres.push(Sphere::new(Vec3::new(125.0, 75.0, 100.0), 20.0));
  spheres.push(Sphere::new(Vec3::new(125.0, 175.0, 100.0), 60.0));
//  spheres.push(Sphere::new(Vec3::new(0.0, 0.0, 100.0), 10.0));
//  for i in 0..15 {
//    let mut rng = rand::thread_rng();
//    let x: f64 = rng.gen::<f64>() * 250.0;
//    let y: f64 = rng.gen::<f64>() * 250.0;
//    let z: f64 = rng.gen::<f64>() * 250.0;
//    let radius: f64 = rng.gen::<f64>() * 40.0;
//    spheres.push(Sphere::new(Vec3::new(x, y, 100.0), radius));
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

          let shadow_ray = Ray {
            pos: hit + (normal.normalize() * 1e-03),
            dir: -light_vec.normalize()
          };

          let mut in_light = false;
          for shadow_sphere in &spheres {
            if shadow_sphere.intersection(&shadow_ray).is_none() {
              in_light = true;
            }

            if in_light {
              println!("in light");
            }

            let light_intensity = if in_light { light.intensity } else { 0.0 };
            let light_power = (normal.normalize().dot(&light_vec.normalize()) as f64).max(0.0) * light_intensity;
            let light_reflected = 2.0 / std::f64::consts::PI;
            let light_calc = light_power * light_reflected;
            camera.plane.set_pixel(x, y, px!(light_calc, light_calc, light_calc));

            if !in_light {
              //camera.plane.set_pixel(x, y, px!(255, 0, 0));
            }
          }

        }
        None => { },
      }
    }
  }

  let _ = camera.plane.save("img.bmp");
}
