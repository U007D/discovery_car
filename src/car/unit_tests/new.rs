use super::*;

#[test]
fn new__create_car_succeeds_with_motors_off() {
    // given
    let periphss = MockPeripherals::new();
    let sut = Car::new;

    // when
    let res = sut(periphss);

    // then
    assert_eq!(
        res.motor_speeds(),
        [
            Norm::default(),
            Norm::default(),
            Norm::default(),
            Norm::default()
        ]
    );
}
