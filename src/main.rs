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

struct OrthoCamera {
  pos: Vec3<f64>,
  plane: bmp::Image,
  spheres: Vec<Sphere>,
  light: LightSrc
}

impl OrthoCamera {
  fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self.spheres.iter()
      .filter_map(|s| s.intersection(ray).map(|d| Intersection::new(d, s) ))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
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

struct Intersection<'a> {
  distance: f64,
  object: &'a Sphere
}

impl<'a> Intersection<'a> {
  fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
    Intersection {
      distance: distance,
      object: & object
    }
  }
}


fn get_color(camera: &OrthoCamera, ray: &Ray, intersection: &Intersection) -> f64 {
  let hit_point = ray.at(intersection.distance);
  let normal = intersection.object.pos - hit_point;
  let light_vec = hit_point - camera.light.pos;

  let light_intensity = camera.light.intensity;
  let light_power = (normal.normalize().dot(&light_vec.normalize()) as f64).max(0.0) * light_intensity;
  let light_reflected = 2.0 / std::f64::consts::PI;
  return light_power * light_reflected;
}

fn main() {
  let mut camera = OrthoCamera {
    pos: Vec3::new(0.0, 0.0, 0.0),
    plane: Image::new(256,256),
    spheres: Vec::new(),
    light: LightSrc::new(Vec3::new(125.0, -100.0, 100.0), 20.0)
  };

  camera.spheres.push(Sphere::new(Vec3::new(125.0, 75.0, 100.0), 20.0));
  camera.spheres.push(Sphere::new(Vec3::new(115.0, 175.0, 100.0), 60.0));
  camera.spheres.push(Sphere::new(Vec3::new(0.0, 0.0, 100.0), 10.0));
  for i in 0..15 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen::<f64>() * 250.0;
    let y: f64 = rng.gen::<f64>() * 250.0;
    let z: f64 = rng.gen::<f64>() * 250.0;
    let radius: f64 = rng.gen::<f64>() * 40.0;
    camera.spheres.push(Sphere::new(Vec3::new(x, y, 100.0), radius));
  }

  for (x, y) in camera.plane.coordinates() {
    camera.plane.set_pixel(x, y, px!(0, y, 0));
    let ray = Ray::new(Vec3::new(x as f64, y as f64, camera.pos.z as f64), Vec3::new(0.0, 0.0, 1.0));
    let result = camera.trace(&ray);
    match result {
      Some(intersection) => {
        let hit_point = ray.at(intersection.distance);
        let normal = hit_point - intersection.object.pos;
        let light_dir = hit_point - camera.light.pos;
        let light_color = get_color(&camera, &ray, &intersection);
        let shadow_ray = Ray {
          pos: hit_point + (normal.normalize()),
          dir: -light_dir.normalize()
        };

        println!("{} {}", shadow_ray.pos, shadow_ray.dir);

        let in_light = camera.trace(&shadow_ray).is_none();
        let light_intensity = if in_light { camera.light.intensity } else { 0.0 };
        if in_light {
          println!("in light");
        } else {
          println!("in shadow");
        }
        camera.plane.set_pixel(x, y, px!(light_color * light_intensity, 0, 0))
      },
      None => { }
    }

  }

//  for (x, y) in camera.plane.coordinates() {
//    for sphere in &spheres {
//      let ray = Ray::new(Vec3::new(x as f64, y as f64, camera.pos.z as f64), Vec3::new(0.0, 0.0, 1.0));
//      let result = sphere.intersection(&ray);
//
//
//      match result {
//        Some(distance) => {
//          let hit = ray.at(distance);
//          let normal = sphere.pos - hit;
//          let light_vec = hit - light.pos;
//
//          let shadow_ray = Ray {
//            pos: hit + (normal.normalize() * 1e-03),
//            dir: -light_vec.normalize()
//          };
//
//          let mut in_light = false;
//          let mut light_calc = 0.0;
//          for shadow_sphere in &spheres {
//            if shadow_sphere.intersection(&shadow_ray).is_none() {
//              in_light = true;
//            }
//
//            let light_intensity = if in_light { light.intensity } else { 0.0 };
//            let light_power = (normal.normalize().dot(&light_vec.normalize()) as f64).max(0.0) * light_intensity;
//            let light_reflected = 2.0 / std::f64::consts::PI;
//            light_calc = light_power * light_reflected;
//
//          }
//          camera.plane.set_pixel(x, y, px!(light_calc, light_calc, light_calc));
//
//        }
//        None => { },
//      }
//    }
//  }

  let _ = camera.plane.save("img.bmp");
}
