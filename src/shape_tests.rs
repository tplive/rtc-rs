#[cfg(test)]
use crate::shape::Sphere;

#[test]
fn creating_a_sphere() {


    let s1 = Sphere::new();
    let s2 = Sphere::new();
    let s3 = Sphere::new();

    assert_eq!(s1.id, 1);
    assert_eq!(s2.id, 2);
    assert_eq!(s3.id, 3);

    
}
