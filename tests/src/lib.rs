use novuskinc::drivers::Driver;
use novuskinc::drivers::empty::EmptyDriver;
use novuskinc::drivers::manager::DeviceDriverManager;
use novuskinc::drivers::names::NONE;

static mut DRIVER: EmptyDriver = EmptyDriver {
    v: 0,
};
static mut MANAGER: DeviceDriverManager = DeviceDriverManager {
    drivers: vec![],
};

#[test]
fn test_mut_driver() {
    {
        unsafe { MANAGER.add_driver(&mut DRIVER as &mut dyn Driver); }
    }

    let mut empty_driver = unsafe { MANAGER.get_driver(NONE).unwrap() };
    empty_driver.set_value(2);

    assert_eq!(2, empty_driver.value());
}
