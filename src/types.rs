#[derive(Default, Clone)]
pub struct Vec2<T> where T: Default {
    pub x: T,
    pub y: T,
}

#[derive(Default, Clone)]
pub struct Vec3<T> where T: Default {
    pub x: T,
    pub y: T,
    pub z: T,
}
