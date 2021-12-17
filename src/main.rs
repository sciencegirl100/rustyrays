use std::mem;

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
  pos: nalgebra::Pnt3<f64>,
  radius: f64
}

impl Sphere {
  fn new(pos: nalgebra::Pnt3<f64>, radius: f64) -> Sphere {
    Sphere {
      pos: pos,
      radius: radius
    }
  }

  fn intersection(primary_ray: Ray) -> Vec3<f64> {
    let t0: f64;
    let t1: f64;

    let center = Vec3::new(pos);

    return center;
  }

  fn solve_quadratic(a: f64, b: f64, c: f64, x0: &mut f64, x1: &mut f64) -> bool {
    let discr: f64 = b * b - 4.0 * a * c;
    if discr < 0.0 {
      return false;
    } else if discr == 0.0 {
      *x0 = - 0.5 * b / a;
      *x1 = - 0.5 * b / a;
    } else {
      let q: f64 = if b > 0.0 { -0.5 * (b + f64::sqrt(discr)) } else { -0.5 * (b - f64::sqrt(discr)) };
      *x0 = q / a;
      *x1 = c / q;
    }
    if x0 > x1 { mem::swap(x0, x1); }
    return true;
  }
}

fn main() {
  let mut camera = OrthoCamera::new(Pnt3::new(0.0, 0.0, 0.0));
  let mut spheres = Vec::new();

  spheres.push(Sphere::new(Pnt3::new(0.0, 0.0, 20.0), 5.0));

  for (x, y) in camera.plane.coordinates() {
    camera.plane.set_pixel(x, y, px!(x, y, 200));
  }

  let _ = camera.plane.save("img.bmp");


  // Testing rays
  let ray = Ray::new(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.25, 0.8));

  let result = ray.at(5.0);

  println!("Result: {}", result);
}
