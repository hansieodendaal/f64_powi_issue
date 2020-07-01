#[allow(clippy::float_cmp)]

use num::pow;

fn main() {
    let base = 0.999_999_560_409_038_5f64;
    let i = [
        9182i32, 9430i32, 10855i32, 10856i32, 10857i32, 11708i32, 30335i32, 33923i32, 34947i32,
    ];
    // Reference results from:
    // - stable-x86_64-unknown-linux-gnu  (Debug)
    // - stable-x86_64-pc-windows-gnu     (Debug, Release)
    // - stable-x86_64-pc-windows-msvc    (Debug)
    // - nightly-x86_64-unknown-linux-gnu (Debug)
    // - nightly-x86_64-pc-windows-gnu    (Debug, Release)
    let result = [
        0.9959718099156295f64,
        0.9958632363991619f64,
        0.9952396058283226f64,
        0.9952391683299874f64,
        0.9952387308318444f64,
        0.9948664896246138f64,
        0.9867535223990311f64,
        0.9851983894664985f64,
        0.9847550108373471f64,
    ];

    println!();
    let iter = i.iter().zip(&result);
    for item in iter {
        let v = pow(base, *item.0 as usize);
        println!("num::pow: {}", &v);
        assert_eq!(v, *item.1)
    }

    println!();
    let iter = i.iter().zip(&result);
    for item in iter {
        let v = base.powi(*item.0);
        println!("f64.powi: {}", &v);
        assert_eq!(v, *item.1)
    }
}
