use fern_simulator::{Fern, run_simulation};

#[test]
fn test_grow_time() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };
    run_simulation(&mut fern, 1000);
    assert_eq!(2.7169239322355985, fern.size);
}