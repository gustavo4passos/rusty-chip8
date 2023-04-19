pub trait Renderer {
    fn init(&mut self);
    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32);
    fn clear(&mut self);
    fn draw_screen(&mut self, fb: &[u8]);
}