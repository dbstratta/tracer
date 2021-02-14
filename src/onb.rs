use crate::vec3::{cross, Vec3};

#[derive(Copy, Clone)]
pub struct Onb {
    pub axes: [Vec3; 3],
}

impl Onb {
    pub fn from_w(n: Vec3) -> Self {
        let w = n.unit();

        let a = if w.x.abs() > 0.9 {
            Vec3::y()
        } else {
            Vec3::x()
        };

        let v = cross(w, a).unit();

        let u = cross(w, v);

        Self { axes: [u, v, w] }
    }

    pub const fn u(&self) -> Vec3 {
        self.axes[0]
    }

    pub const fn v(&self) -> Vec3 {
        self.axes[1]
    }

    pub const fn w(&self) -> Vec3 {
        self.axes[2]
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}
