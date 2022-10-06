use novuskinc::prelude::KernelConsole;

#[test]
fn update_chars_test() {
    let console = KernelConsole::new("Test Console", 0, 0);

    console.update_chars_written(1);

    assert_eq!(1, console.chars_written.get());
}
