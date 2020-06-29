pub trait MatrixProperties {
    type Vector;

    fn add(m1: &Self, m2: &Self) -> Self;
    fn sub(m1: &Self, m2: &Self) -> Self;
    fn mul(m1: &Self, m2: &Self) -> Self;
    fn trans(m: &Self, v: &Self::Vector) -> Self::Vector;

    // Mutators
    fn setIdentity(&mut self);
    fn setZero(&mut self);
    fn transpose(&mut self);
    fn invert(&mut self);
    fn negate(&mut self);

    fn det(&self) -> f64;
}
