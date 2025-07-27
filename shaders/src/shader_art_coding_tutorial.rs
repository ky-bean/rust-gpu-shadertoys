//! Ported to Rust from <https://www.shadertoy.com/view/mtyGWy>
//!
//! Original comment:
//! ```glsl
//! /* This animation is the material of my first youtube tutorial about creative
//!    coding, which is a video in which I try to introduce programmers to GLSL
//!    and to the wonderful world of shaders, while also trying to share my recent
//!    passion for this community.
//!                                        Video URL: https://youtu.be/f4s1h2YETNY
//! */
//! ```

use shared::*;
use spirv_std::glam::{vec3, Vec2, Vec3, Vec3Swizzles, Vec4};

// Note: This cfg is incorrect on its surface, it really should be "are we compiling with std", but
// we tie #[no_std] above to the same condition, so it's fine.
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn palette(t: f32) -> Vec3 {
    let a = vec3(0.5, 0.5, 0.5);
    let b = vec3(0.5, 0.5, 0.5);
    let c = vec3(1.0, 1.0, 1.0);
    let d = vec3(0.263, 0.416, 0.557);

    a + b * (6.28318 * (c * t + d)).cos()
}

pub struct Inputs {
    pub resolution: Vec3,
    pub time: f32,
}

impl Inputs {
    pub fn main_image(&self, frag_color: &mut Vec4, frag_coord: Vec2) {
        let mut uv = (frag_coord * 2.0 - self.resolution.xy()) / self.resolution.y;
        let uv0 = uv;
        let mut final_color = vec3(0.0, 0.0, 0.0);

        for i in 0..4 {
            let i = i as f32;
            uv = (uv * 1.5).fract_gl() - 0.5;

            let d = uv.length() * (-uv0.length()).exp();

            let col = palette(uv0.length() + i * 0.4 + self.time * 0.4);

            let d = (d * 8.0 + self.time).sin() / 8.0;
            let d = d.abs();
            let d = (0.01 / d).powf(1.2);

            final_color += col * d;
        }

        *frag_color = final_color.extend(1.0);
    }
}
