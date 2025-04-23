//! Tests specific to Vector3

/* --- IMPORTS --- */

use crate::Vector3;

/* --- ------- --- */



/* --- CONSTANTS --- */

// Due to floating point impresition we have to allow a small range of
// values around the correct result
const EPSILSON: f64 = 1e-9;

/* --- --------- --- */



/* --- TESTS --- */

#[test]
fn vector_addition() {

    let a = Vector3::new(1.2,  4.5, -2.2);
    let b = Vector3::new(3.2, -7.1,  4.1);


    // Expected result: (4.4, -2.6, 1.9)
    if ((a + b).x - 4.4).abs() > EPSILSON {
        panic!("Expected 4.4, got {}", (a + b).x)
    }

    if ((a + b).y - -2.6).abs() > EPSILSON {
        panic!("Expected -2.6, got {}", (a + b).y)
    }

    if ((a + b).z - 1.9).abs() > EPSILSON {
        panic!("Expected 1.9, got {}", (a + b).z)
    }

}



#[test]
fn vector_subtraction() {

    let a = Vector3::new(2.7, -6.2, 7.9);
    let b = Vector3::new(-1.6, 7.1, 8.2);


    // Expected result: (4.3, -13.3, -0.3)
    if ((a - b).x - 4.3).abs() > EPSILSON {
        panic!("Expected 4.3, got {}", (a - b).x)
    }

    if ((a - b).y - -13.3).abs() > EPSILSON {
        panic!("Expected -13.4, got {}", (a - b).y)
    }

    if ((a - b).z - -0.3).abs() > EPSILSON {
        panic!("Expected -0.3, got {}", (a - b).z)
    }

}



#[test]
fn dot_product() {

    let a = Vector3::new(5.0, -4.2, 0.4);
    let b = Vector3::new(1.7,  4.0, 3.4);
    let c = a.dot(b);

    // Expected result: -6.94
    if (c - -6.94).abs() > EPSILSON {
        panic!("Expected -6.94, got {c}");
    }

}



#[test]
fn cross_product() {

    let a = Vector3::new(2.7, -6.2, 7.9);
    let b = Vector3::new(-1.6, 7.1, 8.2);
    let c = a.cross(b);


    // Expected result: (-106.93, -34.78, 9.25)
    if (c.x - -106.93).abs() > EPSILSON {
        panic!("Expected 4.3, got {}", (a - b).x)
    }

    if (c.y - -34.78).abs() > EPSILSON {
        panic!("Expected -13.4, got {}", (a - b).y)
    }

    if (c.z - 9.25).abs() > EPSILSON {
        panic!("Expected -0.3, got {}", (a - b).z)
    }

}

/* --- ----- --- */
