// apply test attribute to the whole module
#![cfg(test)]

mod lifter;

use lifter::Lifter;

#[test]
fn total_calculates_correctly() {
    let lifter = Lifter::new(
        105.00,
        100.00,
        100.00, 
        100.00
    );

    assert_eq!(lifter.total(), 300.00);
}

#[test]
fn wilks_calculates_correctly() {
    let lifter = Lifter::new(
        100.00,
        100.00,
        100.00, 
        100.00
    );

    assert_eq!(lifter.wilks(), 3);
}