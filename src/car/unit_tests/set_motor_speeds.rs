use super::*;

#[test]
fn set_motor_speeds__sets_all_motor_speeds_as_expected() {
    // given a default-constructed car
    let periphss = MockPeripherals::new();
    let expected_speeds = [
        Norm::new(-0.9).unwrap(),
        Norm::new(-0.4).unwrap(),
        Norm::new(0.4).unwrap(),
        Norm::new(0.9).unwrap(),
    ];
    let mut car = Car::new(periphss);

    // when
    let res = car.set_motor_speeds(expected_speeds);

    // then
    assert_eq!(res.motor_speeds(), expected_speeds);
}
