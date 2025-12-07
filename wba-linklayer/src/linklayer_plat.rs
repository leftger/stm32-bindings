#![allow(dead_code)]

use core::fmt;

use bitflags::bitflags;

use crate::ffi;

/// Runtime configuration applied when bootstrapping the link-layer platform.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Invoke the vendor clock initialisation routine before touching any other entry point.
    pub clock_init: bool,
    /// Optional high-priority radio interrupt handler.
    pub radio_isr: Option<InterruptHandler>,
    /// Optional software low-priority interrupt handler.
    pub sw_low_isr: Option<InterruptHandler>,
    /// Enable the baseband clock as part of the initialisation sequence.
    pub enable_active_clock: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock_init: true,
            radio_isr: None,
            sw_low_isr: None,
            enable_active_clock: true,
        }
    }
}

/// Wrapper around an `extern "C"` ISR pointer consumed by the vendor firmware.
///
/// # Safety
///
/// Callers must guarantee that the wrapped function follows the interrupt-safety
/// requirements imposed by the platform (proper ABI, no unwinding, etc.).
#[derive(Clone, Copy)]
pub struct InterruptHandler {
    raw: unsafe extern "C" fn(),
}

impl InterruptHandler {
    /// Create a new wrapper from a raw interrupt handler pointer.
    ///
    /// # Safety
    ///
    /// The caller must uphold the invariants documented for [`InterruptHandler`].
    pub const unsafe fn new(raw: unsafe extern "C" fn()) -> Self {
        Self { raw }
    }

    #[inline(always)]
    fn raw(self) -> unsafe extern "C" fn() {
        self.raw
    }
}

impl fmt::Debug for InterruptHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("InterruptHandler")
            .field(&(self.raw as usize))
            .finish()
    }
}

impl From<unsafe extern "C" fn()> for InterruptHandler {
    fn from(raw: unsafe extern "C" fn()) -> Self {
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SwLowPriority {
    /// Run the ISR with the default (high) priority.
    High = 0,
    /// Demote the ISR to the lowest radio priority.
    Low = 1,
}

impl Default for SwLowPriority {
    fn default() -> Self {
        SwLowPriority::High
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

/// Mirror the vendor start-up sequence using the safe configuration struct.
pub fn init(config: &Config) {
    if config.clock_init {
        unsafe { ffi::LINKLAYER_PLAT_ClockInit() };
    }

    if let Some(handler) = config.radio_isr {
        unsafe { ffi::LINKLAYER_PLAT_SetupRadioIT(Some(handler.raw())) };
    }

    if let Some(handler) = config.sw_low_isr {
        unsafe { ffi::LINKLAYER_PLAT_SetupSwLowIT(Some(handler.raw())) };
    }

    control_active_clock(config.enable_active_clock);
}

/// Busy-wait for `delay` microseconds.
pub fn delay_us(delay: u32) {
    unsafe { ffi::LINKLAYER_PLAT_DelayUs(delay) };
}

/// Delegate assertion handling to the platform layer.
#[track_caller]
pub fn assert_platform(condition: bool) {
    unsafe { ffi::LINKLAYER_PLAT_Assert(condition as u8) };
}

/// Enable or disable the baseband (active) radio clock.
pub fn control_active_clock(enable: bool) {
    unsafe { ffi::LINKLAYER_PLAT_AclkCtrl(enable as u8) };
}

pub fn notify_wfi_enter() {
    unsafe { ffi::LINKLAYER_PLAT_NotifyWFIEnter() };
}

pub fn notify_wfi_exit() {
    unsafe { ffi::LINKLAYER_PLAT_NotifyWFIExit() };
}

pub fn wait_for_hclk_ready() {
    unsafe { ffi::LINKLAYER_PLAT_WaitHclkRdy() };
}

/// Fill `buffer` with random bytes sourced from the on-chip RNG.
pub fn fill_random(buffer: &mut [u8]) {
    if buffer.is_empty() {
        return;
    }

    assert!(
        buffer.len() <= u32::MAX as usize,
        "buffer length does not fit in u32"
    );

    unsafe {
        ffi::LINKLAYER_PLAT_GetRNG(buffer.as_mut_ptr(), buffer.len() as u32);
    }
}

/// Manually trigger the SW-low interrupt with the requested priority.
pub fn trigger_sw_low_interrupt(priority: SwLowPriority) {
    unsafe { ffi::LINKLAYER_PLAT_TriggerSwLowIT(priority as u8) };
}

pub fn enable_irq() {
    unsafe { ffi::LINKLAYER_PLAT_EnableIRQ() };
}

pub fn disable_irq() {
    unsafe { ffi::LINKLAYER_PLAT_DisableIRQ() };
}

pub fn enable_specific_irqs(mask: InterruptMask) {
    if mask.is_empty() {
        return;
    }
    unsafe { ffi::LINKLAYER_PLAT_EnableSpecificIRQ(mask.bits()) };
}

pub fn disable_specific_irqs(mask: InterruptMask) {
    if mask.is_empty() {
        return;
    }
    unsafe { ffi::LINKLAYER_PLAT_DisableSpecificIRQ(mask.bits()) };
}

pub fn enable_radio_interrupt() {
    unsafe { ffi::LINKLAYER_PLAT_EnableRadioIT() };
}

pub fn disable_radio_interrupt() {
    unsafe { ffi::LINKLAYER_PLAT_DisableRadioIT() };
}

pub fn notify_radio_event_start() {
    unsafe { ffi::LINKLAYER_PLAT_StartRadioEvt() };
}

pub fn notify_radio_event_stop() {
    unsafe { ffi::LINKLAYER_PLAT_StopRadioEvt() };
}

pub fn notify_rco_calibration_start() {
    unsafe { ffi::LINKLAYER_PLAT_RCOStartClbr() };
}

pub fn notify_rco_calibration_stop() {
    unsafe { ffi::LINKLAYER_PLAT_RCOStopClbr() };
}

pub fn request_temperature_measurement() {
    unsafe { ffi::LINKLAYER_PLAT_RequestTemperature() };
}

pub fn phy_start_calibration() {
    unsafe { ffi::LINKLAYER_PLAT_PhyStartClbr() };
}

pub fn phy_stop_calibration() {
    unsafe { ffi::LINKLAYER_PLAT_PhyStopClbr() };
}

/// Forward the latest scheduler timing tuple to the vendor shim.
pub fn notify_scheduler_timing_update(timing: SchedulerTiming) {
    let mut raw = timing.into_raw();
    unsafe { ffi::LINKLAYER_PLAT_SCHLDR_TIMING_UPDATE_NOT(&mut raw as *mut _) };
}

pub fn st_company_id() -> u32 {
    unsafe { ffi::LINKLAYER_PLAT_GetSTCompanyID() }
}

pub fn unique_device_number() -> u32 {
    unsafe { ffi::LINKLAYER_PLAT_GetUDN() }
}
