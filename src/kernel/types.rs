/// The ``KernelFunctionName``` enum is used for defining important kernel functions.
#[allow(non_upper_case_globals)]
#[derive(Copy, Clone, PartialEq)]
pub enum KernelFunctionName {
    /// This is an empty function that does nothing, it's just an empty function.
    empty,

    /// This function is intended for setting some device drivers to ``DEVICE_DRIVERS``
    /// (``DeviceDriverManager``), it helps the early architecture kernel setup.
    early_device_init,

    /// This function is used to initialize the device Novusk is running on, it doesn't have any
    /// arguments or a return type.
    device_init,

    /// This function is used for setting up the IRQ chip before it's initialized. This usually gets
    /// called in [``setup_arch``](todo)
    irqchip_setup,

    /// This function is used to initialize the device's IRQ chip, after this the kernel can start
    /// using IRQs.
    irqchip_init,

    /// This function is used for initializing device specific IRQs. If a device has it's own IRQs
    /// that won't be handled by the kernel, this function can be defined in a device kernel module
    /// to initialize those IRQs.
    device_specific_irqs_init,

    /// This function is used to handle device specific IRQs that the kernel doesn't handle. It
    /// should have one argument with an ``i16`` type, this argument is the IRQ number that was
    /// given.
    device_irq_handler,

    /// This function is for initializing the device timer. It shouldn't take any arguments.
    device_timer_init,

    /// The device's display information, this should return ```((u16, u16), *mut u32)```.
    device_display_info,

    /// For initializing a device specific frame buffer.
    device_fb_init,

    /// Initializes early serial I/O, it gets called by [``ArchSetup``](link) from ``setup`` if the
    /// arch kernel needs it. This is mainly for testing in a virtual machine.
    early_serial_init,

    /// Used to put the device into a certain power mode depending on the argument.
    set_power_mode,

    /// Turns on an on board led for a certain amount of time.
    led_blink,

    /// Initializes ethernet or wireless networking
    net_init,

    /// Sets a value to the index of the mailbox's buffer.
    set_mb_index,
    /// Sets the entire mailbox buffer.
    set_mb_buffer,
    /// Gets an index from the mailbox's buffer.
    get_mb_index,
    /// Get the entire mailbox buffer
    get_mb_buffer,
}
