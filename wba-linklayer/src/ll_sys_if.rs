#![allow(dead_code)]
use bitflags::bitflags;
use core::ffi::c_void;

use crate::ffi;

/// Runtime configuration applied when bootstrapping the link-layer system layer.
#[derive(Debug, Clone, Copy)]
pub struct SystemConfig {
    /// Invoke `ll_sys_config_params` before any other action.
    pub configure_params: bool,
    /// Register the background process with the ST scheduler.
    pub init_background_task: bool,
    /// Enable IRQs once initialisation completes.
    pub enable_irq_on_init: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            configure_params: true,
            init_background_task: true,
            enable_irq_on_init: true,
        }
    }
}

bitflags! {
    /// Convenience bit-mask describing which interrupt groups to toggle.
    #[derive(Default)]
    pub struct InterruptMask: u8 {
        const RADIO_HIGH = ffi::LL_HIGH_ISR_ONLY as u8;
        const RADIO_LOW  = ffi::LL_LOW_ISR_ONLY as u8;
        const SYSTEM_LOW = ffi::SYS_LOW_ISR as u8;
    }
}

/// Triple describing scheduler timings expressed in link-layer sleep-timer cycles (31 µs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerTiming {
    pub drift_time: u32,
    pub execution_time: u32,
    pub scheduling_time: u32,
}

impl SchedulerTiming {
    pub const fn new(drift_time: u32, execution_time: u32, scheduling_time: u32) -> Self {
        Self {
            drift_time,
            execution_time,
            scheduling_time,
        }
    }

    fn into_raw(self) -> ffi::Evnt_timing_t {
        ffi::Evnt_timing_t {
            drift_time: self.drift_time,
            exec_time: self.execution_time,
            schdling_time: self.scheduling_time,
        }
    }
}

/// Status codes returned by several `ll_sys_*` routines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlSysStatus {
    Ok,
    Error,
    Busy,
}

impl LlSysStatus {
    #[inline]
    fn from_raw(value: ffi::ll_sys_status_t) -> Self {
        if value == ffi::LL_SYS_STATUS_T_LL_SYS_OK {
            Self::Ok
        } else if value == ffi::LL_SYS_STATUS_T_LL_SYS_BUSY {
            Self::Busy
        } else {
            Self::Error
        }
    }

    #[inline]
    fn into_result(self) -> Result<(), Self> {
        match self {
            Self::Ok => Ok(()),
            other => Err(other),
        }
    }
}

/// Deep sleep state reported by the vendor system layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeepSleepState {
    Disabled,
    Enabled,
}

/// Initialise the system layer using the provided configuration.
pub fn init(config: &SystemConfig) {
    unsafe {
        if config.configure_params {
            ffi::ll_sys_config_params();
        }

        if config.init_background_task {
            ffi::ll_sys_bg_process_init();
        }

        if config.enable_irq_on_init {
            ffi::ll_sys_enable_irq();
        } else {
            ffi::ll_sys_disable_irq();
        }
    }
}

/// Register the Link Layer background task with the vendor scheduler.
pub fn register_background_task() {
    unsafe { ffi::ll_sys_bg_process_init() };
}

/// Execute the Link Layer background process once.
pub fn run_background_once() {
    unsafe { ffi::ll_sys_bg_process() };
}

/// Schedule the Link Layer background process from thread mode.
pub fn schedule_background() {
    unsafe { ffi::ll_sys_schedule_bg_process() };
}

/// Schedule the Link Layer background process from ISR context.
pub fn schedule_background_from_isr() {
    unsafe { ffi::ll_sys_schedule_bg_process_isr() };
}

/// Request a background temperature measurement (if enabled in the vendor stack).
pub fn request_temperature() {
    unsafe { ffi::ll_sys_request_temperature() };
}

/// Invoke the vendor Host stack process hook.
pub fn host_stack_process() {
    unsafe { ffi::HostStack_Process() };
}

/// Request the PHY calibration routine to start.
pub fn start_phy_calibration() {
    unsafe { ffi::ll_sys_phy_start_clbr() };
}

/// Request the PHY calibration routine to stop.
pub fn stop_phy_calibration() {
    unsafe { ffi::ll_sys_phy_stop_clbr() };
}

/// Set BLE scheduler timings; returns the effective execution time computed by the vendor firmware.
pub fn configure_ble_scheduler_timings(drift_time: u8, exec_time: u8) -> u32 {
    unsafe { ffi::ll_sys_config_BLE_schldr_timings(drift_time, exec_time) }
}

/// Forward updated scheduler timings to the vendor shim.
pub fn notify_scheduler_timing_update(timing: SchedulerTiming) {
    let mut raw = timing.into_raw();
    unsafe { ffi::ll_sys_schldr_timing_update_not(&mut raw as *mut _) };
}

/// Return the current value of the Link Layer sleep timer.
pub fn sleep_timer_value() -> u32 {
    unsafe { ffi::ll_intf_cmn_get_slptmr_value() }
}

/// Return the number of concurrent state machines supported by the vendor firmware.
pub fn concurrent_state_machines() -> u8 {
    unsafe { ffi::ll_sys_get_concurrent_state_machines_num() }
}

/// Retrieve the brief Link Layer firmware version (major/minor/patch packed into a byte).
pub fn brief_firmware_version() -> u8 {
    unsafe { ffi::ll_sys_get_brief_fw_version() }
}

/// Retrieve the system firmware version hash.
pub fn system_firmware_version() -> u32 {
    unsafe { ffi::ll_sys_get_system_fw_version() }
}

/// Retrieve the source firmware version hash.
pub fn source_firmware_version() -> u32 {
    unsafe { ffi::ll_sys_get_source_fw_version() }
}

/// Returns `true` if a pointer refers to a location inside the BLE memory region.
pub fn is_pointer_in_ble_memory(ptr: *mut c_void) -> bool {
    unsafe { ffi::ll_intf_is_ptr_in_ble_mem(ptr) != 0 }
}

/// Host callback signature used by the vendor controller.
pub type HostCallback = unsafe extern "C" fn(*mut ffi::ble_buff_hdr_t) -> u8;

/// Initialise the BLE controller with a host callback.
pub fn init_ble_controller(callback: HostCallback) {
    unsafe { ffi::ll_sys_ble_cntrl_init(Some(callback)) };
}

/// Initialise the IEEE 802.15.4 MAC controller glue.
pub fn init_mac_controller() {
    unsafe { ffi::ll_sys_mac_cntrl_init() };
}

/// Initialise the Thread controller glue.
pub fn init_thread_controller() {
    unsafe { ffi::ll_sys_thread_init() };
}

/// Initialise the vendor sequencer background process slot.
pub fn init_sequencer_background() {
    unsafe { ffi::ll_sys_sequencer_bg_process_init() };
}

/// Request the vendor sequencer to schedule a background process.
pub fn sequencer_schedule_background() {
    unsafe { ffi::ll_sys_sequencer_schedule_bg_process() };
}

/// Trigger the HCI host stack processing routine.
pub fn host_stack_process_once() {
    unsafe { ffi::HostStack_Process() };
}

/// Enter the vendor deep-sleep initialisation flow.
pub fn deep_sleep_init() -> Result<(), LlSysStatus> {
    LlSysStatus::from_raw(unsafe { ffi::ll_sys_dp_slp_init() }).into_result()
}

/// Request the vendor deep-sleep controller to enter sleep for a given duration (in sleep timer cycles).
pub fn deep_sleep_enter(duration: u32) -> Result<(), LlSysStatus> {
    LlSysStatus::from_raw(unsafe { ffi::ll_sys_dp_slp_enter(duration) }).into_result()
}

/// Exit deep sleep mode.
pub fn deep_sleep_exit() -> Result<(), LlSysStatus> {
    LlSysStatus::from_raw(unsafe { ffi::ll_sys_dp_slp_exit() }).into_result()
}

/// Query the current deep sleep state.
pub fn deep_sleep_state() -> DeepSleepState {
    let state = unsafe { ffi::ll_sys_dp_slp_get_state() };
    match state {
        s if s == ffi::LL_SYS_DP_SLP_STATE_T_LL_SYS_DP_SLP_ENABLED => DeepSleepState::Enabled,
        _ => DeepSleepState::Disabled,
    }
}

/// Manually invoke the vendor wake-up callback hook.
///
/// # Safety
///
/// The caller must ensure that `arg` matches the expectations of the underlying vendor firmware.
pub unsafe fn deep_sleep_wakeup_callback(arg: *const c_void) {
    ffi::ll_sys_dp_slp_wakeup_evt_clbk(arg);
}

/// Enable vendor-managed IRQs.
pub fn enable_ll_irq() {
    unsafe { ffi::ll_sys_enable_irq() };
}

/// Disable vendor-managed IRQs.
pub fn disable_ll_irq() {
    unsafe { ffi::ll_sys_disable_irq() };
}

/// Enable specific vendor interrupt groups.
pub fn enable_ll_specific_irq(mask: InterruptMask) {
    if mask.is_empty() {
        return;
    }
    unsafe { ffi::ll_sys_enable_specific_irq(mask.bits()) };
}

/// Disable specific vendor interrupt groups.
pub fn disable_ll_specific_irq(mask: InterruptMask) {
    if mask.is_empty() {
        return;
    }
    unsafe { ffi::ll_sys_disable_specific_irq(mask.bits()) };
}
