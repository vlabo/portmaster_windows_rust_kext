use core::mem::MaybeUninit;

use crate::common::ControlCode;
use crate::device;
use num_traits::FromPrimitive;
use wdk::ffi::{WdfObjectAttributes, WdfObjectContextTypeInfo};
use wdk::irp_helpers::{DeviceControlRequest, ReadRequest, WriteRequest};
use wdk::{err, info, interface};
use windows_sys::Wdk::Foundation::{DEVICE_OBJECT, DRIVER_OBJECT, IRP};
use windows_sys::Win32::Foundation::{HANDLE, NTSTATUS, STATUS_SUCCESS};

static VERSION: [u8; 4] = include!("../../kext_interface/version.txt");

static mut DRIVER_CONFIG: WdfObjectContextTypeInfo =
    WdfObjectContextTypeInfo::default("DriverContext\0");

// Compile time check. Function should not be executed.
#[allow(dead_code)]
pub fn ensure_correctness() {
    // Ensure that zeroed state is a valid state.
    let _: device::Device = unsafe { MaybeUninit::zeroed().assume_init() };
}

// DriverEntry is the entry point of the driver (main function). Will be called when driver is loaded.
// Name should not be changed
#[export_name = "DriverEntry"]
pub extern "system" fn driver_entry(
    driver_object: *mut windows_sys::Wdk::Foundation::DRIVER_OBJECT,
    registry_path: *mut windows_sys::Win32::Foundation::UNICODE_STRING,
) -> windows_sys::Win32::Foundation::NTSTATUS {
    info!("Starting initialization...");

    // Setup object attribute.
    let mut object_attributes = WdfObjectAttributes::new();
    object_attributes.add_context::<device::Device>(unsafe { &mut DRIVER_CONFIG });
    object_attributes.set_cleanup_fn(device_cleanup);

    // Initialize driver object.
    let mut driver = match interface::init_driver_object(
        driver_object,
        registry_path,
        "PortmasterKext",
        object_attributes,
    ) {
        Ok(driver) => driver,
        Err(status) => {
            err!("driver_entry: failed to initialize driver: {}", status);
            return windows_sys::Win32::Foundation::STATUS_FAILED_DRIVER_ENTRY;
        }
    };

    // Set driver functions.
    driver.set_driver_unload(driver_unload);
    driver.set_read_fn(driver_read);
    driver.set_write_fn(driver_write);
    driver.set_device_control_fn(device_control);

    // Initialize device.
    if let Some(device_object) = driver.get_device_object_ref() {
        if let Ok(context) =
            interface::get_device_context_from_device_object::<device::Device>(device_object)
        {
            context.init(&driver);
        }
    }

    STATUS_SUCCESS
}

// device_cleanup cleanup function is called when the current device object will be removed.
extern "system" fn device_cleanup(device: HANDLE) {
    let device = interface::get_device_context_from_wdf_device::<device::Device>(device, unsafe {
        &DRIVER_CONFIG
    });

    unsafe {
        // Call drop without freeing memory. Memory is manged by the kernel.
        if let Some(device) = device.as_mut() {
            device.cleanup();
            core::ptr::drop_in_place(device);
        }
    }
}

// driver_unload function is called when service delete is called from user-space.
unsafe extern "system" fn driver_unload(_object: *const DRIVER_OBJECT) {
    info!("Unloading complete");
}

// driver_read event triggered from user-space on file.Read.
unsafe extern "system" fn driver_read(
    device_object: &mut DEVICE_OBJECT,
    irp: &mut IRP,
) -> NTSTATUS {
    let mut read_request = ReadRequest::new(irp);
    let Ok(device) =
        interface::get_device_context_from_device_object::<device::Device>(device_object)
    else {
        read_request.complete();
        return read_request.get_status();
    };

    device.read(&mut read_request);

    read_request.get_status()
}

/// driver_write event triggered from user-space on file.Write.
unsafe extern "system" fn driver_write(
    device_object: &mut DEVICE_OBJECT,
    irp: &mut IRP,
) -> NTSTATUS {
    let mut write_request = WriteRequest::new(irp);
    let Ok(device) =
        interface::get_device_context_from_device_object::<device::Device>(device_object)
    else {
        write_request.complete();
        return write_request.get_status();
    };

    device.write(&mut write_request);

    write_request.mark_all_as_read();
    write_request.complete();
    write_request.get_status()
}

/// device_control event triggered from user-space on file.deviceIOControl.
unsafe extern "system" fn device_control(
    device_object: &mut DEVICE_OBJECT,
    irp: &mut IRP,
) -> NTSTATUS {
    let mut control_request = DeviceControlRequest::new(irp);
    let Ok(device) =
        interface::get_device_context_from_device_object::<device::Device>(device_object)
    else {
        control_request.complete();
        return control_request.get_status();
    };

    let Some(control_code): Option<ControlCode> =
        FromPrimitive::from_u32(control_request.get_control_code())
    else {
        wdk::info!("Unknown IOCTL code: {}", control_request.get_control_code());
        control_request.not_implemented();
        return control_request.get_status();
    };

    wdk::info!("IOCTL: {}", control_code);

    match control_code {
        ControlCode::Version => {
            control_request.write(&VERSION);
        }
        ControlCode::ShutdownRequest => device.shutdown(),
    };

    control_request.complete();
    control_request.get_status()
}
