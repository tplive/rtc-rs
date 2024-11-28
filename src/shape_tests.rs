#[cfg(test)]
use crate::shape::Sphere;

#[test]
fn creating_a_sphere() {

    // Since id's are given by the global static function in utils.rs, and tests apparently run
    // async, there is no guarantee that the id's will be 1, 2, 3. Instead, we ensure that they are
    // unique and sequential.
    let s1 = Sphere::new(); // .id = n
    let s2 = Sphere::new(); // .id = n + 1
    let s3 = Sphere::new(); // .id = n + 2

    let n = s1.id;

    assert_eq!(s1.id, n);
    assert_eq!(s2.id, n+1);
    assert_eq!(s3.id, n+2);
}
