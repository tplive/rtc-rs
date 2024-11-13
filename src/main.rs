mod rtc;

use rtc::tuples::{point, vector, Tuple};


fn main() {
    // Putting it together Chapter 1
    #[derive(Debug)]
    struct Projectile {
        position: Tuple,
        velocity: Tuple,
    }

    impl Projectile {
        fn new(position: Tuple, velocity: Tuple) -> Self {
            Self {position, velocity}
        }
    }

    struct Environment {
        gravity: Tuple,
        wind: Tuple,
    }

    impl Environment {
        fn new(gravity: Tuple, wind: Tuple) -> Self {
            Self {gravity, wind}
        }
    }

    fn tick(env: &Environment, proj: Projectile) -> Projectile {
        let pos = proj.position + proj.velocity;
        let vel = proj.velocity + env.gravity + env.wind;

        return Projectile::new(pos, vel);
    }

    let mut p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0).normalize());
    let e = Environment::new(vector(0.0, -0.1,0.0), vector(-0.01, 0.0, 0.0));

    while p.position.y >= 0.0 {

        p = tick(&e, p);
        
        print!("Position: {:?}\n", p.position);
    }
}
