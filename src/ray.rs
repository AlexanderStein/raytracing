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

    pub fn at(&self, t: ElemType) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self) -> Color {
        if self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0)
        }
        let unit_direction = unit_vector(&self.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: &Point3, radius: ElemType) -> bool {
        let oc = self.origin() - *center;
        let a = self.direction().dot(&self.direction());
        let b = 2.0 * oc.dot(&self.direction());
        let c = oc.dot(&oc) - radius*radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant > 0.0
    }
}
