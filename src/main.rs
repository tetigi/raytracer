#[macro_use]
extern crate itertools;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

enum Colour {
    BLACK,
    WHITE,
}

struct Pixel {
    colour: Colour,
}

impl Pixel {
    pub fn white() -> Pixel {
        Pixel {
            colour: Colour::WHITE,
        }
    }

    pub fn black() -> Pixel {
        Pixel {
            colour: Colour::BLACK,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.colour {
                Colour::BLACK => 1,
                _ => 0,
            }
        )
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

    pub fn ink(&mut self, x: usize, y: usize) -> bool {
        if let Some(pixel) = self.get_mut(x, y) {
            pixel.colour = Colour::BLACK;
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

    write!(file, "P1\n{} {}\n", canvas.width, canvas.height).unwrap();
    for (x, y) in iproduct!(0..canvas.height, 0..canvas.width) {
        write!(file, "{} ", canvas.get(x, y).unwrap()).unwrap();
    }
}

struct Sphere {
    position: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector, radius: f64) -> Sphere {
        Sphere { position, radius }
    }
}

#[derive(Debug)]
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

    pub fn cross(&self, other: &Vector) -> Vector {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);

        let n = ((x * x) + (y * y) + (z * z)).sqrt();

        Vector::new_with_length(x, y, z, n)
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

impl Camera {
    pub fn new(width: f64, height: f64, pixels_width: usize, pixels_height: usize) -> Camera {
        let default_z = Vector::new(0.0, 0.0, 1.0);
        let default_x = Vector::new(1.0, 0.0, 1.0);
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

    pub fn raytrace(&self, objects: Vec<Sphere>) -> Canvas {
        let mut canvas = Canvas::new(self.pixels_width, self.pixels_height);

        for (x, y) in iproduct!(0..self.pixels_width, 0..self.pixels_height) {
            //let plane_y = 0
        }

        canvas
    }
}

struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            objects: vec![],
            camera,
        }
    }

    pub fn add_object(&mut self, o: Sphere) -> &mut Self {
        self.objects.push(o);
        self
    }

    pub fn raytrace(&self) -> Canvas {
        // iterate over each of the pixels
        // cast into the scene
        // see if intersect with any objects
        //self.camera.trace(&self.objects)
        panic!()
    }
}

fn main() {
    //let mut canvas = Canvas::new(20, 20);

    //for (x, y) in iproduct!(0..20, 0..20).step_by(2) {
    //    canvas.ink(x, y);
    //}

    /*
    let sphere1 = Sphere::new(Vector::new(5.0, 5.0, 5.0), 1.0);
    let sphere2 = Sphere::new(Vector::new(7.0, 7.0, 5.0), 0.2);
    let sphere3 = Sphere::new(Vector::new(3.0, 3.0, 5.0), 0.5);

    let camera = Camera::new(10.0, 10.0, 512, 512);

    let mut scene = Scene::new(camera);
    scene
        .add_object(sphere1)
        .add_object(sphere2)
        .add_object(sphere3);

    let canvas = scene.raytrace();

    render_ppm(Path::new("/tmp/out.pbm"), canvas);
    */

    let v1 = Vector::new(0.0, 0.0, 1.0);
    let v2 = Vector::new(1.0, 0.0, 0.0);

    println!("{:?} x {:?} = {:?}", v1, v2, v1.cross(&v2));
}
