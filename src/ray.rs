use crate::{Color, vec3::*};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray{origin, direction}
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self) -> Color {
        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
        }
        let unit_direction = &self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - *center;
        let a = self.direction().dot(&self.direction());
        let b = 2.0 * oc.dot(&self.direction());
        let c = oc.dot(&oc) - radius*radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt() ) / (2.0*a);
        }
    }
}
