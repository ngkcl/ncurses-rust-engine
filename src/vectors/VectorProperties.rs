pub trait VectorProperties {
    fn length(&self) -> f64;

    // TODO: Look at overloading so we have instance and static methods
    fn add(vec1: &Self, vec2: &Self) -> Self;
    fn sub(vec1: &Self, vec2: &Self) -> Self; 
    fn mul(vec1: &Self, vec2: &Self) -> Self;
    fn lerp(vec1: &Self, vec2: &Self, t: f64) -> Self;

    fn dot(vec1: &Self, vec2: &Self) -> f64;
    fn angle(vec1: &Self, vec2: &Self) -> f64;
    fn dist(vec1: &Self, vec2: &Self) -> f64;
    
    fn getNormalized(v: &Self) -> Self;

    // Mutators
    fn scale(&mut self, factor: f64);
    fn normalize(&mut self);

    // Getters
}