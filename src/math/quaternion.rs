use std::ops::{Add, Mul};
use matrix::*;
use vector::*;
use easing::*;

#[derive(Copy, Clone, Debug)]
pub struct Quat{pub x: f32, pub y: f32, pub z: f32, pub w: f32}

impl Quat {
	pub fn from_raw(x: f32, y: f32, z: f32, w: f32) -> Quat {
		Quat{x,y,z,w}
	}

	pub fn new(axis: Vec3, angle: f32) -> Quat {
		let angle = angle / 2.0;
		let s = angle.sin();

		Quat::from_raw(
			axis.x * s,
			axis.y * s,
			axis.z * s,
			angle.cos()
		)
	}

	pub fn imaginary(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}

	pub fn magnitude(&self) -> f32 {
		(self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
	}

	pub fn normalize(&self) -> Quat {
		let m = self.magnitude();
		Quat::from_raw(self.x/m, self.y/m, self.z/m, self.w/m)
	}

	pub fn conjugate(&self) -> Quat {
		Quat::from_raw(-self.x, -self.y, -self.z, self.w)
	}

	pub fn to_mat4(&self) -> Mat4 {
		Mat4::from_rows([
			(*self * Vec3::new(1.0, 0.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 1.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 0.0, 1.0)).extend(0.0),
			Vec4::new(0.0, 0.0, 0.0, 1.0)
		])
	}
}


impl Add<Quat> for Quat {
	type Output = Quat;
	fn add(self, o: Quat) -> Quat {
		Quat::from_raw(self.x+o.x, self.y+o.y, self.z+o.z, self.w+o.w)
	}
}


impl Mul<Quat> for Quat {
	type Output = Quat;
	fn mul(self, o: Quat) -> Quat {
		Quat::from_raw(
			 self.w*o.x - self.z*o.y + self.y*o.z + self.x*o.w,
			 self.z*o.x + self.w*o.y - self.x*o.z + self.y*o.w,
			-self.y*o.x + self.x*o.y + self.w*o.z + self.z*o.w,
			-self.x*o.x - self.y*o.y - self.z*o.z + self.w*o.w
		)
	}
}

impl Mul<f32> for Quat {
	type Output = Quat;
	fn mul(self, o: f32) -> Quat {
		Quat::from_raw(self.x*o, self.y*o, self.z*o, self.w*o)
	}
}

impl Mul<Vec3> for Quat {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		let q = Quat::from_raw(o.x, o.y, o.z, 0.0);
		(self * q * self.conjugate()).imaginary()
	}
}


macro_rules! impl_ease_for_quat {
	($func: ident) => (
		fn $func(&self, start: Quat, end: Quat) -> Quat {
			Quat {
				x: self.$func(start.x, end.x),
				y: self.$func(start.y, end.y),
				z: self.$func(start.z, end.z),
				w: self.$func(start.w, end.w),
			}
		}
	)
}


impl Ease<Quat> for f32 {
	impl_ease_for_quat!(ease_linear);

	impl_ease_for_quat!(ease_quad_in);
	impl_ease_for_quat!(ease_quad_out);
	impl_ease_for_quat!(ease_quad_inout);

	impl_ease_for_quat!(ease_exp_in);
	impl_ease_for_quat!(ease_exp_out);
	impl_ease_for_quat!(ease_exp_inout);

	impl_ease_for_quat!(ease_elastic_in);
	impl_ease_for_quat!(ease_elastic_out);
	impl_ease_for_quat!(ease_elastic_inout);

	impl_ease_for_quat!(ease_back_in);
	impl_ease_for_quat!(ease_back_out);
	impl_ease_for_quat!(ease_back_inout);

	impl_ease_for_quat!(ease_bounce_in);
	impl_ease_for_quat!(ease_bounce_out);
	impl_ease_for_quat!(ease_bounce_inout);
}