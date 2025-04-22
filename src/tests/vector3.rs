//! Tests specific to Vector3

/* --- IMPORTS --- */

use crate::Vector3;

/* --- ------- --- */



/* --- CONSTANTS --- */

const EPSILSON: f64 = 1e-9;

/* --- --------- --- */



/* --- TESTS --- */

#[test]
fn vector_addition() {

    let a = Vector3::new(1.2,  4.5, -2.2);
    let b = Vector3::new(3.2, -7.1,  4.1);


    // protection against floating point imprecision
    if ((a + b).x - 4.4).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

    if ((a + b).y - -2.6).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

    if ((a + b).z - 1.9).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

}



#[test]
fn vector_subtraction() {

    let a = Vector3::new(2.7, -6.2, 7.9);
    let b = Vector3::new(-1.6, 7.1, 8.2);


    // protection against floating point imprecision
    if ((a - b).x - 4.3).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

    if ((a - b).y - -13.3).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

    if ((a - b).z - -0.3).abs() > EPSILSON {
        panic!("Value outside expected range")
    }

}

/* --- ----- --- */
