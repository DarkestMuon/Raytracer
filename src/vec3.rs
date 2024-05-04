use rand::Rng;
use vector3::Vector3;


pub fn length(v: Vector3) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

pub fn length_squared(v: Vector3) -> f64 {
    v.x * v.x + v.y * v.y + v.z * v.z
}
pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let mut rng = rand::thread_rng();
        // return Vector3::new(rng.gen_range(-0.5..0.5),rng.gen_range(-0.5..0.5), 0.0);
        let p = Vector3::new(rng.gen_range(0.0..1.0),rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
        if length_squared(p) < 1.0
            { return p; }
    }
}
pub fn random_unit_vector()  -> Vector3{
    return unit_vector(random_in_unit_sphere());
}
pub fn unit_vector(v: Vector3) -> Vector3 {
    v / length(v)
}

pub fn random_on_hemisphere( normal:Vector3) -> Vector3{
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(&normal) > 0.0 // In the same hemisphere as the normal
        {return on_unit_sphere;}
    else {return  on_unit_sphere * -1.0};
}