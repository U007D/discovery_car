use super::*;

#[test]
fn with_motor_speeds__sets_motor_speeds_appropriately() {
    // given a `Car` constructor expecting specific motor speeds
    let periphss = MockPeripherals::new();
    let expected_speeds = [
        Norm::new(-1.0).unwrap(),
        Norm::new(-0.5).unwrap(),
        Norm::new(0.5).unwrap(),
        Norm::new(1.0).unwrap(),
    ];
    let sut = Car::with_motor_speeds;

    // when
    let res = sut(periphss, expected_speeds);

    // then
    assert_eq!(res.motor_speeds(), expected_speeds);
}
