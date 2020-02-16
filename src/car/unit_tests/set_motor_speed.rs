use super::*;

#[allow(clippy::indexing_slicing)]
#[test]
fn set_motor_speed__setting_specific_motor_speed_yields_expected_result() {
    // given a default-constructed car
    let periphss = MockPeripherals::new();
    let expected_speeds = [
        Norm::new(-0.8).unwrap(),
        Norm::new(-0.3).unwrap(),
        Norm::new(0.3).unwrap(),
        Norm::new(0.8).unwrap(),
    ];
    let mut car = Car::new(periphss);

    // when
    car.set_motor_speed(Motor::FrontLeft, expected_speeds[Motor::FrontLeft as usize])
        .set_motor_speed(
            Motor::FrontRight,
            expected_speeds[Motor::FrontRight as usize],
        )
        .set_motor_speed(Motor::BackLeft, expected_speeds[Motor::BackLeft as usize])
        .set_motor_speed(Motor::BackRight, expected_speeds[Motor::BackRight as usize]);

    // then
    assert_eq!(
        car.motor_speed(Motor::FrontLeft),
        expected_speeds[Motor::FrontLeft as usize]
    );
    assert_eq!(
        car.motor_speed(Motor::FrontRight),
        expected_speeds[Motor::FrontRight as usize]
    );
    assert_eq!(
        car.motor_speed(Motor::BackLeft),
        expected_speeds[Motor::BackLeft as usize]
    );
    assert_eq!(
        car.motor_speed(Motor::BackRight),
        expected_speeds[Motor::BackRight as usize]
    );
}
