use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn clear();
    pub fn dot(x: f32, y: f32);
}

struct Particle {
    x: f32,
    y: f32,
    angle: f32,
}

static mut PARTICLES: Vec<Particle> = vec![];

#[wasm_bindgen]
pub unsafe fn init() {
    for i in 0..100000 {
        PARTICLES.push(Particle {
            x: Math::random() as f32 * 300f32,
            y: Math::random() as f32 * 300f32,
            angle: Math::random() as f32 * std::f32::consts::PI * 2f32,
        });
    }
}

#[wasm_bindgen]
pub unsafe fn frame() {
    clear();
    for particle in &mut PARTICLES {
        particle.x += particle.angle.cos() * 1f32;
        particle.y += particle.angle.sin() * 1f32;
        dot(particle.x, particle.y)
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //console::log_1(&JsValue::from_str(&format!("cos: {}", 0f32.cos())));
    //console::log_1(&JsValue::from_str(&format!("rand: {}", Math::random())));

    Ok(())
}
