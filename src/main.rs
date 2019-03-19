#[macro_use]
extern crate itertools;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Pixel {
    colour: u8,
}

impl Pixel {
    pub fn white() -> Pixel {
        Pixel { colour: 255 }
    }

    pub fn black() -> Pixel {
        Pixel { colour: 0 }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colour)
    }
}

struct Canvas {
    data: Vec<Vec<Pixel>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut data = Vec::with_capacity(height);
        for _i in 0..height {
            let mut row = Vec::with_capacity(width);
            for _j in 0..width {
                row.push(Pixel::white());
            }
            data.push(row);
        }

        Canvas {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.data.get(x).and_then(|ys| ys.get(y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.data.get_mut(x).and_then(|ys| ys.get_mut(y))
    }

    pub fn ink(&mut self, x: usize, y: usize, intensity: u8) -> bool {
        if let Some(pixel) = self.get_mut(x, y) {
            pixel.colour = intensity;
            true
        } else {
            false
        }
    }
}

fn render_ppm(path: &Path, canvas: Canvas) {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(e) => panic!("Could not create {}: {}", display, e.description()),
        Ok(file) => file,
    };

    write!(file, "P2\n{} {}\n255\n", canvas.width, canvas.height).unwrap();
    for (x, y) in iproduct!(0..canvas.height, 0..canvas.width) {
        write!(file, "{} ", canvas.get(x, y).unwrap()).unwrap();
    }
}

#[derive(Debug)]
struct Sphere {
    position: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector, radius: f64) -> Sphere {
        Sphere { position, radius }
    }

    pub fn collides_with(&self, ray: &Ray) -> Vec<Vector> {
        let mut l = ray.direction.clone();
        l.normalise();

        let c = &self.position;
        let r = &self.radius;
        let mut o = ray.origin.clone();

        let o_minus_c = o.minus(&c);

        let indicator = l.dot(o_minus_c).powi(2) - (o_minus_c.magnitude().powi(2) - r.powi(2));

        if indicator == 0.0 {
            let d = -l.dot(o_minus_c);
            vec![ray.shine_to(d)]
        } else if indicator > 0.0 {
            let d1 = (-l.dot(o_minus_c)) + indicator.sqrt();
            let d2 = (-l.dot(o_minus_c)) - indicator.sqrt();

            if 0.0 <= d1 && 0.0 <= d2 {
                vec![ray.shine_to(d1.min(d2)), ray.shine_to(d1.max(d2))]
            } else if d1 >= 0.0 {
                vec![ray.shine_to(d1)]
            } else if d2 >= 0.0 {
                vec![ray.shine_to(d2)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    n: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, n: 1.0 }
    }

    pub fn new_with_length(x: f64, y: f64, z: f64, n: f64) -> Vector {
        Vector { x, y, z, n }
    }

    pub fn cross(&mut self, other: &Vector) -> &mut Self {
        let new_x = (self.y * other.z) - (self.z * other.y);
        let new_y = (self.z * other.x) - (self.x * other.z);
        let new_z = (self.x * other.y) - (self.y * other.x);

        self.x = new_x;
        self.y = new_y;
        self.z = new_z;

        self.n = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn add(&mut self, other: &Vector) -> &mut Self {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;

        self.n = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self
    }

    pub fn minus(&mut self, other: &Vector) -> &mut Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;

        self.n = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self
    }

    pub fn mult(&mut self, magnitude: f64) -> &mut Self {
        self.x *= magnitude;
        self.y *= magnitude;
        self.z *= magnitude;

        self.n *= magnitude;
        self.n = self.n.abs();

        self
    }

    pub fn normalise(&mut self) -> &mut Self {
        self.x /= self.n;
        self.y /= self.n;
        self.z /= self.n;
        self.n = 1.0;

        self
    }

    pub fn magnitude(&self) -> f64 {
        self.n
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;

        self.n = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self
    }

    pub fn set_as(&mut self, other: &Vector) -> &mut Self {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
        self.n = other.n;

        self
    }
}

struct Camera {
    plane_z: Vector,
    plane_x: Vector,
    pos: (f64, f64),
    width: f64,
    height: f64,
    pixels_width: usize,
    pixels_height: usize,
}

#[derive(Debug)]
struct Ray {
    direction: Vector,
    origin: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn shine_to(&self, distance: f64) -> Vector {
        let mut result = self.origin.clone();
        let mut cast = self.direction.clone();
        cast.mult(distance);
        result.add(&cast);

        result
    }
}

const EPSILON: f64 = 0.000000001;

impl Camera {
    pub fn new(width: f64, height: f64, pixels_width: usize, pixels_height: usize) -> Camera {
        let default_z = Vector::new(0.0, 0.0, 1.0);
        let default_x = Vector::new(1.0, 0.0, 0.0);
        Camera {
            plane_z: default_z,
            plane_x: default_x,
            pos: (0.0, 0.0),
            width,
            height,
            pixels_width,
            pixels_height,
        }
    }

    pub fn with_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.pos = (x, y);
        self
    }

    pub fn with_plane_z(&mut self, vec: Vector) -> &mut Self {
        self.plane_z = vec;
        self
    }

    pub fn with_plane_x(&mut self, vec: Vector) -> &mut Self {
        self.plane_x = vec;
        self
    }

    fn cast_ray(ray: &Ray, light_distance: f64, objects: &Vec<Sphere>) -> u8 {
        for object in objects.iter() {
            let collisions = object.collides_with(&ray);

            for mut collision in collisions {
                let collision_distance = collision.minus(&ray.origin).magnitude();
                if collision_distance > EPSILON && collision_distance < light_distance {
                    return 0;
                }
            }
        }

        255 - (25 * (light_distance as u8)) // TODO make something better
    }

    pub fn raytrace(&self, objects: &Vec<Sphere>, lights: &Vec<Light>) -> Canvas {
        let mut canvas = Canvas::new(self.pixels_width, self.pixels_height);

        let mut plane_y = self.plane_z.clone();
        plane_y.cross(&self.plane_x);

        let width_step = self.width / (self.pixels_width as f64);
        let height_step = self.height / (self.pixels_height as f64);

        let mut ray = Ray::new(Vector::new(0.0, 0.0, 0.0), self.plane_z.clone());
        let mut offset_x = self.plane_x.clone();
        let mut offset_y = plane_y.clone();

        for (x, y) in iproduct!(0..self.pixels_width, 0..self.pixels_height) {
            ray.origin.add(offset_x.mult((x as f64) * width_step));
            ray.origin.add(offset_y.mult((y as f64) * height_step));

            for object in objects.iter() {
                if let Some(collision) = object.collides_with(&ray).first() {
                    let mut intensity = 0;

                    for light in lights.iter() {
                        let mut dir = light.position.clone();
                        let origin = collision.clone();
                        dir.minus(&collision);
                        let light_distance = dir.magnitude();
                        dir.normalise();

                        let ray = Ray::new(origin, dir);

                        intensity = Camera::cast_ray(&ray, light_distance, &objects);
                    }

                    canvas.ink(x, y, intensity);
                    break;
                }
            }

            ray.origin.set(0.0, 0.0, 0.0);
            offset_x.set_as(&self.plane_x);
            offset_y.set_as(&plane_y);
        }

        canvas
    }
}

struct Light {
    position: Vector,
    intensity: f64,
}

impl Light {
    pub fn new(position: Vector, intensity: f64) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            objects: vec![],
            lights: vec![],
            camera,
        }
    }

    pub fn add_object(&mut self, o: Sphere) -> &mut Self {
        self.objects.push(o);
        self
    }

    pub fn add_light(&mut self, l: Light) -> &mut Self {
        self.lights.push(l);
        self
    }

    pub fn raytrace(&self) -> Canvas {
        self.camera.raytrace(&self.objects, &self.lights)
    }
}

fn main() {
    let sphere1 = Sphere::new(Vector::new(5.0, 5.0, 5.0), 2.0);
    let sphere2 = Sphere::new(Vector::new(7.0, 7.0, 5.0), 0.2);
    let sphere3 = Sphere::new(Vector::new(3.0, 3.0, 5.0), 0.5);

    let light = Light::new(Vector::new(5.0, 9.0, 1.0), 1.0);

    let camera = Camera::new(10.0, 10.0, 1024, 1024);

    let mut scene = Scene::new(camera);
    scene
        .add_object(sphere1)
        .add_object(sphere2)
        .add_object(sphere3)
        .add_light(light);

    let canvas = scene.raytrace();

    render_ppm(Path::new("/tmp/out.pbm"), canvas);
}
