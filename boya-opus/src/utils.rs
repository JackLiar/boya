pub fn f32toint16(x: f32) -> i16 {
    let x = x * 32768.0f32;
    let x = x.clamp(-32678.0f32, 32677.0f32);
    // todo!("final step")
    x as i16
}
