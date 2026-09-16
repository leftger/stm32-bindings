#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize)
        };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
pub const __WORDSIZE: u32 = 32;
pub const __HAS_SAFE_BUFFERS: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __HAS_PTRCHECK: u32 = 0;
pub const __HAS_BOUNDS_SAFETY_ATTRIBUTES: u32 = 0;
pub const USE_CLANG_TYPES: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 4088;
pub const __PTHREAD_ATTR_SIZE__: u32 = 36;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 40;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 4;
pub const __PTHREAD_COND_SIZE__: u32 = 24;
pub const __PTHREAD_ONCE_SIZE__: u32 = 4;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 124;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 12;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u32 = 2147483647;
pub const INTPTR_MIN: i32 = -2147483648;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const RSIZE_MAX: u32 = 2147483647;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const Z_DATA_SYNC_STAGES: u32 = 2;
pub const USE_SLPTMR_CAL: u32 = 1;
pub const USE_SCAN_OUT_MUX: u32 = 1;
pub const USE_RNG: u32 = 0;
pub const USE_JTAG: u32 = 1;
pub const Z_RESET_SYNC_STAGES: u32 = 2;
pub const USE_AES: u32 = 1;
pub const SW_ACT_HIGH: u32 = 1;
pub const ANT: u32 = 1;
pub const BUS_CLK: u32 = 32;
pub const PHY_TYPE: u32 = 1;
pub const BLE_MAC_ROLE: u32 = 1;
pub const BLE_LL_ROLE: u32 = 1;
pub const ACT_CLK: u32 = 32;
pub const DF_NO_ANTENNAS: u32 = 8;
pub const LST_ADDR: u32 = 7;
pub const SEQ_RAM_ADDR_WIDTH: u32 = 7;
pub const BLE_STANDARD: u32 = 3;
pub const JTAG_CLK: u32 = 16;
pub const DATA_RAM_ADDR_WIDTH: u32 = 12;
pub const POW_GUARD_TIME: u32 = 5;
pub const INTERRUPT_ENABLE: u32 = 16;
pub const SET_PRIORITY_LEVEL: u32 = 2;
pub const PRIORITY_LEVEL_MASK: u32 = 15;
pub const LL_HIGH_ISR_ONLY: u32 = 1;
pub const LL_LOW_ISR_ONLY: u32 = 2;
pub const SYS_LOW_ISR: u32 = 4;
pub const EBQ_BUILD: u32 = 0;
pub const USE_HCI_TRANSPORT: u32 = 0;
pub const SUPPORT_GNRC_SCHDLR_IF: u32 = 1;
pub const SUPPORT_EXPLCT_OBSERVER_ROLE: u32 = 0;
pub const SUPPORT_EXPLCT_BROADCASTER_ROLE: u32 = 0;
pub const SUPPORT_MASTER_CONNECTION: u32 = 0;
pub const SUPPORT_SLAVE_CONNECTION: u32 = 0;
pub const SUPPORT_LE_ENCRYPTION: u32 = 0;
pub const SUPPORT_PRIVACY: u32 = 0;
pub const SUPPORT_LE_EXTENDED_ADVERTISING: u32 = 0;
pub const SUPPORT_LE_PERIODIC_ADVERTISING: u32 = 0;
pub const SUPPORT_LE_POWER_CLASS_1: u32 = 0;
pub const SUPPORT_AOA_AOD: u32 = 0;
pub const SUPPORT_PERIODIC_SYNC_TRANSFER: u32 = 0;
pub const SUPPORT_SLEEP_CLOCK_ACCURCY_UPDATES: u32 = 0;
pub const SUPPORT_CONNECTED_ISOCHRONOUS: u32 = 0;
pub const SUPPORT_BRD_ISOCHRONOUS: u32 = 0;
pub const SUPPORT_SYNC_ISOCHRONOUS: u32 = 0;
pub const SUPPORT_LE_POWER_CONTROL: u32 = 0;
pub const SUPPORT_CHANNEL_CLASSIFICATION: u32 = 0;
pub const SUPPORT_PERIODIC_ADV_ADI: u32 = 0;
pub const SUPPORT_LE_ENHANCED_CONN_UPDATE: u32 = 0;
pub const SUPPORT_CSSA: u32 = 0;
pub const SUPPORT_LE_PAWR_ADVERTISER_ROLE: u32 = 0;
pub const SUPPORT_LE_PAWR_SYNC_ROLE: u32 = 0;
pub const SUPPORT_CHANNEL_SOUNDING: u32 = 0;
pub const SUPPORT_FRAME_SPACE_UPDATE: u32 = 0;
pub const SUPPORT_EXT_FEATURE_SET: u32 = 0;
pub const SUPPORT_ISO_UNSEG_MODE: u32 = 0;
pub const SUPPORT_LE_ADVERTISERS_MONITORING: u32 = 0;
pub const MAX_NUM_CNCRT_STAT_MCHNS: u32 = 0;
pub const USE_NON_ACCURATE_32K_SLEEP_CLK: u32 = 1;
pub const NUM_OF_CTSM_EMNGR_HNDLS: u32 = 0;
pub const SUPPORT_AUGMENTED_BLE_MODE: u32 = 0;
pub const SUPPORT_PTA: u32 = 1;
pub const SUPPORT_CONFIGURABLE_GAIN_FIX: u32 = 0;
pub const CHECK_ANY_MISSED_EVENT_ON_DEEP_SLEEP_EXIT: u32 = 1;
pub const LL_BASIC: u32 = 0;
pub const FFD_DEVICE_CONFIG: u32 = 1;
pub const RAL_NUMBER_OF_INSTANCE: u32 = 1;
pub const MAX_NUMBER_OF_INDIRECT_DATA: u32 = 10;
pub const SUPPORT_SEC: u32 = 1;
pub const RADIO_CSMA: u32 = 1;
pub const ENHANCED_RX_WHILE_CSMA_BACKOFF_DELAY: u32 = 1;
pub const SUPPORT_A_MAC: u32 = 1;
pub const SMPL_PRTCL_TEST_ENABLE: u32 = 0;
pub const IEEE_EUI64_VENDOR_SPECIFIC_FUNC: u32 = 1;
pub const SUPPORT_ZIGBEE_PHY_CERTIFICATION: u32 = 0;
pub const POOL_BLOCK_SIZE: u32 = 16;
pub const POOL_TOTAL_BLOCKS_SIZE: u32 = 10;
pub const POOL_INDEX_SIZE: u32 = 6;
pub const SUPPORT_BLE: u32 = 1;
pub const SUPPORT_MAC: u32 = 1;
pub const SUPPORT_ANT: u32 = 0;
pub const SUPPORT_ANT_HCI_UART: u32 = 0;
pub const MAC_LAYER_BUILD: u32 = 1;
pub const SUPPORT_MAC_HCI_UART: u32 = 0;
pub const SUPPORT_AUG_MAC_HCI_UART: u32 = 0;
pub const SUPPORT_RADIO_HCI_UART: u32 = 0;
pub const CS_TESTING: u32 = 1;
pub const USE_ANT_MODE_FOR_CS_TESTING: u32 = 0;
pub const PROFILE_DISABLED: u32 = 0;
pub const PROFILE_DETAILED: u32 = 1;
pub const PROFILE_LIGHTWEIGHT: u32 = 2;
pub const PROFILE_BSP: u32 = 3;
pub const SUPPORT_PROFILE: u32 = 0;
pub const SUCCESS: u32 = 0;
pub const GENERAL_FAILURE: i32 = -1;
pub const GENERAL_ERROR_STATUS: u32 = 255;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const UNUSED_VALUE: u32 = 0;
pub const RADIO_MAC_PENDING_DONE_EVENT_MAX: u32 = 1;
pub const ED_TIMER_EVENT_MAX: u32 = 1;
pub const MAX_INDIRECT_DATA_TIMEOUT_EVENT: u32 = 10;
pub const PRDC_CLBR_TMR_EVENT_MAX: u32 = 1;
pub const CSL_RCV_TMR_EVENT_MAX: u32 = 1;
pub const OQPSK_RECEIVER_SENSTIVITY: i32 = -85;
pub const EXT_ADDRESS_LENGTH: u32 = 8;
pub const CONFIG_MAC_CSL_TRANSMITTER_ENABLE: u32 = 1;
pub const CONFIG_MAC_CSL_RECEIVER_ENABLE: u32 = 1;
pub const SUPPORT_RADIO_SECURITY_OT_1_2: u32 = 1;
pub const SUPPORT_ENH_ACK_LINK_METRICS_PROBING_OT_1_2: u32 = 1;
pub const SUPPORT_TIME_SYNC_OT_1_2: u32 = 1;
pub const END_OF_RADIO_ACTIVITY_REPORTING: u32 = 1;
pub const BLE_BUFF_HDR_STRT_PKT: u32 = 1;
pub const BLE_BUFF_HDR_CNTRL_PKT: u32 = 2;
pub const BLE_BUFF_HDR_BUFF_FRGMNTD: u32 = 4;
pub const BLE_BUFF_HDR_EVNT_CMD_PCKT: u32 = 8;
pub const BLE_BUFF_HDR_ACL_DATA_PCKT: u32 = 16;
pub const DEFAULT_PHY_CALIBRATION_PERIOD: u32 = 10;
pub const PHY_CALIBRATION_CONTEXT_BG: u32 = 0;
pub const PHY_CALIBRATION_CONTEXT_ISR: u32 = 1;
pub const SUPPORT_MAC_PHY_CONT_TESTING_CMDS: u32 = 1;
pub const SUPPORT_MAC_CONT_TESTING_CMDS_PHY_SUPPORT: u32 = 0;
pub const EXTERNAL_CUSTOM_CMDS: u32 = 0;
pub const SUPPORT_HCI_EVENT_ONLY: u32 = 1;
pub const SUPPORT_HCI_EVENT_ONLY_TESTING: u32 = 0;
pub const SUPPORT_HW_AUDIO_SYNC_SIGNAL: u32 = 0;
pub const SUPPORT_PAWR_CUSTOM_SYNC: u32 = 0;
pub const PAWR_TESTING: u32 = 0;
pub const SUPPORT_TIM_UPDT: u32 = 1;
pub const SUPPORT_RX_DTP_CONTROL: u32 = 1;
pub const SUPPORT_DYNAMIC_PREEMPH_COEFF: u32 = 0;
pub const GAIN_FIX_WAKEUP_TIME_OVERHEAD: u32 = 0;
pub const SUPPORT_PHY_SHUTDOWN_MODE: u32 = 1;
pub const PHY_SHUTDOWN_MODE_PHY_SUPPORT: u32 = 0;
pub const PHY_SHUTDOWN_WAKEUP_TIME_OVERHEAD: u32 = 0;
pub const SUPPORT_CTE_DEGRADATION_API: u32 = 0;
pub const CTE_DEGRADATION_API_PHY_SUPPORT: u32 = 0;
pub const NEAR_AUX_AFTER_EXT_SLEEP_TIMER_SCHEDULING: u32 = 0;
pub const PHY_USE_APB_TRANSPORT: u32 = 0;
pub const USE_NEW_DEMODULATOR: u32 = 0;
pub const ENABLE_AUTOMOUS_SCHEDULING_TIMING_UPDATE: u32 = 0;
pub const __BOOL_TRUE_FALSE_ARE_DEFINED: u32 = 1;
pub const FRM_CNTR_SIZE: u32 = 4;
pub const KEY_SRC_SIZE_MOD_2: u32 = 4;
pub const KEY_SRC_SIZE_MOD_3: u32 = 8;
pub const MODE_ZERO: u32 = 0;
pub const MODE_ONE: u32 = 1;
pub const MODE_TWO: u32 = 2;
pub const MODE_THREE: u32 = 3;
pub const KEY_SOURCE_SIZE: u32 = 8;
pub const KEY_ID_LOOKUP_DSCRP_LIST_SIZE: u32 = 3;
pub const MAX_NUM_OF_DEVICES_USE_KEY: u32 = 8;
pub const KEY_USAGE_LIST_SIZE: u32 = 5;
pub const KEY_SIZE: u32 = 16;
pub const MAX_MAC_SAFE_PAYLOAD_SIZE: u32 = 118;
pub const MAX_HDR_IE_SIZE: u32 = 3;
pub const MAX_ZIGBEE_EB_IE_LEN: u32 = 27;
pub const MAX_ADDITIONAL_ZIGBEE_PYLDIE_SIZE: u32 = 8;
pub const MAX_ZIGBEE_EBR_IE_LEN: u32 = 28;
pub const MAX_BEACON_FRAME_PENDING_ADDRESSES: u32 = 7;
pub const __API_TO_BE_DEPRECATED: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_IOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_IOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACCATALYST: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACCATALYSTAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_WATCHOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_WATCHOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_TVOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_TVOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_DRIVERKIT: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_VISIONOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_VISIONOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_KERNELKIT: u32 = 100000;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __MAC_10_14: u32 = 101400;
pub const __MAC_10_14_1: u32 = 101401;
pub const __MAC_10_14_4: u32 = 101404;
pub const __MAC_10_14_5: u32 = 101405;
pub const __MAC_10_14_6: u32 = 101406;
pub const __MAC_10_15: u32 = 101500;
pub const __MAC_10_15_1: u32 = 101501;
pub const __MAC_10_15_4: u32 = 101504;
pub const __MAC_10_16: u32 = 101600;
pub const __MAC_11_0: u32 = 110000;
pub const __MAC_11_1: u32 = 110100;
pub const __MAC_11_3: u32 = 110300;
pub const __MAC_11_4: u32 = 110400;
pub const __MAC_11_5: u32 = 110500;
pub const __MAC_11_6: u32 = 110600;
pub const __MAC_12_0: u32 = 120000;
pub const __MAC_12_1: u32 = 120100;
pub const __MAC_12_2: u32 = 120200;
pub const __MAC_12_3: u32 = 120300;
pub const __MAC_12_4: u32 = 120400;
pub const __MAC_12_5: u32 = 120500;
pub const __MAC_12_6: u32 = 120600;
pub const __MAC_12_7: u32 = 120700;
pub const __MAC_13_0: u32 = 130000;
pub const __MAC_13_1: u32 = 130100;
pub const __MAC_13_2: u32 = 130200;
pub const __MAC_13_3: u32 = 130300;
pub const __MAC_13_4: u32 = 130400;
pub const __MAC_13_5: u32 = 130500;
pub const __MAC_13_6: u32 = 130600;
pub const __MAC_13_7: u32 = 130700;
pub const __MAC_14_0: u32 = 140000;
pub const __MAC_14_1: u32 = 140100;
pub const __MAC_14_2: u32 = 140200;
pub const __MAC_14_3: u32 = 140300;
pub const __MAC_14_4: u32 = 140400;
pub const __MAC_14_5: u32 = 140500;
pub const __MAC_14_6: u32 = 140600;
pub const __MAC_14_7: u32 = 140700;
pub const __MAC_15_0: u32 = 150000;
pub const __MAC_15_1: u32 = 150100;
pub const __MAC_15_2: u32 = 150200;
pub const __MAC_15_3: u32 = 150300;
pub const __MAC_15_4: u32 = 150400;
pub const __MAC_15_5: u32 = 150500;
pub const __MAC_15_6: u32 = 150600;
pub const __MAC_16_0: u32 = 160000;
pub const __MAC_26_0: u32 = 260000;
pub const __MAC_26_1: u32 = 260100;
pub const __MAC_26_2: u32 = 260200;
pub const __MAC_26_3: u32 = 260300;
pub const __MAC_26_4: u32 = 260400;
pub const __MAC_26_5: u32 = 260500;
pub const __MAC_26_6: u32 = 260600;
pub const __MAC_27_0: u32 = 270000;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __IPHONE_11_4: u32 = 110400;
pub const __IPHONE_12_0: u32 = 120000;
pub const __IPHONE_12_1: u32 = 120100;
pub const __IPHONE_12_2: u32 = 120200;
pub const __IPHONE_12_3: u32 = 120300;
pub const __IPHONE_12_4: u32 = 120400;
pub const __IPHONE_13_0: u32 = 130000;
pub const __IPHONE_13_1: u32 = 130100;
pub const __IPHONE_13_2: u32 = 130200;
pub const __IPHONE_13_3: u32 = 130300;
pub const __IPHONE_13_4: u32 = 130400;
pub const __IPHONE_13_5: u32 = 130500;
pub const __IPHONE_13_6: u32 = 130600;
pub const __IPHONE_13_7: u32 = 130700;
pub const __IPHONE_14_0: u32 = 140000;
pub const __IPHONE_14_1: u32 = 140100;
pub const __IPHONE_14_2: u32 = 140200;
pub const __IPHONE_14_3: u32 = 140300;
pub const __IPHONE_14_5: u32 = 140500;
pub const __IPHONE_14_6: u32 = 140600;
pub const __IPHONE_14_7: u32 = 140700;
pub const __IPHONE_14_8: u32 = 140800;
pub const __IPHONE_15_0: u32 = 150000;
pub const __IPHONE_15_1: u32 = 150100;
pub const __IPHONE_15_2: u32 = 150200;
pub const __IPHONE_15_3: u32 = 150300;
pub const __IPHONE_15_4: u32 = 150400;
pub const __IPHONE_15_5: u32 = 150500;
pub const __IPHONE_15_6: u32 = 150600;
pub const __IPHONE_15_7: u32 = 150700;
pub const __IPHONE_15_8: u32 = 150800;
pub const __IPHONE_16_0: u32 = 160000;
pub const __IPHONE_16_1: u32 = 160100;
pub const __IPHONE_16_2: u32 = 160200;
pub const __IPHONE_16_3: u32 = 160300;
pub const __IPHONE_16_4: u32 = 160400;
pub const __IPHONE_16_5: u32 = 160500;
pub const __IPHONE_16_6: u32 = 160600;
pub const __IPHONE_16_7: u32 = 160700;
pub const __IPHONE_17_0: u32 = 170000;
pub const __IPHONE_17_1: u32 = 170100;
pub const __IPHONE_17_2: u32 = 170200;
pub const __IPHONE_17_3: u32 = 170300;
pub const __IPHONE_17_4: u32 = 170400;
pub const __IPHONE_17_5: u32 = 170500;
pub const __IPHONE_17_6: u32 = 170600;
pub const __IPHONE_17_7: u32 = 170700;
pub const __IPHONE_18_0: u32 = 180000;
pub const __IPHONE_18_1: u32 = 180100;
pub const __IPHONE_18_2: u32 = 180200;
pub const __IPHONE_18_3: u32 = 180300;
pub const __IPHONE_18_4: u32 = 180400;
pub const __IPHONE_18_5: u32 = 180500;
pub const __IPHONE_18_6: u32 = 180600;
pub const __IPHONE_19_0: u32 = 190000;
pub const __IPHONE_26_0: u32 = 260000;
pub const __IPHONE_26_1: u32 = 260100;
pub const __IPHONE_26_2: u32 = 260200;
pub const __IPHONE_26_3: u32 = 260300;
pub const __IPHONE_26_4: u32 = 260400;
pub const __IPHONE_26_5: u32 = 260500;
pub const __IPHONE_26_6: u32 = 260600;
pub const __IPHONE_27_0: u32 = 270000;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __WATCHOS_5_0: u32 = 50000;
pub const __WATCHOS_5_1: u32 = 50100;
pub const __WATCHOS_5_2: u32 = 50200;
pub const __WATCHOS_5_3: u32 = 50300;
pub const __WATCHOS_6_0: u32 = 60000;
pub const __WATCHOS_6_1: u32 = 60100;
pub const __WATCHOS_6_2: u32 = 60200;
pub const __WATCHOS_7_0: u32 = 70000;
pub const __WATCHOS_7_1: u32 = 70100;
pub const __WATCHOS_7_2: u32 = 70200;
pub const __WATCHOS_7_3: u32 = 70300;
pub const __WATCHOS_7_4: u32 = 70400;
pub const __WATCHOS_7_5: u32 = 70500;
pub const __WATCHOS_7_6: u32 = 70600;
pub const __WATCHOS_8_0: u32 = 80000;
pub const __WATCHOS_8_1: u32 = 80100;
pub const __WATCHOS_8_3: u32 = 80300;
pub const __WATCHOS_8_4: u32 = 80400;
pub const __WATCHOS_8_5: u32 = 80500;
pub const __WATCHOS_8_6: u32 = 80600;
pub const __WATCHOS_8_7: u32 = 80700;
pub const __WATCHOS_8_8: u32 = 80800;
pub const __WATCHOS_9_0: u32 = 90000;
pub const __WATCHOS_9_1: u32 = 90100;
pub const __WATCHOS_9_2: u32 = 90200;
pub const __WATCHOS_9_3: u32 = 90300;
pub const __WATCHOS_9_4: u32 = 90400;
pub const __WATCHOS_9_5: u32 = 90500;
pub const __WATCHOS_9_6: u32 = 90600;
pub const __WATCHOS_10_0: u32 = 100000;
pub const __WATCHOS_10_1: u32 = 100100;
pub const __WATCHOS_10_2: u32 = 100200;
pub const __WATCHOS_10_3: u32 = 100300;
pub const __WATCHOS_10_4: u32 = 100400;
pub const __WATCHOS_10_5: u32 = 100500;
pub const __WATCHOS_10_6: u32 = 100600;
pub const __WATCHOS_10_7: u32 = 100700;
pub const __WATCHOS_11_0: u32 = 110000;
pub const __WATCHOS_11_1: u32 = 110100;
pub const __WATCHOS_11_2: u32 = 110200;
pub const __WATCHOS_11_3: u32 = 110300;
pub const __WATCHOS_11_4: u32 = 110400;
pub const __WATCHOS_11_5: u32 = 110500;
pub const __WATCHOS_11_6: u32 = 110600;
pub const __WATCHOS_12_0: u32 = 120000;
pub const __WATCHOS_26_0: u32 = 260000;
pub const __WATCHOS_26_1: u32 = 260100;
pub const __WATCHOS_26_2: u32 = 260200;
pub const __WATCHOS_26_3: u32 = 260300;
pub const __WATCHOS_26_4: u32 = 260400;
pub const __WATCHOS_26_5: u32 = 260500;
pub const __WATCHOS_26_6: u32 = 260600;
pub const __WATCHOS_27_0: u32 = 270000;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __TVOS_11_4: u32 = 110400;
pub const __TVOS_12_0: u32 = 120000;
pub const __TVOS_12_1: u32 = 120100;
pub const __TVOS_12_2: u32 = 120200;
pub const __TVOS_12_3: u32 = 120300;
pub const __TVOS_12_4: u32 = 120400;
pub const __TVOS_13_0: u32 = 130000;
pub const __TVOS_13_2: u32 = 130200;
pub const __TVOS_13_3: u32 = 130300;
pub const __TVOS_13_4: u32 = 130400;
pub const __TVOS_14_0: u32 = 140000;
pub const __TVOS_14_1: u32 = 140100;
pub const __TVOS_14_2: u32 = 140200;
pub const __TVOS_14_3: u32 = 140300;
pub const __TVOS_14_5: u32 = 140500;
pub const __TVOS_14_6: u32 = 140600;
pub const __TVOS_14_7: u32 = 140700;
pub const __TVOS_15_0: u32 = 150000;
pub const __TVOS_15_1: u32 = 150100;
pub const __TVOS_15_2: u32 = 150200;
pub const __TVOS_15_3: u32 = 150300;
pub const __TVOS_15_4: u32 = 150400;
pub const __TVOS_15_5: u32 = 150500;
pub const __TVOS_15_6: u32 = 150600;
pub const __TVOS_16_0: u32 = 160000;
pub const __TVOS_16_1: u32 = 160100;
pub const __TVOS_16_2: u32 = 160200;
pub const __TVOS_16_3: u32 = 160300;
pub const __TVOS_16_4: u32 = 160400;
pub const __TVOS_16_5: u32 = 160500;
pub const __TVOS_16_6: u32 = 160600;
pub const __TVOS_17_0: u32 = 170000;
pub const __TVOS_17_1: u32 = 170100;
pub const __TVOS_17_2: u32 = 170200;
pub const __TVOS_17_3: u32 = 170300;
pub const __TVOS_17_4: u32 = 170400;
pub const __TVOS_17_5: u32 = 170500;
pub const __TVOS_17_6: u32 = 170600;
pub const __TVOS_18_0: u32 = 180000;
pub const __TVOS_18_1: u32 = 180100;
pub const __TVOS_18_2: u32 = 180200;
pub const __TVOS_18_3: u32 = 180300;
pub const __TVOS_18_4: u32 = 180400;
pub const __TVOS_18_5: u32 = 180500;
pub const __TVOS_18_6: u32 = 180600;
pub const __TVOS_19_0: u32 = 190000;
pub const __TVOS_26_0: u32 = 260000;
pub const __TVOS_26_1: u32 = 260100;
pub const __TVOS_26_2: u32 = 260200;
pub const __TVOS_26_3: u32 = 260300;
pub const __TVOS_26_4: u32 = 260400;
pub const __TVOS_26_5: u32 = 260500;
pub const __TVOS_26_6: u32 = 260600;
pub const __TVOS_27_0: u32 = 270000;
pub const __BRIDGEOS_2_0: u32 = 20000;
pub const __BRIDGEOS_3_0: u32 = 30000;
pub const __BRIDGEOS_3_1: u32 = 30100;
pub const __BRIDGEOS_3_4: u32 = 30400;
pub const __BRIDGEOS_4_0: u32 = 40000;
pub const __BRIDGEOS_4_1: u32 = 40100;
pub const __BRIDGEOS_5_0: u32 = 50000;
pub const __BRIDGEOS_5_1: u32 = 50100;
pub const __BRIDGEOS_5_3: u32 = 50300;
pub const __BRIDGEOS_6_0: u32 = 60000;
pub const __BRIDGEOS_6_2: u32 = 60200;
pub const __BRIDGEOS_6_4: u32 = 60400;
pub const __BRIDGEOS_6_5: u32 = 60500;
pub const __BRIDGEOS_6_6: u32 = 60600;
pub const __BRIDGEOS_7_0: u32 = 70000;
pub const __BRIDGEOS_7_1: u32 = 70100;
pub const __BRIDGEOS_7_2: u32 = 70200;
pub const __BRIDGEOS_7_3: u32 = 70300;
pub const __BRIDGEOS_7_4: u32 = 70400;
pub const __BRIDGEOS_7_6: u32 = 70600;
pub const __BRIDGEOS_8_0: u32 = 80000;
pub const __BRIDGEOS_8_1: u32 = 80100;
pub const __BRIDGEOS_8_2: u32 = 80200;
pub const __BRIDGEOS_8_3: u32 = 80300;
pub const __BRIDGEOS_8_4: u32 = 80400;
pub const __BRIDGEOS_8_5: u32 = 80500;
pub const __BRIDGEOS_8_6: u32 = 80600;
pub const __BRIDGEOS_9_0: u32 = 90000;
pub const __BRIDGEOS_9_1: u32 = 90100;
pub const __BRIDGEOS_9_2: u32 = 90200;
pub const __BRIDGEOS_9_3: u32 = 90300;
pub const __BRIDGEOS_9_4: u32 = 90400;
pub const __BRIDGEOS_9_5: u32 = 90500;
pub const __BRIDGEOS_9_6: u32 = 90600;
pub const __BRIDGEOS_10_0: u32 = 100000;
pub const __BRIDGEOS_10_1: u32 = 100100;
pub const __BRIDGEOS_10_2: u32 = 100200;
pub const __BRIDGEOS_10_3: u32 = 100300;
pub const __BRIDGEOS_10_4: u32 = 100400;
pub const __BRIDGEOS_10_5: u32 = 100500;
pub const __BRIDGEOS_10_6: u32 = 100600;
pub const __DRIVERKIT_19_0: u32 = 190000;
pub const __DRIVERKIT_20_0: u32 = 200000;
pub const __DRIVERKIT_21_0: u32 = 210000;
pub const __DRIVERKIT_22_0: u32 = 220000;
pub const __DRIVERKIT_22_4: u32 = 220400;
pub const __DRIVERKIT_22_5: u32 = 220500;
pub const __DRIVERKIT_22_6: u32 = 220600;
pub const __DRIVERKIT_23_0: u32 = 230000;
pub const __DRIVERKIT_23_1: u32 = 230100;
pub const __DRIVERKIT_23_2: u32 = 230200;
pub const __DRIVERKIT_23_3: u32 = 230300;
pub const __DRIVERKIT_23_4: u32 = 230400;
pub const __DRIVERKIT_23_5: u32 = 230500;
pub const __DRIVERKIT_23_6: u32 = 230600;
pub const __DRIVERKIT_24_0: u32 = 240000;
pub const __DRIVERKIT_24_1: u32 = 240100;
pub const __DRIVERKIT_24_2: u32 = 240200;
pub const __DRIVERKIT_24_3: u32 = 240300;
pub const __DRIVERKIT_24_4: u32 = 240400;
pub const __DRIVERKIT_24_5: u32 = 240500;
pub const __DRIVERKIT_24_6: u32 = 240600;
pub const __DRIVERKIT_25_0: u32 = 250000;
pub const __DRIVERKIT_25_1: u32 = 250100;
pub const __DRIVERKIT_25_2: u32 = 250200;
pub const __DRIVERKIT_25_3: u32 = 250300;
pub const __DRIVERKIT_25_4: u32 = 250400;
pub const __DRIVERKIT_25_5: u32 = 250500;
pub const __DRIVERKIT_25_6: u32 = 250600;
pub const __DRIVERKIT_27_0: u32 = 270000;
pub const __VISIONOS_1_0: u32 = 10000;
pub const __VISIONOS_1_1: u32 = 10100;
pub const __VISIONOS_1_2: u32 = 10200;
pub const __VISIONOS_1_3: u32 = 10300;
pub const __VISIONOS_2_0: u32 = 20000;
pub const __VISIONOS_2_1: u32 = 20100;
pub const __VISIONOS_2_2: u32 = 20200;
pub const __VISIONOS_2_3: u32 = 20300;
pub const __VISIONOS_2_4: u32 = 20400;
pub const __VISIONOS_2_5: u32 = 20500;
pub const __VISIONOS_2_6: u32 = 20600;
pub const __VISIONOS_3_0: u32 = 30000;
pub const __VISIONOS_26_0: u32 = 260000;
pub const __VISIONOS_26_1: u32 = 260100;
pub const __VISIONOS_26_2: u32 = 260200;
pub const __VISIONOS_26_3: u32 = 260300;
pub const __VISIONOS_26_4: u32 = 260400;
pub const __VISIONOS_26_5: u32 = 260500;
pub const __VISIONOS_26_6: u32 = 260600;
pub const __VISIONOS_27_0: u32 = 270000;
pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
pub const MAC_OS_X_VERSION_10_14_5: u32 = 101405;
pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
pub const MAC_OS_X_VERSION_10_15_4: u32 = 101504;
pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
pub const MAC_OS_VERSION_11_0: u32 = 110000;
pub const MAC_OS_VERSION_11_1: u32 = 110100;
pub const MAC_OS_VERSION_11_3: u32 = 110300;
pub const MAC_OS_VERSION_11_4: u32 = 110400;
pub const MAC_OS_VERSION_11_5: u32 = 110500;
pub const MAC_OS_VERSION_11_6: u32 = 110600;
pub const MAC_OS_VERSION_12_0: u32 = 120000;
pub const MAC_OS_VERSION_12_1: u32 = 120100;
pub const MAC_OS_VERSION_12_2: u32 = 120200;
pub const MAC_OS_VERSION_12_3: u32 = 120300;
pub const MAC_OS_VERSION_12_4: u32 = 120400;
pub const MAC_OS_VERSION_12_5: u32 = 120500;
pub const MAC_OS_VERSION_12_6: u32 = 120600;
pub const MAC_OS_VERSION_12_7: u32 = 120700;
pub const MAC_OS_VERSION_13_0: u32 = 130000;
pub const MAC_OS_VERSION_13_1: u32 = 130100;
pub const MAC_OS_VERSION_13_2: u32 = 130200;
pub const MAC_OS_VERSION_13_3: u32 = 130300;
pub const MAC_OS_VERSION_13_4: u32 = 130400;
pub const MAC_OS_VERSION_13_5: u32 = 130500;
pub const MAC_OS_VERSION_13_6: u32 = 130600;
pub const MAC_OS_VERSION_13_7: u32 = 130700;
pub const MAC_OS_VERSION_14_0: u32 = 140000;
pub const MAC_OS_VERSION_14_1: u32 = 140100;
pub const MAC_OS_VERSION_14_2: u32 = 140200;
pub const MAC_OS_VERSION_14_3: u32 = 140300;
pub const MAC_OS_VERSION_14_4: u32 = 140400;
pub const MAC_OS_VERSION_14_5: u32 = 140500;
pub const MAC_OS_VERSION_14_6: u32 = 140600;
pub const MAC_OS_VERSION_14_7: u32 = 140700;
pub const MAC_OS_VERSION_15_0: u32 = 150000;
pub const MAC_OS_VERSION_15_1: u32 = 150100;
pub const MAC_OS_VERSION_15_2: u32 = 150200;
pub const MAC_OS_VERSION_15_3: u32 = 150300;
pub const MAC_OS_VERSION_15_4: u32 = 150400;
pub const MAC_OS_VERSION_15_5: u32 = 150500;
pub const MAC_OS_VERSION_15_6: u32 = 150600;
pub const MAC_OS_VERSION_16_0: u32 = 160000;
pub const MAC_OS_VERSION_26_0: u32 = 260000;
pub const MAC_OS_VERSION_26_1: u32 = 260100;
pub const MAC_OS_VERSION_26_2: u32 = 260200;
pub const MAC_OS_VERSION_26_3: u32 = 260300;
pub const MAC_OS_VERSION_26_4: u32 = 260400;
pub const MAC_OS_VERSION_26_5: u32 = 260500;
pub const MAC_OS_VERSION_26_6: u32 = 260600;
pub const MAC_OS_VERSION_27_0: u32 = 270000;
pub const __AVAILABILITY_VERSIONS_VERSION_HASH: u32 = 93585900;
pub const __AVAILABILITY_VERSIONS_VERSION_STRING: &[u8; 6] = b"Local\0";
pub const __AVAILABILITY_FILE: &[u8; 23] = b"AvailabilityVersions.h\0";
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _ARM_SIGNAL_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
pub const USE_CLANG_STDDEF: u32 = 0;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const SIGEV_KEVENT: u32 = 4;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_V5: u32 = 5;
pub const RUSAGE_INFO_V6: u32 = 6;
pub const RUSAGE_INFO_CURRENT: u32 = 6;
pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
pub const IOPOL_TYPE_VFS_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
pub const IOPOL_TYPE_VFS_DISALLOW_RW_FOR_O_EVTONLY: u32 = 10;
pub const IOPOL_TYPE_VFS_ENTITLED_RESERVE_ACCESS: u32 = 14;
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ORIG: u32 = 4;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_BASIC_MASK: u32 = 3;
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_STRICT: u32 = 2;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_IGNORE: u32 = 2;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_DEFAULT: u32 = 0;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_ON: u32 = 1;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_ON: u32 = 1;
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_OFF: u32 = 0;
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_ON: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub const OT_LOG_LEVEL_NONE: u32 = 0;
pub const OT_LOG_LEVEL_CRIT: u32 = 1;
pub const OT_LOG_LEVEL_WARN: u32 = 2;
pub const OT_LOG_LEVEL_NOTE: u32 = 3;
pub const OT_LOG_LEVEL_INFO: u32 = 4;
pub const OT_LOG_LEVEL_DEBG: u32 = 5;
pub const OPENTHREAD_API_VERSION: u32 = 420;
pub const OT_UPTIME_STRING_SIZE: u32 = 24;
pub const OT_CHANGED_IP6_ADDRESS_ADDED: u32 = 1;
pub const OT_CHANGED_IP6_ADDRESS_REMOVED: u32 = 2;
pub const OT_CHANGED_THREAD_ROLE: u32 = 4;
pub const OT_CHANGED_THREAD_LL_ADDR: u32 = 8;
pub const OT_CHANGED_THREAD_ML_ADDR: u32 = 16;
pub const OT_CHANGED_THREAD_RLOC_ADDED: u32 = 32;
pub const OT_CHANGED_THREAD_RLOC_REMOVED: u32 = 64;
pub const OT_CHANGED_THREAD_PARTITION_ID: u32 = 128;
pub const OT_CHANGED_THREAD_KEY_SEQUENCE_COUNTER: u32 = 256;
pub const OT_CHANGED_THREAD_NETDATA: u32 = 512;
pub const OT_CHANGED_THREAD_CHILD_ADDED: u32 = 1024;
pub const OT_CHANGED_THREAD_CHILD_REMOVED: u32 = 2048;
pub const OT_CHANGED_IP6_MULTICAST_SUBSCRIBED: u32 = 4096;
pub const OT_CHANGED_IP6_MULTICAST_UNSUBSCRIBED: u32 = 8192;
pub const OT_CHANGED_THREAD_CHANNEL: u32 = 16384;
pub const OT_CHANGED_THREAD_PANID: u32 = 32768;
pub const OT_CHANGED_THREAD_NETWORK_NAME: u32 = 65536;
pub const OT_CHANGED_THREAD_EXT_PANID: u32 = 131072;
pub const OT_CHANGED_NETWORK_KEY: u32 = 262144;
pub const OT_CHANGED_PSKC: u32 = 524288;
pub const OT_CHANGED_SECURITY_POLICY: u32 = 1048576;
pub const OT_CHANGED_CHANNEL_MANAGER_NEW_CHANNEL: u32 = 2097152;
pub const OT_CHANGED_SUPPORTED_CHANNEL_MASK: u32 = 4194304;
pub const OT_CHANGED_COMMISSIONER_STATE: u32 = 8388608;
pub const OT_CHANGED_THREAD_NETIF_STATE: u32 = 16777216;
pub const OT_CHANGED_THREAD_BACKBONE_ROUTER_STATE: u32 = 33554432;
pub const OT_CHANGED_THREAD_BACKBONE_ROUTER_LOCAL: u32 = 67108864;
pub const OT_CHANGED_JOINER_STATE: u32 = 134217728;
pub const OT_CHANGED_ACTIVE_DATASET: u32 = 268435456;
pub const OT_CHANGED_PENDING_DATASET: u32 = 536870912;
pub const OT_CHANGED_NAT64_TRANSLATOR_STATE: u32 = 1073741824;
pub const OT_CHANGED_PARENT_LINK_QUALITY: u32 = 2147483648;
pub const OT_CRYPTO_SHA256_HASH_SIZE: u32 = 32;
pub const OT_CRYPTO_ECDSA_MAX_DER_SIZE: u32 = 125;
pub const OT_CRYPTO_ECDSA_PUBLIC_KEY_SIZE: u32 = 64;
pub const OT_CRYPTO_ECDSA_SIGNATURE_SIZE: u32 = 64;
pub const OT_CRYPTO_PBDKF2_MAX_SALT_SIZE: u32 = 30;
pub const OT_PANID_BROADCAST: u32 = 65535;
pub const OT_EXT_ADDRESS_SIZE: u32 = 8;
pub const CSL_IE_HEADER_BYTES_LO: u32 = 4;
pub const CSL_IE_HEADER_BYTES_HI: u32 = 13;
pub const OT_MAC_KEY_SIZE: u32 = 16;
pub const ACK_FRAME_TYPE: u32 = 2;
pub const MAC_ACK_WAIT: u32 = 864;
pub const DEFAULT_PHY_RATE: u32 = 1;
pub const DEFAULT_PHY_TX_LOWLTNCY: u32 = 0;
pub const DEFAULT_PHY_RX_LOWLTNCY: u32 = 0;
pub const DEFAULT_TXPP_BYPASS_CRC: u32 = 0;
pub const DEFAULT_SCAN_BCN: u32 = 0;
pub const DEFAULT_DROP_ON_ERR: u32 = 1;
pub const DEFAULT_PANCOORD: u32 = 0;
pub const DEFAULT_MACIMPLICITBROADCAST: u32 = 0;
pub const DEFAULT_MACGRPRXMODE: u32 = 0;
pub const DEFAULT_MAC_PRMISCMOD: u32 = 0;
pub const DEFAULT_MAC_SFD_LENGTH: u32 = 1;
pub const DEFAULT_MAC_PREAMBLE_LENGTH: u32 = 4;
pub const DEFAULT_MAC_SFD_VALUE: u32 = 167;
pub const DEFAULT_MAC_PEAMBLE_VALUE: u32 = 0;
pub const DEFAULT_A_MAC_SFD_LENGTH_1M: u32 = 4;
pub const DEFAULT_A_MAC_PREAMBLE_LENGTH_1M: u32 = 1;
pub const DEFAULT_A_MAC_SFD_VALUE_1M: u32 = 1903575337;
pub const DEFAULT_A_MAC_PEAMBLE_VALUE_1M: u32 = 85;
pub const DEFAULT_A_MAC_SFD_LENGTH_2M: u32 = 4;
pub const DEFAULT_A_MAC_PREAMBLE_LENGTH_2M: u32 = 2;
pub const DEFAULT_A_MAC_SFD_VALUE_2M: u32 = 1903575337;
pub const DEFAULT_A_MAC_PEAMBLE_VALUE_2M: u32 = 21845;
pub const MAC_SFD_VALUE_SIZE: u32 = 32;
pub const MAC_PREAMBLE_VAL_SIZE: u32 = 32;
pub const MAC_PANID_SIZE: u32 = 16;
pub const MAC_SHORTADDR_SIZE: u32 = 16;
pub const EUI64ADD_LSW_SIZE: u32 = 32;
pub const EUI64ADD_MSW_SIZE: u32 = 32;
pub const MAC_EXTADDR_LSW_SIZE: u32 = 32;
pub const MAC_EXTADDR_MSW_SIZE: u32 = 32;
pub const ERROR_FLAGS_SIZE: u32 = 15;
pub const RX_FRAME_LEN_SIZE: u32 = 7;
pub const DEBUG_PORTS_SIZE: u32 = 5;
pub const POINTER_TO_CURRENT_TX_SIZE: u32 = 16;
pub const POINTER_TO_CURRENT_RX_SIZE: u32 = 16;
pub const FRMLNGTH_SIZE: u32 = 7;
pub const MAC_SFD_LEN_SIZE: u32 = 3;
pub const MAC_PREAMBLE_LEN_SIZE: u32 = 3;
pub const SEQNUM_SIZE: u32 = 8;
pub const TX_MAC_LATENCY_SIZE: u32 = 6;
pub const CONTROL_FLAGS_SIZE: u32 = 16;
pub const PHY_DRV_SEQ_STRT_ADDR_SIZE: u32 = 7;
pub const PHY_DRV_SEQ_END_ADDR_SIZE: u32 = 7;
pub const PHY_DRV_RSSI_VALUE_SIZE: u32 = 16;
pub const PHY_DRV_LQI_VALUE_SIZE: u32 = 8;
pub const AMAXSIFSFRAMESIZE: u32 = 18;
pub const MACMINSIFSPERIOD: u32 = 192;
pub const MACMINLIFSPERIOD: u32 = 640;
pub const MAX_PHY_PACKET_SIZE: u32 = 127;
pub const MAX_ERROR_PER_SINGLE_TX: u32 = 5;
pub const EVENT_PENDING: u32 = 1;
pub const HANDLE_IS_FULL: u32 = 77;
pub const NO_EVENT_PENDING: u32 = 0;
pub const EVENT_NOT_BUSY: u32 = 0;
pub const EVENT_BUSY: u32 = 1;
pub const PTA_HCI_TESTING: u32 = 0;
pub const ADDRESS_SIZE: u32 = 6;
pub const LE_FEATURES_BYTES_NO: u32 = 8;
pub const ISO_CODEC_ID_SIZE: u32 = 5;
pub const RX_DATA_OFFSET: u32 = 0;
pub const LINK_STATUS_SIZE: u32 = 2;
pub const LINK_STATUS_DEFAULT_HANDLE: u32 = 65535;
pub const CHANNEL_CLASSIFICATION_REPORTING_TIMING_PARAM_MIN: u32 = 5;
pub const CHANNEL_CLASSIFICATION_REPORTING_TIMING_PARAM_MAX: u32 = 150;
pub const MAX_IFS_VALUE: u32 = 10000;
pub const SUPPORTED_TIFS_TYPES: u32 = 31;
pub const MAXIMUM_SLP_CLK_ACCURACY: u32 = 1;
pub const MIN_BN: u32 = 1;
pub const UNKNOWN_HCI_COMMAND: u32 = 1;
pub const UNKNOWN_CONNECTION_IDENTIF: u32 = 2;
pub const HARDWARE_FAILURE: u32 = 3;
pub const PAGE_TIMEOUT: u32 = 4;
pub const AUTHENTICATION_FAILURE: u32 = 5;
pub const PIN_OR_KEY_MISSING: u32 = 6;
pub const MEMORY_CAPACITY_EXCEEDED: u32 = 7;
pub const CONNECTION_TIMEOUT: u32 = 8;
pub const CONNECTION_LIMIT_EXCEEDED: u32 = 9;
pub const SYNCHRONOUS_CONNECTION_LIMIT_TO_A_DEVICE_EXCEEDED: u32 = 10;
pub const ACL_CONNECTION_ALREADY_EXISTS: u32 = 11;
pub const COMMAND_DISALLOWED: u32 = 12;
pub const CONNECTION_REJECTED_DUE_TO_LIMITED_RESOURCES: u32 = 13;
pub const CONNECTION_REJECTED_DUE_TO_SECURITY_REASONS: u32 = 14;
pub const CONNECTION_REJECTED_DUE_TO_UNACCEPTABLE_BD_ADDR: u32 = 15;
pub const CONNECTION_ACCEPT_TIMEOUT_EXCEEDED: u32 = 16;
pub const UNSUPPORTED_FEATURE_OR_PARAMETER_VALUE: u32 = 17;
pub const INVALID_HCI_COMMAND_PARAMETERS: u32 = 18;
pub const REMOTE_USER_TERMINATED_CONNECTION: u32 = 19;
pub const REMOTE_DEVICE_TERMINATED_CONNECTION_DUE_TO_LOW_RESOURCES: u32 = 20;
pub const REMOTE_DEVICE_TERMINATED_CONNECTION_DUE_TO_POWER_OFF: u32 = 21;
pub const CONNECTION_TERMINATED_BY_LOCAL_HOST: u32 = 22;
pub const REPEATED_ATTEMPTS: u32 = 23;
pub const PAIRING_NOT_ALLOWED: u32 = 24;
pub const UNKNOWN_LMP_PDU: u32 = 25;
pub const UNSUPPORTED_REMOTE_FEATURE: u32 = 26;
pub const SCO_OFFSET_REJECTED: u32 = 27;
pub const SCO_INTERVAL_REJECTED: u32 = 28;
pub const SCO_AIR_MODE_REJECTED: u32 = 29;
pub const INVALID_LMP_PARAMETERS_INVALID_LL_PARAMETERS: u32 = 30;
pub const UNSPECIFIED_ERROR: u32 = 31;
pub const UNSUPPORTED_LMP_PARAMETER_VALUE_UNSUPPORTED_LL_PARAMETER_VALUE: u32 = 32;
pub const ROLE_CHANGE_NOT_ALLOWED: u32 = 33;
pub const LMP_RESPONSE_TIMEOUT_LL_RESPONSE_TIMEOUT: u32 = 34;
pub const LMP_ERROR_TRANSACTION_COLLISION: u32 = 35;
pub const LMP_PDU_NOT_ALLOWED: u32 = 36;
pub const ENCRYPTION_MODE_NOT_ACCEPTABLE: u32 = 37;
pub const LINK_KEY_CANNOT_BE_CHANGED: u32 = 38;
pub const REQUESTED_QOS_NOT_SUPPORTED: u32 = 39;
pub const INSTANT_PASSED: u32 = 40;
pub const PAIRING_WITH_UNIT_KEY_NOT_SUPPORTED: u32 = 41;
pub const DIFFERENT_TRANSACTION_COLLISION: u32 = 42;
pub const RESERVED_FOR_FUTURE_1: u32 = 43;
pub const QOS_UNACCEPTABLE_PARAMETER: u32 = 44;
pub const QOS_REJECTED: u32 = 45;
pub const CHANNEL_ASSESSMENT_NOT_SUPPORTED: u32 = 46;
pub const INSUFFICIENT_SECURITY: u32 = 47;
pub const PARAMETER_OUT_OF_MANDATORY_RANGE: u32 = 48;
pub const RESERVED_FOR_FUTURE_2: u32 = 49;
pub const ROLE_SWITCH_PENDING: u32 = 50;
pub const RESERVED_FOR_FUTURE_3: u32 = 51;
pub const RESERVED_SLOT_VIOLATION: u32 = 52;
pub const ROLE_SWITCH_FAILED: u32 = 53;
pub const EXTENDED_INQUIRY_RESPONSE_TOO_LARGE: u32 = 54;
pub const SECURE_SIMPLE_PAIRING_NOT_SUPPORTED_BY_HOST: u32 = 55;
pub const HOST_BUSY_PAIRING: u32 = 56;
pub const CONNECTION_REJECTED_DUE_TO_NO_SUITABLE_CHANNEL_FOUND: u32 = 57;
pub const CONTROLLER_BUSY: u32 = 58;
pub const UNACCEPTABLE_CONNECTION_PARAMETERS: u32 = 59;
pub const DIRECTED_ADVERTISING_TIMEOUT: u32 = 60;
pub const CONNECTION_TERMINATED_DUE_TO_MIC_FAILURE: u32 = 61;
pub const CONNECTION_FAILED_TO_BE_ESTABLISHED: u32 = 62;
pub const SYNCHRONIZATION_TIMEOUT: u32 = 62;
pub const MAC_CONNECTION_FAILED: u32 = 63;
pub const COARSE_CLOCK_ADJUSTMENT_REJECTED_BUT_WILL_TRY_TO_ADJUST_USING_CLOCK_DRAGGING: u32 = 64;
pub const UNKNOWN_ADVERTISING_IDENTIFIER: u32 = 66;
pub const LIMIT_REACHED: u32 = 67;
pub const OPERATION_CANCELLED_BY_HOST: u32 = 68;
pub const PACKET_TOO_LONG: u32 = 69;
pub const TOO_LATE: u32 = 70;
pub const INSUFFICIENT_CHANNELS: u32 = 72;
pub const LL_SYS_BRIEF_VERSION_MAJOR: u32 = 1;
pub const LL_SYS_BRIEF_VERSION_MINOR: u32 = 1;
pub const LL_SYS_BRIEF_VERSION_PATCH: u32 = 0;
pub const LL_SYS_BRIEF_VERSION_MAJOR_MASK: u32 = 192;
pub const LL_SYS_BRIEF_VERSION_MAJOR_POS: u32 = 6;
pub const LL_SYS_BRIEF_VERSION_MINOR_MASK: u32 = 60;
pub const LL_SYS_BRIEF_VERSION_MINOR_POS: u32 = 2;
pub const LL_SYS_BRIEF_VERSION_PATCH_MASK: u32 = 3;
pub const LL_SYS_BRIEF_VERSION_PATCH_POS: u32 = 0;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::core::ffi::c_schar;
pub type __uint8_t = ::core::ffi::c_uchar;
pub type __int16_t = ::core::ffi::c_short;
pub type __uint16_t = ::core::ffi::c_ushort;
pub type __int32_t = ::core::ffi::c_int;
pub type __uint32_t = ::core::ffi::c_uint;
pub type __int64_t = ::core::ffi::c_longlong;
pub type __uint64_t = ::core::ffi::c_ulonglong;
pub type __darwin_intptr_t = ::core::ffi::c_long;
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::core::ffi::c_char; 128usize],
    pub _mbstateL: ::core::ffi::c_longlong,
}
impl Default for __mbstate_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::core::ffi::c_int;
pub type __darwin_size_t = ::core::ffi::c_uint;
pub type __darwin_va_list = u32;
pub type __darwin_wchar_t = ::core::ffi::c_uint;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::core::ffi::c_int;
pub type __darwin_clock_t = ::core::ffi::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::core::ffi::c_long;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::core::ffi::c_uint;
pub type __darwin_fsfilcnt_t = ::core::ffi::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::core::ffi::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::core::ffi::c_char; 37usize];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
impl Default for __darwin_pthread_handler_rec {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 36usize],
}
impl Default for _opaque_pthread_attr_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 24usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 40usize],
}
impl Default for _opaque_pthread_mutex_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 124usize],
}
impl Default for _opaque_pthread_rwlock_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 12usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 4088usize],
}
impl Default for _opaque_pthread_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::core::ffi::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type intmax_t = ::core::ffi::c_longlong;
pub type uintmax_t = ::core::ffi::c_ulonglong;
pub const InterruptPriorities_INT_PRIO_HIGHEST: InterruptPriorities = 0;
pub const InterruptPriorities_INT_PRIO_HIGH: InterruptPriorities = 1;
pub const InterruptPriorities_INT_PRIO_LOW: InterruptPriorities = 2;
pub const InterruptPriorities_INT_PRIO_LOWEST: InterruptPriorities = 3;
pub const InterruptPriorities_INT_PRIO_MAX: InterruptPriorities = 4;
#[doc = " @brief InterruptPriorities Enum.\n it is used to define the different ISR priorities in the controller"]
pub type InterruptPriorities = ::core::ffi::c_uint;
pub const EvntNotiState_EVNT_START: EvntNotiState = 0;
pub const EvntNotiState_EVNT_END: EvntNotiState = 1;
pub const EvntNotiState_EVNT_NOT_SPECIFIED: EvntNotiState = 2;
#[doc = " @brief Event notification state  Enum.\n it is used to the state of radio activity ,being started or ended"]
pub type EvntNotiState = ::core::ffi::c_uint;
pub const _PhyClbrState_PHY_CLBR_NOT_RUNNING: _PhyClbrState = 0;
pub const _PhyClbrState_PHY_CLBR_PNDING_OR_RUNNING: _PhyClbrState = 1;
pub const _PhyClbrState_PHY_CLBR_NOT_KNOWN: _PhyClbrState = 2;
#[doc = " @brief periodic calibration state  Enum.\n it is used to the state of Periodic calibration"]
pub type _PhyClbrState = ::core::ffi::c_uint;
#[doc = " @brief periodic calibration state  Enum.\n it is used to the state of Periodic calibration"]
pub use self::_PhyClbrState as PhyClbrState;
#[doc = " @brief link layer bus structure.\n it defines the callback functions that will be be called by the transport bus driver\n after the requested operation is done."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ble_ll_bus {
    #[doc = " Bus read callback that will be called after the requested number of bytes is read from the bus"]
    pub read: ::core::option::Option<unsafe extern "C" fn(buffer: *mut u8)>,
    #[doc = " Bus Write callback that will be called after the requested is written to the bus"]
    pub write: ::core::option::Option<unsafe extern "C" fn(buffer: *mut u8)>,
}
#[doc = " @brief link layer bus structure.\n it defines the callback functions that will be be called by the transport bus driver\n after the requested operation is done."]
pub type ble_ll_bus = _ble_ll_bus;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Evnt_timing_s {
    pub drift_time: u32,
    pub exec_time: u32,
    pub schdling_time: u32,
}
pub type Evnt_timing_t = Evnt_timing_s;
pub const _profiling_state_e_PROFILE_STATE_START: _profiling_state_e = 0;
pub const _profiling_state_e_PROFILE_STATE_CLEAR: _profiling_state_e = 1;
pub const _profiling_state_e_PROFILE_STATE_END: _profiling_state_e = 2;
pub type _profiling_state_e = ::core::ffi::c_uint;
pub use self::_profiling_state_e as profiling_state_e;
pub const Debug_GPIO_e_DBG_IO_HCI_READ_DONE: Debug_GPIO_e = 0;
pub const Debug_GPIO_e_DBG_IO_HCI_RCVD_CMD: Debug_GPIO_e = 1;
pub const Debug_GPIO_e_DBG_IO_HCI_WRITE_DONE: Debug_GPIO_e = 2;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EVNT_UPDATE: Debug_GPIO_e = 3;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_TIMER_SET: Debug_GPIO_e = 4;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_PHY_CLBR_TIMER: Debug_GPIO_e = 5;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EVNT_SKIPPED: Debug_GPIO_e = 6;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_HNDL_NXT_TRACE: Debug_GPIO_e = 7;
pub const Debug_GPIO_e_DBG_IO_ACTIVE_SCHDLR_NEAR_DETEDTED: Debug_GPIO_e = 8;
pub const Debug_GPIO_e_DBG_IO_ACTIVE_SCHDLR_NEAR_GAP_CHECK: Debug_GPIO_e = 9;
pub const Debug_GPIO_e_DBG_IO_ACTIVE_SCHDLR_NEAR_TIME_CHECK: Debug_GPIO_e = 10;
pub const Debug_GPIO_e_DBG_IO_ACTIVE_SCHDLR_NEAR_TRACE: Debug_GPIO_e = 11;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EVNT_RGSTR: Debug_GPIO_e = 12;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_ADD_CONFLICT_Q: Debug_GPIO_e = 13;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_HNDL_MISSED_EVNT: Debug_GPIO_e = 14;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_UNRGSTR_EVNT: Debug_GPIO_e = 15;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EXEC_EVNT_TRACE: Debug_GPIO_e = 16;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EXEC_EVNT_PROFILE: Debug_GPIO_e = 17;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EXEC_EVNT_ERROR: Debug_GPIO_e = 18;
pub const Debug_GPIO_e_DBG_IO_SCHDLR_EXEC_EVNT_WINDOW_WIDENING: Debug_GPIO_e = 19;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CMN_CLR_ISR: Debug_GPIO_e = 20;
pub const Debug_GPIO_e_DBG_IO_LLWCC_CMN_HG_ISR: Debug_GPIO_e = 21;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CMN_LW_ISR: Debug_GPIO_e = 22;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CMN_CLR_TIMER_ERROR: Debug_GPIO_e = 23;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CMN_CLR_ACTTMR_ERROR: Debug_GPIO_e = 24;
pub const Debug_GPIO_e_DBG_IO_LLHWC_LL_ISR: Debug_GPIO_e = 25;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SPLTMR_SET: Debug_GPIO_e = 26;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SPLTMR_GET: Debug_GPIO_e = 27;
pub const Debug_GPIO_e_DBG_IO_LLHWC_LOW_ISR: Debug_GPIO_e = 28;
pub const Debug_GPIO_e_DBG_IO_LLHWC_STOP_SCN: Debug_GPIO_e = 29;
pub const Debug_GPIO_e_DBG_IO_LLHWC_WAIT_ENVT_ON_AIR: Debug_GPIO_e = 30;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SET_CONN_EVNT_PARAM: Debug_GPIO_e = 31;
pub const Debug_GPIO_e_DBG_IO_POST_EVNT: Debug_GPIO_e = 32;
pub const Debug_GPIO_e_DBG_IO_HNDL_ALL_EVNTS: Debug_GPIO_e = 33;
pub const Debug_GPIO_e_DBG_IO_PROCESS_EVNT: Debug_GPIO_e = 34;
pub const Debug_GPIO_e_DBG_IO_PROCESS_ISO_DATA: Debug_GPIO_e = 35;
pub const Debug_GPIO_e_DBG_IO_ALLOC_TX_ISO_EMPTY_PKT: Debug_GPIO_e = 36;
pub const Debug_GPIO_e_DBG_IO_BIG_FREE_EMPTY_PKTS: Debug_GPIO_e = 37;
pub const Debug_GPIO_e_DBG_IO_RECOMBINE_UNFRMD_DATA_OK: Debug_GPIO_e = 38;
pub const Debug_GPIO_e_DBG_IO_RECOMBINE_UNFRMD_DATA_CRC: Debug_GPIO_e = 39;
pub const Debug_GPIO_e_DBG_IO_RECOMBINE_UNFRMD_DATA_NoRX: Debug_GPIO_e = 40;
pub const Debug_GPIO_e_DBG_IO_RECOMBINE_UNFRMD_DATA_TRACE: Debug_GPIO_e = 41;
pub const Debug_GPIO_e_DBG_IO_ISO_HNDL_SDU: Debug_GPIO_e = 42;
pub const Debug_GPIO_e_DBG_IO_LL_INTF_INIT: Debug_GPIO_e = 43;
pub const Debug_GPIO_e_DBG_IO_DATA_TO_CNTRLR: Debug_GPIO_e = 44;
pub const Debug_GPIO_e_DBG_IO_FREE_LL_PKT_HNDLR: Debug_GPIO_e = 45;
pub const Debug_GPIO_e_DBG_IO_PHY_INIT_CLBR_TRACE: Debug_GPIO_e = 46;
pub const Debug_GPIO_e_DBG_IO_PHY_RUNTIME_CLBR_TRACE: Debug_GPIO_e = 47;
pub const Debug_GPIO_e_DBG_IO_PHY_CLBR_ISR: Debug_GPIO_e = 48;
pub const Debug_GPIO_e_DBG_IO_PHY_INIT_CLBR_SINGLE_CH: Debug_GPIO_e = 49;
pub const Debug_GPIO_e_DBG_IO_PHY_CLBR_STRTD: Debug_GPIO_e = 50;
pub const Debug_GPIO_e_DBG_IO_PHY_CLBR_EXEC: Debug_GPIO_e = 51;
pub const Debug_GPIO_e_DBG_IO_RCO_STRT_STOP_RUNTIME_CLBR_ACTV: Debug_GPIO_e = 52;
pub const Debug_GPIO_e_DBG_IO_RCO_STRT_STOP_RUNTIME_RCO_CLBR: Debug_GPIO_e = 53;
pub const Debug_GPIO_e_DBG_IO_STRT_STOP_RUNTIME_RCO_CLBR_SWT: Debug_GPIO_e = 54;
pub const Debug_GPIO_e_DBG_IO_STRT_STOP_RUNTIME_RCO_CLBR_TRACE: Debug_GPIO_e = 55;
pub const Debug_GPIO_e_DBG_IO_RCO_ISR_TRACE: Debug_GPIO_e = 56;
pub const Debug_GPIO_e_DBG_IO_RCO_ISR_COMPENDATE: Debug_GPIO_e = 57;
pub const Debug_GPIO_e_DBG_IO_RAL_STRT_TX: Debug_GPIO_e = 58;
pub const Debug_GPIO_e_DBG_IO_RAL_ISR_TIMER_ERROR: Debug_GPIO_e = 59;
pub const Debug_GPIO_e_DBG_IO_RAL_ISR_TRACE: Debug_GPIO_e = 60;
pub const Debug_GPIO_e_DBG_IO_RAL_STOP_OPRTN: Debug_GPIO_e = 61;
pub const Debug_GPIO_e_DBG_IO_RAL_STRT_RX: Debug_GPIO_e = 62;
pub const Debug_GPIO_e_DBG_IO_RAL_DONE_CLBK_TX: Debug_GPIO_e = 63;
pub const Debug_GPIO_e_DBG_IO_RAL_DONE_CLBK_RX: Debug_GPIO_e = 64;
pub const Debug_GPIO_e_DBG_IO_RAL_DONE_CLBK_ED: Debug_GPIO_e = 65;
pub const Debug_GPIO_e_DBG_IO_RAL_ED_SCAN: Debug_GPIO_e = 66;
pub const Debug_GPIO_e_DBG_IO_ERROR_MEM_CAP_EXCED: Debug_GPIO_e = 67;
pub const Debug_GPIO_e_DBG_IO_ERROR_COMMAND_DISALLOWED: Debug_GPIO_e = 68;
pub const Debug_GPIO_e_DBG_IO_PTA_INIT: Debug_GPIO_e = 69;
pub const Debug_GPIO_e_DBG_IO_PTA_EN: Debug_GPIO_e = 70;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PTA_SET_EN: Debug_GPIO_e = 71;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PTA_SET_PARAMS: Debug_GPIO_e = 72;
pub const Debug_GPIO_e_DBG_IO_COEX_STRT_ON_IDLE: Debug_GPIO_e = 73;
pub const Debug_GPIO_e_DBG_IO_COEX_ASK_FOR_AIR: Debug_GPIO_e = 74;
pub const Debug_GPIO_e_DBG_IO_COEX_TIMER_EVNT_CLBK: Debug_GPIO_e = 75;
pub const Debug_GPIO_e_DBG_IO_COEX_STRT_ONE_SHOT: Debug_GPIO_e = 76;
pub const Debug_GPIO_e_DBG_IO_COEX_FORCE_STOP_RX: Debug_GPIO_e = 77;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ADV_DONE: Debug_GPIO_e = 78;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SCN_DONE: Debug_GPIO_e = 79;
pub const Debug_GPIO_e_DBG_IO_LLHWC_INIT_DONE: Debug_GPIO_e = 80;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CONN_DONE: Debug_GPIO_e = 81;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CIG_DONE: Debug_GPIO_e = 82;
pub const Debug_GPIO_e_DBG_IO_LLHWC_BIG_DONE: Debug_GPIO_e = 83;
pub const Debug_GPIO_e_DBG_IO_OS_TMR_CREATE: Debug_GPIO_e = 84;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_TIMEOUT_CBK: Debug_GPIO_e = 85;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_SCN_DUR_CBK: Debug_GPIO_e = 86;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_SCN_PERIOD_CBK: Debug_GPIO_e = 87;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_PRDC_SCN_TIMEOUT_CBK: Debug_GPIO_e = 88;
pub const Debug_GPIO_e_DBG_IO_BIS_SYNC_TIMEOUT_TMR_CBK: Debug_GPIO_e = 89;
pub const Debug_GPIO_e_DBG_IO_BIS_TERM_TMR_CBK: Debug_GPIO_e = 90;
pub const Debug_GPIO_e_DBG_IO_BIS_TST_MODE_CBK: Debug_GPIO_e = 91;
pub const Debug_GPIO_e_DBG_IO_BIS_TST_MODE_TMR_CBK: Debug_GPIO_e = 92;
pub const Debug_GPIO_e_DBG_IO_ISO_POST_TMR_CBK: Debug_GPIO_e = 93;
pub const Debug_GPIO_e_DBG_IO_ISO_TST_MODE_TMR_CBK: Debug_GPIO_e = 94;
pub const Debug_GPIO_e_DBG_IO_CONN_POST_TMR_CBK: Debug_GPIO_e = 95;
pub const Debug_GPIO_e_DBG_IO_EVNT_SCHDLR_TMR_CBK: Debug_GPIO_e = 96;
pub const Debug_GPIO_e_DBG_IO_HCI_POST_TMR_CBK: Debug_GPIO_e = 97;
pub const Debug_GPIO_e_DBG_IO_LLCP_POST_TMR_CBK: Debug_GPIO_e = 98;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ENRGY_DETECT_CBK: Debug_GPIO_e = 99;
pub const Debug_GPIO_e_DBG_IO_PRVCY_POST_TMR_CBK: Debug_GPIO_e = 100;
pub const Debug_GPIO_e_DBG_IO_ANT_PRPR_TMR_CBK: Debug_GPIO_e = 101;
pub const Debug_GPIO_e_DBG_IO_COEX_TMR_FRC_STOP_AIR_GRANT_CBK: Debug_GPIO_e = 102;
pub const Debug_GPIO_e_DBG_IO_MLME_RX_EN_TMR_CBK: Debug_GPIO_e = 103;
pub const Debug_GPIO_e_DBG_IO_MLME_GNRC_TMR_CBK: Debug_GPIO_e = 104;
pub const Debug_GPIO_e_DBG_IO_MIB_JOIN_LST_TMR_CBK: Debug_GPIO_e = 105;
pub const Debug_GPIO_e_DBG_IO_MLME_PWR_PRES_TMR_CBK: Debug_GPIO_e = 106;
pub const Debug_GPIO_e_DBG_IO_PRESISTENCE_TMR_CBK: Debug_GPIO_e = 107;
pub const Debug_GPIO_e_DBG_IO_RADIO_PHY_PRDC_CLBK_TMR_CBK: Debug_GPIO_e = 108;
pub const Debug_GPIO_e_DBG_IO_RADIO_CSMA_TMR_CBK: Debug_GPIO_e = 109;
pub const Debug_GPIO_e_DBG_IO_RADIO_CSL_RCV_TMR_CBK: Debug_GPIO_e = 110;
pub const Debug_GPIO_e_DBG_IO_ED_TMR_CBK: Debug_GPIO_e = 111;
pub const Debug_GPIO_e_DBG_IO_DIO_EXT_TMR_CBK: Debug_GPIO_e = 112;
pub const Debug_GPIO_e_DBG_IO_RCO_CLBR_TMR_CBK: Debug_GPIO_e = 113;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_ADV_CBK: Debug_GPIO_e = 114;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_SCN_CBK: Debug_GPIO_e = 115;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_SCN_ERR_CBK: Debug_GPIO_e = 116;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PRDC_SCN_CBK: Debug_GPIO_e = 117;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PRDC_SCN_ERR_CBK: Debug_GPIO_e = 118;
pub const Debug_GPIO_e_DBG_IO_BIG_ADV_CBK: Debug_GPIO_e = 119;
pub const Debug_GPIO_e_DBG_IO_BIG_ADV_ERR_CBK: Debug_GPIO_e = 120;
pub const Debug_GPIO_e_DBG_IO_BIG_SYNC_CBK: Debug_GPIO_e = 121;
pub const Debug_GPIO_e_DBG_IO_BIG_SYNC_ERR_CBK: Debug_GPIO_e = 122;
pub const Debug_GPIO_e_DBG_IO_ISO_CIS_PKT_TRNSM_RECEIVED_CBK: Debug_GPIO_e = 123;
pub const Debug_GPIO_e_DBG_IO_ISO_CIG_ERR_CBK: Debug_GPIO_e = 124;
pub const Debug_GPIO_e_DBG_IO_CONN_PKT_TRNSM_RECEIVED_CBK: Debug_GPIO_e = 125;
pub const Debug_GPIO_e_DBG_IO_PRDC_CLBR_EXTRL_CBK: Debug_GPIO_e = 126;
pub const Debug_GPIO_e_DBG_IO_PTR_PRDC_ADV_SYNC_CBK: Debug_GPIO_e = 127;
pub const Debug_GPIO_e_DBG_IO_NCONN_SCN_CBK: Debug_GPIO_e = 128;
pub const Debug_GPIO_e_DBG_IO_NCONN_ADV_CBK: Debug_GPIO_e = 129;
pub const Debug_GPIO_e_DBG_IO_NCONN_INIT_CBK: Debug_GPIO_e = 130;
pub const Debug_GPIO_e_DBG_IO_ANT_RADIO_CMPLT_EVNT_CBK: Debug_GPIO_e = 131;
pub const Debug_GPIO_e_DBG_IO_ANT_STACK_EVNT_CBK: Debug_GPIO_e = 132;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_PROCESS_TMOUT_EVNT_CBK: Debug_GPIO_e = 133;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_SCN_DUR_EVNT: Debug_GPIO_e = 134;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_SCN_PERIODIC_EVNT: Debug_GPIO_e = 135;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PRDC_SCN_TMOUT_EVNT: Debug_GPIO_e = 136;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PRDC_SCN_CNCEL_EVNT: Debug_GPIO_e = 137;
pub const Debug_GPIO_e_DBG_IO_BIS_MNGR_BIG_TERM_CBK: Debug_GPIO_e = 138;
pub const Debug_GPIO_e_DBG_IO_BIS_MNGR_SYNC_TMOUT_CBK: Debug_GPIO_e = 139;
pub const Debug_GPIO_e_DBG_IO_ISOAL_MNGR_SDU_GEN: Debug_GPIO_e = 140;
pub const Debug_GPIO_e_DBG_IO_ISO_MNGR_CIS_PROCESS_EVNT_CBK: Debug_GPIO_e = 141;
pub const Debug_GPIO_e_DBG_IO_CONN_MNGR_PROCESS_EVNT_CLBK: Debug_GPIO_e = 142;
pub const Debug_GPIO_e_DBG_IO_CONN_MNGR_UPDT_CONN_PARAM_CBK: Debug_GPIO_e = 143;
pub const Debug_GPIO_e_DBG_IO_CONN_MNGR_DATA_LEN_UPDT_CBK: Debug_GPIO_e = 144;
pub const Debug_GPIO_e_DBG_IO_EVNT_SCHDLR_HW_EVNT_CMPLT: Debug_GPIO_e = 145;
pub const Debug_GPIO_e_DBG_IO_HCI_EVENT_HNDLR: Debug_GPIO_e = 146;
pub const Debug_GPIO_e_DBG_IO_MLME_TMRS_CBK: Debug_GPIO_e = 147;
pub const Debug_GPIO_e_DBG_IO_DIRECT_TX_EVNT_CBK: Debug_GPIO_e = 148;
pub const Debug_GPIO_e_DBG_IO_INDIRECT_PKT_TOUR_CBK: Debug_GPIO_e = 149;
pub const Debug_GPIO_e_DBG_IO_RADIO_CSMA_TMR: Debug_GPIO_e = 150;
pub const Debug_GPIO_e_DBG_IO_RAL_SM_DONE_EVNT_CBK: Debug_GPIO_e = 151;
pub const Debug_GPIO_e_DBG_IO_ED_TMR_HNDL: Debug_GPIO_e = 152;
pub const Debug_GPIO_e_DBG_IO_OS_TMR_EVNT_CBK: Debug_GPIO_e = 153;
pub const Debug_GPIO_e_DBG_IO_PROFILE_MARKER_PHY_WAKEUP_TIME: Debug_GPIO_e = 154;
pub const Debug_GPIO_e_DBG_IO_PROFILE_MARKER_BLOCKING_PHY_WAKEUP_TIME: Debug_GPIO_e = 155;
pub const Debug_GPIO_e_DBG_IO_PROFILE_END_DRIFT_TIME: Debug_GPIO_e = 156;
pub const Debug_GPIO_e_DBG_IO_PROC_RADIO_RCV: Debug_GPIO_e = 157;
pub const Debug_GPIO_e_DBG_IO_EVNT_TIME_UPDT: Debug_GPIO_e = 158;
pub const Debug_GPIO_e_DBG_IO_MAC_RECEIVE_DONE: Debug_GPIO_e = 159;
pub const Debug_GPIO_e_DBG_IO_MAC_TX_DONE: Debug_GPIO_e = 160;
pub const Debug_GPIO_e_DBG_IO_RADIO_APPLY_CSMA: Debug_GPIO_e = 161;
pub const Debug_GPIO_e_DBG_IO_RADIO_TRANSMIT: Debug_GPIO_e = 162;
pub const Debug_GPIO_e_DBG_IO_PROC_RADIO_TX: Debug_GPIO_e = 163;
pub const Debug_GPIO_e_DBG_IO_RAL_TX_DONE: Debug_GPIO_e = 164;
pub const Debug_GPIO_e_DBG_IO_RAL_TX_DONE_INCREMENT_BACKOFF_COUNT: Debug_GPIO_e = 165;
pub const Debug_GPIO_e_DBG_IO_RAL_TX_DONE_RST_BACKOFF_COUNT: Debug_GPIO_e = 166;
pub const Debug_GPIO_e_DBG_IO_RAL_CONTINUE_RX: Debug_GPIO_e = 167;
pub const Debug_GPIO_e_DBG_IO_RAL_PERFORM_CCA: Debug_GPIO_e = 168;
pub const Debug_GPIO_e_DBG_IO_RAL_ENABLE_TRANSMITTER: Debug_GPIO_e = 169;
pub const Debug_GPIO_e_DBG_IO_LLHWC_GET_CH_IDX_ALGO_2: Debug_GPIO_e = 170;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PAWR_ADV_SE_CBK: Debug_GPIO_e = 171;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PAWR_ADV_SE_ERR_CBK: Debug_GPIO_e = 172;
pub const Debug_GPIO_e_DBG_IO_ADV_EXT_MNGR_PAWR_SCN_ERR_CBK: Debug_GPIO_e = 173;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SET_ADV_PAWR_SE_PARAM: Debug_GPIO_e = 174;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ADV_PAWR_SE_DONE: Debug_GPIO_e = 175;
pub const Debug_GPIO_e_DBG_IO_LLHWC_SET_PAWR_RSP_PARAM: Debug_GPIO_e = 176;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ADV_PAWR_RSP_DONE: Debug_GPIO_e = 177;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ADV_PAWR_RSP_DONE_RCV_PCKT: Debug_GPIO_e = 178;
pub const Debug_GPIO_e_DBG_IO_LLHWC_ADV_PAWR_RSP_DONE_FREE_PCKT: Debug_GPIO_e = 179;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_PING_PONG_HNDL: Debug_GPIO_e = 180;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_PING_PONG_RCV_PCKT: Debug_GPIO_e = 181;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_ADV_STOP_RSPS: Debug_GPIO_e = 182;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_SYNC_SET_PARAM: Debug_GPIO_e = 183;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_SYNC_DONE: Debug_GPIO_e = 184;
pub const Debug_GPIO_e_DBG_IO_LLHWC_PAWR_SYNC_SEND_RSP: Debug_GPIO_e = 185;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_SE_MISS_RSP: Debug_GPIO_e = 186;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_FORCE_RP: Debug_GPIO_e = 187;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_PUSH_STRT_TIM_FORW: Debug_GPIO_e = 188;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_RSP_NEAR: Debug_GPIO_e = 189;
pub const Debug_GPIO_e_DBG_IO_EVNT_STRT_TIM_PUSHED: Debug_GPIO_e = 190;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_RSP_SWITCH_SE: Debug_GPIO_e = 191;
pub const Debug_GPIO_e_DBG_IO_PAWR_ADV_QUEUE_WIN_UPDT: Debug_GPIO_e = 192;
pub const Debug_GPIO_e_DBG_IO_PAWR_SYNC_REFUSE_INST_RSP: Debug_GPIO_e = 193;
pub const Debug_GPIO_e_DBG_IO_PAWR_SYNC_ABOUT_TIMEOUT: Debug_GPIO_e = 194;
pub const Debug_GPIO_e_DBG_IO_PAWR_SYNC_INST_RSP_TOO_LATE: Debug_GPIO_e = 195;
pub const Debug_GPIO_e_DBG_IO_PAWR_SYNC_EXEC_SKIPPED: Debug_GPIO_e = 196;
pub const Debug_GPIO_e_DBG_IO_NULL_PKT_STATUS: Debug_GPIO_e = 197;
pub const Debug_GPIO_e_DBG_IO_PAWR_MULTIPLE_EVNTS_MISSED: Debug_GPIO_e = 198;
pub const Debug_GPIO_e_DBG_IO_PAWR_CHM_UPDT_END: Debug_GPIO_e = 199;
pub const Debug_GPIO_e_DBG_IO_LLHWC_CMN_INIT: Debug_GPIO_e = 200;
pub const Debug_GPIO_e_DBG_IO_RADIO_SET_PENDING_TX_FULL: Debug_GPIO_e = 201;
pub const Debug_GPIO_e_DBG_IO_RADIO_SET_PENDING_TX_CONTINUE: Debug_GPIO_e = 202;
pub const Debug_GPIO_e_DBG_IO_RADIO_HANDLE_PENDING_TX: Debug_GPIO_e = 203;
pub const Debug_GPIO_e_DBG_IO_RAL_AD_SET_MEASUREMENT_STATE: Debug_GPIO_e = 204;
pub const Debug_GPIO_e_DBG_IO_PROFILE_CS_GEN: Debug_GPIO_e = 205;
pub const Debug_GPIO_e_DBG_IO_PROFILE_CS_CHNL_SHUFFLING: Debug_GPIO_e = 206;
pub const Debug_GPIO_e_DBG_IO_SET_DEEP_SLEEP_MODE: Debug_GPIO_e = 207;
pub const Debug_GPIO_e_DBG_IO_BACK_FROM_DEEP_SLEEP: Debug_GPIO_e = 208;
pub const Debug_GPIO_e_DBG_IO_CS_EVENT_MISSED: Debug_GPIO_e = 209;
pub const Debug_GPIO_e_DBG_IO_CS_EVENT_CONFLICTING: Debug_GPIO_e = 210;
pub const Debug_GPIO_e_DBG_IO_CS_GENERATOR_IS_LATE: Debug_GPIO_e = 211;
pub const Debug_GPIO_e_DBG_IO_CS_SCHDLR_OUT_OF_BOUND: Debug_GPIO_e = 212;
pub const Debug_GPIO_e_DBG_IO_CS_EXECUTION_FAILED: Debug_GPIO_e = 213;
pub const Debug_GPIO_e_DBG_IO_CS_OFFSET_FAILED: Debug_GPIO_e = 214;
pub const Debug_GPIO_e_DBG_IO_CS_STRT_TIM_FRM_ACL: Debug_GPIO_e = 215;
pub const Debug_GPIO_e_DBG_IO_RAL_TX_ACK: Debug_GPIO_e = 216;
pub const Debug_GPIO_e_DBG_IO_RAL_HANDLE_PHY_ISR: Debug_GPIO_e = 217;
pub const Debug_GPIO_e_Debug_GPIO_num: Debug_GPIO_e = 218;
#[doc = " @brief enum holding all debugging gpio\n\n\n"]
pub type Debug_GPIO_e = ::core::ffi::c_uint;
#[doc = " @brief enum holding all debugging gpio\n\n\n"]
pub use self::Debug_GPIO_e as Debug_GPIO_t;
unsafe extern "C" {
    #[doc = "  @ingroup BSP_APIS\n  @{\n/\n/**\n @brief   Bus initialization Function\n\n \tthis function is used to initialize the used transport bus and link functions is @ref _ble_ll_bus to the ISRs of bus driver\n @param  op[in]   : pointer to @ref _ble_ll_bus structure that stores the bus callback functions\n @retval None\n"]
    pub fn bus_init(op: *mut ble_ll_bus);
}
unsafe extern "C" {
    #[doc = " @brief   Bus read Function\n This function is used read @ref size bytes from the Bus in the given @ref buffer\n @note this function is asynchronous, it is expected that Bus read callback function is called to indicate that the requested  data is available\n the read callback function will be set by calling @ref bus_init\n @param   *buffer [in]: pointer to the buffer of the data\n @param   size [in]: size of bytes read from the buffer\n\n @retval None"]
    pub fn bus_read(buffer: *mut u8, size: u16);
}
unsafe extern "C" {
    #[doc = " @brief   Bus write Function\n This function is used read @ref size bytes from the Bus in the given @ref buffer\n @note this function is asynchronous, it is expected that Bus will call read callback function when wrriting data is done\n the Write callback function will be set by calling @ref bus_init\n @param   *buffer[in]: pointer to the buffer of the data\n @param   size[in]: size of bytes written to the buffer\n\n @retval None"]
    pub fn bus_write(buffer: *mut u8, size: u16);
}
unsafe extern "C" {
    #[doc = " @brief   logger port initialization\n\nthis function is used to initialize the logger\n @param  None\n @retval None\n"]
    pub fn logger_init();
}
unsafe extern "C" {
    #[doc = " @brief   logger write\n Thin function is used to log the data described by the input parameters\n @param   *buffer: pointer to the buffer of the data to be written to logger\n @param   size: size of bytes to be logged from the buffer\n\n @retval None"]
    pub fn logger_write(buffer: *mut u8, size: u32);
}
unsafe extern "C" {
    #[doc = " @brief   enable interrupt request function\n This function enable the MCU interrupt ,after calling this function the LL code can be interrupted by the controller\n @param   None\n\n @retval None"]
    pub fn enable_irq();
}
unsafe extern "C" {
    #[doc = " @brief   disable interrupt request function\n This function disable the MCU interrupt ,after calling this function the LL code must not be interrupted as it is in critical section\n @param   None\n\n @retval None"]
    pub fn disable_irq();
}
unsafe extern "C" {
    #[doc = " @brief this function is used to enable a specific ISR\n @param[in]  isr_type that holds specific ISR to be enabled by this function\n \t\t\t\tBIT[0] for LL_HIGH_ISR\n \t\t\t\tBIT[1] for LL_LOW_ISR\n \t\t\t\tBIT[2] for SYS_LOW_ISR\n @return None"]
    pub fn enable_specific_irq(isr_type: u8);
}
unsafe extern "C" {
    #[doc = " @brief this function is used to disable a specific ISR\n @param[in]  isr_type that holds specific ISR to be disabled by this function\n \t\t\t\tBIT[0] for LL_HIGH_ISR\n \t\t\t\tBIT[1] for LL_LOW_ISR\n \t\t\t\tBIT[2] for SYS_LOW_ISR\n @return None"]
    pub fn disable_specific_irq(isr_type: u8);
}
unsafe extern "C" {
    #[doc = " @brief   broad  initialization Function\n\n \tthis function is used to initialize the used MCU\n @param  op[in]   : pointer to @ref _ble_ll_bus structure that stores the bus callback functions\n @retval 0 if SUCCESS\n otherwise Not SUCCESS\n"]
    pub fn bsp_init() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief   dealy us  Function\n\n \tthis function is  microsecond delay function\n @param  op[in]   : Nunmber of microseconds that function this function execution should take\n @retval  None\n"]
    pub fn bsp_delay_us(delay: u32);
}
unsafe extern "C" {
    #[doc = " @brief   interrupt enable  Function\n\n \tthis function is used to enabled and register ISR for the given interrupt line\n @param  intrNum[in]   : number to the interrupt line to be enabled\n @param  intr_cb[in]   : pointer to ISR function the will be called when this interrupt is fired\n @retval  None\n"]
    pub fn bsp_intr_enable(
        intrNum: u32,
        intr_cb: ::core::option::Option<unsafe extern "C" fn()>,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief   interrupt set pri  Function\n\n \tthis function is used to set the interrupt priority and register ISR for the given interrupt line\n @param  intrNum[in]   : number to the configured interrupt line\n @param  intr_cb[in]   : pointer to ISR function the will be called when this interrupt is fired\n @param  intpri[in]    : the priority oto used for the given interrupt\n @retval  None\n"]
    pub fn bsp_intr_set_pri(
        intrNum: u32,
        intr_cb: ::core::option::Option<unsafe extern "C" fn()>,
        intpri: i32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  is in LL BLE memory  function\n it Checks whether the given pointer in BLE Memory or outside it  if the pointer points to a location in BLE memory\n @param   ptr [in] pointer to check if in BLE memory or not\n\n @retval 1   the given pointer is in LL BLE memory\n @retval 0   the given pointer is not in LL BLE memory"]
    pub fn bsp_is_ptr_in_ble_mem(ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief\tClear GPIO pin output value\n\n @param\tenable: enable/disable flag\n\n @retval None."]
    pub fn bsp_gpio_clear(gpio_num: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tSet GPIO pin output value\n\n @param\tenable: enable/disable flag\n\n @retval None."]
    pub fn bsp_gpio_set(gpio_num: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tEnables/Disables the bus clock.\n\n @param\tenable: enable/disable flag\n\n @retval None."]
    pub fn bsp_control_hclk(enable: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tEnables/Disables the active clock.\n\n @param\tenable: enable/disable flag\n\n @retval None."]
    pub fn bsp_control_aclk(enable: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tNotification that LL FW will start or end a radio activity .\n\n @param\tenable: EVNT_START , radio activity started\n \t\t\t\t  : Evnt_END     Radio event completed\n\n @retval None."]
    pub fn bsp_evnt_not(enable: EvntNotiState);
}
unsafe extern "C" {
    #[doc = " @brief\tNotification that LL FW will start or end RCO Calibration .\n\n@note this is an optional wrapper that used to inform the upper layer of the state of RCO calibration.\n@note the upper layer may ignore this wrapper\n @param\tenable: EVNT_START , RCO calibration will be started\n \t\t\t\t  : Evnt_END     RCO calibration has  completed\n\n @retval None."]
    pub fn bsp_rco_clbr_not(enable: EvntNotiState);
}
unsafe extern "C" {
    #[doc = " @brief used to assert/trigger the low priority interrupt from the SW.\n\n @param priority: if 1 then this SW ISR should be treated as if it was High priority HW ISR\n\n @retval None."]
    pub fn bsp_switch_to_lw_isr(priority: u8);
}
unsafe extern "C" {
    #[doc = " @brief  wait for bus clock ready signal\n\n A platform that has more accurate information about the readiness of the bus clock should implement this function to\n avoid redundant delay while reading sleep timer .\n\n @note this function will be called only if @ref USE_SOC_ACCURATE_BUS_CLK_READY_API is set to one otherwise LL FW will wait for a change in sleep timer reading.\n\n @param None.\n\n @retval None."]
    pub fn bsp_wait_for_busclkrdy();
}
unsafe extern "C" {
    #[doc = " @brief used to start temperature calculations\n if the upper layer has informed the link layer by the existence of temperature sensor through @ref ll_intf_set_temperature_sensor_stat() or llhwc_cmn_set_temperature_sensor_stat()  .\n New temperature will be requested in the following cases:\n 1- at initialization when the @ref ll_intf_set_temperature_sensor_state or llhwc_cmn_set_temperature_sensor_stat is called\n 2- before any radio event\n Once temperature is ready the upper layer should call @ref ll_intf_set_temperature_value() or llhwc_cmn_set_temperature_value() to inform Link Layer with new Temperature value\n @retval None."]
    pub fn bsp_request_temperature();
}
unsafe extern "C" {
    #[doc = " @brief a function to set a certain gpio pin.\n\n @param gpio[in]: one of the gpios defined in Debug_GPIO_t enum to be set\n\n @retval None.\n\n @note : some of the signals can be mapped to physical hardware and some may not be connected to a physical GPIO based in availability."]
    pub fn bsp_debug_gpio_set(gpio: Debug_GPIO_t);
}
unsafe extern "C" {
    #[doc = " @brief a function to clear a certain gpio pin.\n\n @param gpio[in]: one of the gpios defined in Debug_GPIO_t enum to be cleared\n\n @retval None.\n\n @note : some of the signals can be mapped to physical hardware and some may not be connected to a physical GPIO based in availability."]
    pub fn bsp_debug_gpio_clear(gpio: Debug_GPIO_t);
}
unsafe extern "C" {
    #[doc = " @brief a function to toggle a certain gpio pin.\n\n @param gpio[in]: one of the gpios defined in Debug_GPIO_t enum to be toggled\n\n @retval None.\n\n @note : some of the signals can be mapped to physical hardware and some may not be connected to a physical GPIO based in availability."]
    pub fn bsp_debug_gpio_toggle(gpio: Debug_GPIO_t);
}
unsafe extern "C" {
    #[doc = " @brief a function to inform the upper layer by state of periodic calibration state.\n\n @param state[in]: Value from @ref PhyClbrState, NO_PHY_CLBR_NEEDED  when the calibration is completed , PHY_CLBR_PNDING_OR_RUNNING when calibration is started or about to start from background task\n\n @retval None."]
    pub fn bsp_set_phy_clbr_state(state: PhyClbrState);
}
unsafe extern "C" {
    #[doc = " @brief a function to notify the upper layer to switch the clock.\n\n @param evnt_timing[in]: Evnt_timing_t pointer to structure contains drift time , execution time and scheduling time. For the execution time, it shall follow this equation MAX(EXEC_TIME_PROFILED, PHY_WAKEUP_TIME) - PHY_WAKEUP_TIME + EXEC_TIME_MARGIN\n\n @retval None."]
    pub fn bsp_evnt_schldr_timing_update_not(p_evnt_timing: *mut Evnt_timing_t);
}
unsafe extern "C" {
    #[doc = " @}"]
    pub fn logCons(
        devHandle: *mut ::core::ffi::c_void,
        logStr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn logFileInit(fileName: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn logFileClose(fileHandle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn logFile(
        devHandle: *mut ::core::ffi::c_void,
        logStr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn logUart(
        devHandle: *mut ::core::ffi::c_void,
        logStr: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn bsp_assert_log(
        condition: u8,
        severity: u8,
        ptr_func_name: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn bsp_assert(condition: u8, severity: u8);
}
unsafe extern "C" {
    #[doc = " @brief Communicates the state of the execution time profiling\n\n @param[in] state: Signals the start, end or clearance of the execution time"]
    pub fn bsp_exec_time_profiling(state: profiling_state_e);
}
unsafe extern "C" {
    #[doc = " @brief Communicates the profiled value for the drift time\n\n @param[in] value: Profiled value in HW Cycles"]
    pub fn bsp_drift_time_profiling(value: u32);
}
unsafe extern "C" {
    #[doc = " @brief  Coping memory from position to another.\n @param  ptr_dstntion\t: pointer to the destination array where the content is to be copied.\n @param  ptr_src\t: pointer to the source of data to be copied.\n @param  n \t\t: the number of bytes to be copied.\n @retval pointer to destination."]
    pub fn ble_memcpy(
        ptr_dstntion: *mut ::core::ffi::c_void,
        ptr_src: *const ::core::ffi::c_void,
        n: u16,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  Setting a certain block of memory with a certain value.\n @param  ptr_mem\t: pointer to the block of memory to fill.\n @param  value\t: the value to be set. The value is passed as an int.\n @param  n \t\t: the number of bytes to be set to the value.\n @retval pointer to destination."]
    pub fn ble_memset(
        ptr_mem: *mut ::core::ffi::c_void,
        value: u8,
        n: u16,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  comparing a certain block of memory with another.\n @param  ptr_dstntion\t: pointer to the destination array where the content is to be compared.\n @param  ptr_src\t: pointer to the source of data to be compared.\n @param  n \t\t: the number of bytes to be compared.\n @retval  \t< 0 ptr_dstntion is less than ptr_src.\n \t\t> 0 ptr_src is less than ptr_dstntion.\n \t\t  0 ptr_dstntion is equal to ptr_src."]
    pub fn ble_memcmp(
        ptr_dstntion: *const ::core::ffi::c_void,
        ptr_src: *const ::core::ffi::c_void,
        n: u16,
    ) -> i8;
}
unsafe extern "C" {
    #[doc = " @brief  Moving  memory from position to another.\n @param  ptr_dstntion\t: pointer to the destination array where the content is to be moved.\n @param  ptr_src\t: pointer to the source of data to be moved.\n @param  n \t\t: the number of bytes to be moved.\n @retval pointer to destination."]
    pub fn ble_memmov(
        ptr_dstntion: *mut ::core::ffi::c_void,
        ptr_src: *const ::core::ffi::c_void,
        n: u16,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  Coping n bytes of memory from position to another.\n @param  destination\t: pointer to the destination array where the content is to be copied.\n @param  source\t    : pointer to the source of data to be copied.\n @param  num_bytes    : the number of bytes to be copied.\n @param  keep_endian  : flag to keep or change the endian\n @retval pointer to destination."]
    pub fn ble_memcpy_n_bytes(
        destination: *mut u8,
        source: *const u8,
        num_bytes: u8,
        keep_endian: u8,
    );
}
pub const int_state_NOT_ACTIVE: int_state = 0;
pub const int_state_LINK_LAYER_INTRPT: int_state = 1;
pub const int_state_LINK_LAYER_LOW_PRIORITY_INTRPT: int_state = 2;
pub const int_state_UART_READ_INTRPT: int_state = 3;
pub const int_state_UART_WRITE_INTRPT: int_state = 4;
pub const int_state_TIMER_INTRPT: int_state = 5;
pub const int_state_MAC_INTRPT: int_state = 6;
pub const int_state_TOTAL_INTERRUPTS: int_state = 7;
#[doc = " @brief Interrupt status."]
pub type int_state = ::core::ffi::c_uint;
#[doc = " @brief Interrupt status."]
pub use self::int_state as int_state_e;
pub const os_priority_os_priority_high: os_priority = 0;
pub const os_priority_os_priority_normal: os_priority = 1;
pub const os_priority_os_priority_low: os_priority = 2;
#[doc = " @brief Priority used for thread control."]
pub type os_priority = ::core::ffi::c_uint;
pub const _sw_timer_activity_status_e_SW_TIMER_NOT_ACTIVE: _sw_timer_activity_status_e = 0;
pub const _sw_timer_activity_status_e_SW_TIMER_ACTIVE: _sw_timer_activity_status_e = 1;
pub const _sw_timer_activity_status_e_SW_TIMER_MAY_BE_NOT_ACTIVE: _sw_timer_activity_status_e = 2;
#[doc = " @brief SW Timer Activity Status."]
pub type _sw_timer_activity_status_e = ::core::ffi::c_uint;
#[doc = " @brief SW Timer Activity Status."]
pub use self::_sw_timer_activity_status_e as sw_timer_activity_status_e;
#[doc = "< one-shot timer"]
pub const os_timer_type_os_timer_once: os_timer_type = 0;
#[doc = "< repeating timer"]
pub const os_timer_type_os_timer_periodic: os_timer_type = 1;
#[doc = " @brief SW Timer Type."]
pub type os_timer_type = ::core::ffi::c_uint;
pub const os_timer_prio_lw_prio_tmr: os_timer_prio = 0;
pub const os_timer_prio_hg_prio_tmr: os_timer_prio = 1;
#[doc = " @brief SW Timer Priority."]
pub type os_timer_prio = ::core::ffi::c_uint;
#[doc = "< @brief Active timer : Timer in the list waiting for its time to fire"]
pub const os_timer_state_osTimerActive: os_timer_state = 0;
#[doc = "< @brief Expired timer: Timer fired and removed form the list, or created and not exist in the list"]
pub const os_timer_state_osTimerExpired: os_timer_state = 1;
#[doc = "< @brief Stopped timer: Timer stopped and removed form the list"]
pub const os_timer_state_osTimerStopped: os_timer_state = 2;
#[doc = " @brief Software Timer State Active, Expired or Stopped"]
pub type os_timer_state = ::core::ffi::c_uint;
pub type os_pthread =
    ::core::option::Option<unsafe extern "C" fn(argument: *const ::core::ffi::c_void)>;
pub type os_thread_id = *mut ::core::ffi::c_void;
pub type t_timer_callbk =
    ::core::option::Option<unsafe extern "C" fn(arg1: *const ::core::ffi::c_void)>;
pub type os_timer_id = *mut ::core::ffi::c_void;
pub type os_timer_activity_cb_t =
    ::core::option::Option<unsafe extern "C" fn(timer_activity: sw_timer_activity_status_e)>;
#[doc = " @brief   Software Timer structure."]
pub type sw_timer_t = sw_timer;
pub type os_mutex_def_t = ::core::ffi::c_void;
pub type os_mutex_id = *mut ::core::ffi::c_void;
pub type os_semaphore_def_t = ::core::ffi::c_void;
pub type os_semaphore_id = *mut ::core::ffi::c_void;
#[doc = " @brief   Software Timer structure."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_timer {
    #[doc = "< @brief Next timer in the timers list."]
    pub stnext: *mut sw_timer_t,
    #[doc = "< @brief value of timer"]
    pub vtime: u32,
    #[doc = "< @brief remain time."]
    pub rtime: u32,
    #[doc = "< @brief Timer callback function pointer."]
    pub ptimer: t_timer_callbk,
    #[doc = "< @brief Timer callback function arguments."]
    pub argument: *mut ::core::ffi::c_void,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    #[doc = "< @brief Timer State : Active or Expired or Stopped"]
    pub state: u8,
    pub _bitfield_align_2: [u8; 0],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl Default for sw_timer {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl sw_timer {
    #[inline]
    pub fn overflow_flag(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_overflow_flag(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn overflow_flag_raw(this: *const Self) -> u16 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u16)
        }
    }
    #[inline]
    pub unsafe fn set_overflow_flag_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn frac_time(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 5u8) as u16) }
    }
    #[inline]
    pub fn set_frac_time(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn frac_time_raw(this: *const Self) -> u16 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                5u8,
            ) as u16)
        }
    }
    #[inline]
    pub unsafe fn set_frac_time_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn cycles(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 5u8) as u16) }
    }
    #[inline]
    pub fn set_cycles(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cycles_raw(this: *const Self) -> u16 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                5u8,
            ) as u16)
        }
    }
    #[inline]
    pub unsafe fn set_cycles_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rem_time(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 5u8) as u16) }
    }
    #[inline]
    pub fn set_rem_time(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rem_time_raw(this: *const Self) -> u16 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                11usize,
                5u8,
            ) as u16)
        }
    }
    #[inline]
    pub unsafe fn set_rem_time_raw(this: *mut Self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        overflow_flag: u16,
        frac_time: u16,
        cycles: u16,
        rem_time: u16,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let overflow_flag: u16 = unsafe { ::core::mem::transmute(overflow_flag) };
            overflow_flag as u64
        });
        __bindgen_bitfield_unit.set(1usize, 5u8, {
            let frac_time: u16 = unsafe { ::core::mem::transmute(frac_time) };
            frac_time as u64
        });
        __bindgen_bitfield_unit.set(6usize, 5u8, {
            let cycles: u16 = unsafe { ::core::mem::transmute(cycles) };
            cycles as u64
        });
        __bindgen_bitfield_unit.set(11usize, 5u8, {
            let rem_time: u16 = unsafe { ::core::mem::transmute(rem_time) };
            rem_time as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn type_(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_2.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_2.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn type__raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_2),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_type_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_2),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn prio(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_2.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_prio(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_2.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn prio_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_2),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_prio_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_2),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_2(type_: u8, prio: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let type_: u8 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let prio: u8 = unsafe { ::core::mem::transmute(prio) };
            prio as u64
        });
        __bindgen_bitfield_unit
    }
}
#[doc = " @brief Memory Block Structure"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _mem_blck_t {
    pub flag: u32,
    pub next: *mut _mem_blck_t,
}
impl Default for _mem_blck_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Memory Block Structure"]
pub type mem_blck_t = _mem_blck_t;
#[doc = " @brief Memory Pool Block Structure"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_pool_def_t {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub next_blck: *mut mem_blck_t,
}
impl Default for os_pool_def_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl os_pool_def_t {
    #[inline]
    pub fn blck_size(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_blck_size(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn blck_size_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_blck_size_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn total_blcks(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 10u8) as u32) }
    }
    #[inline]
    pub fn set_total_blcks(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn total_blcks_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                16usize,
                10u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_total_blcks_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                10u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn indx(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(26usize, 6u8) as u32) }
    }
    #[inline]
    pub fn set_indx(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(26usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn indx_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                26usize,
                6u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_indx_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                26usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        blck_size: u32,
        total_blcks: u32,
        indx: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let blck_size: u32 = unsafe { ::core::mem::transmute(blck_size) };
            blck_size as u64
        });
        __bindgen_bitfield_unit.set(16usize, 10u8, {
            let total_blcks: u32 = unsafe { ::core::mem::transmute(total_blcks) };
            total_blcks as u64
        });
        __bindgen_bitfield_unit.set(26usize, 6u8, {
            let indx: u32 = unsafe { ::core::mem::transmute(indx) };
            indx as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    #[doc = " @brief Creates a thread\n\n @param thread\t\tPointer to a function to be executed by the thread\n @param name\t\t\tThread's name\n @param pri\t\t\tThread's priority\n @param argu\t\t\tArguments to be passed to the function executed by the thread\n @param stack_size\tThread stack size\n\n @retval Handle of the created task"]
    pub fn os_thread_create(
        thread: os_pthread,
        name: *mut ::core::ffi::c_char,
        pri: os_priority,
        argu: *mut ::core::ffi::c_void,
        stack_size: u32,
    ) -> os_thread_id;
}
unsafe extern "C" {
    #[doc = " @brief Registers an interrupt function corresponding to the passed interrupt ID\n\n @param ptr_int_hndlr Interrupt function\n @param int_id \t\tInterrupt ID"]
    pub fn intr_hndlr_reg(
        ptr_int_hndlr: ::core::option::Option<unsafe extern "C" fn()>,
        int_id: int_state_e,
    );
}
unsafe extern "C" {
    #[doc = " @brief initialize function to to os_wrapper"]
    pub fn os_wrapper_init();
}
unsafe extern "C" {
    #[doc = " @brief reset function to os_wrapper component"]
    pub fn os_wrapper_reset();
}
unsafe extern "C" {
    #[doc = " @brief initialize timer function"]
    pub fn os_timer_init();
}
unsafe extern "C" {
    #[doc = " @brief initialize timer function"]
    pub fn os_timer_reset();
}
unsafe extern "C" {
    #[doc = "  @ingroup SW_TIMER\n @{\n/\n/**\n @brief  create a new timer\n\n @param  p_callbk      pointer to the call_back function.\n @param  type          os_timer_once for one-shot or os_timer_periodic for periodic behavior.\n @param  argument      argument to the timer call back function.\n\n @retval timer ID for reference by other functions or NULL in case of error."]
    pub fn os_timer_create(
        p_callbk: t_timer_callbk,
        type_: os_timer_type,
        argument: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  set the timer priority\n\n @param  timer id\n @param  tmr_prio: the new priority of the timer in case of allow_lw_isr==1\n\n @retval None"]
    pub fn os_timer_set_prio(timer_id: os_timer_id, tmr_prio: os_timer_prio);
}
unsafe extern "C" {
    #[doc = " @brief  get the timer priority\n\n @retval get the priority of the SW timers head"]
    pub fn os_timer_is_any_near_sw_timer_hg_prio() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  start a running timer.\n\n @param  timer_id      timer Id.\n @param  steps         number of steps in 31.25 us resolution\n\n @retval error code."]
    pub fn os_timer_start(timer_id: os_timer_id, steps: u32) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  start a running timer.\n\n @param  timer_id      timer Id.\n @param  time_us       time in us\n\n @retval error code."]
    pub fn os_timer_start_in_us(timer_id: os_timer_id, time_us: u32) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief\tstop a running timer.\n\n @param\ttimer_id\t  timer Id.\n\n @retval error code."]
    pub fn os_timer_stop(timer_id: os_timer_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief\tfree an allocated timer.\n\n @param\ttimer_id\t  timer Id.\n\n @retval error code."]
    pub fn os_timer_free(timer_id: os_timer_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief Stop the timer if it is running and delete it.\n\n @param  ptr_timer_id     pointer to the timer ID obtained by  os_timer_create.\n\n @retval status code that indicates the execution status of the function."]
    pub fn os_timer_stop_free(ptr_timer_id: *mut os_timer_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  Stop the timer if it is running and start it with the new value.\n\n @param  timer      timer ID obtained by \\ref os_timer_create.\n @param  steps\t  steps to set the timer with.\n\n @retval status code that indicates the execution status of the function."]
    pub fn os_timer_set(timer: os_timer_id, steps: u32) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief\tget the starte of the timer.\n\n @param\ttimer_id\t  timer Id.\n\n @retval os_timer_state. Active , Expired, or stopped"]
    pub fn os_get_tmr_state(timer_id: os_timer_id) -> os_timer_state;
}
unsafe extern "C" {
    #[doc = "@}\n *\n/\n/**\n @brief\tGet the number of active SW timers.\n\n @retval active_sw_timers_num: The number of currently active SW timers"]
    pub fn os_timer_get_active_sw_timers_number() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief\tRegister a callback function to show whether the timer is in use or not.\n\n @param\tcbk\t  : [in] Callback function."]
    pub fn os_timer_rgstr_timer_activity_cbk(cbk: os_timer_activity_cb_t);
}
unsafe extern "C" {
    #[doc = " @brief\tGets the remaining time of the first time set to fire, if exists\n\n @retval\tRemaining time. 0 if no timers exist."]
    pub fn os_timer_get_earliest_time() -> u64;
}
unsafe extern "C" {
    #[doc = " @brief  This function calls the proper handling based on the incming interrupt\n\n @param  intrpt_fired      current interrupt to be served."]
    pub fn os_process_isr(intrpt_fired: int_state_e);
}
unsafe extern "C" {
    #[doc = "  @ingroup os_wrappers\n  @{\n/\n/**\n @brief  disables system Interrupts"]
    pub fn os_disable_isr();
}
unsafe extern "C" {
    #[doc = " @brief  enables system Interrupts, the imp. should respect the nested disable calls"]
    pub fn os_enable_isr();
}
unsafe extern "C" {
    #[doc = " @brief  create a new recursive mutex\n\n @retval handle to the created mutex"]
    pub fn os_rcrsv_mutex_create() -> os_mutex_id;
}
unsafe extern "C" {
    #[doc = " @brief  Wait until a mutex becames available\n\n @param  mutex_id      mutex id.\n @param millisec      time-out value, 0 for no time-out.\n\n @retval status code , 0 for success"]
    pub fn os_rcrsv_mutex_wait(mutex_id: os_mutex_id, millisec: u32) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  Release a mutex\n\n @param  mutex_id      mutex id.\n\n @retval status code, 0 for success"]
    pub fn os_rcrsv_mutex_release(mutex_id: os_mutex_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  Create and initialize a semaphore\n\n @param  max_count\t      The max value to which the semaphore can count.\n @param  initial_count      initial value assigned to the count.\n\n @retval semaphore id for reference"]
    pub fn os_semaphore_create(max_count: i32, initial_count: i32) -> os_semaphore_id;
}
unsafe extern "C" {
    #[doc = " @brief  Wait until a semaphore becomes available\n\n @param  semaphore_id semaphore id.\n @param  millisec      time-out value, 0 for no time-out.\n\n @retval status code, 0 for success"]
    pub fn os_semaphore_wait(semaphore_id: os_semaphore_id, millisec: u32) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  Release a semaphore\n\n @param  semaphore_id semaphore id.\n\n @retval status code, 0 for success"]
    pub fn os_semaphore_release(semaphore_id: os_semaphore_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief  Release an ISR semaphore\n\n @param  semaphore_id semaphore id.\n\n @retval status code, 0 for success"]
    pub fn os_semaphore_release_isr(semaphore_id: os_semaphore_id) -> i32;
}
unsafe extern "C" {
    #[doc = " @brief Allocates from the passed memory pool\n\n @param pool Pointer to the pool to allocate from\n\n @retval Pointer at the allocated block"]
    pub fn os_mem_pool_alloc(pool: *mut os_pool_def_t) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief Frees from the passed memory pool\n\n @param block Pointer at the block that will be freed"]
    pub fn os_mem_pool_free(block: *mut ::core::ffi::c_void);
}
unsafe extern "C" {
    #[doc = " @brief Allocates from the shared memory pool\n\n @param pool Pointer to the pool to allocate from\n\n @retval Pointer at the allocated block"]
    pub fn os_shrd_mem_alloc(pool: *mut os_pool_def_t) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @fn uint8_t os_wrapper_is_rtos_used()\n\n @brief This function used to detect whether RTOS configuration is enabled or not.\n @param None.\n @return is_rtos_enabled : TRUE: RTOS enabled. FALSE: otherwise"]
    pub fn os_wrapper_is_rtos_used() -> u8;
}
unsafe extern "C" {
    #[link_name = "\u{1}g_ll_lock"]
    pub static mut G_LL_LOCK: os_mutex_id;
}
#[doc = " @brief Global error definition across different components.\n refer the error codes defined in @ref  ll_error.h for more  information about  the values that this type should set"]
pub type ble_stat_t = u32;
pub const tx_rx_phy_e_LE_NO_CHANGE: tx_rx_phy_e = 0;
pub const tx_rx_phy_e_LE_1M: tx_rx_phy_e = 1;
pub const tx_rx_phy_e_LE_2M: tx_rx_phy_e = 2;
pub const tx_rx_phy_e_LE_CODED_S8: tx_rx_phy_e = 3;
pub const tx_rx_phy_e_LE_CODED: tx_rx_phy_e = 4;
pub type tx_rx_phy_e = ::core::ffi::c_uint;
#[doc = " @brief time stamp structure."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct time_st {
    pub time_stamp_base: u32,
    pub time_stamp_fine: u16,
    pub overflow_flag: u8,
}
#[doc = " @brief time stamp structure."]
pub type ble_time_t = time_st;
#[doc = " @brief time stamp structure."]
pub type ble_time_p = *mut time_st;
pub const dpslp_state_DEEP_SLEEP_ENABLE: dpslp_state = 1;
pub const dpslp_state_DEEP_SLEEP_DISABLE: dpslp_state = 0;
pub type dpslp_state = ::core::ffi::c_uint;
pub use self::dpslp_state as dpslp_state_e;
pub const crypto_endian_enum_t_CRYPTO_LITTLE_ENDIAN: crypto_endian_enum_t = 0;
pub const crypto_endian_enum_t_CRYPTO_BIG_ENDIAN: crypto_endian_enum_t = 1;
#[doc = " @enum crypto_endian_enum_t\n @brief States the available endian formats.\n"]
pub type crypto_endian_enum_t = ::core::ffi::c_uint;
pub const security_mode_enum_t_ECB_DECRYPT: security_mode_enum_t = 0;
pub const security_mode_enum_t_CCM_DECRYPT: security_mode_enum_t = 1;
pub const security_mode_enum_t_ECB_ENCRYPT: security_mode_enum_t = 2;
pub const security_mode_enum_t_CCM_ENCRYPT: security_mode_enum_t = 3;
pub const security_mode_enum_t_CTR_ENCRYPT: security_mode_enum_t = 4;
pub const security_mode_enum_t_CTR_DECRYPT: security_mode_enum_t = 5;
pub const security_mode_enum_t_MODES_MAX_NUM: security_mode_enum_t = 6;
#[doc = " @enum security_mode_enum_t\n @brief Contains the available security modes.\n\n Note: The enum values should be the same as specified in the register\n  address header file."]
pub type security_mode_enum_t = ::core::ffi::c_uint;
pub const ral_phy_rate_enum_RAL_RATE_125K: ral_phy_rate_enum = 0;
pub const ral_phy_rate_enum_RAL_RATE_1M: ral_phy_rate_enum = 2;
pub const ral_phy_rate_enum_RAL_RATE_2M: ral_phy_rate_enum = 3;
pub const ral_phy_rate_enum_RAL_RATE_256K: ral_phy_rate_enum = 1;
#[doc = " @brief Enum defines the various PHY data rates supported by the RAL.\n"]
pub type ral_phy_rate_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines the various PHY data rates supported by the RAL.\n"]
pub use self::ral_phy_rate_enum as ral_phy_rate_enum_t;
pub const _extrnl_evnt_priority_e_PRIORITY_DEFAULT: _extrnl_evnt_priority_e = 0;
pub const _extrnl_evnt_priority_e_PRIORITY_HIGH: _extrnl_evnt_priority_e = 1;
pub const _extrnl_evnt_priority_e_PRIORITY_CRITICAL: _extrnl_evnt_priority_e = 2;
#[doc = " @enum extrnl_evnt_priority_e\n @brief External Event priority"]
pub type _extrnl_evnt_priority_e = ::core::ffi::c_uint;
#[doc = " @enum extrnl_evnt_priority_e\n @brief External Event priority"]
pub use self::_extrnl_evnt_priority_e as extrnl_evnt_priority_e;
pub const _extrnl_evnt_state_e_STATE_BLOCKED_UNKNOWN: _extrnl_evnt_state_e = 0;
pub const _extrnl_evnt_state_e_STATE_BLOCKED_PRIORITY: _extrnl_evnt_state_e = 1;
pub const _extrnl_evnt_state_e_STATE_BLOCKED_CANCELLED: _extrnl_evnt_state_e = 2;
pub const _extrnl_evnt_state_e_STATE_BLOCKED_LATE: _extrnl_evnt_state_e = 3;
pub const _extrnl_evnt_state_e_STATE_BLOCKED_DEADLINE: _extrnl_evnt_state_e = 4;
pub const _extrnl_evnt_state_e_STATE_NOT_BLOCKED: _extrnl_evnt_state_e = 5;
#[doc = " @enum extrnl_evnt_state_e\n @brief External Event Blocked State and reason"]
pub type _extrnl_evnt_state_e = ::core::ffi::c_uint;
#[doc = " @enum extrnl_evnt_state_e\n @brief External Event Blocked State and reason"]
pub use self::_extrnl_evnt_state_e as extrnl_evnt_state_e;
pub const _slptmr_src_type_e_CRYSTAL_OSCILLATOR_SLPTMR: _slptmr_src_type_e = 0;
pub const _slptmr_src_type_e_RCO_SLPTMR: _slptmr_src_type_e = 1;
pub const _slptmr_src_type_e_RTC_SLPTMR: _slptmr_src_type_e = 2;
#[doc = " @brief Enumeration of the source type used to drive the sleep timer."]
pub type _slptmr_src_type_e = ::core::ffi::c_uint;
#[doc = " @brief Enumeration of the source type used to drive the sleep timer."]
pub use self::_slptmr_src_type_e as slptmr_src_type_e;
pub const ant_intrv_type_enum_NO_TYPE: ant_intrv_type_enum = 0;
pub const ant_intrv_type_enum_FIXED_TIME: ant_intrv_type_enum = 1;
pub const ant_intrv_type_enum_PACKETS_NUMBER: ant_intrv_type_enum = 2;
#[doc = " @brief Enum defines the antenna diversity interval types.\n"]
pub type ant_intrv_type_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines the antenna diversity interval types.\n"]
pub use self::ant_intrv_type_enum as ant_intrv_type_enum_t;
#[doc = " @brief Structure represents antenna diversity parameters.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _antenna_diversity_st {
    #[doc = "< Antenna interval type: FIXED_TIME(us) or PACKETS_NUMBER(n)"]
    pub ant_intrv_type: ant_intrv_type_enum_t,
    #[doc = "< Antenna interval value based on type; us for FIXED_TIME, n for PACKETS_NUMBER"]
    pub ant_intrv_value: u32,
    #[doc = "< Wanted coordinator/router short address"]
    pub wntd_coord_shrt_addr: u16,
    #[doc = "< Wanted coordinator/router extended address"]
    pub wntd_coord_ext_addr: [u8; 8usize],
    #[doc = "< Maximum number of retries to receive ACK in case of ACK error reception"]
    pub max_rx_ack_retries: u8,
}
impl Default for _antenna_diversity_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents antenna diversity parameters.\n"]
pub type antenna_diversity_st = _antenna_diversity_st;
#[doc = " @brief Structure represents configurable library parameters.\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _config_lib_st {
    #[doc = "< Disable/Enable MAC layer build"]
    pub mac_layer_build: u8,
    #[doc = "< Disable/Enable FW parts related to new features introduced in OpenThread 1.2."]
    pub support_openthread_1_2: u8,
    #[doc = "< Disable/Enable sending ACK for all received frames with AR bit set"]
    pub ack_all_received_frames_with_ar_bit_set: u8,
}
#[doc = " @brief Structure represents configurable library parameters.\n"]
pub type config_lib_st = _config_lib_st;
unsafe extern "C" {
    #[doc = "< Configurable library parameters"]
    #[link_name = "\u{1}g_config_lib_params"]
    pub static mut G_CONFIG_LIB_PARAMS: config_lib_st;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ble_buff_hdr_st {
    pub buff_start: *mut u8,
    pub next_pkt: *mut ble_buff_hdr_st,
    pub total_len: u16,
    pub data_offset: u16,
    pub data_size: u16,
    pub ble_hdr_flags: u8,
}
impl Default for ble_buff_hdr_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type pkt_buff_hdr_t = ble_buff_hdr_st;
pub type ble_buff_hdr_t = ble_buff_hdr_st;
pub type ble_buff_hdr_p = *mut ble_buff_hdr_st;
pub const event_t_HCI_CMD_PCKT: event_t = 1;
pub const event_t_HCI_ACL_DATA_PCKT: event_t = 2;
pub const event_t_HCI_EVNT_PCKT: event_t = 4;
pub const event_t_HCI_ISO_DATA_PCKT: event_t = 5;
pub type event_t = ::core::ffi::c_uint;
#[doc = " No error."]
pub const otError_OT_ERROR_NONE: otError = 0;
#[doc = " Operational failed."]
pub const otError_OT_ERROR_FAILED: otError = 1;
#[doc = " Message was dropped."]
pub const otError_OT_ERROR_DROP: otError = 2;
#[doc = " Insufficient buffers."]
pub const otError_OT_ERROR_NO_BUFS: otError = 3;
#[doc = " No route available."]
pub const otError_OT_ERROR_NO_ROUTE: otError = 4;
#[doc = " Service is busy and could not service the operation."]
pub const otError_OT_ERROR_BUSY: otError = 5;
#[doc = " Failed to parse message."]
pub const otError_OT_ERROR_PARSE: otError = 6;
#[doc = " Input arguments are invalid."]
pub const otError_OT_ERROR_INVALID_ARGS: otError = 7;
#[doc = " Security checks failed."]
pub const otError_OT_ERROR_SECURITY: otError = 8;
#[doc = " Address resolution requires an address query operation."]
pub const otError_OT_ERROR_ADDRESS_QUERY: otError = 9;
#[doc = " Address is not in the source match table."]
pub const otError_OT_ERROR_NO_ADDRESS: otError = 10;
#[doc = " Operation was aborted."]
pub const otError_OT_ERROR_ABORT: otError = 11;
#[doc = " Function or method is not implemented."]
pub const otError_OT_ERROR_NOT_IMPLEMENTED: otError = 12;
#[doc = " Cannot complete due to invalid state."]
pub const otError_OT_ERROR_INVALID_STATE: otError = 13;
#[doc = " No acknowledgment was received after macMaxFrameRetries (IEEE 802.15.4-2006)."]
pub const otError_OT_ERROR_NO_ACK: otError = 14;
#[doc = " A transmission could not take place due to activity on the channel, i.e., the CSMA-CA mechanism has failed\n (IEEE 802.15.4-2006)."]
pub const otError_OT_ERROR_CHANNEL_ACCESS_FAILURE: otError = 15;
#[doc = " Not currently attached to a Thread Partition."]
pub const otError_OT_ERROR_DETACHED: otError = 16;
#[doc = " FCS check failure while receiving."]
pub const otError_OT_ERROR_FCS: otError = 17;
#[doc = " No frame received."]
pub const otError_OT_ERROR_NO_FRAME_RECEIVED: otError = 18;
#[doc = " Received a frame from an unknown neighbor."]
pub const otError_OT_ERROR_UNKNOWN_NEIGHBOR: otError = 19;
#[doc = " Received a frame from an invalid source address."]
pub const otError_OT_ERROR_INVALID_SOURCE_ADDRESS: otError = 20;
#[doc = " Received a frame filtered by the address filter (allowlisted or denylisted)."]
pub const otError_OT_ERROR_ADDRESS_FILTERED: otError = 21;
#[doc = " Received a frame filtered by the destination address check."]
pub const otError_OT_ERROR_DESTINATION_ADDRESS_FILTERED: otError = 22;
#[doc = " The requested item could not be found."]
pub const otError_OT_ERROR_NOT_FOUND: otError = 23;
#[doc = " The operation is already in progress."]
pub const otError_OT_ERROR_ALREADY: otError = 24;
#[doc = " The creation of IPv6 address failed."]
pub const otError_OT_ERROR_IP6_ADDRESS_CREATION_FAILURE: otError = 26;
#[doc = " Operation prevented by mode flags"]
pub const otError_OT_ERROR_NOT_CAPABLE: otError = 27;
#[doc = " Coap response or acknowledgment or DNS, SNTP response not received."]
pub const otError_OT_ERROR_RESPONSE_TIMEOUT: otError = 28;
#[doc = " Received a duplicated frame."]
pub const otError_OT_ERROR_DUPLICATED: otError = 29;
#[doc = " Message is being dropped from reassembly list due to timeout."]
pub const otError_OT_ERROR_REASSEMBLY_TIMEOUT: otError = 30;
#[doc = " Message is not a TMF Message."]
pub const otError_OT_ERROR_NOT_TMF: otError = 31;
#[doc = " Received a non-lowpan data frame."]
pub const otError_OT_ERROR_NOT_LOWPAN_DATA_FRAME: otError = 32;
#[doc = " The link margin was too low."]
pub const otError_OT_ERROR_LINK_MARGIN_LOW: otError = 34;
#[doc = " Input (CLI) command is invalid."]
pub const otError_OT_ERROR_INVALID_COMMAND: otError = 35;
#[doc = " Special error code used to indicate success/error status is pending and not yet known.\n"]
pub const otError_OT_ERROR_PENDING: otError = 36;
#[doc = " Request rejected."]
pub const otError_OT_ERROR_REJECTED: otError = 37;
#[doc = " The number of defined errors."]
pub const otError_OT_NUM_ERRORS: otError = 38;
#[doc = " Generic error (should not use)."]
pub const otError_OT_ERROR_GENERIC: otError = 255;
#[doc = " Represents error codes used throughout OpenThread.\n"]
pub type otError = ::core::ffi::c_uint;
unsafe extern "C" {
    #[doc = " Converts an otError enum into a string.\n\n @param[in]  aError     An otError enum.\n\n @returns  A string representation of an otError.\n"]
    pub fn otThreadErrorToString(aError: otError) -> *const ::core::ffi::c_char;
}
pub const mac_status_enum_t_BEACON_LOSS: mac_status_enum_t = 224;
pub const mac_status_enum_t_CHANNEL_ACCESS_FAILURE: mac_status_enum_t = 225;
pub const mac_status_enum_t_COUNTER_ERROR: mac_status_enum_t = 219;
pub const mac_status_enum_t_DENIED: mac_status_enum_t = 226;
pub const mac_status_enum_t_DISABLE_TRX_FAILURE: mac_status_enum_t = 227;
pub const mac_status_enum_t_SECURITY_ERROR: mac_status_enum_t = 228;
pub const mac_status_enum_t_FRAME_TOO_LONG: mac_status_enum_t = 229;
pub const mac_status_enum_t_IMPROPER_KEY_TYPE: mac_status_enum_t = 220;
pub const mac_status_enum_t_IMPROPER_SECURITY_LEVEL: mac_status_enum_t = 221;
pub const mac_status_enum_t_INVALID_ADDRESS: mac_status_enum_t = 245;
pub const mac_status_enum_t_INVALID_GTS: mac_status_enum_t = 230;
pub const mac_status_enum_t_INVALID_HANDLE: mac_status_enum_t = 231;
pub const mac_status_enum_t_INVALID_INDEX: mac_status_enum_t = 249;
pub const mac_status_enum_t_INVALID_PARAMETER: mac_status_enum_t = 232;
pub const mac_status_enum_t_NO_ACK: mac_status_enum_t = 233;
pub const mac_status_enum_t_NO_BEACON: mac_status_enum_t = 234;
pub const mac_status_enum_t_NO_DATA: mac_status_enum_t = 235;
pub const mac_status_enum_t_NO_SHORT_ADDRESS: mac_status_enum_t = 236;
pub const mac_status_enum_t_ON_TIME_TOO_LONG: mac_status_enum_t = 246;
pub const mac_status_enum_t_OUT_OF_CAP: mac_status_enum_t = 237;
pub const mac_status_enum_t_PAN_ID_CONFLICT: mac_status_enum_t = 238;
pub const mac_status_enum_t_PAST_TIME: mac_status_enum_t = 247;
pub const mac_status_enum_t_READ_ONLY: mac_status_enum_t = 251;
pub const mac_status_enum_t_REALIGNMENT: mac_status_enum_t = 239;
pub const mac_status_enum_t_SCAN_IN_PROGRESS: mac_status_enum_t = 252;
pub const mac_status_enum_t_SUPERFRAME_OVERLAP: mac_status_enum_t = 253;
pub const mac_status_enum_t_TRACKING_OFF: mac_status_enum_t = 248;
pub const mac_status_enum_t_TRANSACTION_EXPIRED: mac_status_enum_t = 240;
pub const mac_status_enum_t_TRANSACTION_OVERFLOW: mac_status_enum_t = 241;
pub const mac_status_enum_t_TX_ACTIVE: mac_status_enum_t = 242;
pub const mac_status_enum_t_UNAVAILABLE_KEY: mac_status_enum_t = 243;
pub const mac_status_enum_t_UNSUPPORTED_ATTRIBUTE: mac_status_enum_t = 244;
pub const mac_status_enum_t_UNSUPPORTED_LEGACY: mac_status_enum_t = 222;
pub const mac_status_enum_t_UNSUPPORTED_SECURITY: mac_status_enum_t = 223;
pub const mac_status_enum_t_MAC_LIMIT_REACHED: mac_status_enum_t = 250;
pub const mac_status_enum_t_UNAVAILABLE_DEVICE: mac_status_enum_t = 251;
pub const mac_status_enum_t_UNAVAILABLE_SECURITY_LEVEL: mac_status_enum_t = 252;
pub const mac_status_enum_t_RANGING_NOT_SUPPORTED: mac_status_enum_t = 253;
pub const mac_status_enum_t_INTERNAL_ERROR: mac_status_enum_t = 254;
pub const mac_status_enum_t_CONDITINALLY_PASSED: mac_status_enum_t = 255;
pub const mac_status_enum_t_INVALID_STATE: mac_status_enum_t = 208;
pub const mac_status_enum_t_MAC_STATUS_SUCCESS: mac_status_enum_t = 0;
#[doc = " @brief Enumeration representing any error codes that could result from calling a MAC API"]
pub type mac_status_enum_t = ::core::ffi::c_uint;
pub const mac_pib_id_ACK_WAIT_DUR_ID: mac_pib_id = 64;
pub const mac_pib_id_ASSOC_PAN_COORD_ID: mac_pib_id = 86;
pub const mac_pib_id_ASSOC_PERMIT_ID: mac_pib_id = 65;
pub const mac_pib_id_AUTO_REQ_ID: mac_pib_id = 66;
pub const mac_pib_id_BEACON_PAY_ID: mac_pib_id = 69;
pub const mac_pib_id_BECON_PAY_LEN_ID: mac_pib_id = 70;
pub const mac_pib_id_BEACON_ORDER_ID: mac_pib_id = 71;
pub const mac_pib_id_BEACON_TX_TIME_ID: mac_pib_id = 72;
pub const mac_pib_id_BSN_ID: mac_pib_id = 73;
pub const mac_pib_id_COORD_EXT_ADDR_ID: mac_pib_id = 74;
pub const mac_pib_id_COORD_SHRT_ADDR_ID: mac_pib_id = 75;
pub const mac_pib_id_DSN_ID: mac_pib_id = 76;
pub const mac_pib_id_MAX_BE_ID: mac_pib_id = 87;
pub const mac_pib_id_MAX_CSMA_ID: mac_pib_id = 78;
pub const mac_pib_id_MAX_FRAME_WAIT_TIME_ID: mac_pib_id = 88;
pub const mac_pib_id_MAX_FRAME_RETRY_ID: mac_pib_id = 89;
pub const mac_pib_id_MIN_BE_ID: mac_pib_id = 79;
pub const mac_pib_id_PAN_ID_ID: mac_pib_id = 80;
pub const mac_pib_id_PROMIS_MODE_ID: mac_pib_id = 81;
pub const mac_pib_id_RSP_WAIT_TIME_ID: mac_pib_id = 90;
pub const mac_pib_id_RX_WHEN_IDLE_ID: mac_pib_id = 82;
pub const mac_pib_id_SEC_ENABLED_ID: mac_pib_id = 93;
pub const mac_pib_id_SHORT_ADDR_ID: mac_pib_id = 83;
pub const mac_pib_id_SUPER_FRAME_ORD_ID: mac_pib_id = 84;
pub const mac_pib_id_SYNC_SYMB_OFFS_ID: mac_pib_id = 91;
pub const mac_pib_id_TIME_STAMP_SUPP_ID: mac_pib_id = 92;
pub const mac_pib_id_PERSISTEN_TIME_ID: mac_pib_id = 85;
pub const mac_pib_id_PHY_CHANNEL_ID: mac_pib_id = 0;
pub const mac_pib_id_PHY_CHANNEL_SUPPORTED: mac_pib_id = 1;
pub const mac_pib_id_PHY_CCA_MODE: mac_pib_id = 3;
pub const mac_pib_id_PHY_CURRENT_PAGE: mac_pib_id = 4;
pub const mac_pib_id_PHY_MAX_FRAME_DUR: mac_pib_id = 5;
pub const mac_pib_id_PHY_SHR_DUR: mac_pib_id = 6;
pub const mac_pib_id_PHY_SYMBOLS_PER_OCTET: mac_pib_id = 7;
pub const mac_pib_id_KEY_TBL_ID: mac_pib_id = 113;
pub const mac_pib_id_DEVICE_TBL_ID: mac_pib_id = 115;
pub const mac_pib_id_SEC_LVL_TBL_ID: mac_pib_id = 117;
pub const mac_pib_id_FRM_CNTR_ID: mac_pib_id = 119;
pub const mac_pib_id_AUTO_REQ_SEC_LVL_ID: mac_pib_id = 120;
pub const mac_pib_id_AUTO_REQ_KEY_MODE_ID: mac_pib_id = 121;
pub const mac_pib_id_AUTO_REQ_KEY_SRC_ID: mac_pib_id = 122;
pub const mac_pib_id_AUTO_REQ_KEY_INDX_ID: mac_pib_id = 123;
pub const mac_pib_id_DFLT_KEY_SRC_ID: mac_pib_id = 124;
pub const mac_pib_id_PAN_COORD_EXT_ADDR_ID: mac_pib_id = 125;
pub const mac_pib_id_PAN_COORD_SHRT_ADDR_ID: mac_pib_id = 126;
pub const mac_pib_id_CUSTOM_IEEE_EXTD_ADDRS: mac_pib_id = 127;
pub const mac_pib_id_EB_HDR_IE_LIST_ID: mac_pib_id = 128;
pub const mac_pib_id_EB_PYLD_IE_LIST_ID: mac_pib_id = 129;
pub const mac_pib_id_EB_FLTR_Enbld_ID: mac_pib_id = 130;
pub const mac_pib_id_EBSN_ID: mac_pib_id = 131;
pub const mac_pib_id_EB_AUTO_SA_ID: mac_pib_id = 132;
pub const mac_pib_id_EBR_PRMT_JOIN_ID: mac_pib_id = 133;
pub const mac_pib_id_EBR_FLTRS_ID: mac_pib_id = 134;
pub const mac_pib_id_EBR_LQ_ID: mac_pib_id = 135;
pub const mac_pib_id_EBR_PRCNT_FLTR_ID: mac_pib_id = 136;
pub const mac_pib_id_EBR_AUTO_RSPND_ID: mac_pib_id = 137;
pub const mac_pib_id_MAX_FULL_CSMA_FRAME_RETRY_ID: mac_pib_id = 138;
pub const mac_pib_id_IMPLICIT_BROADCAST_ID: mac_pib_id = 139;
pub const mac_pib_id_MIB_JOINING_IEEE_LIST_ID: mac_pib_id = 160;
pub const mac_pib_id_MIB_JOINING_POLICY_ID: mac_pib_id = 161;
pub const mac_pib_id_MIB_EXPIRY_INTRVL_ID: mac_pib_id = 162;
pub const mac_pib_id_MIB_EXPIRY_INTRVL_COUNTDOWN_ID: mac_pib_id = 163;
#[doc = " @brief enum represents MAC PIBs IDs\n"]
pub type mac_pib_id = ::core::ffi::c_uint;
pub const mac_addrs_mode_enum_t_NO_ADDRS: mac_addrs_mode_enum_t = 0;
pub const mac_addrs_mode_enum_t_RESERVED: mac_addrs_mode_enum_t = 1;
pub const mac_addrs_mode_enum_t_SHORT_ADDRS_16: mac_addrs_mode_enum_t = 2;
pub const mac_addrs_mode_enum_t_EXT_ADDRS_64: mac_addrs_mode_enum_t = 3;
pub const mac_addrs_mode_enum_t_NOT_USED: mac_addrs_mode_enum_t = 255;
#[doc = " @brief enum represents device address mode defined in 802.15.4 std"]
pub type mac_addrs_mode_enum_t = ::core::ffi::c_uint;
pub const scn_type_t_ED_SCAN: scn_type_t = 0;
pub const scn_type_t_MAC_ACTIV_SCAN: scn_type_t = 1;
pub const scn_type_t_MAC_PASS_SCAN: scn_type_t = 2;
pub const scn_type_t_ORPH_SCAN: scn_type_t = 3;
pub const scn_type_t_ENHANCED_ACTIV_SCAN: scn_type_t = 4;
#[doc = " @brief enum represents scanning types defined in 802.15.4 std"]
pub type scn_type_t = ::core::ffi::c_uint;
pub const bcon_typ_t_NORMAL_BEACON: bcon_typ_t = 0;
pub const bcon_typ_t_ENHANCED_BEACON: bcon_typ_t = 1;
#[doc = " @brief enum represents Beacon Frame types defined in 802.15.4 std"]
pub type bcon_typ_t = ::core::ffi::c_uint;
pub const mac_cmd_frm_t_ASSOC_REQ: mac_cmd_frm_t = 1;
pub const mac_cmd_frm_t_ASSOC_RSP: mac_cmd_frm_t = 2;
pub const mac_cmd_frm_t_DISASSOC_NOTF: mac_cmd_frm_t = 3;
pub const mac_cmd_frm_t_DATA_REQ: mac_cmd_frm_t = 4;
pub const mac_cmd_frm_t_PAN_ID_CONF_NOTF: mac_cmd_frm_t = 5;
pub const mac_cmd_frm_t_ORPH_NOTF: mac_cmd_frm_t = 6;
pub const mac_cmd_frm_t_BEACON_REQ: mac_cmd_frm_t = 7;
pub const mac_cmd_frm_t_COORD_REALIGN: mac_cmd_frm_t = 8;
#[doc = " @brief Enum represents MAC command frames\n"]
pub type mac_cmd_frm_t = ::core::ffi::c_uint;
pub const mac_frm_t_BEACON: mac_frm_t = 0;
pub const mac_frm_t_DATA: mac_frm_t = 1;
pub const mac_frm_t_ACK: mac_frm_t = 2;
pub const mac_frm_t_COMMAND: mac_frm_t = 3;
pub const mac_frm_t_FT_RESERVED: mac_frm_t = 4;
pub const mac_frm_t_FT_MULTIPURPOSE: mac_frm_t = 5;
pub const mac_frm_t_FT_FRAK: mac_frm_t = 6;
pub const mac_frm_t_FT_EXTENDED: mac_frm_t = 7;
pub const mac_frm_t_NO_FRAME: mac_frm_t = 255;
#[doc = " @brief Enum represents frame types defined in IEEE 802.15.4 standard"]
pub type mac_frm_t = ::core::ffi::c_uint;
pub const security_level_enum_no_security: security_level_enum = 0;
pub const security_level_enum_mic_32_sec_level: security_level_enum = 1;
pub const security_level_enum_mic_64_sec_level: security_level_enum = 2;
pub const security_level_enum_mic_128_sec_level: security_level_enum = 3;
pub const security_level_enum_enc_sec_level: security_level_enum = 4;
pub const security_level_enum_enc_mic_32_sec_level: security_level_enum = 5;
pub const security_level_enum_enc_mic_64_sec_level: security_level_enum = 6;
pub const security_level_enum_enc_mic_128_sec_level: security_level_enum = 7;
pub const security_level_enum_sec_lvls_num: security_level_enum = 8;
#[doc = " @brief indicates the actual frame protection that is provided. This value can be adapted on a frame-by-frame basis and allows for\n        varying levels of data authenticity (to allow minimization of security overhead in transmitted frames where required) and for\n        optional data confidentiality. The cryptographic protection offered by the various security levels is shown in Table 95.\n        When nontrivial protection is required, replay protection is always provided."]
pub type security_level_enum = ::core::ffi::c_uint;
#[doc = " @brief indicates the actual frame protection that is provided. This value can be adapted on a frame-by-frame basis and allows for\n        varying levels of data authenticity (to allow minimization of security overhead in transmitted frames where required) and for\n        optional data confidentiality. The cryptographic protection offered by the various security levels is shown in Table 95.\n        When nontrivial protection is required, replay protection is always provided."]
pub use self::security_level_enum as sec_level_enum_t;
#[doc = " @brief struct contains PIB attribute data"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr_arr_e {
    #[doc = "< PIB attribute data"]
    pub data: *mut u8,
    #[doc = "< length of PIB attribute data"]
    pub length: u16,
}
impl Default for attr_arr_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief struct contains PIB attribute data"]
pub type attr_arr_t = attr_arr_e;
#[doc = " @brief union contains PIB attribute value [bool/ single int / octet string]"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union attr_value_e {
    #[doc = "< PIB attribute value in case of single integer"]
    pub attr_int: u32,
    #[doc = "< PIB attribute value in case of boolean"]
    pub attr_bool: u32,
    #[doc = "< PIB attribute value in case of array"]
    pub attr_array: attr_arr_t,
}
impl Default for attr_value_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief union contains PIB attribute value [bool/ single int / octet string]"]
pub type attr_val_t = attr_value_e;
#[doc = " @brief structure represents common security parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prim_sec_param_e {
    #[doc = "< Frame counter field of the auxiliary security header"]
    pub frm_cntr: u32,
    #[doc = "< Key source field of the Key Identifier field of the auxiliary security header"]
    pub key_src: [u8; 8usize],
    #[doc = "< Security level field of the security control field of the auxiliary security header"]
    pub sec_level: sec_level_enum_t,
    #[doc = "< Key index field of the Key Identifier field of the auxiliary security header"]
    pub key_indx: u8,
    #[doc = "< Key ID mode field of  the security control field of the auxiliary security header"]
    pub key_id_mod: u8,
}
impl Default for prim_sec_param_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief structure represents common security parameters"]
pub type prim_sec_param_st = prim_sec_param_e;
#[doc = " @brief Union represents the device address\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _dev_addrs_un {
    #[doc = "< Extended address of the device"]
    pub dev_addrs_arr: [u8; 8usize],
    #[doc = "< Short address of the device"]
    pub shrt_addrs: u16,
}
impl Default for _dev_addrs_un {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Union represents the device address\n"]
pub type dev_addrs_un_t = _dev_addrs_un;
#[doc = " @brief Structure represents the elements of device descriptor\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _device_dscrp_st {
    #[doc = " The PAN identifier of the device in this DeviceDescriptor"]
    pub device_pan_id: u16,
    #[doc = " The short address of the device in this DeviceDescriptor.\n A value of 0xfffe indicates that this device is using only its extended address.\n A value of 0xffff indicates that this value is unknown"]
    pub device_short_addrs: u16,
    #[doc = " The extended IEEE address of the device"]
    pub dev_extd_addrs_arr: [u8; 8usize],
    #[doc = " The incoming frame counter of the device"]
    pub incoming_frame_counter: u32,
    #[doc = " Indication of whether the device may override the minimum security level settings"]
    pub exempt_min_sec_level: u8,
    #[doc = " TRUE means that this entry is used, FALSE means this entry not used"]
    pub tbl_set_flag: u8,
}
#[doc = " @brief Structure represents the elements of device descriptor\n"]
pub type device_dscrp_st_t = _device_dscrp_st;
#[doc = " @brief Structure represents the elements of key ID lookup descriptor"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _key_id_lookup_dscrp_st {
    #[doc = " Information used to identify the key. Present only if KeyIdMode is not equal to 0x00"]
    pub key_index: u32,
    #[doc = " The device address for this descriptor. Present only if KeyIdMode is equal to 0x00"]
    pub un_dev_addrs: dev_addrs_un_t,
    #[doc = " The PAN identifier for this descriptor. Present only if KeyIdMode is equal to 0x00"]
    pub dev_pan_id: u16,
    #[doc = " The mode used to for this descriptor"]
    pub key_id_mode: u8,
    #[doc = " Information to identify the key. Present only if KeyIdMode is equal to 0x02 or 0x03"]
    pub key_src_arr: [u8; 8usize],
    #[doc = " The addressing mode for this descriptor. Present only if KeyIdMode is equal to 0x00"]
    pub enum_device_addrs_mode: u8,
}
impl Default for _key_id_lookup_dscrp_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents the elements of key ID lookup descriptor"]
pub type key_id_lookup_dscrp_st_t = _key_id_lookup_dscrp_st;
#[doc = " @brief Structure represents the elements of key usage descriptor\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _key_usage_dscrp_st {
    #[doc = "< MAC frame type"]
    pub enum_mac_frm_type: u8,
    #[doc = "< MAC command frame ID"]
    pub enum_cmd_frm_id: u8,
}
#[doc = " @brief Structure represents the elements of key usage descriptor\n"]
pub type key_usage_dscrp_st_t = _key_usage_dscrp_st;
#[doc = " @brief Structure represents the elements of key descriptor\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _key_dscrp_st {
    #[doc = " A list of KeyIdLookupDescriptor entries used to identify this KeyDescriptor"]
    pub key_id_lookup_dscrp_list: [key_id_lookup_dscrp_st_t; 3usize],
    #[doc = " A list of implementation specific handles to DeviceDescriptor entries in macDeviceTable\n for each of the devices that are currently using this key"]
    pub device_dscrp_list: [device_dscrp_st_t; 8usize],
    #[doc = " A list of KeyUsageDescriptor entries indicating the frame types with which this key may be used"]
    pub key_usage_dscrp_st_list: [key_usage_dscrp_st_t; 5usize],
    #[doc = " The value of the key"]
    pub key_arr: [u8; 16usize],
    #[doc = " TRUE means that this entry is used, FALSE otherwise"]
    pub tbl_set_flag: u8,
}
impl Default for _key_dscrp_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents the elements of key descriptor\n"]
pub type key_dscrp_st_t = _key_dscrp_st;
pub const security_tbl_enum_MAC_SEC_LEVEL_TABLE: security_tbl_enum = 0;
pub const security_tbl_enum_MAC_DEVICE_TABLE: security_tbl_enum = 1;
pub const security_tbl_enum_MAC_KEY_TABLE: security_tbl_enum = 2;
pub const security_tbl_enum_KEY_ID_LOOKUP_DSCRP_LIST: security_tbl_enum = 3;
pub const security_tbl_enum_DEVICE_DSCRP_LIST: security_tbl_enum = 4;
pub const security_tbl_enum_KEY_USAGE_DSCRP_ST_LIST: security_tbl_enum = 5;
#[doc = " @brief Enum represents security table types that can be used in mlme-remove-sec_table"]
pub type security_tbl_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum represents security table types that can be used in mlme-remove-sec_table"]
pub use self::security_tbl_enum as sec_tbl_enum_t;
#[doc = " @brief Structure represents PAN descriptor parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pan_descr_e {
    #[doc = " Security information"]
    pub sec_params: prim_sec_param_st,
    #[doc = " The time at which the beacon frame was received, in symbols.\n This value is equal to the timestamp taken when the beacon frame was received\n The precision of this value shall be a minimum of 20 bits, with the lowest 4 bits being the least significant"]
    pub time_stamp: u32,
    #[doc = " The address of the coordinator as specified in the received beacon frame"]
    pub coord_addr: [u8; 8usize],
    #[doc = " The PAN ID of the coordinator as specified in the received beacon frame"]
    pub coord_pan_id: u16,
    #[doc = " The superframe specification as specified in the received beacon frame"]
    pub super_frm_spec: u16,
    #[doc = " The coordinator addressing mode corresponding to the received beacon frame"]
    pub coord_addr_mod: u8,
    #[doc = " The current channel number occupied by the network"]
    pub logic_chanl: u8,
    #[doc = " The current channel page occupied by the network"]
    pub chnl_pge: u8,
    #[doc = " The LQI at which the network beacon was received.\n Lower values represent lower LQI"]
    pub link_qual: u8,
    #[doc = " TRUE if the beacon is from the PAN coordinator that is accepting GTS requests"]
    pub gts_perm: u8,
    #[doc = " SUCCESS if there was no error in the security processing of the frame.\n One of the other status codes indicating an error in the security processing otherwise"]
    pub sec_fail_status: u8,
}
impl Default for pan_descr_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents PAN descriptor parameters"]
pub type pan_descr_st = pan_descr_e;
#[doc = " @brief structure represents MLME-SCAN.confirm parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_scan_conf_param_e {
    #[doc = " A list of the channels given in the request which were not scanned"]
    pub unscan_chnls: u32,
    #[doc = " The list of energy measurements, one for each channel searched during an ED scan"]
    pub enrgy_detec_lst: *mut u8,
    #[doc = " The list of PAN descriptors, one for each beacon found during an active or passive scan\nif macAutoRequest is set to TRUE"]
    pub pan_descr_lst: *mut pan_descr_st,
    #[doc = " Scan type"]
    pub scn_type: scn_type_t,
    #[doc = " The status of the scan request"]
    pub status: u8,
    #[doc = " The channel page on which the scan was performed"]
    pub chnl_pge: u8,
    #[doc = " The number of elements returned in the appropriate result lists"]
    pub rslt_size: u8,
}
impl Default for mlme_scan_conf_param_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief structure represents MLME-SCAN.confirm parameters"]
pub type mlme_scn_cmf_param_st = mlme_scan_conf_param_e;
#[doc = " @brief structure represents MLME-BEACON-NOTIFY.indication parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_bcon_notfy_params_e {
    #[doc = " Pointer to the PANDescriptor for the received Beacon frame."]
    pub pan_desc_ptr: *mut pan_descr_st,
    #[doc = " The list of addresses of the devices for which the beacon source has data"]
    pub addr_list: [u8; 112usize],
    #[doc = " The set of octets comprising the beacon payload to be transferred from the MAC\nsublayer entity to the next higher layer"]
    pub sdu: *mut u8,
    #[doc = " Pointer to header IEs"]
    pub ptr_hdr_ie_list: *mut u8,
    #[doc = "Pointer to payload IEs"]
    pub ptr_pyld_ie_list: *mut u8,
    #[doc = " Indicates a beacon(value 0x00) or enhanced beacon (value 0x01) was received."]
    pub bcon_type: u8,
    #[doc = " Number of header IEs"]
    pub hdr_ie_list_count: u8,
    #[doc = " Number of payload IEs"]
    pub pyld_ie_list_count: u8,
    #[doc = " Bits (0-2) indicates Number of Short Addresses Pending ,\n Bits (4-6) indicates Number of Extended Addresses Pending Bits 3,7 are reserved"]
    pub pend_addrs: u8,
    #[doc = " The beacon sequence number (0x00 - 0xff)"]
    pub bsn: u8,
    #[doc = " BSN used for Enhanced Beacon frames (0x00 - 0xff)"]
    pub ebsn: u8,
    #[doc = " The number of octets contained in the beacon payload of the beacon frame\n received by the MAC sublayer"]
    pub sdu_length: u8,
}
impl Default for mlme_bcon_notfy_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief structure represents MLME-BEACON-NOTIFY.indication parameters"]
pub type mlme_bcon_notfy_params_st = mlme_bcon_notfy_params_e;
#[doc = " @brief structure represents pan MLME-SYNC-LOSS.indication parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_sync_loss_params_e {
    #[doc = " Security Information"]
    pub sec_params: prim_sec_param_st,
    #[doc = " The PAN ID with which the device lost synchronization or to which it was realigned."]
    pub pan_id: u16,
    #[doc = " The reason that synchronization was lost"]
    pub loss_reason: u8,
    #[doc = " The channel number on which the device lost synchronization or to which it was realigned"]
    pub chnl_num: u8,
    #[doc = " The channel page on which the device lost synchronization or to which it was realigned"]
    pub chnl_pge: u8,
}
impl Default for mlme_sync_loss_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief structure represents pan MLME-SYNC-LOSS.indication parameters"]
pub type mlme_sync_loss_params_st_t = mlme_sync_loss_params_e;
#[doc = " @brief Structure represents MCPS-DATA.indication parameters\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcps_indicate_params_st_t {
    #[doc = " Optional. The time, in symbols, at which the data were received"]
    pub timestamp: u32,
    #[doc = " The individual device address of the entity from which the MSDU was received."]
    pub src_addrs: [u8; 8usize],
    #[doc = " The individual device address of the entity to which the MSDU is being transferred"]
    pub dstn_addrs: [u8; 8usize],
    #[doc = " The 16-bit PAN identifier of the entity from which the MSDU was received."]
    pub src_pan_id: u16,
    #[doc = " The 16-bit PAN identifier of the entity to which the MSDU is being transferred."]
    pub dstn_pan_id: u16,
    #[doc = " The destination addressing mode for this primitive corresponding to the received MPDU. This value can take one of the\nfollowing values:\n0x00 = no address, 0x01 = reserved, 0x02 = 16-bit short address, 0x03 = 64-bit extended address."]
    pub enum_dstn_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = " The source addressing mode for this primitive corresponding to the received MPDU.. This value can take one of the following values:\n 0x00 = no address, 0x01 = reserved, 0x02 = 16-bit short address, 0x03 = 64-bit extended address."]
    pub enum_src_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = " The number of octets contained in the MSDU being indicated by the MAC sublayer entity."]
    pub msdu_len: u8,
    #[doc = " The set of octets forming the MSDU being indicated by the MAC sublayer entity.."]
    pub ptr_msdu: *mut u8,
    #[doc = "  The DSN of the received data frame."]
    pub dsn: u8,
    #[doc = " LQI value measured during reception of the MPDU. Lower values represent lower LQI"]
    pub mpdu_link_qlty: u8,
    #[doc = " The Received Signal Strength Indicator is a measure of the RF power level at the input of the transceiver\n measured during the PHR and is valid after the SFD is detected"]
    pub mpdu_rssi: u8,
    #[doc = " The security level purportedly used by the received data frame"]
    pub enum_mcps_security_level: sec_level_enum_t,
    #[doc = " The mode used to identify the key purportedly used by the originator of the received frame"]
    pub key_id_mode: u8,
    #[doc = " The index of the key purportedly used by the originator of the received frame"]
    pub key_index: u8,
    #[doc = " The originator of the key purportedly used by the originator of the received frame"]
    pub key_src: [u8; 8usize],
    #[doc = " Frame pending bit field from received data frame"]
    pub frame_pending: u8,
}
impl Default for mcps_indicate_params_st_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const mlme_assoc_status_enum_ASSOCIATION_SUCCESS: mlme_assoc_status_enum = 0;
pub const mlme_assoc_status_enum_AT_CAPACITY: mlme_assoc_status_enum = 1;
pub const mlme_assoc_status_enum_ACCESS_DENIED: mlme_assoc_status_enum = 2;
#[doc = " @brief Enum represents MLME association status\n"]
pub type mlme_assoc_status_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum represents MLME association status\n"]
pub use self::mlme_assoc_status_enum as mlme_assoc_status_enum_t;
#[doc = " @brief Structure represents MLME-ASSOCIATE.indication parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_assoc_ind_params_e {
    #[doc = "< Security information"]
    pub sec_params: prim_sec_param_st,
    #[doc = "< The address of the device requesting association"]
    pub dev_addr: [u8; 8usize],
    #[doc = "< The operational capabilities of the device requesting association"]
    pub cap_info: u8,
}
impl Default for mlme_assoc_ind_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-ASSOCIATE.indication parameters"]
pub type mlme_assoc_ind_param_st = mlme_assoc_ind_params_e;
#[doc = " @brief Structure represents MLME-ASSOCIATE.response input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_assoc_res_params_e {
    #[doc = "< The address of the device requesting association"]
    pub dev_addr: [u8; 8usize],
    #[doc = " The short device address allocated by the coordinator on successful association.\n This parameter is set to 0xffff if the association was unsuccessful"]
    pub dev_short_addr: u16,
    #[doc = "< The status of the association attempt"]
    pub status: mlme_assoc_status_enum_t,
    #[doc = "< Security Information"]
    pub sec_params: prim_sec_param_st,
}
impl Default for mlme_assoc_res_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-ASSOCIATE.response input parameters"]
pub type mlme_assoc_res_param_st = mlme_assoc_res_params_e;
#[doc = " @brief Structure represents MLME-ASSOCIATE.request input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_assoc_req_params_e {
    #[doc = "< The coordinator addressing mode for this primitive and subsequent MPDU"]
    pub coord_addr_mod: mac_addrs_mode_enum_t,
    #[doc = "< The address of the coordinator with which to associate"]
    pub coord_addr: [u8; 8usize],
    #[doc = "< The identifier of the PAN with which to associate"]
    pub coord_pan_id: u16,
    #[doc = "< The channel page on which to attempt association"]
    pub chnl_pge: u8,
    #[doc = "< The channel number on which to attempt association"]
    pub chann_num: u8,
    #[doc = "< Specifies the operational capabilities of the associating device"]
    pub capab_info: u8,
    #[doc = "< Security information"]
    pub sec_params: prim_sec_param_st,
}
impl Default for mlme_assoc_req_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-ASSOCIATE.request input parameters"]
pub type mlme_assoc_req_param_st = mlme_assoc_req_params_e;
#[doc = " @brief Structure represents MLME-DISASSOCIATE.request input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_disassoc_req_params_e {
    #[doc = "< Security information"]
    pub sec_params: prim_sec_param_st,
    #[doc = "< The address of the device to which to send the command"]
    pub dev_addr: [u8; 8usize],
    #[doc = "< The PAN ID of the device to which to send the command"]
    pub dev_pan_id: u16,
    #[doc = "< set to one if the Disassociation Notification command is to be sent indirectly"]
    pub tx_indirect: u8,
    #[doc = "< The addressing mode of the device to which to send the command"]
    pub addr_mod: mac_addrs_mode_enum_t,
    #[doc = "< The reason for the disassociation"]
    pub reason: u8,
}
impl Default for mlme_disassoc_req_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-DISASSOCIATE.request input parameters"]
pub type mlme_disassoc_req_param_st = mlme_disassoc_req_params_e;
#[doc = " @brief Structure represents MLME-SCAN.request input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_scn_req_params_e {
    #[doc = "< The channel numbers to be scanned"]
    pub scn_chnls: u32,
    #[doc = "< Pointer to header IE"]
    pub ptr_hdr_ie_list: *mut u8,
    #[doc = "< Pointer to payload IE"]
    pub ptr_pyld_ie_list: *mut u8,
    #[doc = "< Security Information"]
    pub sec_params: prim_sec_param_st,
    #[doc = "< Indicates the type of scan performed"]
    pub scn_type: scn_type_t,
    #[doc = "< Time to spend scanning on each channe"]
    pub scn_dur: u8,
    #[doc = "< The channel page on which to perform the scan"]
    pub chnl_pge: u8,
    #[doc = "< Number of header IE"]
    pub hdr_ie_list_count: u8,
    #[doc = "< Number of payload IE"]
    pub pyld_ie_list_count: u8,
    #[doc = "< Set to one if the sequence number is suppressed in the frame"]
    pub sn_supr: u8,
    #[doc = "< Information Element present(either header or payload IE"]
    pub ie_present: u8,
    #[doc = "< Total length of the header IE including its headers"]
    pub ie_hdr_total_len: u8,
    #[doc = "< Total length of the payload IE including its headers"]
    pub ie_payld_total_len: u8,
}
impl Default for mlme_scn_req_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-SCAN.request input parameters"]
pub type mlme_scn_req_param_st = mlme_scn_req_params_e;
#[doc = " @brief Structure represents MLME-POLL.request input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_poll_req_params_e {
    #[doc = "< coordinator address mode"]
    pub coord_addr_mod: mac_addrs_mode_enum_t,
    #[doc = "< coordinator address based on the mode"]
    pub coord_addr: [u8; 8usize],
    #[doc = "< Security Information"]
    pub sec_params: prim_sec_param_st,
    #[doc = "< PanId for coordinator"]
    pub coord_pan_id: u16,
}
impl Default for mlme_poll_req_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-POLL.request input parameters"]
pub type mlme_poll_req_param_st = mlme_poll_req_params_e;
#[doc = " @brief Structure represents MLME-Start.Request input parameters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_start_req_params_e {
    #[doc = " Security information to be used in coordinator realignment command"]
    pub coord_realign_sec_parms: prim_sec_param_st,
    #[doc = " Security information to be used in beacon frame"]
    pub beacn_sec_params: prim_sec_param_st,
    #[doc = " Pointer to header IEs that will be attached to the beacon frame"]
    pub ptr_hdr_ie_list: *mut u8,
    #[doc = " Pointer to payload IEs that will be attached to the beacon frame"]
    pub ptr_pyld_ie_list: *mut u8,
    #[doc = " This parameter is ignored for non-beacon enabled"]
    pub start_time: u32,
    #[doc = " Flag to indicate the role of this device (coordinator/pan-coordinator)"]
    pub pan_coord: u32,
    #[doc = " This parameter is ignored in non-beacon enabled network"]
    pub batt_life_ext: u32,
    #[doc = " This flag indicate if coordinator realignment is to be transmitted"]
    pub coord_realign: u32,
    #[doc = " The PAN identifier to be used by the device"]
    pub pan_id: u16,
    #[doc = " Channel to transmit beacon frame or coordinator realignment command frame on"]
    pub logic_chanl: u8,
    #[doc = " The channel page on which to begin listen and transmit"]
    pub chnl_pge: u8,
    #[doc = " Value of 15 indicates that the coordinator will not transmit periodic beacons (non-beacon enabled)"]
    pub bcon_ord: u8,
    #[doc = " Ignored in case of non-beacon enabled network"]
    pub super_frm_ord: u8,
    #[doc = " Flag to indicate existence of IEs in the sent frame"]
    pub ie_present: u8,
    #[doc = " Number of header IEs passed"]
    pub hdr_ie_list_count: u8,
    #[doc = " Number of payload IEs passed"]
    pub pyld_ie_list_count: u8,
    #[doc = " Total length of the header IE including its headers"]
    pub ie_hdr_total_len: u8,
    #[doc = " Total length of the payload IE including its headers"]
    pub ie_payld_total_len: u8,
}
impl Default for mlme_start_req_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MLME-Start.Request input parameters"]
pub type mlme_start_req_param_st = mlme_start_req_params_e;
#[doc = " @brief Structure contains all the information required for MLME-DISASSOCIATE.confirm primitive"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_disassoc_cfm_params_e {
    #[doc = "< Status of disassociation operation"]
    pub status: mac_status_enum_t,
    #[doc = "< Device addressing mode"]
    pub dev_addr_mode: mac_addrs_mode_enum_t,
    #[doc = "< Device PAN ID"]
    pub dev_pan_id: u16,
    #[doc = "< Device address"]
    pub dev_addr: [u8; 8usize],
}
impl Default for mlme_disassoc_cfm_params_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all the information required for MLME-DISASSOCIATE.confirm primitive"]
pub type mlme_disassoc_cfm_params_st = mlme_disassoc_cfm_params_e;
#[doc = " @brief Structure contains all the information required for MLME-DISASSOCIATE.indication primitive"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_disassoc_ind_e {
    #[doc = "< The address of the device requesting disassociation"]
    pub dev_addr: [u8; 8usize],
    #[doc = "< The reason for the disassociation"]
    pub reason: u8,
    #[doc = "< Security Information"]
    pub beacn_sec_params: prim_sec_param_st,
}
impl Default for mlme_disassoc_ind_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all the information required for MLME-DISASSOCIATE.indication primitive"]
pub type mlme_disassoc_ind_st = mlme_disassoc_ind_e;
#[doc = "< acknowledged transmission is required"]
pub const mcps_tx_options_mask_enum_ack_transm_msk: mcps_tx_options_mask_enum = 1;
#[doc = "< indirect transmission bit shift"]
pub const mcps_tx_options_mask_enum_indirect_transm_shift: mcps_tx_options_mask_enum = 2;
#[doc = "< indirect transmission"]
pub const mcps_tx_options_mask_enum_indirect_transm_msk: mcps_tx_options_mask_enum = 4;
#[doc = "mcps_data_types ***********************/\n/**\n @brief Enum represents TX options passed in mcps-data.request\n"]
pub type mcps_tx_options_mask_enum = ::core::ffi::c_uint;
#[doc = "mcps_data_types ***********************/\n/**\n @brief Enum represents TX options passed in mcps-data.request\n"]
pub use self::mcps_tx_options_mask_enum as mcps_tx_options_mask_enum_t;
#[doc = " @brief Structure contains all the input parameters for a the MCPS-DATA.Request"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_mcps_data_req_params {
    #[doc = " The source addressing mode for this primitive and subsequent MPDU. This value can take one of the following values:\n 0x00 = no address, 0x01 = reserved, 0x02 = 16-bit short address, 0x03 = 64-bit extended address."]
    pub enum_src_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = " The destination addressing mode for this primitive and subsequent MPDU. This value can take one of the\nfollowing values:\n0x00 = no address, 0x01 = reserved, 0x02 = 16-bit short address, 0x03 = 64-bit extended address."]
    pub enum_dstn_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = " The 16-bit PAN identifier of the entity to which the MSDU is being transferred."]
    pub dstn_pan_id: u16,
    #[doc = " The individual device address of the entity to which the MSDU is being transferred"]
    pub dstn_addrs: [u8; 8usize],
    #[doc = " The number of octets contained in the MSDU to be transmitted by the MAC sublayer entity."]
    pub msdu_len: u8,
    #[doc = " The set of octets forming the MSDU to be transmitted by the MAC sublayer entity."]
    pub ptr_msdu: *mut u8,
    #[doc = "  The handle associated with the MSDU to be transmitted by the MAC sublayer entity"]
    pub msdu_hndl: u8,
    #[doc = " The 3 bits (b0, b1, b2) indicate the transmission options for this MSDU.  For b0, 1 = acknowledged transmission,\n0 = unacknowledged transmission, For b1, 1 = GTS transmission, 0 = CAP transmission for a bcon-enabled PAN,\nFor b2, 1 = indirect transmission, 0 = direct transmission. For a nonbcon-enabled PAN, bit b1 should always be set to 0."]
    pub tx_options: u8,
    #[doc = " The security level to be used"]
    pub enum_mcps_security_level: sec_level_enum_t,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The originator of the key to be used (see 7.6.2.4.1). This parameter is ignored if the KeyIdMode parameter is ignored or set to 0x00"]
    pub key_src: [u8; 8usize],
    #[doc = " The index of the key to be used (see 7.6.2.4.2). This parameter is ignored if the KeyIdMode parameter is ignored or set to 0x00."]
    pub key_index: u8,
}
impl Default for st_mcps_data_req_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all the input parameters for a the MCPS-DATA.Request"]
pub type mcps_data_req_params_st_t = st_mcps_data_req_params;
#[doc = " @brief Structure contains the private data to be used in persistent timer call back\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct persis_tmr_data_e {
    #[doc = "< Pointer to the MAC context"]
    pub ptr_mac_cntx: *mut ::core::ffi::c_void,
    #[doc = "< Pointer to the indirect packet which its persistent timer is fired"]
    pub ptr_persist_data_loc: *mut ::core::ffi::c_void,
}
impl Default for persis_tmr_data_e {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains the private data to be used in persistent timer call back\n"]
pub type persis_tmr_data_st = persis_tmr_data_e;
#[doc = " @brief Structure contains all the information required for MCPS-DATA.confirm primitive\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct mcps_data_cfm_params_st_t {
    #[doc = "< Optional. The time, in symbols, at which the data were transmitted"]
    pub timestamp: u32,
    #[doc = "< The handle associated with the MSDU being confirmed"]
    pub msdu_hndl: u8,
    #[doc = "< The status of the last MSDU transmission"]
    pub enum_data_tx_status: u8,
}
#[doc = " @brief Structure contains all information related to MLME-COMM-STATUS.indication primitive"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_comm_status_st_t {
    #[doc = " Security Information"]
    pub sec_params: prim_sec_param_st,
    #[doc = " Source address"]
    pub src_add: [u8; 8usize],
    #[doc = "The address of the device for which the frame was intended"]
    pub dst_addr: [u8; 8usize],
    #[doc = " The PAN ID of the device from which the frame was received or to which the frame was being sent"]
    pub pan_id: u16,
    #[doc = " Source addressing mode"]
    pub src_addr_mode: u8,
    #[doc = " Destination addressing mode"]
    pub dst_addr_mode: u8,
    #[doc = " The communications status"]
    pub status: u8,
}
impl Default for mlme_comm_status_st_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all information related to MLME-ORPHAN.indication primitive\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_orphan_ind_st_t {
    #[doc = " Orphan device address"]
    pub orphan_addr: [u8; 8usize],
    #[doc = " Security Information for orphan notification command"]
    pub sec_params: prim_sec_param_st,
}
impl Default for mlme_orphan_ind_st_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all information related to MLME-ORPHAN.response primitive\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlme_orphan_rsp_st_t {
    #[doc = " Security Information for orphan response frame"]
    pub sec_params: prim_sec_param_st,
    #[doc = " Orphan device address"]
    pub orphan_addr: [u8; 8usize],
    #[doc = " The short address allocated to the orphaned device if it is associated with this coordinator.\n The special short address 0xfffe indicates that no short address was allocated,and the device\n will use its extended address in all communications. If the device was not associated with\n this coordinator, this field will contain the value 0xffff and be ignored on receipt"]
    pub shrt_addr: u16,
    #[doc = " TRUE if the orphaned device is associated with this coordinator or FALSE otherwise.\n If False function will ignore the call for MLME-ORPHAN.Response"]
    pub assoc_member: u8,
}
impl Default for mlme_orphan_rsp_st_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all information related to MLME-BEAON-Req.Indication primitive\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct mlme_bcon_req_ind_params_e {
    #[doc = "< Source address for the device sending beacon request"]
    pub src_addr: [u8; 8usize],
    #[doc = "< Parsed header IEs from beacon request frame"]
    pub ptr_hdr_ie_list: [u8; 3usize],
    #[doc = "< Parsed payload IEs from beacon request frame"]
    pub ptr_pyld_ie_list: [u8; 28usize],
    #[doc = "< PANID for the device receiving beacon request"]
    pub dst_pan_id: u16,
    #[doc = "< Type of beacon frame required to be sent on response for beacon request Beacon/Enhanced Beacon"]
    pub bcon_type: u8,
    #[doc = "< Source address mode for the device sending beacon request"]
    pub src_addr_mode: u8,
    #[doc = "< Count of header IEs after filtering the unrecognized IEs"]
    pub hdr_ie_list_count: u8,
    #[doc = "< Count of payload IEs after filtering the unrecognized IEs"]
    pub pyld_ie_list_count: u8,
    #[doc = "< Length of header IEs after filtering the unrecognized IEs"]
    pub pyld_ie_total_len: u8,
    #[doc = "< Length of payload IEs after filtering the unrecognized HDR IEs"]
    pub hdr_ie_total_len: u8,
}
#[doc = " @brief Structure contains all information related to MLME-BEAON-Req.Indication primitive\n"]
pub type mlme_bcon_req_ind_params_st = mlme_bcon_req_ind_params_e;
#[doc = " @brief Structure contains all information related to MLME-BEAON-SEND.Request primitive\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_mlme_bcon_send_req_params {
    #[doc = "< Key source for Security information"]
    pub key_src: [u8; 8usize],
    #[doc = "< Address of the device to send beacon to in response of beacon request"]
    pub dstn_addrs: [u8; 8usize],
    #[doc = "< Pointer to header IEs to be attached to beacon frame in case of Enhanced beacon"]
    pub ptr_hdr_ie_list: *mut u8,
    #[doc = "< Pointer to payload IEs to be attached to beacon frame in case of Enhanced beacon"]
    pub ptr_pyld_ie_list: *mut u8,
    #[doc = "< Beacon type: Enhanced Beacon / Beacon"]
    pub enum_bcon_typ: bcon_typ_t,
    #[doc = "< Destination address mode"]
    pub enum_dstn_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = "< Source address mode"]
    pub enum_src_addrs_mode: mac_addrs_mode_enum_t,
    #[doc = "< Security level for security information"]
    pub enum_mcps_security_level: sec_level_enum_t,
    #[doc = "< Channel to transmit beacon frame on"]
    pub chnnl_num: u8,
    #[doc = "< Channel page to transmit beacon frame on"]
    pub chnnl_page: u8,
    #[doc = "< Ignored in non-beacon enabled"]
    pub super_frm_ord: u8,
    #[doc = "< Count of header IEs"]
    pub hdr_ie_list_count: u8,
    #[doc = "< Count of payload IEs"]
    pub pyld_ie_list_count: u8,
    #[doc = "< Total len of header IEs"]
    pub hdr_ie_total_len: u8,
    #[doc = "< Total len of header IEs"]
    pub pyld_ie_total_len: u8,
    #[doc = "< Key id mode used for security processing"]
    pub key_id_mode: u8,
    #[doc = "< Key index mode used for security processing"]
    pub key_index: u8,
    #[doc = "< Beacon sequence number suppression flag used to suppress SN"]
    pub bsn_supr: u8,
}
impl Default for st_mlme_bcon_send_req_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure contains all information related to MLME-BEAON-SEND.Request primitive\n"]
pub type mlme_bcon_send_req_params_st_t = st_mlme_bcon_send_req_params;
#[doc = " @brief Structure representing all the call backs that could be called from MAC to the upper host\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct mac_dispatch_tbl {
    #[doc = " @brief  MLME-RX-ENABLE primitive callback\n\n @param  status\t\t\t: [in] indicates the status of rx enable or disable operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_rx_enable_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-RESET primitive callback\n\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_rst_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-REMOVE-SEC-TABLE.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param   tbl_type\t \t: [in] Indicate type of removed table\n\n @retval None."]
    pub mlme_rmv_sec_tbl_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8, tbl_type: u8),
    >,
    #[doc = " @brief  MLME-SET.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  pib_attr_id\t\t: [in] Identifier of PibAttribute sent in set request primitive\n @param  pib_attr_indx\t: [in] Index for MAC PIBAttributes\n\n @retval None."]
    pub mlme_set_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            pib_attr_id: u8,
            pib_attr_indx: u8,
        ),
    >,
    #[doc = " @brief  MLME-GET.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  pib_attr_id\t\t: [in] PIB attribute ID\n @param  pib_attr_indx\t: [in] PIB attribute index\n @param  pib_attr_val\t\t: [in] PIB value\n\n @retval None."]
    pub mlme_get_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            pib_attr_id: u8,
            pib_attr_indx: u8,
            pib_attr_val: *mut attr_val_t,
        ),
    >,
    #[doc = " @brief  MLME-SET-ENABLE-CSMA.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n\n @retval None."]
    pub mlme_set_csma_en_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-SET-ENABLE-CCA.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n\n @retval None."]
    pub mlme_set_cca_en_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-SET-CCA-THRESHOLD.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param cca_thresold\t\t: [in] indicates cca threshold that set\n\n @retval None."]
    pub mlme_set_cca_threshold_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8, cca_thresold: i8),
    >,
    #[doc = " @brief  MLME-GET-CCA-THRESHOLD.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param cca_thresold\t\t: [in] indicates currently used cca threshold\n\n @retval None."]
    pub mlme_get_cca_threshold_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8, cca_thresold: i8),
    >,
    #[doc = " @brief  MLME-SET-ANT-DIV-PARAMS.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param cca_thresold\t\t: [in] indicates currently used cca threshold\n\n @retval None."]
    pub mlme_set_ant_div_params_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            ptr_ant_div_params: *mut antenna_diversity_st,
        ),
    >,
    #[doc = " @brief  MLME-SET-ANT-DIV-EN.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param enable\t\t    : [in] enable/disable antenna diversity\n\n @retval None."]
    pub mlme_set_ant_div_en_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8, enable: u8),
    >,
    #[doc = " @brief  MLME-SET-ANT-DIV-EN.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param default_ant_id\t: [in] indicates the antenna id to be used as default\n\n @retval None."]
    pub mlme_set_default_ant_id_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            default_ant_id: u8,
        ),
    >,
    #[doc = " @brief  MLME-SET-ANT-DIV-EN.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  status\t\t\t: [in] indicates the status of reset operation\n @param rssi_threshold\t: [in] indicates good quality rssi threshold\n\n @retval None."]
    pub mlme_set_ant_div_rssi_threshold_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            rssi_threshold: i8,
        ),
    >,
    #[doc = " @brief  MLME-SET-CONFIG-LIB-PARAMS.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t     : [in] indicate the used mac context\n @param  status\t\t\t     : [in] indicates the status of setting configurable library params\n @param  ptr_config_lib_params : [in] indicates the pointer of currently used configurable library parameters\n\n @retval None."]
    pub mlme_set_config_lib_params_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            ptr_config_lib_params: *mut config_lib_st,
        ),
    >,
    #[doc = " @brief  MLME-INIT.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t     : [in] indicate the used mac context\n @param  status\t\t\t     : [in] indicates the status of init operation\n\n @retval None."]
    pub mlme_init_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-GET_KEY_TBL.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of get operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  pib_attr_id\t\t: [in] PIB attribute ID\n @param  pib_attr_indx\t: [in] PIB attribute index\n @param  pib_attr_val\t\t: [in] PIB value\n\n @retval None."]
    pub mlme_get_key_tbl_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            pib_attr_id: u8,
            pib_attr_indx: u8,
            pib_attr_val: *mut attr_val_t,
        ),
    >,
    #[doc = " @brief  MLME-POLL.CONFIRM confirm callback\n\n @param  status\t\t\t: [in] indicates the status of poll request\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_poll_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MLME-SCAN.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t: [in] indicate the used mac context\n @param  mlme_scan_cmf_params\t\t: [in] scan confirm parameters returned to host\n\n @retval None."]
    pub mlme_scn_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            mlme_scan_cmf_params: *mut mlme_scn_cmf_param_st,
        ),
    >,
    #[doc = " @brief  MLME_ASSOCIATION.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of assoc request\n @param  short_addr\t\t: [in] indicates device short address received in assoc confirm\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n @param  sec_params\t\t: [in] security parameters\n\n @retval None."]
    pub mlme_assoc_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            status: u8,
            short_addr: u16,
            ptr_sec_params: *mut prim_sec_param_st,
        ),
    >,
    #[doc = " @brief  MLME-BEACON-NOTIFY.INDICATION primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t: [in] indicate the used mac context\n @param  ptr_bcon_notify_params\t: [in] beacon notify parameters\n\n @retval None."]
    pub mlme_bcon_notfy: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_bcon_notify_params: *mut mlme_bcon_notfy_params_st,
        ),
    >,
    #[doc = " @brief  MCPS-DATA.INDICATION primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t: [in] indicate the used mac context\n @param  ptr_mcps_data_ind\t\t: [in] data indication parameters\n\n @retval None."]
    pub mcps_data_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mcps_data_ind: *mut mcps_indicate_params_st_t,
        ),
    >,
    #[doc = " @brief  MLME-SYNC-LOSS.Indication primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t: [in] indicate the used mac context\n @param  ptr_st_mlme_sync_loss\t: [in] sync loss indication parameters\n\n @retval None."]
    pub mlme_sync_loss_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_st_mlme_sync_loss: *mut mlme_sync_loss_params_st_t,
        ),
    >,
    #[doc = " @brief  MLME-START.CONFIRM primitive callback\n\n @param  status\t\t\t: [in] indicates the status of start operation\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_strt_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  MCPS-DATA.CONFIRM primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t : [in] indicate the used mac context\n @param  mcps_data_cfm_params_st_t : [in] data confirm parameters\n\n @retval None."]
    pub mcps_data_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mcps_data_ind: *mut mcps_data_cfm_params_st_t,
        ),
    >,
    #[doc = " @brief  MLME-POLL.indication primitive callback\n\n @param  mac_cntx_ptr\t\t\t: [in] indicate the used mac context\n @param  addr_mode \t\t\t: [in] device address mode of received data request\n @param  dev_addr \t\t\t: [in] pointer to device address of received data request\n\n @retval None."]
    pub mlme_poll_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            addr_mode: u8,
            dev_addr: *mut u8,
        ),
    >,
    #[doc = " @brief  MLME-COMM-STATUS.Indication primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t: [in] indicate the used mac context\n @param  ptr_mlme_comm_status \t: [in] communication status primitive parameters\n\n @retval None."]
    pub mlme_comm_status: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mlme_comm_status: *mut mlme_comm_status_st_t,
        ),
    >,
    #[doc = " @brief   MLME-Disassociate.confirm primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t \t: [in] indicate the used mac context\n @param  mlme_disassoc_cfm_params_st  : [in] disassoc confirm parameters\n\n @retval None."]
    pub mlme_disassoc_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mlme_disassoc_cfm_params: *mut mlme_disassoc_cfm_params_st,
        ),
    >,
    #[doc = " @brief  MLME-DISASSOCIATION.INDICATION primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t \t: [in] indicate the used mac context\n @param  ptr_mlme_disassoc_ind_params : [in] disassoc indication parameters\n\n @retval None."]
    pub mlme_disassoc_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mlme_disassoc_ind_params: *mut mlme_disassoc_ind_st,
        ),
    >,
    #[doc = " @brief  MLME-ASSOCIATION.Indication callback\n\n @param  mlme_assoc_ind_param_st\t\t: [in] indicates the parameters of assoc request\n\n @param  mac_cntx_ptr\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_assoc_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            mlme_assoc_ind_params: *mut mlme_assoc_ind_param_st,
        ),
    >,
    #[doc = " @brief  MCPS-PURGE.confirm is generated as response for MCPS-PURGE.request primitive\n\n @param  mac_cntx_ptr\t: [in] indicate the used mac context\n @param  msdu_hndl    : [in] msdu handle\n @param  status\t    : [in] status for the purge request\n\n @retval None."]
    pub mcps_purge_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, msdu_hndl: u8, status: u8),
    >,
    #[doc = " @brief  MLME-orphan.indication primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t \t: [in] indicate the used mac context\n @param  ptr_mlme_orphan_ind_params   : [in] orphan indication parameters\n\n @retval None."]
    pub mlme_orphan_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_mlme_orphan_ind_params: *mut mlme_orphan_ind_st_t,
        ),
    >,
    #[doc = " @brief  MLME-BEACON-Request.indication primitive callback\n\n @param  mac_cntx_ptr\t\t\t\t \t: [in] indicate the used mac context\n @param  ptr_bcon_req_ind_params   \t: [in] beacon request indication parameters\n\n @retval None."]
    pub mlme_bcon_req_ind: ::core::option::Option<
        unsafe extern "C" fn(
            mac_cntx_ptr: *mut ::core::ffi::c_void,
            ptr_bcon_req_ind_params: *mut mlme_bcon_req_ind_params_st,
        ),
    >,
    #[doc = " @brief  MLME-BEACON.confirm primitive callback\n\n @param  status\t    \t\t: [in] status of beacon send request\n @param  mac_cntx_ptr\t\t\t: [in] indicate the used mac context\n\n @retval None."]
    pub mlme_bcon_cfm: ::core::option::Option<
        unsafe extern "C" fn(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  returns the status and the information re-quested by the MLME-SET-POWER-INFORMATION-TABLE.request.\n\n @param  mac_cntx_ptr\t: [in] indicate the used mac context\n @param  status\t    : [in] status error for the information re-quested\n\n @retval None."]
    pub mlme_set_pwr_info_table_cfm: ::core::option::Option<
        unsafe extern "C" fn(ptr_mac_cntx: *mut ::core::ffi::c_void, status: u8),
    >,
    #[doc = " @brief  returns the status and the information re-quested by the MLME-GET-POWER-INFORMATION-TABLE.request.\n\n @param  mac_cntx_ptr\t: [in] indicate the used mac context\n @param  status\t    : [in] status error for the information re-quested\n @param  short_addrs \t: [in] Short address of the link pair to transmit the packet to.\n @param  ptr_ext_addrs: [in] Pointer to Extended (IEEE) address of the link pair to transmit the packet to.\n @param  tx_pwr_level : [in] Tx power level of the link pair to transmit the packet to.\n @param  last_rssi_level : [in] RSSI of last packet received on the link pair\n @param  nwk_negotiated  : [in] Flag = 0 during the joining / rejoining process , 1 after joining/rejoining\n\n @retval None."]
    pub mlme_get_pwr_info_table_cfm: ::core::option::Option<
        unsafe extern "C" fn(
            ptr_mac_cntx: *mut ::core::ffi::c_void,
            status: u8,
            short_addrs: u16,
            ptr_ext_addrs: *mut u8,
            tx_pwr_level: i8,
            last_rssi_level: i8,
            nwk_negotiated: u8,
        ),
    >,
}
unsafe extern "C" {
    #[doc = " @brief   Mac Initialization function, it must be invoked once at the beginning\n\n @param[in]   mac_hndl \t\t\t: The MAC instance handle\n @param[in]   ptr_dispatch_tbl \t: Pointer to the dispatch table\n\n @retval Status to be sent to the Host"]
    pub fn mac_init(
        mac_hndl: *mut u32,
        ptr_dispatch_tbl: *mut mac_dispatch_tbl,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-BEACON.request primitive\n\n @param[in] mac_hndl \t\t\t\t\t\t: The MAC instance handle\n @param[in] ptr_st_bcon_send_req_params \t: Pointer to the beacon send request parameters\n\n @retval  Status to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_bcon_send_req(
        mac_hndl: u32,
        ptr_st_bcon_send_req_params: *mut mlme_bcon_send_req_params_st_t,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-ASSOCIATE.request, used by a device to request an association with a coordinator,\n\n @param[in] mac_hndl  \t\t\t\t: The MAC instance handle\n @param[in] ptr_st_assoc_req_param \t: Pointer to the association request parameters\n\n @retval  Status to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_assoc_req(
        mac_hndl: u32,
        ptr_st_assoc_req_param: *mut mlme_assoc_req_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-ASSOCIATE.response primitive.\n\n @param[in] mac_hndl \t\t\t\t\t: The MAC instance handle\n @param[in] ptr_st_assoc_res_param \t: Pointer to the association response parameters\n\n @retval \tStatus to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_assoc_res(
        mac_hndl: u32,
        ptr_st_assoc_res_param: *mut mlme_assoc_res_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-DISASSOCIATE.request primitive\n \t\t\tused by the device to disassociate from a PAN or used by the coordinator to disassociate a device from a PAN\n\n @param[in] mac_hndl \t\t\t\t\t: The MAC instance handle\n @param[in] mlme_disassoc_req_param \t: Pointer to disassociation request primitive params\n\n @retval \tStatus to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_disassoc_req(
        mac_hndl: u32,
        mlme_disassoc_req_param: *mut mlme_disassoc_req_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-GET.request primitive\n\n @param[in] mac_hndl\t \t: The MAC instance handle\n @param[in] pib_attr_id   : PIB attribute ID\n @param[in] pib_attr_indx : PIB attribute index\n @param[out] pib_attr_val : PIB value"]
    pub fn mlme_get_req(
        mac_hndl: u32,
        pib_attr_id: u8,
        pib_attr_indx: u8,
        pib_attr_val: *mut attr_val_t,
    );
}
unsafe extern "C" {
    #[doc = " @brief   MLME-RESET.request primitive\n\n @param[in] mac_hndl\t\t: The MAC instance handle\n @param[in] set_def_pib \t: Set default PIB flag"]
    pub fn mlme_rst_req(mac_hndl: u32, set_def_pib: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Destroy all the MAC handles registered\n"]
    pub fn mac_destroy();
}
unsafe extern "C" {
    #[doc = " @brief   MLME-SCAN.request primitive\n\n @param[in] mac_hndl\t\t\t\t: The MAC instance handle\n @param[in] mlme_scn_req_params \t: Pointer to request primitive params\n\n @retval \tStatus to be sent to the Host\n\n @note   \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk\n\t\t\tin cases of Active/Enhanced Active/Orphan Scaning only"]
    pub fn mlme_scn_req(
        mac_hndl: u32,
        mlme_scn_req_params: *mut mlme_scn_req_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Remove a mac_sec_level_table specified by pib_attr_indx\n\n @param[in] mac_hndl \t \t: The MAC instance handle\n @param[in] tbl_type\t \t: Type of removed table\n @param[in] tbl_index\t \t: Index of SecTable of main security attribute to be removed\n @param[in] tbl_sub_index : Index of SecTable of sub security attribute to be removed."]
    pub fn mlme_rmv_sec_table(mac_hndl: u32, tbl_type: u8, tbl_index: u8, tbl_sub_index: u8);
}
unsafe extern "C" {
    #[doc = " @brief   MLME-SET.request primitive\n\n @param[in] mac_hndl \t\t: The MAC instance handle\n @param[in] pib_attr_id \t: PIB attribute ID\n @param[in] pib_attr_indx : PIB attribute index\n @param[in] pib_attr_val \t: PIB value"]
    pub fn mlme_set_req(
        mac_hndl: u32,
        pib_attr_id: u8,
        pib_attr_indx: u8,
        pib_attr_val: *mut attr_val_t,
    );
}
unsafe extern "C" {
    #[doc = " @brief   MLME-START.request is used by the PAN coordinator to initiate a new PAN or to begin\n\t\t\tusing a new superframe configuration,mlme_strt_cfm callback is called to send\n\t\t\tMLME-START.confirm to the upper layers\n\t\t\tAs stated in standard the impact of new updates will be added immediately in case\n\t\t\tof idle state or after transmission of beacon in case a beacon prepared to be sent.\n\n @param[in] mac_hndl \t\t\t\t: The MAC instance handle\n @param[in] ptr_strt_req_params \t: Pointer to the start request primitive params\n\n @retval Status to be sent to the Host"]
    pub fn mlme_strt_req(
        mac_hndl: u32,
        ptr_strt_req_params: *mut mlme_start_req_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   The MLME-POLL.request primitive prompts the device to request data from the coordinator,\n\t\t\t mlme_poll_cfm callback will called by MLME to send MLME-POLL.confirm to the upper layers\n\n @param[in] mac_hndl \t\t\t\t: The MAC instance handle\n @param[in] ptr_st_poll_req_param : Pointer to the polling request parameters\n\n @retval  Status to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_poll_req(
        mac_hndl: u32,
        ptr_st_poll_req_param: *mut mlme_poll_req_param_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   MLME-ORPHAN.response primitive used to respond to orphan notification command\n\n @param[in] mac_hndl \t\t\t\t\t: The MAC instance handle\n @param[in] ptr_st_orphan_rsp_param \t: Pointer to the orphan response parameters\n\n @retval  Status to be sent to the Host\n\n @note    this function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_orphan_rsp(
        mac_hndl: u32,
        ptr_st_orphan_rsp_param: *mut mlme_orphan_rsp_st_t,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = "MCPS-SAP public functions*******************************************/\n/**\n @brief  MCPS-DATA.request primitive used to request the transfer of a data SPDU (i.e., MSDU) from a local SSCS\n         entity to a single peer SSCS entity.\n\n @param[in] ptr_st_mcps_data_req_params\t: Pointer to the data request parameters\n @param[in] mac_hndl\t            \t\t: The MAC instance handle that initiated data send request\n\n @retval \tStatus to be sent to the Host.\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t\twhen call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mcps_data_send_req(
        ptr_st_mcps_data_req_params: *mut mcps_data_req_params_st_t,
        mac_hndl: u32,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  The MCPS-PURGE.request primitive allows the next\n \t\t   higher layer to purge an MSDU from the transaction queue\n\n @param[in] mac_hndl \t: The MAC instance handle\n @param[in] msdu_hndl : The handle of the MSDU"]
    pub fn mcps_purge_req(mac_hndl: u32, msdu_hndl: u8);
}
unsafe extern "C" {
    #[doc = " @brief   MLME-RX-ENABLE.request primitive used to enable/disable RX for a given duration\n\n @param[in] mac_hndl      \t: The MAC instance handle\n @param[in] RxOnDuration     \t: Duration to enable RX with it starting from the current time , 0 means disable.\n @param[in] RangingRxControl \t: Ranging RX control it should be set to 0 \" RANGING_OFF \".\n\n @retval Status to be sent to the Host"]
    pub fn mlme_rx_enable_req(
        mac_hndl: u32,
        RxOnDuration: u32,
        RangingRxControl: u8,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Adds new entry to the Power Control Information table or update an existing entry\n\n @param[in] mac_hndl \t\t\t: The MAC instance handle\n @param[in] short_addrs \t \t: Short address of the link pair to transmit the packet to.\n @param[in] ptr_ext_addrs \t: Pointer to Extended (IEEE) address of the link pair to transmit the packet to.\n @param[in] tx_pwr_level \t \t: Tx power level of the link pair to transmit the packet to.\n @param[in] last_rssi_level  \t: RSSI of last packet received on the link pair\n @param[in] nwk_negotiated   \t: Flag = 0 during the joining / rejoining process , 1 after joining/rejoining\n @param[in] cfm_flag\t\t \t: Flag = 1 means that cfm is to be sent / = 0 otherwise\n\n @retval Status to be sent to the Host."]
    pub fn mlme_set_pwr_info_table_req(
        mac_hndl: u32,
        short_addrs: u16,
        ptr_ext_addrs: *mut u8,
        tx_pwr_level: i8,
        last_rssi_level: i8,
        nwk_negotiated: u8,
        cfm_flag: u8,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Returns the Power Control Information entry for the link pair\n\n @param[in] mac_hndl \t\t\t: The MAC instance handle\n @param[in] enum_addr_mode \t: The address mode of the link pair.\n @param[in] ptr_addrs \t\t: Pointer to the address of the link."]
    pub fn mlme_get_pwr_info_table_req(
        mac_hndl: u32,
        enum_addr_mode: mac_addrs_mode_enum_t,
        ptr_addrs: *mut u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief   MLME-BEACON.request primitive used to send beacon frame\n\n @param[in] mac_hndl\t  \t\t\t\t\t: The MAC instance handle\n @param[in] enum_bcon_typ \t\t\t\t: Beacon type [Beacon / Enhanced Beacon]\n @param[in] bcon_tx_pwr   \t\t\t\t: Tx power to be added in the tx poweer IE\n @param[in] ptr_st_bcon_send_req_params \t: Pointer to send beacon parameters\n\n @retval Status to be sent to the Host\n\n @note \tthis function will post DIRECT_DATA_TX_EVENT event that will be handled\n\t\t    when call emngr_handle_all_events to call the cbk direct_tx_evnt_cbk"]
    pub fn mlme_send_bcon_frm(
        mac_hndl: u32,
        enum_bcon_typ: bcon_typ_t,
        bcon_tx_pwr: i8,
        ptr_st_bcon_send_req_params: *mut mlme_bcon_send_req_params_st_t,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set CSMA enable flag\n\n @param[in] csma_en : Value for CSMA enable flag to be set"]
    pub fn mac_set_csma_en(csma_en: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Set CCA enable flag\n\n @param[in] cca_en : Value for CCA enable flag to be set"]
    pub fn mac_set_cca_en(cca_en: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Set CCA threshold\n\n @param[in] mac_hndl\t \t: The MAC instance handle\n @param[in] cca_thresold \t: Value of CCA threshold that set\n\n @retval Status to be sent to the Host ."]
    pub fn mac_set_cca_threshold(mac_hndl: u32, cca_thresold: i8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Get CCA threshold\n\n @param[in] mac_hndl\t \t: The MAC instance handle\n @param[out] cca_thresold : Pointer to the value of CCA threshold\n\n @retval Status to be sent to the Host"]
    pub fn mac_get_cca_threshold(mac_hndl: u32, cca_thresold: *mut i8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set antenna diversity parameters\n\n @param[in] mac_hndl\t   \t\t\t: The MAC instance handle\n @param[in] ptr_ant_div_params \t: Pointer to antenna diversity params\n\n @retval Status to be sent to the Host"]
    pub fn mac_set_ant_div_params(
        mac_hndl: u32,
        ptr_ant_div_params: *mut antenna_diversity_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Get antenna diversity parameters\n\n @param[in] mac_hndl\t   \t\t\t: The MAC instance handle\n @param[out] ptr_ant_div_params \t: Pointer to antenna diversity params"]
    pub fn mac_get_ant_div_params(mac_hndl: u32, ptr_ant_div_params: *mut antenna_diversity_st);
}
unsafe extern "C" {
    #[doc = " @brief   Enable/disable antenna diversity feature\n\n @param[in] mac_hndl\t: The MAC instance handle\n @param[in] enable    : Enable:1 / Disable:0\n\n @retval Status to be sent to the Host"]
    pub fn mac_set_ant_div_enable(mac_hndl: u32, enable: u8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set the default antenna id to be used for transmission and reception\n\n @param[in] mac_hndl\t    \t: The MAC instance handle\n @param[in] default_ant_id  \t: Antenna ID to be used as default\n\n @retval Status to be sent to the Host"]
    pub fn mac_set_default_ant_id(mac_hndl: u32, default_ant_id: u8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set antenna diversity RSSI threshold\n\n @param[in] mac_hndl\t     \t: The MAC instance handle\n @param[in] rssi_threshold   \t: RSSI threshold to compare with during antenna diversity measurements\n\n @retval Status to be sent to the Host"]
    pub fn mac_set_ant_div_rssi_threshold(mac_hndl: u32, rssi_threshold: i8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set configurable library parameters\n\n @param[in] mac_hndl\t          \t: The MAC instance handle\n @param[in] ptr_config_lib_params : Pointer to configurable library parameters\n\n @retval Status to be sent to the Host"]
    pub fn mac_set_config_lib_params(
        mac_hndl: u32,
        ptr_config_lib_params: *mut config_lib_st,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Get configurable library parameters\n\n @param[in] mac_hndl\t          \t\t: The MAC instance handle\n @param[out] ptr_config_lib_params\t: Pointer to configurable library parameters"]
    pub fn mac_get_config_lib_params(mac_hndl: u32, ptr_config_lib_params: *mut config_lib_st);
}
unsafe extern "C" {
    #[doc = " @brief   Set RTL polling time\n\n @param[in] mac_hndl\t     \t: The MAC instance handle\n @param[in] rtl_polling_time \t: RTL polling time"]
    pub fn mac_set_rtl_polling_time(mac_hndl: u32, rtl_polling_time: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Get RTL polling time\n\n @param[in] mac_hndl : The MAC instance handle\n\n @retval Current RTL polling time"]
    pub fn mac_get_rtl_polling_time(mac_hndl: u32) -> u8;
}
pub type __darwin_nl_item = ::core::ffi::c_int;
pub type __darwin_wctrans_t = ::core::ffi::c_int;
pub type __darwin_wctype_t = ::core::ffi::c_ulong;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::core::ffi::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::core::ffi::c_int;
pub type u_int8_t = ::core::ffi::c_uchar;
pub type u_int16_t = ::core::ffi::c_ushort;
pub type u_int32_t = ::core::ffi::c_uint;
pub type u_int64_t = ::core::ffi::c_ulonglong;
pub type register_t = i32;
pub type user_addr_t = u_int32_t;
pub type user_size_t = u_int32_t;
pub type user_ssize_t = i32;
pub type user_long_t = i32;
pub type user_ulong_t = u_int32_t;
pub type user_time_t = i32;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int32_t;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_exception_state {
    pub __exception: __uint32_t,
    pub __fsr: __uint32_t,
    pub __far: __uint32_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_exception_state64 {
    pub __far: __uint64_t,
    pub __esr: __uint32_t,
    pub __exception: __uint32_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_exception_state64_v2 {
    pub __far: __uint64_t,
    pub __esr: __uint64_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [__uint32_t; 13usize],
    pub __sp: __uint32_t,
    pub __lr: __uint32_t,
    pub __pc: __uint32_t,
    pub __cpsr: __uint32_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [__uint64_t; 29usize],
    pub __fp: __uint64_t,
    pub __lr: __uint64_t,
    pub __sp: __uint64_t,
    pub __pc: __uint64_t,
    pub __cpsr: __uint32_t,
    pub __pad: __uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_arm_vfp_state {
    pub __r: [__uint32_t; 64usize],
    pub __fpscr: __uint32_t,
}
impl Default for __darwin_arm_vfp_state {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct __darwin_arm_neon_state64 {
    pub opaque: [::core::ffi::c_char; 520usize],
}
impl Default for __darwin_arm_neon_state64 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct __darwin_arm_neon_state {
    pub opaque: [::core::ffi::c_char; 264usize],
}
impl Default for __darwin_arm_neon_state {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __arm_pagein_state {
    pub __pagein_error: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_sme_state {
    pub __svcr: __uint64_t,
    pub __tpidr2_el0: __uint64_t,
    pub __svl_b: __uint16_t,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub struct __darwin_arm_sve_z_state {
    pub __z: [[::core::ffi::c_char; 256usize]; 16usize],
}
impl Default for __darwin_arm_sve_z_state {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_sve_p_state {
    pub __p: [[::core::ffi::c_char; 32usize]; 16usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub struct __darwin_arm_sme_za_state {
    pub __za: [::core::ffi::c_char; 4096usize],
}
impl Default for __darwin_arm_sme_za_state {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone)]
pub struct __darwin_arm_sme2_state {
    pub __zt0: [::core::ffi::c_char; 64usize],
}
impl Default for __darwin_arm_sme2_state {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_debug_state {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_debug_state32 {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_debug_state64 {
    pub __bvr: [__uint64_t; 16usize],
    pub __bcr: [__uint64_t; 16usize],
    pub __wvr: [__uint64_t; 16usize],
    pub __wcr: [__uint64_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct __darwin_arm_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_arm_exception_state,
    pub __ss: __darwin_arm_thread_state,
    pub __fs: __darwin_arm_vfp_state,
}
impl Default for __darwin_mcontext32 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_arm_exception_state64,
    pub __ss: __darwin_arm_thread_state64,
    pub __ns: __darwin_arm_neon_state64,
}
impl Default for __darwin_mcontext64 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type mcontext_t = *mut __darwin_mcontext32;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::core::ffi::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::core::ffi::c_int,
}
impl Default for __darwin_sigaltstack {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type stack_t = __darwin_sigaltstack;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::core::ffi::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext32,
}
impl Default for __darwin_ucontext {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
impl Default for sigval {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::core::ffi::c_int,
    pub sigev_signo: ::core::ffi::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::core::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
impl Default for sigevent {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_value: sigval,
    pub si_band: ::core::ffi::c_long,
    pub __pad: [::core::ffi::c_ulong; 7usize],
}
impl Default for __siginfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type siginfo_t = __siginfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>,
    pub __sa_sigaction: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: ::core::ffi::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::core::ffi::c_void,
        ),
    >,
}
impl Default for __sigaction_u {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: ::core::ffi::c_int,
            arg3: ::core::ffi::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::core::ffi::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::core::ffi::c_int,
}
impl Default for __sigaction {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::core::ffi::c_int,
}
impl Default for sigaction {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type sig_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>,
    pub sv_mask: ::core::ffi::c_int,
    pub sv_flags: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::core::ffi::c_char,
    pub ss_onstack: ::core::ffi::c_int,
}
impl Default for sigstack {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn signal(
        arg1: ::core::ffi::c_int,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>,
    ) -> ::core::option::Option<
        unsafe extern "C" fn(
            arg1: ::core::ffi::c_int,
            arg2: ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>,
        ),
    >;
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::core::ffi::c_long,
    pub ru_ixrss: ::core::ffi::c_long,
    pub ru_idrss: ::core::ffi::c_long,
    pub ru_isrss: ::core::ffi::c_long,
    pub ru_minflt: ::core::ffi::c_long,
    pub ru_majflt: ::core::ffi::c_long,
    pub ru_nswap: ::core::ffi::c_long,
    pub ru_inblock: ::core::ffi::c_long,
    pub ru_oublock: ::core::ffi::c_long,
    pub ru_msgsnd: ::core::ffi::c_long,
    pub ru_msgrcv: ::core::ffi::c_long,
    pub ru_nsignals: ::core::ffi::c_long,
    pub ru_nvcsw: ::core::ffi::c_long,
    pub ru_nivcsw: ::core::ffi::c_long,
}
pub type rusage_info_t = *mut ::core::ffi::c_void;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v5 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rusage_info_v6 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
    pub ri_user_ptime: u64,
    pub ri_system_ptime: u64,
    pub ri_pinstructions: u64,
    pub ri_pcycles: u64,
    pub ri_energy_nj: u64,
    pub ri_penergy_nj: u64,
    pub ri_secure_time_in_system: u64,
    pub ri_secure_ptime_in_system: u64,
    pub ri_neural_footprint: u64,
    pub ri_lifetime_max_neural_footprint: u64,
    pub ri_interval_max_neural_footprint: u64,
    pub ri_conclave_footprint: u64,
    pub ri_page_wait_time_mach: u64,
    pub ri_page_cache_hits: u64,
    pub ri_reserved: [u64; 6usize],
}
pub type rusage_info_current = rusage_info_v6;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
unsafe extern "C" {
    pub fn getpriority(arg1: ::core::ffi::c_int, arg2: id_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getiopolicy_np(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getrlimit(arg1: ::core::ffi::c_int, arg2: *mut rlimit) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getrusage(arg1: ::core::ffi::c_int, arg2: *mut rusage) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setpriority(
        arg1: ::core::ffi::c_int,
        arg2: id_t,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setiopolicy_np(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setrlimit(arg1: ::core::ffi::c_int, arg2: *const rlimit) -> ::core::ffi::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::core::ffi::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Termsig_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                7u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Termsig_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Coredump_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Coredump_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Retcode_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Retcode_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Filler_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                16usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Filler_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::core::ffi::c_uint,
        w_Coredump: ::core::ffi::c_uint,
        w_Retcode: ::core::ffi::c_uint,
        w_Filler: ::core::ffi::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::core::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::core::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::core::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::core::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Stopval_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Stopval_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Stopsig_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Stopsig_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn w_Filler_raw(this: *const Self) -> ::core::ffi::c_uint {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                16usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_w_Filler_raw(this: *mut Self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::core::ffi::c_uint,
        w_Stopsig: ::core::ffi::c_uint,
        w_Filler: ::core::ffi::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::core::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::core::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::core::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for wait {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn wait(arg1: *mut ::core::ffi::c_int) -> pid_t;
}
unsafe extern "C" {
    pub fn waitpid(arg1: pid_t, arg2: *mut ::core::ffi::c_int, arg3: ::core::ffi::c_int) -> pid_t;
}
unsafe extern "C" {
    pub fn waitid(
        arg1: idtype_t,
        arg2: id_t,
        arg3: *mut siginfo_t,
        arg4: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn wait3(
        arg1: *mut ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: *mut rusage,
    ) -> pid_t;
}
unsafe extern "C" {
    pub fn wait4(
        arg1: pid_t,
        arg2: *mut ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: *mut rusage,
    ) -> pid_t;
}
unsafe extern "C" {
    pub fn alloca(__size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct div_t {
    pub quot: ::core::ffi::c_int,
    pub rem: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::core::ffi::c_long,
    pub rem: ::core::ffi::c_long,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::core::ffi::c_longlong,
    pub rem: ::core::ffi::c_longlong,
}
unsafe extern "C" {
    #[link_name = "\u{1}__mb_cur_max"]
    pub static mut __MB_CUR_MAX: ::core::ffi::c_int;
}
pub type malloc_type_id_t = ::core::ffi::c_ulonglong;
unsafe extern "C" {
    #[must_use]
    pub fn malloc(__size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[must_use]
    pub fn calloc(
        __count: ::core::ffi::c_uint,
        __size: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn free(arg1: *mut ::core::ffi::c_void);
}
unsafe extern "C" {
    #[must_use]
    pub fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[must_use]
    pub fn reallocf(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[must_use]
    pub fn valloc(__size: usize) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[must_use]
    pub fn aligned_alloc(
        __alignment: ::core::ffi::c_uint,
        __size: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::core::ffi::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn abort() -> !;
}
unsafe extern "C" {
    pub fn abs(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn atexit(arg1: ::core::option::Option<unsafe extern "C" fn()>) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn at_quick_exit(
        arg1: ::core::option::Option<unsafe extern "C" fn()>,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn atof(arg1: *const ::core::ffi::c_char) -> f64;
}
unsafe extern "C" {
    pub fn atoi(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn atol(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn atoll(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_longlong;
}
unsafe extern "C" {
    pub fn bsearch(
        __key: *const ::core::ffi::c_void,
        __base: *const ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn div(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int) -> div_t;
}
unsafe extern "C" {
    pub fn exit(arg1: ::core::ffi::c_int) -> !;
}
unsafe extern "C" {
    pub fn getenv(arg1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn labs(arg1: ::core::ffi::c_long) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn ldiv(arg1: ::core::ffi::c_long, arg2: ::core::ffi::c_long) -> ldiv_t;
}
unsafe extern "C" {
    pub fn llabs(arg1: ::core::ffi::c_longlong) -> ::core::ffi::c_longlong;
}
unsafe extern "C" {
    pub fn lldiv(arg1: ::core::ffi::c_longlong, arg2: ::core::ffi::c_longlong) -> lldiv_t;
}
unsafe extern "C" {
    pub fn mblen(__s: *const ::core::ffi::c_char, __n: usize) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn mbstowcs(arg1: *mut wchar_t, arg2: *const ::core::ffi::c_char, __n: usize) -> usize;
}
unsafe extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const ::core::ffi::c_char,
        __n: usize,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
unsafe extern "C" {
    pub fn quick_exit(arg1: ::core::ffi::c_int) -> !;
}
unsafe extern "C" {
    pub fn rand() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn srand(arg1: ::core::ffi::c_uint);
}
unsafe extern "C" {
    pub fn strtod(arg1: *const ::core::ffi::c_char, arg2: *mut *mut ::core::ffi::c_char) -> f64;
}
unsafe extern "C" {
    pub fn strtof(arg1: *const ::core::ffi::c_char, arg2: *mut *mut ::core::ffi::c_char) -> f32;
}
unsafe extern "C" {
    pub fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn strtold(arg1: *const ::core::ffi::c_char, arg2: *mut *mut ::core::ffi::c_char) -> f64;
}
unsafe extern "C" {
    pub fn strtoll(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
}
unsafe extern "C" {
    pub fn strtoul(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
}
unsafe extern "C" {
    pub fn strtoull(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
    pub fn system(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn wcstombs(arg1: *mut ::core::ffi::c_char, arg2: *const wchar_t, __n: usize) -> usize;
}
unsafe extern "C" {
    pub fn wctomb(arg1: *mut ::core::ffi::c_char, arg2: wchar_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn _Exit(arg1: ::core::ffi::c_int) -> !;
}
unsafe extern "C" {
    pub fn a64l(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn drand48() -> f64;
}
unsafe extern "C" {
    pub fn ecvt(
        arg1: f64,
        arg2: ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn erand48(arg1: *mut ::core::ffi::c_ushort) -> f64;
}
unsafe extern "C" {
    pub fn fcvt(
        arg1: f64,
        arg2: ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
        arg4: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn gcvt(
        arg1: f64,
        arg2: ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *const *mut ::core::ffi::c_char,
        arg3: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn grantpt(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn initstate(
        arg1: ::core::ffi::c_uint,
        arg2: *mut ::core::ffi::c_char,
        __size: usize,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn jrand48(arg1: *mut ::core::ffi::c_ushort) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn l64a(arg1: ::core::ffi::c_long) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn lcong48(arg1: *mut ::core::ffi::c_ushort);
}
unsafe extern "C" {
    pub fn lrand48() -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn mktemp(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn mkstemp(arg1: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn mrand48() -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn nrand48(arg1: *mut ::core::ffi::c_ushort) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn posix_openpt(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ptsname(arg1: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn ptsname_r(
        fildes: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_char,
        buflen: usize,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn putenv(arg1: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn random() -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn rand_r(arg1: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
    pub fn realpath(
        arg1: *const ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn seed48(arg1: *mut ::core::ffi::c_ushort) -> *mut ::core::ffi::c_ushort;
}
unsafe extern "C" {
    pub fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __overwrite: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setkey(arg1: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    pub fn setstate(arg1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn srand48(arg1: ::core::ffi::c_long);
}
unsafe extern "C" {
    pub fn srandom(arg1: ::core::ffi::c_uint);
}
unsafe extern "C" {
    pub fn unlockpt(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn unsetenv(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
unsafe extern "C" {
    pub fn arc4random() -> u32;
}
unsafe extern "C" {
    pub fn arc4random_addrandom(arg1: *mut ::core::ffi::c_uchar, __datlen: ::core::ffi::c_int);
}
unsafe extern "C" {
    pub fn arc4random_buf(__buf: *mut ::core::ffi::c_void, __nbytes: usize);
}
unsafe extern "C" {
    pub fn arc4random_stir();
}
unsafe extern "C" {
    pub fn arc4random_uniform(__upper_bound: u32) -> u32;
}
unsafe extern "C" {
    pub fn cgetcap(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn cgetclose() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetent(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *mut *mut ::core::ffi::c_char,
        arg3: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetfirst(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetmatch(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetnext(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetnum(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: *mut ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetset(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetstr(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn cgetustr(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn daemon(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn devname_r(
        arg1: dev_t,
        arg2: mode_t,
        buf: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn getbsize(
        arg1: *mut ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_long,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn getloadavg(arg1: *mut f64, __nelem: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getprogname() -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn setprogname(arg1: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    pub fn heapsort(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn mergesort(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn psort(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
unsafe extern "C" {
    pub fn psort_r(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::core::ffi::c_void,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
                arg3: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
unsafe extern "C" {
    pub fn qsort_r(
        __base: *mut ::core::ffi::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::core::ffi::c_void,
        __compar: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_void,
                arg3: *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
unsafe extern "C" {
    pub fn radixsort(
        __base: *mut *const ::core::ffi::c_uchar,
        __nel: ::core::ffi::c_int,
        __table: *const ::core::ffi::c_uchar,
        __endbyte: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn rpmatch(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sradixsort(
        __base: *mut *const ::core::ffi::c_uchar,
        __nel: ::core::ffi::c_int,
        __table: *const ::core::ffi::c_uchar,
        __endbyte: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sranddev();
}
unsafe extern "C" {
    pub fn srandomdev();
}
unsafe extern "C" {
    pub fn strtonum(
        __numstr: *const ::core::ffi::c_char,
        __minval: ::core::ffi::c_longlong,
        __maxval: ::core::ffi::c_longlong,
        __errstrp: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_longlong;
}
unsafe extern "C" {
    pub fn strtoq(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
}
unsafe extern "C" {
    pub fn strtouq(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
    #[link_name = "\u{1}suboptarg"]
    pub static mut SUBOPTARG: *mut ::core::ffi::c_char;
}
pub type __gnuc_va_list = u32;
pub type va_list = __gnuc_va_list;
#[doc = " Represents the log level.\n"]
pub type otLogLevel = ::core::ffi::c_int;
#[doc = "< OpenThread API"]
pub const otLogRegion_OT_LOG_REGION_API: otLogRegion = 1;
#[doc = "< MLE"]
pub const otLogRegion_OT_LOG_REGION_MLE: otLogRegion = 2;
#[doc = "< EID-to-RLOC mapping."]
pub const otLogRegion_OT_LOG_REGION_ARP: otLogRegion = 3;
#[doc = "< Network Data"]
pub const otLogRegion_OT_LOG_REGION_NET_DATA: otLogRegion = 4;
#[doc = "< ICMPv6"]
pub const otLogRegion_OT_LOG_REGION_ICMP: otLogRegion = 5;
#[doc = "< IPv6"]
pub const otLogRegion_OT_LOG_REGION_IP6: otLogRegion = 6;
#[doc = "< TCP"]
pub const otLogRegion_OT_LOG_REGION_TCP: otLogRegion = 7;
#[doc = "< IEEE 802.15.4 MAC"]
pub const otLogRegion_OT_LOG_REGION_MAC: otLogRegion = 8;
#[doc = "< Memory"]
pub const otLogRegion_OT_LOG_REGION_MEM: otLogRegion = 9;
#[doc = "< NCP"]
pub const otLogRegion_OT_LOG_REGION_NCP: otLogRegion = 10;
#[doc = "< Mesh Commissioning Protocol"]
pub const otLogRegion_OT_LOG_REGION_MESH_COP: otLogRegion = 11;
#[doc = "< Network Diagnostic"]
pub const otLogRegion_OT_LOG_REGION_NET_DIAG: otLogRegion = 12;
#[doc = "< Platform"]
pub const otLogRegion_OT_LOG_REGION_PLATFORM: otLogRegion = 13;
#[doc = "< CoAP"]
pub const otLogRegion_OT_LOG_REGION_COAP: otLogRegion = 14;
#[doc = "< CLI"]
pub const otLogRegion_OT_LOG_REGION_CLI: otLogRegion = 15;
#[doc = "< OpenThread Core"]
pub const otLogRegion_OT_LOG_REGION_CORE: otLogRegion = 16;
#[doc = "< Utility module"]
pub const otLogRegion_OT_LOG_REGION_UTIL: otLogRegion = 17;
#[doc = "< Backbone Router (available since Thread 1.2)"]
pub const otLogRegion_OT_LOG_REGION_BBR: otLogRegion = 18;
#[doc = "< Multicast Listener Registration (available since Thread 1.2)"]
pub const otLogRegion_OT_LOG_REGION_MLR: otLogRegion = 19;
#[doc = "< Domain Unicast Address (available since Thread 1.2)"]
pub const otLogRegion_OT_LOG_REGION_DUA: otLogRegion = 20;
#[doc = "< Border Router"]
pub const otLogRegion_OT_LOG_REGION_BR: otLogRegion = 21;
#[doc = "< Service Registration Protocol (SRP)"]
pub const otLogRegion_OT_LOG_REGION_SRP: otLogRegion = 22;
#[doc = "< DNS"]
pub const otLogRegion_OT_LOG_REGION_DNS: otLogRegion = 23;
#[doc = " Represents log regions.\n\n The support for log region is removed and instead each core module can define its own name to appended to the logs.\n However, the `otLogRegion` enumeration is still defined as before to help with platforms which we may be using it\n in their `otPlatLog()` implementation. The OT core will always emit all logs with `OT_LOG_REGION_CORE`.\n"]
pub type otLogRegion = ::core::ffi::c_uint;
unsafe extern "C" {
    #[doc = " Outputs logs.\n\n Note that the support for log region is removed. The OT core will always emit all logs with `OT_LOG_REGION_CORE`\n as @p aLogRegion.\n\n @param[in]  aLogLevel   The log level.\n @param[in]  aLogRegion  The log region.\n @param[in]  aFormat     A pointer to the format string.\n @param[in]  ...         Arguments for the format specification.\n"]
    pub fn otPlatLog(
        aLogLevel: otLogLevel,
        aLogRegion: otLogRegion,
        aFormat: *const ::core::ffi::c_char,
        ...
    );
}
unsafe extern "C" {
    #[doc = " Handles OpenThread log level changes.\n\n This platform function is called whenever the OpenThread log level changes.\n This platform function is optional since an empty weak implementation has been provided.\n\n @note Only applicable when `OPENTHREAD_CONFIG_LOG_LEVEL_DYNAMIC_ENABLE=1`.\n\n @param[in]  aLogLevel  The new OpenThread log level.\n"]
    pub fn otPlatLogHandleLevelChanged(aLogLevel: otLogLevel);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otInstance {
    _unused: [u8; 0],
}
unsafe extern "C" {
    #[doc = " Initializes the OpenThread library.\n\n Initializes OpenThread and prepares it for subsequent OpenThread API calls. This function must be\n called before any other calls to OpenThread.\n\n Is available and can only be used when support for multiple OpenThread instances is enabled.\n\n @param[in]     aInstanceBuffer      The buffer for OpenThread to use for allocating the otInstance structure.\n @param[in,out] aInstanceBufferSize  On input, the size of aInstanceBuffer. On output, if not enough space for\n                                     otInstance, the number of bytes required for otInstance.\n\n @returns  A pointer to the new OpenThread instance.\n\n @sa otInstanceFinalize\n"]
    pub fn otInstanceInit(
        aInstanceBuffer: *mut ::core::ffi::c_void,
        aInstanceBufferSize: *mut usize,
    ) -> *mut otInstance;
}
unsafe extern "C" {
    #[doc = " Initializes the static single instance of the OpenThread library.\n\n Initializes OpenThread and prepares it for subsequent OpenThread API calls. This function must be\n called before any other calls to OpenThread.\n\n Is available and can only be used when support for multiple OpenThread instances is disabled.\n\n @returns A pointer to the single OpenThread instance.\n"]
    pub fn otInstanceInitSingle() -> *mut otInstance;
}
unsafe extern "C" {
    #[doc = " Initializes the OpenThread instance.\n\n This function initializes OpenThread and prepares it for subsequent OpenThread API calls. This function must be\n called before any other calls to OpenThread. This method utilizes static buffer to initialize the OpenThread\n instance.\n\n This function is available and can only be used when support for multiple OpenThread static instances is\n enabled (`OPENTHREAD_CONFIG_MULTIPLE_STATIC_INSTANCE_ENABLE`)\n\n @param[in] aIdx The index of the OpenThread instance to initialize.\n\n @returns  A pointer to the new OpenThread instance.\n"]
    pub fn otInstanceInitMultiple(aIdx: u8) -> *mut otInstance;
}
unsafe extern "C" {
    #[doc = " Gets the instance identifier.\n\n The instance identifier is set to a random value when the instance is constructed, and then its value will not\n change after initialization.\n\n @returns The instance identifier.\n"]
    pub fn otInstanceGetId(aInstance: *mut otInstance) -> u32;
}
unsafe extern "C" {
    #[doc = " Indicates whether or not the instance is valid/initialized.\n\n The instance is considered valid if it is acquired and initialized using either `otInstanceInitSingle()` (in single\n instance case) or `otInstanceInit()` (in multi instance case). A subsequent call to `otInstanceFinalize()` causes\n the instance to be considered as uninitialized.\n\n @param[in] aInstance A pointer to an OpenThread instance.\n\n @returns TRUE if the given instance is valid/initialized, FALSE otherwise.\n"]
    pub fn otInstanceIsInitialized(aInstance: *mut otInstance) -> bool;
}
unsafe extern "C" {
    #[doc = " Disables the OpenThread library.\n\n Call this function when OpenThread is no longer in use.\n\n @param[in] aInstance A pointer to an OpenThread instance.\n"]
    pub fn otInstanceFinalize(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Returns the current instance uptime (in msec).\n\n Requires `OPENTHREAD_CONFIG_UPTIME_ENABLE` to be enabled.\n\n The uptime is given as number of milliseconds since OpenThread instance was initialized.\n\n @param[in] aInstance A pointer to an OpenThread instance.\n\n @returns The uptime (number of milliseconds).\n"]
    pub fn otInstanceGetUptime(aInstance: *mut otInstance) -> u64;
}
unsafe extern "C" {
    #[doc = " Returns the current instance uptime as a human-readable string.\n\n Requires `OPENTHREAD_CONFIG_UPTIME_ENABLE` to be enabled.\n\n The string follows the format \"<hh>:<mm>:<ss>.<mmmm>\" for hours, minutes, seconds and millisecond (if uptime is\n shorter than one day) or \"<dd>d.<hh>:<mm>:<ss>.<mmmm>\" (if longer than a day).\n\n If the resulting string does not fit in @p aBuffer (within its @p aSize characters), the string will be truncated\n but the outputted string is always null-terminated.\n\n @param[in]  aInstance A pointer to an OpenThread instance.\n @param[out] aBuffer   A pointer to a char array to output the string.\n @param[in]  aSize     The size of @p aBuffer (in bytes). Recommended to use `OT_UPTIME_STRING_SIZE`.\n"]
    pub fn otInstanceGetUptimeAsString(
        aInstance: *mut otInstance,
        aBuffer: *mut ::core::ffi::c_char,
        aSize: u16,
    );
}
#[doc = " Represents a bit-field indicating specific state/configuration that has changed. See `OT_CHANGED_*`\n definitions.\n"]
pub type otChangedFlags = u32;
#[doc = " Pointer is called to notify certain configuration or state changes within OpenThread.\n\n @param[in]  aFlags    A bit-field indicating specific state that has changed.  See `OT_CHANGED_*` definitions.\n @param[in]  aContext  A pointer to application-specific context.\n"]
pub type otStateChangedCallback = ::core::option::Option<
    unsafe extern "C" fn(aFlags: otChangedFlags, aContext: *mut ::core::ffi::c_void),
>;
unsafe extern "C" {
    #[doc = " Registers a callback to indicate when certain configuration or state changes within OpenThread.\n\n @param[in]  aInstance  A pointer to an OpenThread instance.\n @param[in]  aCallback  A pointer to a function that is called with certain configuration or state changes.\n @param[in]  aContext   A pointer to application-specific context.\n\n @retval OT_ERROR_NONE     Added the callback to the list of callbacks.\n @retval OT_ERROR_ALREADY  The callback was already registered.\n @retval OT_ERROR_NO_BUFS  Could not add the callback due to resource constraints.\n"]
    pub fn otSetStateChangedCallback(
        aInstance: *mut otInstance,
        aCallback: otStateChangedCallback,
        aContext: *mut ::core::ffi::c_void,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Removes a callback to indicate when certain configuration or state changes within OpenThread.\n\n @param[in]  aInstance   A pointer to an OpenThread instance.\n @param[in]  aCallback   A pointer to a function that is called with certain configuration or state changes.\n @param[in]  aContext    A pointer to application-specific context.\n"]
    pub fn otRemoveStateChangeCallback(
        aInstance: *mut otInstance,
        aCallback: otStateChangedCallback,
        aContext: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    #[doc = " Triggers a platform reset.\n\n The reset process ensures that all the OpenThread state/info (stored in volatile memory) is erased. Note that the\n `otPlatformReset` does not erase any persistent state/info saved in non-volatile memory.\n\n @param[in]  aInstance  A pointer to an OpenThread instance.\n"]
    pub fn otInstanceReset(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Triggers a platform reset to bootloader mode, if supported.\n\n Requires `OPENTHREAD_CONFIG_PLATFORM_BOOTLOADER_MODE_ENABLE`.\n\n @param[in]  aInstance  A pointer to an OpenThread instance.\n\n @retval OT_ERROR_NONE         Reset to bootloader successfully.\n @retval OT_ERROR_BUSY         Failed due to another operation is ongoing.\n @retval OT_ERROR_NOT_CAPABLE  Not capable of resetting to bootloader.\n"]
    pub fn otInstanceResetToBootloader(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Deletes all the settings stored on non-volatile memory, and then triggers a platform reset.\n\n @param[in]  aInstance  A pointer to an OpenThread instance.\n"]
    pub fn otInstanceFactoryReset(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Resets the internal states of the OpenThread radio stack.\n\n Callbacks and configurations are preserved.\n\n This API is only available under radio builds (`OPENTHREAD_RADIO = 1`).\n\n @param[in]  aInstance  A pointer to an OpenThread instance.\n"]
    pub fn otInstanceResetRadioStack(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Erases all the OpenThread persistent info (network settings) stored on non-volatile memory.\n Erase is successful only if the device is in `disabled` state/role.\n\n @param[in]  aInstance A pointer to an OpenThread instance.\n\n @retval OT_ERROR_NONE           All persistent info/state was erased successfully.\n @retval OT_ERROR_INVALID_STATE  Device is not in `disabled` state/role.\n"]
    pub fn otInstanceErasePersistentInfo(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Gets the OpenThread version string.\n\n @returns A pointer to the OpenThread version.\n"]
    pub fn otGetVersionString() -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    #[doc = " Gets the OpenThread radio version string.\n\n @param[in]  aInstance A pointer to an OpenThread instance.\n\n @returns A pointer to the OpenThread radio version.\n"]
    pub fn otGetRadioVersionString(aInstance: *mut otInstance) -> *const ::core::ffi::c_char;
}
#[doc = "< Key Type: Raw Data."]
pub const otCryptoKeyType_OT_CRYPTO_KEY_TYPE_RAW: otCryptoKeyType = 0;
#[doc = "< Key Type: AES."]
pub const otCryptoKeyType_OT_CRYPTO_KEY_TYPE_AES: otCryptoKeyType = 1;
#[doc = "< Key Type: HMAC."]
pub const otCryptoKeyType_OT_CRYPTO_KEY_TYPE_HMAC: otCryptoKeyType = 2;
#[doc = "< Key Type: ECDSA."]
pub const otCryptoKeyType_OT_CRYPTO_KEY_TYPE_ECDSA: otCryptoKeyType = 3;
#[doc = " Defines the key types.\n"]
pub type otCryptoKeyType = ::core::ffi::c_uint;
#[doc = "< Key Algorithm: Vendor Defined."]
pub const otCryptoKeyAlgorithm_OT_CRYPTO_KEY_ALG_VENDOR: otCryptoKeyAlgorithm = 0;
#[doc = "< Key Algorithm: AES ECB."]
pub const otCryptoKeyAlgorithm_OT_CRYPTO_KEY_ALG_AES_ECB: otCryptoKeyAlgorithm = 1;
#[doc = "< Key Algorithm: HMAC SHA-256."]
pub const otCryptoKeyAlgorithm_OT_CRYPTO_KEY_ALG_HMAC_SHA_256: otCryptoKeyAlgorithm = 2;
#[doc = "< Key Algorithm: ECDSA."]
pub const otCryptoKeyAlgorithm_OT_CRYPTO_KEY_ALG_ECDSA: otCryptoKeyAlgorithm = 3;
#[doc = " Defines the key algorithms.\n"]
pub type otCryptoKeyAlgorithm = ::core::ffi::c_uint;
#[doc = "< Key Usage: Key Usage is empty."]
pub const OT_CRYPTO_KEY_USAGE_NONE: _bindgen_ty_1 = 0;
#[doc = "< Key Usage: Key can be exported."]
pub const OT_CRYPTO_KEY_USAGE_EXPORT: _bindgen_ty_1 = 1;
#[doc = "< Key Usage: Encryption (vendor defined)."]
pub const OT_CRYPTO_KEY_USAGE_ENCRYPT: _bindgen_ty_1 = 2;
#[doc = "< Key Usage: AES ECB."]
pub const OT_CRYPTO_KEY_USAGE_DECRYPT: _bindgen_ty_1 = 4;
#[doc = "< Key Usage: Sign Hash."]
pub const OT_CRYPTO_KEY_USAGE_SIGN_HASH: _bindgen_ty_1 = 8;
#[doc = "< Key Usage: Verify Hash."]
pub const OT_CRYPTO_KEY_USAGE_VERIFY_HASH: _bindgen_ty_1 = 16;
#[doc = " Defines the key usage flags.\n"]
pub type _bindgen_ty_1 = ::core::ffi::c_uint;
#[doc = "< Key Persistence: Key is volatile."]
pub const otCryptoKeyStorage_OT_CRYPTO_KEY_STORAGE_VOLATILE: otCryptoKeyStorage = 0;
#[doc = "< Key Persistence: Key is persistent."]
pub const otCryptoKeyStorage_OT_CRYPTO_KEY_STORAGE_PERSISTENT: otCryptoKeyStorage = 1;
#[doc = " Defines the key storage types.\n"]
pub type otCryptoKeyStorage = ::core::ffi::c_uint;
#[doc = " This datatype represents the key reference.\n"]
pub type otCryptoKeyRef = u32;
#[doc = " @struct otCryptoKey\n\n Represents the Key Material required for Crypto operations.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otCryptoKey {
    #[doc = "< Pointer to the buffer containing key. NULL indicates to use `mKeyRef`."]
    pub mKey: *const u8,
    #[doc = "< The key length in bytes (applicable when `mKey` is not NULL)."]
    pub mKeyLength: u16,
    #[doc = "< The PSA key ref (requires `mKey` to be NULL)."]
    pub mKeyRef: u32,
}
impl Default for otCryptoKey {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @struct otCryptoContext\n\n Stores the context object for platform APIs.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otCryptoContext {
    #[doc = "< Pointer to the context."]
    pub mContext: *mut ::core::ffi::c_void,
    #[doc = "< The length of the context in bytes."]
    pub mContextSize: u16,
}
impl Default for otCryptoContext {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @struct otPlatCryptoSha256Hash\n\n Represents a SHA-256 hash.\n"]
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct otPlatCryptoSha256Hash {
    #[doc = "< Hash bytes."]
    pub m8: [u8; 32usize],
}
#[doc = " @struct otPlatCryptoEcdsaKeyPair\n\n Represents an ECDSA key pair (public and private keys).\n\n The key pair is stored using Distinguished Encoding Rules (DER) format (per RFC 5915).\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otPlatCryptoEcdsaKeyPair {
    pub mDerBytes: [u8; 125usize],
    pub mDerLength: u8,
}
impl Default for otPlatCryptoEcdsaKeyPair {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @struct otPlatCryptoEcdsaPublicKey\n\n Represents a ECDSA public key.\n\n The public key is stored as a byte sequence representation of an uncompressed curve point (RFC 6605 - sec 4).\n"]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct otPlatCryptoEcdsaPublicKey {
    pub m8: [u8; 64usize],
}
impl Default for otPlatCryptoEcdsaPublicKey {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @struct otPlatCryptoEcdsaSignature\n\n Represents an ECDSA signature.\n\n The signature is encoded as the concatenated binary representation of two MPIs `r` and `s` which are calculated\n during signing (RFC 6605 - section 4).\n"]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct otPlatCryptoEcdsaSignature {
    pub m8: [u8; 64usize],
}
impl Default for otPlatCryptoEcdsaSignature {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[doc = " Initialize the Crypto module.\n"]
    pub fn otPlatCryptoInit();
}
unsafe extern "C" {
    #[doc = " Import a key into PSA ITS.\n\n @param[in,out] aKeyRef           Pointer to the key ref to be used for crypto operations.\n @param[in]     aKeyType          Key Type encoding for the key.\n @param[in]     aKeyAlgorithm     Key algorithm encoding for the key.\n @param[in]     aKeyUsage         Key Usage encoding for the key (combinations of `OT_CRYPTO_KEY_USAGE_*`).\n @param[in]     aKeyPersistence   Key Persistence for this key\n @param[in]     aKey              Actual key to be imported.\n @param[in]     aKeyLen           Length of the key to be imported.\n\n @retval OT_ERROR_NONE          Successfully imported the key.\n @retval OT_ERROR_FAILED        Failed to import the key.\n @retval OT_ERROR_INVALID_ARGS  @p aKey was set to NULL.\n\n @note If OT_CRYPTO_KEY_STORAGE_PERSISTENT is passed for aKeyPersistence then @p aKeyRef is input and platform\n       should use the given aKeyRef and MUST not change it.\n\n       If OT_CRYPTO_KEY_STORAGE_VOLATILE is passed for aKeyPersistence then @p aKeyRef is output, the initial\n       value does not matter and platform API MUST update it to return the new key ref.\n\n       This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoImportKey(
        aKeyRef: *mut otCryptoKeyRef,
        aKeyType: otCryptoKeyType,
        aKeyAlgorithm: otCryptoKeyAlgorithm,
        aKeyUsage: ::core::ffi::c_int,
        aKeyPersistence: otCryptoKeyStorage,
        aKey: *const u8,
        aKeyLen: usize,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Export a key stored in PSA ITS.\n\n @param[in]   aKeyRef           The key ref to be used for crypto operations.\n @param[out]  aBuffer           Pointer to the buffer where key needs to be exported.\n @param[in]   aBufferLen        Length of the buffer passed to store the exported key.\n @param[out]  aKeyLen           Pointer to return the length of the exported key.\n\n @retval OT_ERROR_NONE          Successfully exported  @p aKeyRef.\n @retval OT_ERROR_FAILED        Failed to export @p aKeyRef.\n @retval OT_ERROR_INVALID_ARGS  @p aBuffer was NULL\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoExportKey(
        aKeyRef: otCryptoKeyRef,
        aBuffer: *mut u8,
        aBufferLen: usize,
        aKeyLen: *mut usize,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Destroy a key stored in PSA ITS.\n\n @param[in]   aKeyRef          The key ref to be destroyed\n\n @retval OT_ERROR_NONE          Successfully destroyed the key.\n @retval OT_ERROR_FAILED        Failed to destroy the key.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoDestroyKey(aKeyRef: otCryptoKeyRef) -> otError;
}
unsafe extern "C" {
    #[doc = " Check if the key ref passed has an associated key in PSA ITS.\n\n @param[in]  aKeyRef          The Key Ref to check.\n\n @retval TRUE                 There is an associated key with @p aKeyRef.\n @retval FALSE                There is no associated key with @p aKeyRef.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoHasKey(aKeyRef: otCryptoKeyRef) -> bool;
}
unsafe extern "C" {
    #[doc = " Initialize the HMAC operation.\n\n @param[in]  aContext          Context for HMAC operation.\n\n @retval OT_ERROR_NONE          Successfully initialized HMAC operation.\n @retval OT_ERROR_FAILED        Failed to initialize HMAC operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n\n @note The platform driver shall point the context to the correct object such as psa_mac_operation_t or\n       mbedtls_md_context_t.\n"]
    pub fn otPlatCryptoHmacSha256Init(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Uninitialize the HMAC operation.\n\n @param[in]  aContext          Context for HMAC operation.\n\n @retval OT_ERROR_NONE          Successfully uninitialized HMAC operation.\n @retval OT_ERROR_FAILED        Failed to uninitialized HMAC operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoHmacSha256Deinit(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Start HMAC operation.\n\n @param[in]  aContext           Context for HMAC operation.\n @param[in]  aKey               Key material to be used for HMAC operation.\n\n @retval OT_ERROR_NONE          Successfully started HMAC operation.\n @retval OT_ERROR_FAILED        Failed to start HMAC operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aKey was NULL\n"]
    pub fn otPlatCryptoHmacSha256Start(
        aContext: *mut otCryptoContext,
        aKey: *const otCryptoKey,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Update the HMAC operation with new input.\n\n @param[in]  aContext           Context for HMAC operation.\n @param[in]  aBuf               A pointer to the input buffer.\n @param[in]  aBufLength         The length of @p aBuf in bytes.\n\n @retval OT_ERROR_NONE          Successfully updated HMAC with new input operation.\n @retval OT_ERROR_FAILED        Failed to update HMAC operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aBuf was NULL\n"]
    pub fn otPlatCryptoHmacSha256Update(
        aContext: *mut otCryptoContext,
        aBuf: *const ::core::ffi::c_void,
        aBufLength: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Complete the HMAC operation.\n\n @param[in]  aContext           Context for HMAC operation.\n @param[out] aBuf               A pointer to the output buffer.\n @param[in]  aBufLength         The length of @p aBuf in bytes.\n\n @retval OT_ERROR_NONE          Successfully completed HMAC operation.\n @retval OT_ERROR_FAILED        Failed to complete HMAC operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aBuf was NULL\n"]
    pub fn otPlatCryptoHmacSha256Finish(
        aContext: *mut otCryptoContext,
        aBuf: *mut u8,
        aBufLength: usize,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Initialise the AES operation.\n\n @param[in]  aContext           Context for AES operation.\n\n @retval OT_ERROR_NONE          Successfully Initialised AES operation.\n @retval OT_ERROR_FAILED        Failed to Initialise AES operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n @retval OT_ERROR_NO_BUFS       Cannot allocate the context.\n\n @note The platform driver shall point the context to the correct object such as psa_key_id\n       or mbedtls_aes_context_t.\n"]
    pub fn otPlatCryptoAesInit(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Set the key for AES operation.\n\n @param[in]  aContext           Context for AES operation.\n @param[out] aKey               Key to use for AES operation.\n\n @retval OT_ERROR_NONE          Successfully set the key for AES operation.\n @retval OT_ERROR_FAILED        Failed to set the key for AES operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aKey was NULL\n"]
    pub fn otPlatCryptoAesSetKey(
        aContext: *mut otCryptoContext,
        aKey: *const otCryptoKey,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Encrypt the given data.\n\n @param[in]  aContext           Context for AES operation.\n @param[in]  aInput             Pointer to the input buffer.\n @param[in]  aOutput            Pointer to the output buffer.\n\n @retval OT_ERROR_NONE          Successfully encrypted @p aInput.\n @retval OT_ERROR_FAILED        Failed to encrypt @p aInput.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aKey or @p aOutput were NULL\n"]
    pub fn otPlatCryptoAesEncrypt(
        aContext: *mut otCryptoContext,
        aInput: *const u8,
        aOutput: *mut u8,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Free the AES context.\n\n @param[in]  aContext           Context for AES operation.\n\n @retval OT_ERROR_NONE          Successfully freed AES context.\n @retval OT_ERROR_FAILED        Failed to free AES context.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoAesFree(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Initialise the HKDF context.\n\n @param[in]  aContext           Context for HKDF operation.\n\n @retval OT_ERROR_NONE          Successfully Initialised AES operation.\n @retval OT_ERROR_FAILED        Failed to Initialise AES operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n\n @note The platform driver shall point the context to the correct object such as psa_key_derivation_operation_t\n       or HmacSha256::Hash\n"]
    pub fn otPlatCryptoHkdfInit(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Perform HKDF Expand step.\n\n @param[in]  aContext           Operation context for HKDF operation.\n @param[in]  aInfo              Pointer to the Info sequence.\n @param[in]  aInfoLength        Length of the Info sequence.\n @param[out] aOutputKey         Pointer to the output Key.\n @param[in]  aOutputKeyLength   Size of the output key buffer.\n\n @retval OT_ERROR_NONE          HKDF Expand was successful.\n @retval OT_ERROR_FAILED        HKDF Expand failed.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoHkdfExpand(
        aContext: *mut otCryptoContext,
        aInfo: *const u8,
        aInfoLength: u16,
        aOutputKey: *mut u8,
        aOutputKeyLength: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Perform HKDF Extract step.\n\n @param[in]  aContext           Operation context for HKDF operation.\n @param[in]  aSalt              Pointer to the Salt for HKDF.\n @param[in]  aSaltLength        Length of Salt.\n @param[in]  aInputKey          Pointer to the input key.\n\n @retval OT_ERROR_NONE          HKDF Extract was successful.\n @retval OT_ERROR_FAILED        HKDF Extract failed.\n"]
    pub fn otPlatCryptoHkdfExtract(
        aContext: *mut otCryptoContext,
        aSalt: *const u8,
        aSaltLength: u16,
        aInputKey: *const otCryptoKey,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Uninitialize the HKDF context.\n\n @param[in]  aContext           Context for HKDF operation.\n\n @retval OT_ERROR_NONE          Successfully un-initialised HKDF operation.\n @retval OT_ERROR_FAILED        Failed to un-initialised HKDF operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoHkdfDeinit(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Initialise the SHA-256 operation.\n\n @param[in]  aContext           Context for SHA-256 operation.\n\n @retval OT_ERROR_NONE          Successfully initialised SHA-256 operation.\n @retval OT_ERROR_FAILED        Failed to initialise SHA-256 operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n\n\n @note The platform driver shall point the context to the correct object such as psa_hash_operation_t\n       or mbedtls_sha256_context."]
    pub fn otPlatCryptoSha256Init(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Uninitialize the SHA-256 operation.\n\n @param[in]  aContext           Context for SHA-256 operation.\n\n @retval OT_ERROR_NONE          Successfully un-initialised SHA-256 operation.\n @retval OT_ERROR_FAILED        Failed to un-initialised SHA-256 operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoSha256Deinit(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Start SHA-256 operation.\n\n @param[in]  aContext           Context for SHA-256 operation.\n\n @retval OT_ERROR_NONE          Successfully started SHA-256 operation.\n @retval OT_ERROR_FAILED        Failed to start SHA-256 operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext was NULL\n"]
    pub fn otPlatCryptoSha256Start(aContext: *mut otCryptoContext) -> otError;
}
unsafe extern "C" {
    #[doc = " Update SHA-256 operation with new input.\n\n @param[in]  aContext           Context for SHA-256 operation.\n @param[in]  aBuf               A pointer to the input buffer.\n @param[in]  aBufLength         The length of @p aBuf in bytes.\n\n @retval OT_ERROR_NONE          Successfully updated SHA-256 with new input operation.\n @retval OT_ERROR_FAILED        Failed to update SHA-256 operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aBuf was NULL\n"]
    pub fn otPlatCryptoSha256Update(
        aContext: *mut otCryptoContext,
        aBuf: *const ::core::ffi::c_void,
        aBufLength: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Finish SHA-256 operation.\n\n @param[in]  aContext           Context for SHA-256 operation.\n @param[in]  aHash              A pointer to the output buffer, where hash needs to be stored.\n @param[in]  aHashSize          The length of @p aHash in bytes.\n\n @retval OT_ERROR_NONE          Successfully completed the SHA-256 operation.\n @retval OT_ERROR_FAILED        Failed to complete SHA-256 operation.\n @retval OT_ERROR_INVALID_ARGS  @p aContext or @p aHash was NULL\n"]
    pub fn otPlatCryptoSha256Finish(
        aContext: *mut otCryptoContext,
        aHash: *mut u8,
        aHashSize: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Initialize cryptographically-secure pseudorandom number generator (CSPRNG).\n"]
    pub fn otPlatCryptoRandomInit();
}
unsafe extern "C" {
    #[doc = " Deinitialize cryptographically-secure pseudorandom number generator (CSPRNG).\n"]
    pub fn otPlatCryptoRandomDeinit();
}
unsafe extern "C" {
    #[doc = " Fills a given buffer with cryptographically secure random bytes.\n\n @param[out] aBuffer            A pointer to a buffer to fill with the random bytes.\n @param[in]  aSize              Size of buffer (number of bytes to fill).\n\n @retval OT_ERROR_NONE          Successfully filled buffer with random values.\n @retval OT_ERROR_FAILED        Operation failed.\n"]
    pub fn otPlatCryptoRandomGet(aBuffer: *mut u8, aSize: u16) -> otError;
}
unsafe extern "C" {
    #[doc = " Generate and populate the output buffer with a new ECDSA key-pair.\n\n @param[out] aKeyPair           A pointer to an ECDSA key-pair structure to store the generated key-pair.\n\n @retval OT_ERROR_NONE          A new key-pair was generated successfully.\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for key generation.\n @retval OT_ERROR_NOT_CAPABLE   Feature not supported.\n @retval OT_ERROR_FAILED        Failed to generate key-pair.\n"]
    pub fn otPlatCryptoEcdsaGenerateKey(aKeyPair: *mut otPlatCryptoEcdsaKeyPair) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the associated public key from the input context.\n\n @param[in]  aKeyPair           A pointer to an ECDSA key-pair structure where the key-pair is stored.\n @param[out] aPublicKey         A pointer to an ECDSA public key structure to store the public key.\n\n @retval OT_ERROR_NONE          Public key was retrieved successfully, and @p aBuffer is updated.\n @retval OT_ERROR_PARSE         The key-pair DER format could not be parsed (invalid format).\n @retval OT_ERROR_INVALID_ARGS  The @p aContext is NULL.\n"]
    pub fn otPlatCryptoEcdsaGetPublicKey(
        aKeyPair: *const otPlatCryptoEcdsaKeyPair,
        aPublicKey: *mut otPlatCryptoEcdsaPublicKey,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Calculate the ECDSA signature for a hashed message using the private key from the input context.\n\n Uses the deterministic digital signature generation procedure from RFC 6979.\n\n @param[in]  aKeyPair           A pointer to an ECDSA key-pair structure where the key-pair is stored.\n @param[in]  aHash              A pointer to a SHA-256 hash structure where the hash value for signature calculation\n                                is stored.\n @param[out] aSignature         A pointer to an ECDSA signature structure to output the calculated signature.\n\n @retval OT_ERROR_NONE          The signature was calculated successfully, @p aSignature was updated.\n @retval OT_ERROR_PARSE         The key-pair DER format could not be parsed (invalid format).\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for signature calculation.\n @retval OT_ERROR_INVALID_ARGS  The @p aContext is NULL.\n"]
    pub fn otPlatCryptoEcdsaSign(
        aKeyPair: *const otPlatCryptoEcdsaKeyPair,
        aHash: *const otPlatCryptoSha256Hash,
        aSignature: *mut otPlatCryptoEcdsaSignature,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Use the key from the input context to verify the ECDSA signature of a hashed message.\n\n @param[in]  aPublicKey         A pointer to an ECDSA public key structure where the public key for signature\n                                verification is stored.\n @param[in]  aHash              A pointer to a SHA-256 hash structure where the hash value for signature verification\n                                is stored.\n @param[in]  aSignature         A pointer to an ECDSA signature structure where the signature value to be verified is\n                                stored.\n\n @retval OT_ERROR_NONE          The signature was verified successfully.\n @retval OT_ERROR_SECURITY      The signature is invalid.\n @retval OT_ERROR_INVALID_ARGS  The key or hash is invalid.\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for signature verification.\n"]
    pub fn otPlatCryptoEcdsaVerify(
        aPublicKey: *const otPlatCryptoEcdsaPublicKey,
        aHash: *const otPlatCryptoSha256Hash,
        aSignature: *const otPlatCryptoEcdsaSignature,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Calculate the ECDSA signature for a hashed message using the Key reference passed.\n\n Uses the deterministic digital signature generation procedure from RFC 6979.\n\n @param[in]  aKeyRef            Key Reference to the slot where the key-pair is stored.\n @param[in]  aHash              A pointer to a SHA-256 hash structure where the hash value for signature calculation\n                                is stored.\n @param[out] aSignature         A pointer to an ECDSA signature structure to output the calculated signature.\n\n @retval OT_ERROR_NONE          The signature was calculated successfully, @p aSignature was updated.\n @retval OT_ERROR_PARSE         The key-pair DER format could not be parsed (invalid format).\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for signature calculation.\n @retval OT_ERROR_INVALID_ARGS  The @p aContext is NULL.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoEcdsaSignUsingKeyRef(
        aKeyRef: otCryptoKeyRef,
        aHash: *const otPlatCryptoSha256Hash,
        aSignature: *mut otPlatCryptoEcdsaSignature,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the associated public key from the key reference passed.\n\n The public key is stored differently depending on the crypto backend library being used\n (OPENTHREAD_CONFIG_CRYPTO_LIB).\n\n This API must make sure to return the public key as a byte sequence representation of an\n uncompressed curve point (RFC 6605 - sec 4)\n\n @param[in]  aKeyRef            Key Reference to the slot where the key-pair is stored.\n @param[out] aPublicKey         A pointer to an ECDSA public key structure to store the public key.\n\n @retval OT_ERROR_NONE          Public key was retrieved successfully, and @p aBuffer is updated.\n @retval OT_ERROR_PARSE         The key-pair DER format could not be parsed (invalid format).\n @retval OT_ERROR_INVALID_ARGS  The @p aContext is NULL.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoEcdsaExportPublicKey(
        aKeyRef: otCryptoKeyRef,
        aPublicKey: *mut otPlatCryptoEcdsaPublicKey,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Generate and import a new ECDSA key-pair at reference passed.\n\n @param[in]  aKeyRef            Key Reference to the slot where the key-pair is stored.\n\n @retval OT_ERROR_NONE          A new key-pair was generated successfully.\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for key generation.\n @retval OT_ERROR_NOT_CAPABLE   Feature not supported.\n @retval OT_ERROR_FAILED        Failed to generate key-pair.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoEcdsaGenerateAndImportKey(aKeyRef: otCryptoKeyRef) -> otError;
}
unsafe extern "C" {
    #[doc = " Use the keyref to verify the ECDSA signature of a hashed message.\n\n @param[in]  aKeyRef            Key Reference to the slot where the key-pair is stored.\n @param[in]  aHash              A pointer to a SHA-256 hash structure where the hash value for signature verification\n                                is stored.\n @param[in]  aSignature         A pointer to an ECDSA signature structure where the signature value to be verified is\n                                stored.\n\n @retval OT_ERROR_NONE          The signature was verified successfully.\n @retval OT_ERROR_SECURITY      The signature is invalid.\n @retval OT_ERROR_INVALID_ARGS  The key or hash is invalid.\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for signature verification.\n\n @note This API is only used by OT core when `OPENTHREAD_CONFIG_PLATFORM_KEY_REFERENCES_ENABLE` is enabled.\n"]
    pub fn otPlatCryptoEcdsaVerifyUsingKeyRef(
        aKeyRef: otCryptoKeyRef,
        aHash: *const otPlatCryptoSha256Hash,
        aSignature: *const otPlatCryptoEcdsaSignature,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Perform PKCS#5 PBKDF2 using CMAC (AES-CMAC-PRF-128).\n\n @param[in]     aPassword          Password to use when generating key.\n @param[in]     aPasswordLen       Length of password.\n @param[in]     aSalt              Salt to use when generating key.\n @param[in]     aSaltLen           Length of salt.\n @param[in]     aIterationCounter  Iteration count.\n @param[in]     aKeyLen            Length of generated key in bytes.\n @param[out]    aKey               A pointer to the generated key.\n\n @retval OT_ERROR_NONE          A new key-pair was generated successfully.\n @retval OT_ERROR_NO_BUFS       Failed to allocate buffer for key generation.\n @retval OT_ERROR_NOT_CAPABLE   Feature not supported.\n @retval OT_ERROR_FAILED        Failed to generate key."]
    pub fn otPlatCryptoPbkdf2GenerateKey(
        aPassword: *const u8,
        aPasswordLen: u16,
        aSalt: *const u8,
        aSaltLen: u16,
        aIterationCounter: u32,
        aKeyLen: u16,
        aKey: *mut u8,
    ) -> otError;
}
#[doc = "< aMaxPHYPacketSize (IEEE 802.15.4-2006)"]
pub const OT_RADIO_FRAME_MAX_SIZE: _bindgen_ty_2 = 127;
#[doc = "< Minimal size of frame FCS + CONTROL"]
pub const OT_RADIO_FRAME_MIN_SIZE: _bindgen_ty_2 = 3;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_SYMBOLS_PER_OCTET: _bindgen_ty_2 = 2;
#[doc = "< 2.4 GHz IEEE 802.15.4 (bits per second)"]
pub const OT_RADIO_BIT_RATE: _bindgen_ty_2 = 250000;
#[doc = "< Number of bits per octet"]
pub const OT_RADIO_BITS_PER_OCTET: _bindgen_ty_2 = 8;
#[doc = "< The O-QPSK PHY symbol rate when operating in the 780MHz, 915MHz, 2380MHz, 2450MHz"]
pub const OT_RADIO_SYMBOL_RATE: _bindgen_ty_2 = 62500;
#[doc = "< Symbol duration time in unit of microseconds"]
pub const OT_RADIO_SYMBOL_TIME: _bindgen_ty_2 = 16;
#[doc = "< Time for 10 symbols in unit of microseconds"]
pub const OT_RADIO_TEN_SYMBOLS_TIME: _bindgen_ty_2 = 160;
#[doc = "< LQI measurement not supported"]
pub const OT_RADIO_LQI_NONE: _bindgen_ty_2 = 0;
#[doc = "< Invalid or unknown RSSI value"]
pub const OT_RADIO_RSSI_INVALID: _bindgen_ty_2 = 127;
#[doc = "< Invalid or unknown power value"]
pub const OT_RADIO_POWER_INVALID: _bindgen_ty_2 = 127;
#[doc = "< Invalid short address."]
pub const OT_RADIO_INVALID_SHORT_ADDR: _bindgen_ty_2 = 65534;
#[doc = "< Broadcast short address."]
pub const OT_RADIO_BROADCAST_SHORT_ADDR: _bindgen_ty_2 = 65535;
#[doc = " @defgroup radio-types Radio Types\n\n @brief\n   This module includes the platform abstraction for a radio frame.\n\n @{"]
pub type _bindgen_ty_2 = ::core::ffi::c_uint;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_CHANNEL_PAGE_0: _bindgen_ty_3 = 0;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_CHANNEL_PAGE_0_MASK: _bindgen_ty_3 = 1;
#[doc = "< 915 MHz IEEE 802.15.4-2006"]
pub const OT_RADIO_CHANNEL_PAGE_2: _bindgen_ty_3 = 2;
#[doc = "< 915 MHz IEEE 802.15.4-2006"]
pub const OT_RADIO_CHANNEL_PAGE_2_MASK: _bindgen_ty_3 = 4;
#[doc = " Defines the channel page."]
pub type _bindgen_ty_3 = ::core::ffi::c_uint;
#[doc = "< 915 MHz IEEE 802.15.4-2006"]
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MIN: _bindgen_ty_4 = 1;
#[doc = "< 915 MHz IEEE 802.15.4-2006"]
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MAX: _bindgen_ty_4 = 10;
#[doc = "< 915 MHz IEEE 802.15.4-2006"]
pub const OT_RADIO_915MHZ_OQPSK_CHANNEL_MASK: _bindgen_ty_4 = 2046;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MIN: _bindgen_ty_4 = 11;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MAX: _bindgen_ty_4 = 26;
#[doc = "< 2.4 GHz IEEE 802.15.4-2006"]
pub const OT_RADIO_2P4GHZ_OQPSK_CHANNEL_MASK: _bindgen_ty_4 = 134215680;
#[doc = " Defines the frequency band channel range."]
pub type _bindgen_ty_4 = ::core::ffi::c_uint;
#[doc = " Represents radio capabilities.\n\n The value is a bit-field indicating the capabilities supported by the radio. See `OT_RADIO_CAPS_*` definitions."]
pub type otRadioCaps = u16;
#[doc = "< Radio supports no capability."]
pub const OT_RADIO_CAPS_NONE: _bindgen_ty_5 = 0;
#[doc = "< Radio supports AckTime event."]
pub const OT_RADIO_CAPS_ACK_TIMEOUT: _bindgen_ty_5 = 1;
#[doc = "< Radio supports Energy Scans."]
pub const OT_RADIO_CAPS_ENERGY_SCAN: _bindgen_ty_5 = 2;
#[doc = "< Radio supports tx retry logic with collision avoidance (CSMA)."]
pub const OT_RADIO_CAPS_TRANSMIT_RETRIES: _bindgen_ty_5 = 4;
#[doc = "< Radio supports CSMA backoff for frame tx (but no retry)."]
pub const OT_RADIO_CAPS_CSMA_BACKOFF: _bindgen_ty_5 = 8;
#[doc = "< Radio supports direct transition from sleep to TX with CSMA."]
pub const OT_RADIO_CAPS_SLEEP_TO_TX: _bindgen_ty_5 = 16;
#[doc = "< Radio supports tx security."]
pub const OT_RADIO_CAPS_TRANSMIT_SEC: _bindgen_ty_5 = 32;
#[doc = "< Radio supports tx at specific time."]
pub const OT_RADIO_CAPS_TRANSMIT_TIMING: _bindgen_ty_5 = 64;
#[doc = "< Radio supports rx at specific time."]
pub const OT_RADIO_CAPS_RECEIVE_TIMING: _bindgen_ty_5 = 128;
#[doc = "< Radio supports RxOnWhenIdle handling."]
pub const OT_RADIO_CAPS_RX_ON_WHEN_IDLE: _bindgen_ty_5 = 256;
#[doc = "< Radio supports setting per-frame transmit power."]
pub const OT_RADIO_CAPS_TRANSMIT_FRAME_POWER: _bindgen_ty_5 = 512;
#[doc = "< Radio supports setting alternate short address."]
pub const OT_RADIO_CAPS_ALT_SHORT_ADDR: _bindgen_ty_5 = 1024;
#[doc = " Defines constants that are used to indicate different radio capabilities. See `otRadioCaps`."]
pub type _bindgen_ty_5 = ::core::ffi::c_uint;
#[doc = " Represents the IEEE 802.15.4 PAN ID."]
pub type otPanId = u16;
#[doc = " Represents the IEEE 802.15.4 Short Address."]
pub type otShortAddress = u16;
#[doc = "< Size of IE header in bytes."]
pub const OT_IE_HEADER_SIZE: _bindgen_ty_6 = 2;
#[doc = "< Size of CSL IE content in bytes."]
pub const OT_CSL_IE_SIZE: _bindgen_ty_6 = 4;
#[doc = "< Max length for header IE in ACK."]
pub const OT_ACK_IE_MAX_SIZE: _bindgen_ty_6 = 16;
#[doc = "< Max length of Link Metrics data in Vendor-Specific IE."]
pub const OT_ENH_PROBING_IE_DATA_MAX_SIZE: _bindgen_ty_6 = 2;
#[doc = " Defines constants about size of header IE in ACK."]
pub type _bindgen_ty_6 = ::core::ffi::c_uint;
#[doc = " @struct otExtAddress\n\n Represents the IEEE 802.15.4 Extended Address."]
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct otExtAddress {
    #[doc = "< IEEE 802.15.4 Extended Address bytes"]
    pub m8: [u8; 8usize],
}
#[doc = " @struct otMacKey\n\n Represents a MAC Key."]
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct otMacKey {
    #[doc = "< MAC Key bytes."]
    pub m8: [u8; 16usize],
}
#[doc = " Represents a MAC Key Ref used by PSA."]
pub type otMacKeyRef = otCryptoKeyRef;
#[doc = " @struct otMacKeyMaterial\n\n Represents a MAC Key."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otMacKeyMaterial {
    pub mKeyMaterial: otMacKeyMaterial__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union otMacKeyMaterial__bindgen_ty_1 {
    #[doc = "< Reference to the key stored."]
    pub mKeyRef: otMacKeyRef,
    #[doc = "< Key stored as literal."]
    pub mKey: otMacKey,
}
impl Default for otMacKeyMaterial__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for otMacKeyMaterial {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "< Use Literal Keys."]
pub const otRadioKeyType_OT_KEY_TYPE_LITERAL_KEY: otRadioKeyType = 0;
#[doc = "< Use Reference to Key."]
pub const otRadioKeyType_OT_KEY_TYPE_KEY_REF: otRadioKeyType = 1;
#[doc = " Defines constants about key types."]
pub type otRadioKeyType = ::core::ffi::c_uint;
#[doc = " Represents the IEEE 802.15.4 Header IE (Information Element) related information of a radio frame."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct otRadioIeInfo {
    #[doc = "< The time offset to the Thread network time."]
    pub mNetworkTimeOffset: i64,
    #[doc = "< The Time IE offset from the start of PSDU."]
    pub mTimeIeOffset: u8,
    #[doc = "< The Time sync sequence."]
    pub mTimeSyncSeq: u8,
}
#[doc = " Represents an IEEE 802.15.4 radio frame."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otRadioFrame {
    #[doc = "< The PSDU."]
    pub mPsdu: *mut u8,
    #[doc = "< Length of the PSDU."]
    pub mLength: u16,
    #[doc = "< Channel used to transmit/receive the frame."]
    pub mChannel: u8,
    #[doc = "< Radio link type - should be ignored by radio driver."]
    pub mRadioType: u8,
    pub mInfo: otRadioFrame__bindgen_ty_1,
}
#[doc = " The union of transmit and receive information for a radio frame."]
#[repr(C)]
#[derive(Copy, Clone)]
pub union otRadioFrame__bindgen_ty_1 {
    pub mTxInfo: otRadioFrame__bindgen_ty_1__bindgen_ty_1,
    pub mRxInfo: otRadioFrame__bindgen_ty_1__bindgen_ty_2,
}
#[doc = " Structure representing radio frame transmit information."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otRadioFrame__bindgen_ty_1__bindgen_ty_1 {
    #[doc = "< The key material used for AES-CCM frame security."]
    pub mAesKey: *const otMacKeyMaterial,
    #[doc = "< The pointer to the Header IE(s) related information."]
    pub mIeInfo: *mut otRadioIeInfo,
    #[doc = " The base time in microseconds for scheduled transmissions\n relative to the local radio clock, see `otPlatRadioGetNow` and\n `mTxDelay`.\n\n If this field is non-zero, `mMaxCsmaBackoffs` should be ignored.\n\n This field does not affect CCA behavior which is controlled by `mCsmaCaEnabled`."]
    pub mTxDelayBaseTime: u32,
    #[doc = " The delay time in microseconds for this transmission referenced\n to `mTxDelayBaseTime`.\n\n Note: `mTxDelayBaseTime` + `mTxDelay` SHALL point to the point in\n time when the end of the SFD will be present at the local\n antenna, relative to the local radio clock.\n\n If this field is non-zero, `mMaxCsmaBackoffs` should be ignored.\n\n This field does not affect CCA behavior which is controlled by `mCsmaCaEnabled`."]
    pub mTxDelay: u32,
    #[doc = " Maximum number of CSMA backoff attempts before declaring channel access failure.\n\n This is applicable and MUST be used when radio platform provides the `OT_RADIO_CAPS_CSMA_BACKOFF` and/or\n `OT_RADIO_CAPS_TRANSMIT_RETRIES`.\n\n This field MUST be ignored if `mCsmaCaEnabled` is set to `false` (CCA is disabled) or\n either `mTxDelayBaseTime` or `mTxDelay` is non-zero (frame transmission is expected at a specific time).\n\n It can be set to `0` to skip backoff mechanism (note that CCA MUST still be performed assuming\n `mCsmaCaEnabled` is `true`)."]
    pub mMaxCsmaBackoffs: u8,
    #[doc = "< Maximum number of retries allowed after a transmission failure."]
    pub mMaxFrameRetries: u8,
    #[doc = " The RX channel after frame TX is done (after all frame retries - ack received, or timeout, or abort).\n\n Radio platforms can choose to fully ignore this. OT stack will make sure to call `otPlatRadioReceive()`\n with the desired RX channel after a frame TX is done and signaled in `otPlatRadioTxDone()` callback.\n Radio platforms that don't provide `OT_RADIO_CAPS_TRANSMIT_RETRIES` must always ignore this.\n\n This is intended for situations where there may be delay in interactions between OT stack and radio, as\n an example this is used in RCP/host architecture to make sure RCP switches to PAN channel more quickly.\n In particular, this can help with CSL tx to a sleepy child, where the child may use a different channel\n for CSL than the PAN channel. After frame tx, we want the radio/RCP to go back to the PAN channel\n quickly to ensure that parent does not miss tx from child afterwards, e.g., child responding to the\n earlier CSL transmitted frame from parent using PAN channel while radio still staying on CSL channel.\n\n The switch to the RX channel MUST happen after the frame TX is fully done, i.e., after all retries and\n when ack is received (when \"Ack Request\" flag is set on the TX frame) or ack timeout. Note that ack is\n expected on the same channel that frame is sent on."]
    pub mRxChannelAfterTxDone: u8,
    #[doc = " The transmit power in dBm.\n\n If the platform layer does not provide `OT_RADIO_CAPS_TRANSMIT_FRAME_POWER` capability, it can ignore\n this value.\n\n If the value is OT_RADIO_POWER_INVALID, then the platform should ignore this value and transmit the frame\n with its default transmit power.\n\n Otherwise, the platform should transmit this frame with the maximum power no larger than minimal of the\n following values:\n     1. mTxPower,\n     2. The power limit set by otPlatRadioSetChannelTargetPower(),\n     3. The power limit set by otPlatRadioSetChannelMaxTransmitPower(),\n     4. The power limit set by otPlatRadioSetRegion()."]
    pub mTxPower: i8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    #[doc = " The time of the local radio clock in microseconds when the end of\n the SFD was present at the local antenna.\n\n The platform should update this field before otPlatRadioTxStarted() is fired for each transmit attempt."]
    pub mTimestamp: u64,
}
impl Default for otRadioFrame__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl otRadioFrame__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn mIsHeaderUpdated(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mIsHeaderUpdated(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mIsHeaderUpdated_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mIsHeaderUpdated_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mIsARetx(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mIsARetx(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mIsARetx_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mIsARetx_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mCsmaCaEnabled(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mCsmaCaEnabled(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mCsmaCaEnabled_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mCsmaCaEnabled_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mCslPresent(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mCslPresent(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mCslPresent_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mCslPresent_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mIsSecurityProcessed(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mIsSecurityProcessed(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mIsSecurityProcessed_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mIsSecurityProcessed_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mIsHeaderUpdated: bool,
        mIsARetx: bool,
        mCsmaCaEnabled: bool,
        mCslPresent: bool,
        mIsSecurityProcessed: bool,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mIsHeaderUpdated: u8 = unsafe { ::core::mem::transmute(mIsHeaderUpdated) };
            mIsHeaderUpdated as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let mIsARetx: u8 = unsafe { ::core::mem::transmute(mIsARetx) };
            mIsARetx as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let mCsmaCaEnabled: u8 = unsafe { ::core::mem::transmute(mCsmaCaEnabled) };
            mCsmaCaEnabled as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let mCslPresent: u8 = unsafe { ::core::mem::transmute(mCslPresent) };
            mCslPresent as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let mIsSecurityProcessed: u8 = unsafe { ::core::mem::transmute(mIsSecurityProcessed) };
            mIsSecurityProcessed as u64
        });
        __bindgen_bitfield_unit
    }
}
#[doc = " Structure representing radio frame receive information."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct otRadioFrame__bindgen_ty_1__bindgen_ty_2 {
    #[doc = " The time of the local radio clock in microseconds when the end of\n the SFD was present at the local antenna."]
    pub mTimestamp: u64,
    #[doc = "< ACK security frame counter (applicable when `mAckedWithSecEnhAck` is set)."]
    pub mAckFrameCounter: u32,
    #[doc = "< ACK security key index (applicable when `mAckedWithSecEnhAck` is set)."]
    pub mAckKeyId: u8,
    #[doc = "< Received signal strength indicator in dBm for received frames."]
    pub mRssi: i8,
    #[doc = "< Link Quality Indicator for received frames."]
    pub mLqi: u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl otRadioFrame__bindgen_ty_1__bindgen_ty_2 {
    #[inline]
    pub fn mAckedWithFramePending(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mAckedWithFramePending(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mAckedWithFramePending_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mAckedWithFramePending_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mAckedWithSecEnhAck(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mAckedWithSecEnhAck(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mAckedWithSecEnhAck_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mAckedWithSecEnhAck_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mAckedWithFramePending: bool,
        mAckedWithSecEnhAck: bool,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mAckedWithFramePending: u8 =
                unsafe { ::core::mem::transmute(mAckedWithFramePending) };
            mAckedWithFramePending as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let mAckedWithSecEnhAck: u8 = unsafe { ::core::mem::transmute(mAckedWithSecEnhAck) };
            mAckedWithSecEnhAck as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for otRadioFrame__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for otRadioFrame {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const otRadioState_OT_RADIO_STATE_DISABLED: otRadioState = 0;
pub const otRadioState_OT_RADIO_STATE_SLEEP: otRadioState = 1;
pub const otRadioState_OT_RADIO_STATE_RECEIVE: otRadioState = 2;
pub const otRadioState_OT_RADIO_STATE_TRANSMIT: otRadioState = 3;
pub const otRadioState_OT_RADIO_STATE_INVALID: otRadioState = 255;
#[doc = " Represents the state of a radio.\n Initially, a radio is in the Disabled state."]
pub type otRadioState = ::core::ffi::c_uint;
#[doc = " Represents radio coexistence metrics."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct otRadioCoexMetrics {
    #[doc = "< Number of grant glitches."]
    pub mNumGrantGlitch: u32,
    #[doc = "< Number of tx requests."]
    pub mNumTxRequest: u32,
    #[doc = "< Number of tx requests while grant was active."]
    pub mNumTxGrantImmediate: u32,
    #[doc = "< Number of tx requests while grant was inactive."]
    pub mNumTxGrantWait: u32,
    #[doc = "< Number of tx requests while grant was inactive that were ultimately granted."]
    pub mNumTxGrantWaitActivated: u32,
    #[doc = "< Number of tx requests while grant was inactive that timed out."]
    pub mNumTxGrantWaitTimeout: u32,
    #[doc = "< Number of tx that were in progress when grant was deactivated."]
    pub mNumTxGrantDeactivatedDuringRequest: u32,
    #[doc = "< Number of tx requests that were not granted within 50us."]
    pub mNumTxDelayedGrant: u32,
    #[doc = "< Average time in usec from tx request to grant."]
    pub mAvgTxRequestToGrantTime: u32,
    #[doc = "< Number of rx requests."]
    pub mNumRxRequest: u32,
    #[doc = "< Number of rx requests while grant was active."]
    pub mNumRxGrantImmediate: u32,
    #[doc = "< Number of rx requests while grant was inactive."]
    pub mNumRxGrantWait: u32,
    #[doc = "< Number of rx requests while grant was inactive that were ultimately granted."]
    pub mNumRxGrantWaitActivated: u32,
    #[doc = "< Number of rx requests while grant was inactive that timed out."]
    pub mNumRxGrantWaitTimeout: u32,
    #[doc = "< Number of rx that were in progress when grant was deactivated."]
    pub mNumRxGrantDeactivatedDuringRequest: u32,
    #[doc = "< Number of rx requests that were not granted within 50us."]
    pub mNumRxDelayedGrant: u32,
    #[doc = "< Average time in usec from rx request to grant."]
    pub mAvgRxRequestToGrantTime: u32,
    #[doc = "< Number of rx requests that completed without receiving grant."]
    pub mNumRxGrantNone: u32,
    #[doc = "< Stats collection stopped due to saturation."]
    pub mStopped: bool,
}
#[doc = " Represents what metrics are specified to query."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct otLinkMetrics {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl otLinkMetrics {
    #[inline]
    pub fn mPduCount(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mPduCount(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mPduCount_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mPduCount_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mLqi(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mLqi(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mLqi_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mLqi_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mLinkMargin(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mLinkMargin(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mLinkMargin_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mLinkMargin_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mRssi(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mRssi(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mRssi_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mRssi_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mReserved(&self) -> bool {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mReserved(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mReserved_raw(this: *const Self) -> bool {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mReserved_raw(this: *mut Self, val: bool) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mPduCount: bool,
        mLqi: bool,
        mLinkMargin: bool,
        mRssi: bool,
        mReserved: bool,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mPduCount: u8 = unsafe { ::core::mem::transmute(mPduCount) };
            mPduCount as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let mLqi: u8 = unsafe { ::core::mem::transmute(mLqi) };
            mLqi as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let mLinkMargin: u8 = unsafe { ::core::mem::transmute(mLinkMargin) };
            mLinkMargin as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let mRssi: u8 = unsafe { ::core::mem::transmute(mRssi) };
            mRssi as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let mReserved: u8 = unsafe { ::core::mem::transmute(mReserved) };
            mReserved as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    #[doc = " Get the radio capabilities.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns The radio capability bit vector (see `OT_RADIO_CAP_*` definitions)."]
    pub fn otPlatRadioGetCaps(aInstance: *mut otInstance) -> otRadioCaps;
}
unsafe extern "C" {
    #[doc = " Get the radio version string.\n\n This is an optional radio driver platform function. If not provided by platform radio driver, OpenThread uses\n the OpenThread version instead (@sa otGetVersionString()).\n\n @param[in]  aInstance   The OpenThread instance structure.\n\n @returns A pointer to the OpenThread radio version."]
    pub fn otPlatRadioGetVersionString(aInstance: *mut otInstance) -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    #[doc = " Get the radio receive sensitivity value.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns The radio receive sensitivity value in dBm."]
    pub fn otPlatRadioGetReceiveSensitivity(aInstance: *mut otInstance) -> i8;
}
unsafe extern "C" {
    #[doc = " Gets the factory-assigned IEEE EUI-64 for this interface.\n\n @param[in]  aInstance   The OpenThread instance structure.\n @param[out] aIeeeEui64  A pointer to the factory-assigned IEEE EUI-64."]
    pub fn otPlatRadioGetIeeeEui64(aInstance: *mut otInstance, aIeeeEui64: *mut u8);
}
unsafe extern "C" {
    #[doc = " Set the PAN ID for address filtering.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[in] aPanId     The IEEE 802.15.4 PAN ID."]
    pub fn otPlatRadioSetPanId(aInstance: *mut otInstance, aPanId: otPanId);
}
unsafe extern "C" {
    #[doc = " Set the Extended Address for address filtering.\n\n @param[in] aInstance    The OpenThread instance structure.\n @param[in] aExtAddress  A pointer to the IEEE 802.15.4 Extended Address stored in little-endian byte order."]
    pub fn otPlatRadioSetExtendedAddress(
        aInstance: *mut otInstance,
        aExtAddress: *const otExtAddress,
    );
}
unsafe extern "C" {
    #[doc = " Set the Short Address for address filtering.\n\n @param[in] aInstance      The OpenThread instance structure.\n @param[in] aShortAddress  The IEEE 802.15.4 Short Address."]
    pub fn otPlatRadioSetShortAddress(aInstance: *mut otInstance, aShortAddress: otShortAddress);
}
unsafe extern "C" {
    #[doc = " Set the alternate short address.\n\n This is an optional radio platform API. The radio platform MUST indicate support for this API by including the\n capability `OT_RADIO_CAPS_ALT_SHORT_ADDR` in `otPlatRadioGetCaps()`.\n\n When supported, the radio should accept received frames destined to the specified alternate short address in\n addition to the short address provided in `otPlatRadioSetShortAddress()`.\n\n The @p aShortAddress can be set to `OT_RADIO_INVALID_SHORT_ADDR` (0xfffe) to clear any previously set alternate\n short address.\n\n This function is used by OpenThread stack during child-to-router role transitions, allowing the device to continue\n receiving frames addressed to its previous short address for a short period.\n\n @param[in] aInstance      The OpenThread instance structure.\n @param[in] aShortAddress  The alternate IEEE 802.15.4 short address. `OT_RADIO_INVALID_SHORT_ADDR` to clear."]
    pub fn otPlatRadioSetAlternateShortAddress(
        aInstance: *mut otInstance,
        aShortAddress: otShortAddress,
    );
}
unsafe extern "C" {
    #[doc = " Get the radio's transmit power in dBm.\n\n @note The transmit power returned will be no larger than the power specified in the max power table for\n the current channel.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[out] aPower    The transmit power in dBm.\n\n @retval OT_ERROR_NONE             Successfully retrieved the transmit power.\n @retval OT_ERROR_INVALID_ARGS     @p aPower was NULL.\n @retval OT_ERROR_NOT_IMPLEMENTED  Transmit power configuration via dBm is not implemented."]
    pub fn otPlatRadioGetTransmitPower(aInstance: *mut otInstance, aPower: *mut i8) -> otError;
}
unsafe extern "C" {
    #[doc = " Set the radio's transmit power in dBm for all channels.\n\n @note The real transmit power will be no larger than the power specified in the max power table for\n the current channel that was configured by `otPlatRadioSetChannelMaxTransmitPower()`.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[in] aPower     The transmit power in dBm.\n\n @retval OT_ERROR_NONE             Successfully set the transmit power.\n @retval OT_ERROR_NOT_IMPLEMENTED  Transmit power configuration via dBm is not implemented."]
    pub fn otPlatRadioSetTransmitPower(aInstance: *mut otInstance, aPower: i8) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the radio's CCA ED threshold in dBm measured at antenna connector per IEEE 802.15.4 - 2015 section 10.1.4.\n\n @param[in] aInstance    The OpenThread instance structure.\n @param[out] aThreshold  The CCA ED threshold in dBm.\n\n @retval OT_ERROR_NONE             Successfully retrieved the CCA ED threshold.\n @retval OT_ERROR_INVALID_ARGS     @p aThreshold was NULL.\n @retval OT_ERROR_NOT_IMPLEMENTED  CCA ED threshold configuration via dBm is not implemented."]
    pub fn otPlatRadioGetCcaEnergyDetectThreshold(
        aInstance: *mut otInstance,
        aThreshold: *mut i8,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Set the radio's CCA ED threshold in dBm measured at antenna connector per IEEE 802.15.4 - 2015 section 10.1.4.\n\n @param[in] aInstance   The OpenThread instance structure.\n @param[in] aThreshold  The CCA ED threshold in dBm.\n\n @retval OT_ERROR_NONE             Successfully set the transmit power.\n @retval OT_ERROR_INVALID_ARGS     Given threshold is out of range.\n @retval OT_ERROR_NOT_IMPLEMENTED  CCA ED threshold configuration via dBm is not implemented."]
    pub fn otPlatRadioSetCcaEnergyDetectThreshold(
        aInstance: *mut otInstance,
        aThreshold: i8,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Gets the external FEM's Rx LNA gain in dBm.\n\n @param[in]  aInstance  The OpenThread instance structure.\n @param[out] aGain     The external FEM's Rx LNA gain in dBm.\n\n @retval OT_ERROR_NONE             Successfully retrieved the external FEM's LNA gain.\n @retval OT_ERROR_INVALID_ARGS     @p aGain was NULL.\n @retval OT_ERROR_NOT_IMPLEMENTED  External FEM's LNA setting is not implemented."]
    pub fn otPlatRadioGetFemLnaGain(aInstance: *mut otInstance, aGain: *mut i8) -> otError;
}
unsafe extern "C" {
    #[doc = " Sets the external FEM's Rx LNA gain in dBm.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[in] aGain      The external FEM's Rx LNA gain in dBm.\n\n @retval OT_ERROR_NONE             Successfully set the external FEM's LNA gain.\n @retval OT_ERROR_NOT_IMPLEMENTED  External FEM's LNA gain setting is not implemented."]
    pub fn otPlatRadioSetFemLnaGain(aInstance: *mut otInstance, aGain: i8) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the status of promiscuous mode.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @retval TRUE   Promiscuous mode is enabled.\n @retval FALSE  Promiscuous mode is disabled."]
    pub fn otPlatRadioGetPromiscuous(aInstance: *mut otInstance) -> bool;
}
unsafe extern "C" {
    #[doc = " Enable or disable promiscuous mode.\n\n @param[in]  aInstance The OpenThread instance structure.\n @param[in]  aEnable   TRUE to enable or FALSE to disable promiscuous mode."]
    pub fn otPlatRadioSetPromiscuous(aInstance: *mut otInstance, aEnable: bool);
}
unsafe extern "C" {
    #[doc = " Sets the rx-on-when-idle state to the radio platform.\n\n There are a few situations that the radio can enter sleep state if the device is in rx-off-when-idle state but\n it's hard and costly for the SubMac to identify these situations and instruct the radio to enter sleep:\n\n - Finalization of a regular frame reception task, provided that:\n   - The frame is received without errors and passes the filtering and it's not an spurious ACK.\n   - ACK is not requested or transmission of ACK is not possible due to internal conditions.\n - Finalization of a frame transmission or transmission of an ACK frame, when ACK is not requested in the transmitted\n   frame.\n - Finalization of the reception operation of a requested ACK due to:\n   - ACK timeout expiration.\n   - Reception of an invalid ACK or not an ACK frame.\n   - Reception of the proper ACK, unless the transmitted frame was a Data Request Command and the frame pending bit\n     on the received ACK is set to true. In this case the radio platform implementation SHOULD keep the receiver on\n     until a determined timeout which triggers an idle period start.`OPENTHREAD_CONFIG_MAC_DATA_POLL_TIMEOUT` can be\n     taken as a reference for this.\n - Finalization of a stand alone CCA task.\n - Finalization of a CCA operation with busy result during CSMA/CA procedure.\n - Finalization of an Energy Detection task.\n - Finalization of a radio reception window scheduled with `otPlatRadioReceiveAt`.\n\n If a platform supports `OT_RADIO_CAPS_RX_ON_WHEN_IDLE` it must also support `OT_RADIO_CAPS_CSMA_BACKOFF` and handle\n idle periods after CCA as described above.\n\n Upon the transition of the \"RxOnWhenIdle\" flag from TRUE to FALSE, the radio platform should enter sleep mode.\n If the radio is currently in receive mode, it should enter sleep mode immediately. Otherwise, it should enter sleep\n mode after the current operation is completed.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[in]  aEnable      TRUE to keep radio in Receive state, FALSE to put to Sleep state during idle periods."]
    pub fn otPlatRadioSetRxOnWhenIdle(aInstance: *mut otInstance, aEnable: bool);
}
unsafe extern "C" {
    #[doc = " Update MAC keys and key index\n\n Is used when radio provides OT_RADIO_CAPS_TRANSMIT_SEC capability.\n\n The radio platform should reset the current security MAC frame counter tracked by the radio on this call. While this\n is highly recommended, the OpenThread stack, as a safeguard, will also reset the frame counter using the\n `otPlatRadioSetMacFrameCounter()` before calling this API.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n @param[in]   aKeyIdMode   The key ID mode.\n @param[in]   aKeyId       Current MAC key index.\n @param[in]   aPrevKey     A pointer to the previous MAC key.\n @param[in]   aCurrKey     A pointer to the current MAC key.\n @param[in]   aNextKey     A pointer to the next MAC key.\n @param[in]   aKeyType     Key Type used."]
    pub fn otPlatRadioSetMacKey(
        aInstance: *mut otInstance,
        aKeyIdMode: u8,
        aKeyId: u8,
        aPrevKey: *const otMacKeyMaterial,
        aCurrKey: *const otMacKeyMaterial,
        aNextKey: *const otMacKeyMaterial,
        aKeyType: otRadioKeyType,
    );
}
unsafe extern "C" {
    #[doc = " Sets the current MAC frame counter value.\n\n Is used when radio provides `OT_RADIO_CAPS_TRANSMIT_SEC` capability.\n\n @param[in]   aInstance         A pointer to an OpenThread instance.\n @param[in]   aMacFrameCounter  The MAC frame counter value."]
    pub fn otPlatRadioSetMacFrameCounter(aInstance: *mut otInstance, aMacFrameCounter: u32);
}
unsafe extern "C" {
    #[doc = " Sets the current MAC frame counter value only if the new given value is larger than the current value.\n\n Is used when radio provides `OT_RADIO_CAPS_TRANSMIT_SEC` capability.\n\n @param[in]   aInstance         A pointer to an OpenThread instance.\n @param[in]   aMacFrameCounter  The MAC frame counter value."]
    pub fn otPlatRadioSetMacFrameCounterIfLarger(aInstance: *mut otInstance, aMacFrameCounter: u32);
}
unsafe extern "C" {
    #[doc = " Get the current time in microseconds referenced to a continuous monotonic\n local radio clock (64 bits width).\n\n The radio clock SHALL NOT wrap during the device's uptime. Implementations\n SHALL therefore identify and compensate for internal counter overflows. The\n clock does not have a defined epoch and it SHALL NOT introduce any continuous\n or discontinuous adjustments (e.g. leap seconds). Implementations SHALL\n compensate for any sleep times of the device.\n\n Implementations MAY choose to discipline the radio clock and compensate for\n sleep times by any means (e.g. by combining a high precision/low power RTC\n with a high resolution counter) as long as the exposed combined clock\n provides continuous monotonic microsecond resolution ticks within the\n accuracy limits announced by @ref otPlatRadioGetCslAccuracy.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n\n @returns The current time in microseconds. UINT64_MAX when platform does not\n support or radio time is not ready."]
    pub fn otPlatRadioGetNow(aInstance: *mut otInstance) -> u64;
}
unsafe extern "C" {
    #[doc = " Get the bus speed in bits/second between the host and the radio chip.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n\n @returns The bus speed in bits/second between the host and the radio chip.\n          Return 0 when the MAC and above layer and Radio layer resides on the same chip."]
    pub fn otPlatRadioGetBusSpeed(aInstance: *mut otInstance) -> u32;
}
unsafe extern "C" {
    #[doc = " Get the bus latency in microseconds between the host and the radio chip.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n\n @returns The bus latency in microseconds between the host and the radio chip.\n          Return 0 when the MAC and above layer and Radio layer resides on the same chip."]
    pub fn otPlatRadioGetBusLatency(aInstance: *mut otInstance) -> u32;
}
unsafe extern "C" {
    #[doc = " Get current state of the radio.\n\n Is not required by OpenThread. It may be used for debugging and/or application-specific purposes.\n\n @note This function may be not implemented. It does not affect OpenThread.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @return  Current state of the radio."]
    pub fn otPlatRadioGetState(aInstance: *mut otInstance) -> otRadioState;
}
unsafe extern "C" {
    #[doc = " Enable the radio.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @retval OT_ERROR_NONE     Successfully enabled.\n @retval OT_ERROR_FAILED   The radio could not be enabled."]
    pub fn otPlatRadioEnable(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Disable the radio.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @retval OT_ERROR_NONE            Successfully transitioned to Disabled.\n @retval OT_ERROR_INVALID_STATE   The radio was not in sleep state."]
    pub fn otPlatRadioDisable(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Check whether radio is enabled or not.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns TRUE if the radio is enabled, FALSE otherwise."]
    pub fn otPlatRadioIsEnabled(aInstance: *mut otInstance) -> bool;
}
unsafe extern "C" {
    #[doc = " Transition the radio from Receive to Sleep (turn off the radio).\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @retval OT_ERROR_NONE          Successfully transitioned to Sleep.\n @retval OT_ERROR_BUSY          The radio was transmitting.\n @retval OT_ERROR_INVALID_STATE The radio was disabled."]
    pub fn otPlatRadioSleep(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Transition the radio from Sleep to Receive (turn on the radio).\n\n @param[in]  aInstance  The OpenThread instance structure.\n @param[in]  aChannel   The channel to use for receiving.\n\n @retval OT_ERROR_NONE          Successfully transitioned to Receive.\n @retval OT_ERROR_INVALID_STATE The radio was disabled or transmitting."]
    pub fn otPlatRadioReceive(aInstance: *mut otInstance, aChannel: u8) -> otError;
}
unsafe extern "C" {
    #[doc = " Schedule a radio reception window at a specific time and duration.\n\n @param[in]  aChannel   The radio channel on which to receive.\n @param[in]  aStart     The receive window start time relative to the local\n                        radio clock, see `otPlatRadioGetNow`. The radio\n                        receiver SHALL be on and ready to receive the first\n                        symbol of a frame's SHR at the window start time.\n @param[in]  aDuration  The receive window duration, in microseconds, as\n                        measured by the local radio clock. The radio SHOULD be\n                        turned off (or switched to TX mode if an ACK frame\n                        needs to be sent) after that duration unless it is\n                        still actively receiving a frame. In the latter case\n                        the radio SHALL be kept in reception mode until frame\n                        reception has either succeeded or failed.\n\n @retval OT_ERROR_NONE    Successfully scheduled receive window.\n @retval OT_ERROR_FAILED  The receive window could not be scheduled."]
    pub fn otPlatRadioReceiveAt(
        aInstance: *mut otInstance,
        aChannel: u8,
        aStart: u32,
        aDuration: u32,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread of a received frame.\n\n @param[in]  aInstance The OpenThread instance structure.\n @param[in]  aFrame    A pointer to the received frame or NULL if the receive operation failed.\n @param[in]  aError    OT_ERROR_NONE when successfully received a frame,\n                       OT_ERROR_ABORT when reception was aborted and a frame was not received,\n                       OT_ERROR_NO_BUFS when a frame could not be received due to lack of rx buffer space."]
    pub fn otPlatRadioReceiveDone(
        aInstance: *mut otInstance,
        aFrame: *mut otRadioFrame,
        aError: otError,
    );
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread diagnostics module of a received frame.\n\n Is used when diagnostics is enabled.\n\n @param[in]  aInstance The OpenThread instance structure.\n @param[in]  aFrame    A pointer to the received frame or NULL if the receive operation failed.\n @param[in]  aError    OT_ERROR_NONE when successfully received a frame,\n                       OT_ERROR_ABORT when reception was aborted and a frame was not received,\n                       OT_ERROR_NO_BUFS when a frame could not be received due to lack of rx buffer space."]
    pub fn otPlatDiagRadioReceiveDone(
        aInstance: *mut otInstance,
        aFrame: *mut otRadioFrame,
        aError: otError,
    );
}
unsafe extern "C" {
    #[doc = " Get the radio transmit frame buffer.\n\n OpenThread forms the IEEE 802.15.4 frame in this buffer then calls `otPlatRadioTransmit()` to request transmission.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns A pointer to the transmit frame buffer."]
    pub fn otPlatRadioGetTransmitBuffer(aInstance: *mut otInstance) -> *mut otRadioFrame;
}
unsafe extern "C" {
    #[doc = " Begin the transmit sequence on the radio.\n\n The caller must form the IEEE 802.15.4 frame in the buffer provided by `otPlatRadioGetTransmitBuffer()` before\n requesting transmission.  The channel and transmit power are also included in the otRadioFrame structure.\n\n The transmit sequence consists of:\n 1. Transitioning the radio to Transmit from one of the following states:\n    - Receive if RX is on when the device is idle or OT_RADIO_CAPS_SLEEP_TO_TX is not supported\n    - Sleep if RX is off when the device is idle and OT_RADIO_CAPS_SLEEP_TO_TX is supported.\n 2. Transmits the psdu on the given channel and at the given transmit power.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[in] aFrame     A pointer to the frame to be transmitted.\n\n @retval OT_ERROR_NONE          Successfully transitioned to Transmit.\n @retval OT_ERROR_INVALID_STATE The radio was not in the Receive state."]
    pub fn otPlatRadioTransmit(aInstance: *mut otInstance, aFrame: *mut otRadioFrame) -> otError;
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread that the transmission has started.\n\n @note  This function should be called by the same thread that executes all of the other OpenThread code. It should\n        not be called by ISR or any other task.\n\n @param[in]  aInstance  A pointer to the OpenThread instance structure.\n @param[in]  aFrame     A pointer to the frame that is being transmitted."]
    pub fn otPlatRadioTxStarted(aInstance: *mut otInstance, aFrame: *mut otRadioFrame);
}
unsafe extern "C" {
    #[doc = " The radio driver calls this function to notify OpenThread that the transmit operation has completed,\n providing both the transmitted frame and, if applicable, the received ack frame.\n\n When radio provides `OT_RADIO_CAPS_TRANSMIT_SEC` capability, radio platform layer updates @p aFrame\n with the security frame counter and key index values maintained by the radio.\n\n @param[in]  aInstance  The OpenThread instance structure.\n @param[in]  aFrame     A pointer to the frame that was transmitted.\n @param[in]  aAckFrame  A pointer to the ACK frame, NULL if no ACK was received.\n @param[in]  aError     OT_ERROR_NONE when the frame was transmitted,\n                        OT_ERROR_NO_ACK when the frame was transmitted but no ACK was received,\n                        OT_ERROR_CHANNEL_ACCESS_FAILURE tx could not take place due to activity on the channel,\n                        OT_ERROR_ABORT when transmission was aborted for other reasons."]
    pub fn otPlatRadioTxDone(
        aInstance: *mut otInstance,
        aFrame: *mut otRadioFrame,
        aAckFrame: *mut otRadioFrame,
        aError: otError,
    );
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread diagnostics module that the transmission has completed.\n\n Is used when diagnostics is enabled.\n\n @param[in]  aInstance      The OpenThread instance structure.\n @param[in]  aFrame         A pointer to the frame that was transmitted.\n @param[in]  aError         OT_ERROR_NONE when the frame was transmitted,\n                            OT_ERROR_CHANNEL_ACCESS_FAILURE tx could not take place due to activity on the channel,\n                            OT_ERROR_ABORT when transmission was aborted for other reasons."]
    pub fn otPlatDiagRadioTransmitDone(
        aInstance: *mut otInstance,
        aFrame: *mut otRadioFrame,
        aError: otError,
    );
}
unsafe extern "C" {
    #[doc = " Get the most recent RSSI measurement.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns The RSSI in dBm when it is valid.  127 when RSSI is invalid."]
    pub fn otPlatRadioGetRssi(aInstance: *mut otInstance) -> i8;
}
unsafe extern "C" {
    #[doc = " Begin the energy scan sequence on the radio.\n\n Is used when radio provides OT_RADIO_CAPS_ENERGY_SCAN capability.\n\n @param[in] aInstance      The OpenThread instance structure.\n @param[in] aScanChannel   The channel to perform the energy scan on.\n @param[in] aScanDuration  The duration, in milliseconds, for the channel to be scanned.\n\n @retval OT_ERROR_NONE             Successfully started scanning the channel.\n @retval OT_ERROR_BUSY             The radio is performing energy scanning.\n @retval OT_ERROR_NOT_IMPLEMENTED  The radio doesn't support energy scanning."]
    pub fn otPlatRadioEnergyScan(
        aInstance: *mut otInstance,
        aScanChannel: u8,
        aScanDuration: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread that the energy scan is complete.\n\n Is used when radio provides OT_RADIO_CAPS_ENERGY_SCAN capability.\n\n @param[in]  aInstance           The OpenThread instance structure.\n @param[in]  aEnergyScanMaxRssi  The maximum RSSI encountered on the scanned channel."]
    pub fn otPlatRadioEnergyScanDone(aInstance: *mut otInstance, aEnergyScanMaxRssi: i8);
}
unsafe extern "C" {
    #[doc = " The radio driver calls this method to notify OpenThread that the spinel bus latency has been changed.\n\n @param[in]  aInstance  The OpenThread instance structure."]
    pub fn otPlatRadioBusLatencyChanged(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Enable/Disable source address match feature.\n\n The source address match feature controls how the radio layer decides the \"frame pending\" bit for acks sent in\n response to data request commands from children.\n\n If disabled, the radio layer must set the \"frame pending\" on all acks to data request commands.\n\n If enabled, the radio layer uses the source address match table to determine whether to set or clear the \"frame\n pending\" bit in an ack to a data request command.\n\n The source address match table provides the list of children for which there is a pending frame. Either a short\n address or an extended/long address can be added to the source address match table.\n\n @param[in]  aInstance   The OpenThread instance structure.\n @param[in]  aEnable     Enable/disable source address match feature."]
    pub fn otPlatRadioEnableSrcMatch(aInstance: *mut otInstance, aEnable: bool);
}
unsafe extern "C" {
    #[doc = " Add a short address to the source address match table.\n\n @param[in]  aInstance      The OpenThread instance structure.\n @param[in]  aShortAddress  The short address to be added.\n\n @retval OT_ERROR_NONE      Successfully added short address to the source match table.\n @retval OT_ERROR_NO_BUFS   No available entry in the source match table."]
    pub fn otPlatRadioAddSrcMatchShortEntry(
        aInstance: *mut otInstance,
        aShortAddress: otShortAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Add an extended address to the source address match table.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[in]  aExtAddress  The extended address to be added stored in little-endian byte order.\n\n @retval OT_ERROR_NONE      Successfully added extended address to the source match table.\n @retval OT_ERROR_NO_BUFS   No available entry in the source match table."]
    pub fn otPlatRadioAddSrcMatchExtEntry(
        aInstance: *mut otInstance,
        aExtAddress: *const otExtAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Remove a short address from the source address match table.\n\n @param[in]  aInstance      The OpenThread instance structure.\n @param[in]  aShortAddress  The short address to be removed.\n\n @retval OT_ERROR_NONE        Successfully removed short address from the source match table.\n @retval OT_ERROR_NO_ADDRESS  The short address is not in source address match table."]
    pub fn otPlatRadioClearSrcMatchShortEntry(
        aInstance: *mut otInstance,
        aShortAddress: otShortAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Remove an extended address from the source address match table.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[in]  aExtAddress  The extended address to be removed stored in little-endian byte order.\n\n @retval OT_ERROR_NONE        Successfully removed the extended address from the source match table.\n @retval OT_ERROR_NO_ADDRESS  The extended address is not in source address match table."]
    pub fn otPlatRadioClearSrcMatchExtEntry(
        aInstance: *mut otInstance,
        aExtAddress: *const otExtAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Clear all short addresses from the source address match table.\n\n @param[in]  aInstance   The OpenThread instance structure."]
    pub fn otPlatRadioClearSrcMatchShortEntries(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Clear all the extended/long addresses from source address match table.\n\n @param[in]  aInstance   The OpenThread instance structure."]
    pub fn otPlatRadioClearSrcMatchExtEntries(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " Get the radio supported channel mask that the device is allowed to be on.\n\n @param[in]  aInstance   The OpenThread instance structure.\n\n @returns The radio supported channel mask."]
    pub fn otPlatRadioGetSupportedChannelMask(aInstance: *mut otInstance) -> u32;
}
unsafe extern "C" {
    #[doc = " Gets the radio preferred channel mask that the device prefers to form on.\n\n @param[in]  aInstance   The OpenThread instance structure.\n\n @returns The radio preferred channel mask."]
    pub fn otPlatRadioGetPreferredChannelMask(aInstance: *mut otInstance) -> u32;
}
unsafe extern "C" {
    #[doc = " Enable the radio coex.\n\n Is used when feature OPENTHREAD_CONFIG_PLATFORM_RADIO_COEX_ENABLE is enabled.\n\n @param[in] aInstance  The OpenThread instance structure.\n @param[in] aEnabled   TRUE to enable the radio coex, FALSE otherwise.\n\n @retval OT_ERROR_NONE     Successfully enabled.\n @retval OT_ERROR_FAILED   The radio coex could not be enabled."]
    pub fn otPlatRadioSetCoexEnabled(aInstance: *mut otInstance, aEnabled: bool) -> otError;
}
unsafe extern "C" {
    #[doc = " Check whether radio coex is enabled or not.\n\n Is used when feature OPENTHREAD_CONFIG_PLATFORM_RADIO_COEX_ENABLE is enabled.\n\n @param[in] aInstance  The OpenThread instance structure.\n\n @returns TRUE if the radio coex is enabled, FALSE otherwise."]
    pub fn otPlatRadioIsCoexEnabled(aInstance: *mut otInstance) -> bool;
}
unsafe extern "C" {
    #[doc = " Get the radio coexistence metrics.\n\n Is used when feature OPENTHREAD_CONFIG_PLATFORM_RADIO_COEX_ENABLE is enabled.\n\n @param[in]  aInstance     The OpenThread instance structure.\n @param[out] aCoexMetrics  A pointer to the coexistence metrics structure.\n\n @retval OT_ERROR_NONE          Successfully retrieved the coex metrics.\n @retval OT_ERROR_INVALID_ARGS  @p aCoexMetrics was NULL."]
    pub fn otPlatRadioGetCoexMetrics(
        aInstance: *mut otInstance,
        aCoexMetrics: *mut otRadioCoexMetrics,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Enable or disable CSL receiver.\n\n @param[in]  aInstance     The OpenThread instance structure.\n @param[in]  aCslPeriod    CSL period, 0 for disabling CSL. CSL period is in unit of 10 symbols.\n @param[in]  aShortAddr    The short source address of CSL receiver's peer.\n @param[in]  aExtAddr      The extended source address of CSL receiver's peer.\n\n @note Platforms should use CSL peer addresses to include CSL IE when generating enhanced acks.\n\n @retval  OT_ERROR_NOT_IMPLEMENTED Radio driver doesn't support CSL.\n @retval  OT_ERROR_FAILED          Other platform specific errors.\n @retval  OT_ERROR_NONE            Successfully enabled or disabled CSL."]
    pub fn otPlatRadioEnableCsl(
        aInstance: *mut otInstance,
        aCslPeriod: u32,
        aShortAddr: otShortAddress,
        aExtAddr: *const otExtAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Reset CSL receiver in the platform.\n\n @note Defaults to `otPlatRadioEnableCsl(aInstance,0, Mac::kShortAddrInvalid, nullptr);`\n\n @param[in]  aInstance     The OpenThread instance structure.\n\n @retval  OT_ERROR_NOT_IMPLEMENTED Radio driver doesn't support CSL.\n @retval  OT_ERROR_FAILED          Other platform specific errors.\n @retval  OT_ERROR_NONE            Successfully disabled CSL."]
    pub fn otPlatRadioResetCsl(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Update CSL sample time in radio driver.\n\n Sample time is stored in radio driver as a copy to calculate phase when\n sending ACK with CSL IE. The CSL sample (window) of the CSL receiver extends\n before and after the sample time. The CSL sample time marks a timestamp in\n the CSL sample window when a frame should be received in \"ideal conditions\"\n if there would be no inaccuracy/clock-drift.\n\n @param[in]  aInstance         The OpenThread instance structure.\n @param[in]  aCslSampleTime    The next sample time, in microseconds. It is\n                               the time when the first symbol of the MHR of\n                               the frame is expected."]
    pub fn otPlatRadioUpdateCslSampleTime(aInstance: *mut otInstance, aCslSampleTime: u32);
}
unsafe extern "C" {
    #[doc = " Get the current estimated worst case accuracy (maximum +/- deviation from the\n nominal frequency) of the local radio clock in units of PPM. This is the\n clock used to schedule CSL operations.\n\n @note Implementations MAY estimate this value based on current operating\n conditions (e.g. temperature).\n\n In case the implementation does not estimate the current value but returns a\n fixed value, this value MUST be the worst-case accuracy over all possible\n foreseen operating conditions (temperature, pressure, etc) of the\n implementation.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n\n @returns The current CSL rx/tx scheduling drift, in PPM."]
    pub fn otPlatRadioGetCslAccuracy(aInstance: *mut otInstance) -> u8;
}
unsafe extern "C" {
    #[doc = " The fixed uncertainty (i.e. random jitter) of the arrival time of CSL\n transmissions received by this device in units of 10 microseconds.\n\n This designates the worst case constant positive or negative deviation of\n the actual arrival time of a transmission from the transmission time\n calculated relative to the local radio clock independent of elapsed time. In\n addition to uncertainty accumulated over elapsed time, the CSL channel sample\n (\"RX window\") must be extended by twice this deviation such that an actual\n transmission is guaranteed to be detected by the local receiver in the\n presence of random arrival time jitter.\n\n @param[in]   aInstance    A pointer to an OpenThread instance.\n\n @returns The CSL Uncertainty in units of 10 us."]
    pub fn otPlatRadioGetCslUncertainty(aInstance: *mut otInstance) -> u8;
}
unsafe extern "C" {
    #[doc = " Set the max transmit power for a specific channel.\n\n @note This function will be deprecated in October 2027. It is recommended to use the function\n       `otPlatRadioSetChannelTargetPower()`.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[in]  aChannel     The radio channel.\n @param[in]  aMaxPower    The max power in dBm, passing OT_RADIO_RSSI_INVALID will disable this channel.\n\n @retval  OT_ERROR_NOT_IMPLEMENTED  The feature is not implemented\n @retval  OT_ERROR_INVALID_ARGS     The specified channel is not valid.\n @retval  OT_ERROR_FAILED           Other platform specific errors.\n @retval  OT_ERROR_NONE             Successfully set max transmit power."]
    pub fn otPlatRadioSetChannelMaxTransmitPower(
        aInstance: *mut otInstance,
        aChannel: u8,
        aMaxPower: i8,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Set the region code.\n\n The radio region format is the 2-bytes ascii representation of the\n ISO 3166 alpha-2 code.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[in]  aRegionCode  The radio region code. The `aRegionCode >> 8` is first ascii char\n                          and the `aRegionCode & 0xff` is the second ascii char.\n\n @retval  OT_ERROR_FAILED           Other platform specific errors.\n @retval  OT_ERROR_NONE             Successfully set region code.\n @retval  OT_ERROR_NOT_IMPLEMENTED  The feature is not implemented."]
    pub fn otPlatRadioSetRegion(aInstance: *mut otInstance, aRegionCode: u16) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the region code.\n\n The radio region format is the 2-bytes ascii representation of the\n ISO 3166 alpha-2 code.\n\n @param[in]  aInstance    The OpenThread instance structure.\n @param[out] aRegionCode  The radio region.\n\n @retval  OT_ERROR_INVALID_ARGS     @p aRegionCode is nullptr.\n @retval  OT_ERROR_FAILED           Other platform specific errors.\n @retval  OT_ERROR_NONE             Successfully got region code.\n @retval  OT_ERROR_NOT_IMPLEMENTED  The feature is not implemented."]
    pub fn otPlatRadioGetRegion(aInstance: *mut otInstance, aRegionCode: *mut u16) -> otError;
}
unsafe extern "C" {
    #[doc = " Enable/disable or update Enhanced-ACK Based Probing in radio for a specific Initiator.\n\n After Enhanced-ACK Based Probing is configured by a specific Probing Initiator, the Enhanced-ACK sent to that\n node should include Vendor-Specific IE containing Link Metrics data. This method informs the radio to start/stop to\n collect Link Metrics data and include Vendor-Specific IE that containing the data in Enhanced-ACK sent to that\n Probing Initiator.\n\n @param[in]  aInstance     The OpenThread instance structure.\n @param[in]  aLinkMetrics  This parameter specifies what metrics to query. Per spec 4.11.3.4.4.6, at most 2 metrics\n                           can be specified. The probing would be disabled if @p `aLinkMetrics` is bitwise 0.\n @param[in]  aShortAddress The short address of the Probing Initiator.\n @param[in]  aExtAddress   The extended source address of the Probing Initiator. @p aExtAddr MUST NOT be `NULL`.\n\n @retval  OT_ERROR_NONE            Successfully configured the Enhanced-ACK Based Probing.\n @retval  OT_ERROR_INVALID_ARGS    @p aExtAddress is `NULL`.\n @retval  OT_ERROR_NOT_FOUND       The Initiator indicated by @p aShortAddress is not found when trying to clear.\n @retval  OT_ERROR_NO_BUFS         No more Initiator can be supported.\n @retval  OT_ERROR_NOT_IMPLEMENTED The feature is not implemented."]
    pub fn otPlatRadioConfigureEnhAckProbing(
        aInstance: *mut otInstance,
        aLinkMetrics: otLinkMetrics,
        aShortAddress: otShortAddress,
        aExtAddress: *const otExtAddress,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Add a calibrated power of the specified channel to the power calibration table.\n\n @note This API is an optional radio platform API. It's up to the platform layer to implement it.\n\n The @p aActualPower is the actual measured output power when the parameters of the radio hardware modules\n are set to the @p aRawPowerSetting.\n\n The raw power setting is an opaque byte array. OpenThread doesn't define the format of the raw power setting.\n Its format is radio hardware related and it should be defined by the developers in the platform radio driver.\n For example, if the radio hardware contains both the radio chip and the FEM chip, the raw power setting can be\n a combination of the radio power register and the FEM gain value.\n\n @param[in] aInstance               The OpenThread instance structure.\n @param[in] aChannel                The radio channel.\n @param[in] aActualPower            The actual power in 0.01dBm.\n @param[in] aRawPowerSetting        A pointer to the raw power setting byte array.\n @param[in] aRawPowerSettingLength  The length of the @p aRawPowerSetting.\n\n @retval OT_ERROR_NONE             Successfully added the calibrated power to the power calibration table.\n @retval OT_ERROR_NO_BUFS          No available entry in the power calibration table.\n @retval OT_ERROR_INVALID_ARGS     The @p aChannel, @p aActualPower or @p aRawPowerSetting is invalid or the\n                                   @p aActualPower already exists in the power calibration table.\n @retval OT_ERROR_NOT_IMPLEMENTED  This feature is not implemented."]
    pub fn otPlatRadioAddCalibratedPower(
        aInstance: *mut otInstance,
        aChannel: u8,
        aActualPower: i16,
        aRawPowerSetting: *const u8,
        aRawPowerSettingLength: u16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Clear all calibrated powers from the power calibration table.\n\n @note This API is an optional radio platform API. It's up to the platform layer to implement it.\n\n @param[in]  aInstance   The OpenThread instance structure.\n\n @retval OT_ERROR_NONE             Successfully cleared all calibrated powers from the power calibration table.\n @retval OT_ERROR_NOT_IMPLEMENTED  This feature is not implemented."]
    pub fn otPlatRadioClearCalibratedPowers(aInstance: *mut otInstance) -> otError;
}
unsafe extern "C" {
    #[doc = " Set the target power for the given channel.\n\n @note This API is an optional radio platform API. It's up to the platform layer to implement it.\n       If this function and `otPlatRadioSetTransmitPower()` are implemented at the same time:\n       - If neither of these two functions is called, the radio outputs the platform-defined default power.\n       - If both functions are called, the last one to be called takes effect.\n\n The radio driver should set the actual output power to be less than or equal to the @p aTargetPower and as close\n as possible to the @p aTargetPower. If the @p aTargetPower is lower than the minimum output power supported\n by the platform, the output power should be set to the minimum output power supported by the platform.  If the\n @p aTargetPower is higher than the maximum output power supported by the platform, the output power should be\n set to the maximum output power supported by the platform. If the @p aTargetPower is set to `INT16_MAX`, the\n corresponding channel is disabled.\n\n @param[in]  aInstance     The OpenThread instance structure.\n @param[in]  aChannel      The radio channel.\n @param[in]  aTargetPower  The target power in 0.01dBm.\n\n @retval  OT_ERROR_NONE             Successfully set the target power.\n @retval  OT_ERROR_INVALID_ARGS     The @p aChannel is invalid.\n @retval  OT_ERROR_NOT_IMPLEMENTED  The feature is not implemented."]
    pub fn otPlatRadioSetChannelTargetPower(
        aInstance: *mut otInstance,
        aChannel: u8,
        aTargetPower: i16,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " Get the raw power setting for the given channel.\n\n @note OpenThread `src/core/utils` implements a default implementation of the API `otPlatRadioAddCalibratedPower()`,\n       `otPlatRadioClearCalibratedPowers()` and `otPlatRadioSetChannelTargetPower()`. This API is provided by\n       the default implementation to get the raw power setting for the given channel. If the platform doesn't\n       use the default implementation, it can ignore this API.\n\n Platform radio layer should parse the raw power setting based on the radio layer defined format and set the\n parameters of each radio hardware module.\n\n @param[in]      aInstance               The OpenThread instance structure.\n @param[in]      aChannel                The radio channel.\n @param[out]     aRawPowerSetting        A pointer to the raw power setting byte array.\n @param[in,out]  aRawPowerSettingLength  On input, a pointer to the size of @p aRawPowerSetting.\n                                         On output, a pointer to the length of the raw power setting data.\n\n @retval  OT_ERROR_NONE          Successfully got the target power.\n @retval  OT_ERROR_INVALID_ARGS  The @p aChannel is invalid, @p aRawPowerSetting or @p aRawPowerSettingLength is NULL\n                                 or @aRawPowerSettingLength is too short.\n @retval  OT_ERROR_NOT_FOUND     The raw power setting for the @p aChannel was not found."]
    pub fn otPlatRadioGetRawPowerSetting(
        aInstance: *mut otInstance,
        aChannel: u8,
        aRawPowerSetting: *mut u8,
        aRawPowerSettingLength: *mut u16,
    ) -> otError;
}
#[doc = "< Sending packet (not ACK)"]
pub const _radio_evnt_type_enum_TX_PCKT_STATE: _radio_evnt_type_enum = 0;
#[doc = "< Sending ACK"]
pub const _radio_evnt_type_enum_TX_ACK_STATE: _radio_evnt_type_enum = 1;
#[doc = "< Receiving packet (not ACK)"]
pub const _radio_evnt_type_enum_RX_PCKT_STATE: _radio_evnt_type_enum = 2;
#[doc = "< Receiving Ack"]
pub const _radio_evnt_type_enum_RX_ACK_STATE: _radio_evnt_type_enum = 3;
#[doc = "< Energy detection"]
pub const _radio_evnt_type_enum_RADIO_ED: _radio_evnt_type_enum = 4;
#[doc = "< Sleep state"]
pub const _radio_evnt_type_enum_RADIO_SLEEP_STATE: _radio_evnt_type_enum = 5;
#[doc = "< Disable state"]
pub const _radio_evnt_type_enum_RADIO_DISABLED_STATE: _radio_evnt_type_enum = 6;
#[doc = "< Tx at specific time"]
pub const _radio_evnt_type_enum_TX_AT_STATE: _radio_evnt_type_enum = 7;
#[doc = "< Rx at specific time"]
pub const _radio_evnt_type_enum_RX_AT_STATE: _radio_evnt_type_enum = 8;
#[doc = "//**\n  \t\t\t\t\t\t\t\t Enums\n/\n/**\n @brief Enum defines the states of the radio event that used in coexistence\n"]
pub type _radio_evnt_type_enum = ::core::ffi::c_uint;
#[doc = "//**\n  \t\t\t\t\t\t\t\t Enums\n/\n/**\n @brief Enum defines the states of the radio event that used in coexistence\n"]
pub use self::_radio_evnt_type_enum as radio_event_enum_t;
#[doc = "< No calibration waiting or required"]
pub const _calibration_status_CALIBRATION_NOT_RUNNING: _calibration_status = 0;
#[doc = "< Calibration started its timer and waits for timer firing"]
pub const _calibration_status_CALIBRATION_READY: _calibration_status = 1;
#[doc = "< Timer of calibration fired and started executing the calibration"]
pub const _calibration_status_CALIBRATION_RUNNING: _calibration_status = 2;
#[doc = "< Timer of calibration fired or temperature has changed with +- 5 degrees but there's in progress operation like TX, ED or RX with no continuous reception"]
pub const _calibration_status_PRDC_OR_TEMP_CALIBRATION_WAITING: _calibration_status = 3;
#[doc = "< MAC channel calibration is pending due to starting TX or RX at specific time."]
pub const _calibration_status_MAC_CH_CALIBRATION_WAITING: _calibration_status = 4;
pub type _calibration_status = ::core::ffi::c_uint;
pub use self::_calibration_status as calibration_status;
#[doc = "//**\n  \t\t\t\t\t\t\t\t Structures\n/\n/**  @ingroup systm_layer\n  @{\n/\n/**\n @brief Structure defines all callback functions used to notify MAC layer after specific event compeletion\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct mac_cbk_dispatch_tbl {
    #[doc = " @brief  Energy detection scan done callback\n\n @param[in] aInstance\t\t\t: Radio instance\n @param[in] aEnergyScanMaxRssi: The result for ED operation which is RSSI value\n\n @note  This function is called after handling SM_DONE event in ral through a sequence of calls\n \t\t  ral_sm_done() -> ral_ed_scan_done() -> callback"]
    pub mac_ed_scan_done: ::core::option::Option<
        unsafe extern "C" fn(aInstance: *mut otInstance, aEnergyScanMaxRssi: i8),
    >,
    #[doc = " @brief  Transmission done callback\n\n @param[in] aInstance\t: Radio instance\n @param[in] aFrame\t: Pointer to the transmitted frame\n @param[in] aAckFrame\t: Pointer to the received ACK frame (in case of AR bit is detected in aFrame)\n @param[in] aError\t: Error code if happens in transmission\n\n @note  This function is called after handling SM_DONE event in ral through a sequence of calls\n \t\t  ral_sm_done() -> ral_tx_done() -> callback"]
    pub mac_tx_done: ::core::option::Option<
        unsafe extern "C" fn(
            aInstance: *mut otInstance,
            aFrame: *mut otRadioFrame,
            aAckFrame: *mut otRadioFrame,
            aError: otError,
        ),
    >,
    #[doc = " @brief  Reception done callback\n\n @param[in] aInstance\t: Radio instance\n @param[in] aFrame\t: Pointer to the received frame\n @param[in] aError\t: Error code if happens in transmission\n\n @note  This function is called after handling SM_DONE event in ral through a sequence of calls\n \t\t  ral_sm_done() -> ral_rx_done() -> callback"]
    pub mac_rx_done: ::core::option::Option<
        unsafe extern "C" fn(
            aInstance: *mut otInstance,
            aFrame: *mut otRadioFrame,
            aError: otError,
        ),
    >,
    #[doc = " @brief  The start of successful transmission callback\n\n @param[in] aInstance\t: Radio instance\n @param[in] aFrame\t: Pointer to the transmitted frame\n @param[in] aError\t: Error code if happens in transmission\n\n @note  This function is called after calling proc_radio_tx which is called with calling of otplatRadioTransmit\n        in case of no error returned. These errors may be (CCA channel access failure, ral busy due to ongoing transmission ...etc )"]
    pub mac_tx_strtd: ::core::option::Option<
        unsafe extern "C" fn(aInstance: *mut otInstance, aFrame: *mut otRadioFrame),
    >,
    #[doc = " @brief The frame update callback\n\n @param[in] aInstance\t: Radio instance\n @param[in] aFrame\t: Pointer to the frame"]
    pub mac_frm_updtd: ::core::option::Option<
        unsafe extern "C" fn(aInstance: *mut otInstance, aFrame: *mut otRadioFrame),
    >,
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/******************************************************************************************//**\n  \t\t\t\t\t\t\t\t APIs\n/\n/**  @ingroup systm_layer_cmn_proc\n  @{\n/\n/**\n @brief  Initialize radio layer including registration for RAL callbacks, some filter configurations,\n \t\t   and some automatic configurations like auto_sleep and auto_ack...etc\n"]
    pub fn radio_init();
}
unsafe extern "C" {
    #[doc = " @brief  Register upper layer callback functions.\n \t\t   This function called from upper layer init function (mac_init in case of MAC integration e.g. Zigbee stack).\n\n @param[in] ptr_cbk_dispatch_tbl : Pointer to callback functions to be registered"]
    pub fn radio_call_back_funcs_init(ptr_cbk_dispatch_tbl: *mut mac_cbk_dispatch_tbl);
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**\n @brief Initialize openthread dispatch table\n\n @param sInstance\t: Radio instance"]
    pub fn otDispatch_tbl_init(sInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = "  @ingroup systm_layer_enc\n  @{\n/\n/**\n @brief  Encrypt frame pointed to by ptr_pckt\n\n @param[in] ptr_pckt \t\t\t: Pointer to the data buffer (variable length in case of CCM mode, 16 bytes in case of ECB mode).\n \t\t\t\t\t\t\t\t  The resulting Encrypted/Decrypted data overwrites this buffer.\n @param[in] ptr_session_key \t: Pointer to the security key buffer (16 bytes)\n @param[in] ptr_ccm_nonce \t: Pointer to the security nonce buffer (13 bytes in case of CCM mode, a Null pointer in case of ECB mode).\n @param[in] mic_len \t\t\t: Length of MIC, supported values are 0, 4, 6, 8, 10, 12, 14, and 16 in case of CCM, 0 only in case of ECB.\n @param[in] ad_len \t\t\t: Length of Data to be authenticated\n @param[in] md_len \t\t\t: Length of Data to be encrypted\n @retval Status of the encryption process"]
    pub fn radio_encrypt_pckts(
        ptr_pckt: *mut u8,
        ptr_session_key: *const u8,
        ptr_ccm_nonce: *mut u8,
        mic_len: u32,
        ad_len: u32,
        md_len: u32,
    ) -> u32;
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**  @ingroup systm_layer_cmn_config\n  @{\n/\n/**\n @brief  Configure automatic ACK response by RAL layer including AIFS, ACK frame timeout.. etc\n\n @param[in] auto_ack_state : Auto ACK state (TRUE to enable, FALSE to disable)\n @retval Status of configuration"]
    pub fn radio_set_auto_Ack_state(auto_ack_state: u8) -> otError;
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**  @ingroup systm_layer_cmn_proc\n  @{\n/\n/**\n @brief Reset radio layer operation which in turns stop all running operations\n\n @return Status"]
    pub fn radio_reset() -> u32;
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**\n @brief Control RAL setup for reception\n\n @param[in] aChannel : Channel to receive on\n @param[in] duration : Reception duration in microsecond, 0 means receive forever\n @return Status"]
    pub fn proc_radio_rcv(aChannel: u8, duration: u32) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Control RAL setup for transmission based on CCA procedure.\n\n @param[in] aFrame \t: Pointer to the transmitted frame\n @param[in] strt_time : Pointer to TX request time\n @return Status"]
    pub fn proc_radio_tx(aFrame: *mut otRadioFrame, strt_time: *mut ble_time_t) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Control RAL setup for energy detection scan\n\n @param[in] aScanChannel \t: Channel to perform ED on\n @param[in] aScanDuration : Scan duration\n @return Status"]
    pub fn proc_radio_ed(aScanChannel: u8, aScanDuration: u16) -> otError;
}
unsafe extern "C" {
    #[doc = "  @ingroup systm_layer_cmn_config\n  @{\n/\n/**\n @brief Set PAN coordinator role in HW to be used while applying MAC 802.15.4 filtration policies\n\n @param[in] aEnable : PAN coordinator role flag (TRUE to enable, FALSE to disable)"]
    pub fn setPANcoordinator(aEnable: u8);
}
unsafe extern "C" {
    #[doc = " @brief Notify RAL layer of RX on when idle state\n\n @param[in] aEnable : Continuous reception state (TRUE to enable, FALSE to disable)"]
    pub fn setContRecp(aEnable: u8);
}
unsafe extern "C" {
    #[doc = " @brief Disable filtration policies while performing scan\n\n @param[in] aInstance : Radio instance"]
    pub fn enableScanFilters(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " @brief Restore filtration policies after the scan is completed\n\n @param[in] aInstance : Radio instance"]
    pub fn disableScanFilters(aInstance: *mut otInstance);
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**  @ingroup systm_layer_cmn_proc\n  @{\n/\n/**\n @brief  Get the latest LQI value\n\n @retval LQI value"]
    pub fn radio_GetLQIValue() -> u8;
}
unsafe extern "C" {
    #[doc = "  @ingroup systm_layer_cmn_config\n  @{\n/\n/**\n @brief Set minimum CSMA backoff exponent\n\n @param[in] value : Minimum csma backoff exponent"]
    pub fn set_min_csma_be(value: u8);
}
unsafe extern "C" {
    #[doc = " @brief Set maximum CSMA backoff exponent\n\n @param[in] value : Maximum CSMA backoff exponent"]
    pub fn set_max_csma_be(value: u8);
}
unsafe extern "C" {
    #[doc = " @brief Set maximum CSMA backoff counter\n\n @param[in] value : Maximum CSMA backoff counter"]
    pub fn set_max_csma_backoff(value: u8);
}
unsafe extern "C" {
    #[doc = " @brief Set custom maximum full CSMA frame retries\n\n @param[in] value : Maximum full CSMA retrials"]
    pub fn set_max_full_csma_frm_retries(value: u8);
}
unsafe extern "C" {
    #[doc = " @brief Set maximum frame retries\n\n @param[in] value : Maximum frame retries"]
    pub fn set_max_frm_retries(value: u8);
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**  @ingroup systm_layer_cmn_proc\n  @{\n/\n/**\n @brief Generate random number\n\n @param[out] ptr_rnd\t\t: Pointer to the output random bytes\n @param[in] len\t \t\t: Number of required random bytes\n @param[in] check_cont_rx : Flag to check continuous reception\n @return Status"]
    pub fn mac_gen_rnd_num(ptr_rnd: *mut u8, len: u16, check_cont_rx: u8) -> u32;
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**  @ingroup systm_layer_enc\n  @{\n/\n/**\n @brief  Decrypt frame pointed to by ptr_pckt\n\n @param[in] ptr_pckt \t\t\t: Pointer to the data buffer (variable length in case of CCM mode, 16 bytes in case of ECB mode).\n \t\t\t\t\t\t\t\t  The resulting Encrypted/Decrypted data overwrites this buffer.\n @param[in] ptr_session_key \t: Pointer to the security key buffer (16 bytes)\n @param[in] ptr_ccm_nonce \t: Pointer to the security nonce buffer (13 bytes in case of CCM mode, a Null pointer in case of ECB mode).\n @param[in] mic_len \t\t\t: Length of MIC, supported values are 0, 4, 6, 8, 10, 12, 14, and 16 in case of CCM, 0 only in case of ECB.\n @param[in] ad_len \t\t\t: Length of Data to be authenticated\n @param[in] md_len \t\t\t: Length of Data to be decrypted\n @retval Status of the decryption process"]
    pub fn radio_decrypt_pckts(
        ptr_pckt: *mut u8,
        ptr_session_key: *mut u8,
        ptr_ccm_nonce: *mut u8,
        mic_len: u32,
        ad_len: u32,
        md_len: u32,
    ) -> u32;
}
unsafe extern "C" {
    #[doc = " @}\n\n/\n/**\n @brief Call MAC radio rx done callback after the end of reception.\n\n @param[in] aFrame : Pointer to the received frame\n @param[in] aError : Reception error"]
    pub fn radio_mac_rx_done(aFrame: *mut otRadioFrame, aError: otError);
}
unsafe extern "C" {
    #[doc = " @brief Set the link metrics noise floor value needed to calculate the link margin\n\n @param[in] noise_floor : The noise floor used by Link Metrics.\n \t\t\t\t\t\t\tIt should be set to the platform's noise floor (measured noise floor, receiver sensitivity or a constant)."]
    pub fn radio_link_metrics_set_noise_floor(noise_floor: i8);
}
unsafe extern "C" {
    #[doc = " @brief Call MAC radio TX done\n\n @param[in] tx_frame\t: Pointer to the transmitted frame\n @param[in] aError\t: TX error"]
    pub fn radio_mac_tx_done_error(tx_frame: *mut otRadioFrame, aError: otError);
}
unsafe extern "C" {
    #[doc = " @brief Get the calibration state of MAC channel.\n\n @param[in] channel\t\t: MAC channel (11:26).\n @retval Calibration state"]
    pub fn radio_get_mac_ch_clbr_state(channel: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief Get the duration of single MAC channel calibration in sleep timer steps\n\n @return Duration of single MAC channel calibration in sleep timer steps"]
    pub fn radio_get_mac_ch_clbr_durn() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief Check any pending PHY calibration for MAC channels and run it before releasing the scheduler grant."]
    pub fn radio_check_mac_ch_phy_clbr_pending();
}
unsafe extern "C" {
    #[doc = " @brief Set antenna diversity feature parameters\n\n @param[in] aInstance     \t: Radio instance\n @param[in] ptr_ant_div_params: Pointer to antenna diversity feature parameters\n @retval Status"]
    pub fn radio_set_ant_div_params(
        aInstance: *mut otInstance,
        ptr_ant_div_params: *mut antenna_diversity_st,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Get antenna diversity feature parameters\n\n @param[in] aInstance \t\t\t: Radio instance\n @param[out] ptr_ant_div_params\t: Pointer to antenna diversity feature parameters"]
    pub fn radio_get_ant_div_params(
        aInstance: *mut otInstance,
        ptr_ant_div_params: *mut antenna_diversity_st,
    );
}
unsafe extern "C" {
    #[doc = " @brief Set antenna diversity feature state\n\n @param[in] aInstance\t: Radio instance\n @param[in] enable\t: Antenna diversity feature state (TRUE to enable, FALSE to disable)\n @retval Status"]
    pub fn radio_set_ant_div_enable(aInstance: *mut otInstance, enable: u8) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Set the default antenna ID of the antenna diversity feature to be used for transmission and reception\n\n @param[in] aInstance\t    \t: Radio instance\n @param[in] default_ant_id\t: Default antenna ID\n @retval Status"]
    pub fn radio_set_default_ant_id(aInstance: *mut otInstance, default_ant_id: u8) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Set the RSSI threshold for antenna diversity feature\n\n @param[in] aInstance\t    : Radio instance\n @param[in] rssi_threshold: RSSI threshold to compare with during antenna diversity measurements\n @retval Status"]
    pub fn radio_set_ant_div_rssi_threshold(
        aInstance: *mut otInstance,
        rssi_threshold: i8,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Set bus latency between thread core and radio platform\n\n @param[in] aInstance\t    : Radio instance\n @param[in] bus_latency\t: Time in microseconds for latency between thread core and radio platform"]
    pub fn otPlatRadioSetBusLatency(aInstance: *mut otInstance, bus_latency: u32);
}
unsafe extern "C" {
    #[doc = " @brief   Set clock accuracy\n\n @param[in] clk_acc: Clock accuracy in PPM"]
    pub fn radio_set_clk_accuracy(clk_acc: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Set clock uncertainty\n\n @param[in] clk_uncer: Clock uncertainty in units of 10 us."]
    pub fn radio_set_clk_uncertainty(clk_uncer: u8);
}
unsafe extern "C" {
    #[doc = " @brief Set configurable library feature parameters\n\n @param[in] aInstance \t\t\t: Radio instance\n @param[in] ptr_config_lib_params : pointer to configurable library feature parameters\n @retval Status"]
    pub fn radio_set_config_lib_params(
        aInstance: *mut otInstance,
        ptr_config_lib_params: *mut config_lib_st,
    ) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief Get current configurable library parameters\n\n @param[in] aInstance\t            : Radio instance\n @param[out] ptr_config_lib_params: Pointer to configurable library feature parameters"]
    pub fn radio_get_config_lib_params(
        aInstance: *mut otInstance,
        ptr_config_lib_params: *mut config_lib_st,
    );
}
unsafe extern "C" {
    #[doc = " @brief Set RTL polling time\n\n @param[in] aInstance       : Radio instance\n @param[in] rtl_polling_time: RTL polling time value"]
    pub fn radio_set_rtl_polling_time(aInstance: *mut otInstance, rtl_polling_time: u8);
}
unsafe extern "C" {
    #[doc = " @brief Get current RTL polling time\n\n @param[in] aInstance : Radio instance\n @retval Current RTL polling time"]
    pub fn radio_get_rtl_polling_time(aInstance: *mut otInstance) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Get minimum block counter of TX packet to increase priority to critical.\n\n @retval Minimum block number of TX packets to be critical"]
    pub fn radio_get_min_blck_cnt_to_be_critical() -> u8;
}
#[doc = " An index used in multi-instance for specifying which instance to perform operation on"]
pub type ral_instance_t = u8;
pub const ral_event_state_enum_RAL_IDLE: ral_event_state_enum = 0;
pub const ral_event_state_enum_RAL_RX_PKT: ral_event_state_enum = 1;
pub const ral_event_state_enum_RAL_TX_PKT: ral_event_state_enum = 2;
pub const ral_event_state_enum_RAL_RX_ACK: ral_event_state_enum = 3;
pub const ral_event_state_enum_RAL_TX_ACK: ral_event_state_enum = 4;
pub const ral_event_state_enum_RAL_ED: ral_event_state_enum = 5;
#[doc = " @brief Enum defines the current state for RAL events\n\n\n\t\t\t\t\t\tral event allocation\n\t\t\t\t\t\t  ==============\nral_add_tx_fifo() _ _ _ =  RAL_IDLE  =\n\t\t\t\t\t\t  ==============\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t      ==============\n\t\t\t\t\t\t\t\t\t\t\t      \t\t\t_ _(Requires ack)_ _ _ _ _ _ _= RAL_RX_ACK =\n\t\t\t\t\t\tral event allocation     \t\t  /             \t\t\t      ==============\n\t\t\t\t\t\t  =============                \t /\n ral_start_tx() _ _ _ _ = RAL_TX_PKT =_ _ ral_isr()_ _/\n\t\t\t\t\t\t  =============\t\t   \t\t\t\\\n\t\t\t\t\t\t  \t\t\t\t\t    \t\t \\\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\t  \t\t\t\t\t     \t\t  \\_ _(Doesn't require ack)_ _ _ _= RAL_TX_PKT =\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t  ==============\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t      \t  ==============\n\t\t\t\t\t\t\t\t\t\t\t      \t\t\t_ _(Requires ack)_ _ _ _ _ _ _= RAL_TX_ACK =\n\t\t\t\t\t\tral event allocation     \t\t  /             \t\t\t      ==============\n\t\t\t\t\t\t  =============                \t /\n ral_start_rx() _ _ _ _ = RAL_RX_PKT =_ _ ral_isr()_ _/\n\t\t\t\t\t\t  =============\t\t   \t\t\t\\\n\t\t\t\t\t\t  \t\t\t\t\t    \t\t \\\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\t  \t\t\t\t\t     \t\t  \\_ _(Doesn't require ack)_ _ _ _= RAL_RX_PKT =\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\tral event allocation\n\t\t\t\t\t\t  ==============\nral_ed_scan() _ _ _ _ _ =   RAL_ED   =\n\t\t\t\t\t\t  ==============\n\n"]
pub type ral_event_state_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines the current state for RAL events\n\n\n\t\t\t\t\t\tral event allocation\n\t\t\t\t\t\t  ==============\nral_add_tx_fifo() _ _ _ =  RAL_IDLE  =\n\t\t\t\t\t\t  ==============\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t      ==============\n\t\t\t\t\t\t\t\t\t\t\t      \t\t\t_ _(Requires ack)_ _ _ _ _ _ _= RAL_RX_ACK =\n\t\t\t\t\t\tral event allocation     \t\t  /             \t\t\t      ==============\n\t\t\t\t\t\t  =============                \t /\n ral_start_tx() _ _ _ _ = RAL_TX_PKT =_ _ ral_isr()_ _/\n\t\t\t\t\t\t  =============\t\t   \t\t\t\\\n\t\t\t\t\t\t  \t\t\t\t\t    \t\t \\\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\t  \t\t\t\t\t     \t\t  \\_ _(Doesn't require ack)_ _ _ _= RAL_TX_PKT =\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t  ==============\n\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t      \t  ==============\n\t\t\t\t\t\t\t\t\t\t\t      \t\t\t_ _(Requires ack)_ _ _ _ _ _ _= RAL_TX_ACK =\n\t\t\t\t\t\tral event allocation     \t\t  /             \t\t\t      ==============\n\t\t\t\t\t\t  =============                \t /\n ral_start_rx() _ _ _ _ = RAL_RX_PKT =_ _ ral_isr()_ _/\n\t\t\t\t\t\t  =============\t\t   \t\t\t\\\n\t\t\t\t\t\t  \t\t\t\t\t    \t\t \\\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\t  \t\t\t\t\t     \t\t  \\_ _(Doesn't require ack)_ _ _ _= RAL_RX_PKT =\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t  ==============\n\t\t\t\t\t\tral event allocation\n\t\t\t\t\t\t  ==============\nral_ed_scan() _ _ _ _ _ =   RAL_ED   =\n\t\t\t\t\t\t  ==============\n\n"]
pub use self::ral_event_state_enum as ral_event_state_enum_t;
pub const ral_state_enum_RAL_DISABLE: ral_state_enum = 0;
pub const ral_state_enum_RAL_ENABLE: ral_state_enum = 1;
#[doc = " @brief Enum used for RAL configuration assignment\n"]
pub type ral_state_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum used for RAL configuration assignment\n"]
pub use self::ral_state_enum as ral_state_enum_t;
pub const ral_error_enum_RAL_ERROR_NONE: ral_error_enum = 0;
pub const ral_error_enum_RAL_ERROR_FAILED: ral_error_enum = 1;
pub const ral_error_enum_RAL_ERROR_DROP: ral_error_enum = 2;
pub const ral_error_enum_RAL_ERROR_NO_BUFS: ral_error_enum = 3;
pub const ral_error_enum_RAL_ERROR_BUSY: ral_error_enum = 5;
pub const ral_error_enum_RAL_ERROR_INVALID_PARAMETERS: ral_error_enum = 7;
pub const ral_error_enum_RAL_ERROR_SECURITY: ral_error_enum = 8;
pub const ral_error_enum_RAL_ERROR_NO_ADDRESS: ral_error_enum = 10;
pub const ral_error_enum_RAL_ERROR_ABORT: ral_error_enum = 11;
pub const ral_error_enum_RAL_ERROR_NOT_SUPPORTED: ral_error_enum = 12;
pub const ral_error_enum_RAL_ERROR_INVALID_STATE: ral_error_enum = 13;
pub const ral_error_enum_RAL_ERROR_NO_ACK: ral_error_enum = 14;
pub const ral_error_enum_RAL_ERROR_CCA_FAILURE: ral_error_enum = 15;
pub const ral_error_enum_RAL_ERROR_FCS: ral_error_enum = 17;
pub const ral_error_enum_RAL_ERROR_NO_FRAME_RECEIVED: ral_error_enum = 18;
pub const ral_error_enum_RAL_ERROR_INVALID_SOURCE_ADDRESS: ral_error_enum = 20;
pub const ral_error_enum_RAL_ERROR_DESTINATION_ADDRESS_FILTERED: ral_error_enum = 22;
pub const ral_error_enum_RAL_ERROR_TIMER_ISR: ral_error_enum = 23;
pub const ral_error_enum_RAL_ERROR_LINK_METRICS_INVALID_ARGS: ral_error_enum = 24;
pub const ral_error_enum_RAL_ERROR_LINK_METRICS_NOT_FOUND: ral_error_enum = 25;
pub const ral_error_enum_RAL_ERROR_LINK_METRICS_NO_BUF: ral_error_enum = 26;
pub const ral_error_enum_RAL_ERROR_AD_NOT_IN_CONFIG_STATE: ral_error_enum = 27;
pub const ral_error_enum_RAL_ERROR_CONFIG_LIB_NOT_IN_CONFIG_STATE: ral_error_enum = 28;
pub const ral_error_enum_RAL_ERROR_GENERIC: ral_error_enum = 255;
#[doc = " @brief Enum defines RAL error codes\n"]
pub type ral_error_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines RAL error codes\n"]
pub use self::ral_error_enum as ral_error_enum_t;
pub const ral_power_state_enum_RAL_POWER_SLEEP: ral_power_state_enum = 0;
pub const ral_power_state_enum_RAL_POWER_ACTIVE: ral_power_state_enum = 1;
#[doc = " @brief Enum defines power states for RAL where:\n\n RAL_POWER_SLEEP: power state of the RAL when not executing any event.\n RAL_POWER_ACTIVE: power state of the RAL before starting of any event."]
pub type ral_power_state_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines power states for RAL where:\n\n RAL_POWER_SLEEP: power state of the RAL when not executing any event.\n RAL_POWER_ACTIVE: power state of the RAL before starting of any event."]
pub use self::ral_power_state_enum as ral_power_state_enum_t;
pub const tx_new_retry_enum_CONTINUE_CSMA_RETRY: tx_new_retry_enum = 0;
pub const tx_new_retry_enum_START_NEW_FULL_TX_RETRY: tx_new_retry_enum = 1;
#[doc = " @brief Enum defines new TX retrial type:\n\n CONTINUE_CSMA_RETRY: continue of CSMA retry from csma_backoff_count.\n START_NEW_FULL_TX_RETRY: start new TX retry from frm_retries_count."]
pub type tx_new_retry_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines new TX retrial type:\n\n CONTINUE_CSMA_RETRY: continue of CSMA retry from csma_backoff_count.\n START_NEW_FULL_TX_RETRY: start new TX retry from frm_retries_count."]
pub use self::tx_new_retry_enum as tx_new_retry_enum_t;
pub const ral_pkt_src_enum_RAL_SOURCE_FIFO: ral_pkt_src_enum = 0;
pub const ral_pkt_src_enum_RAL_SOURCE_PACKET: ral_pkt_src_enum = 1;
#[doc = " @brief Enum defines packet source for transmission when calling ral_start_tx() to be added properly in RAL packet.\n\n RAL_SOURCE_PACKET: used in case of single packet transmission mode the packet is passed in ral_start_tx() and then copied to the allocated event."]
pub type ral_pkt_src_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines packet source for transmission when calling ral_start_tx() to be added properly in RAL packet.\n\n RAL_SOURCE_PACKET: used in case of single packet transmission mode the packet is passed in ral_start_tx() and then copied to the allocated event."]
pub use self::ral_pkt_src_enum as ral_pkt_src_enum_t;
pub const ral_ack_type_enum_RAL_NO_ACK: ral_ack_type_enum = 0;
pub const ral_ack_type_enum_RAL_ACK_CUSTOM: ral_ack_type_enum = 1;
pub const ral_ack_type_enum_RAL_ACK_MAC: ral_ack_type_enum = 2;
#[doc = " @brief Enum defines type of acknowledgment packet used in MAC 802.15.4\n"]
pub type ral_ack_type_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines type of acknowledgment packet used in MAC 802.15.4\n"]
pub use self::ral_ack_type_enum as ral_ack_type_enum_t;
#[doc = "< Initial state"]
pub const config_lib_state_enum_NOT_CONFIGURED_NOT_INITIALIZED: config_lib_state_enum = 0;
#[doc = "< The state of post-reset or configuring using API"]
pub const config_lib_state_enum_CONFIGURED_NOT_INITIALIZED: config_lib_state_enum = 1;
#[doc = "< Post-initialization state"]
pub const config_lib_state_enum_CONFIGURED_INITIALIZED: config_lib_state_enum = 2;
#[doc = " @brief Enum defines configurable library feature states\n"]
pub type config_lib_state_enum = ::core::ffi::c_uint;
#[doc = " @brief Enum defines configurable library feature states\n"]
pub use self::config_lib_state_enum as config_lib_state_enum_t;
#[doc = " Define RAL time structure that contains fine and base"]
pub type ral_time_st = ble_time_t;
#[doc = " @brief Structure represents RAL packet and contains the parameters of the sent/received RAL packet\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_pkt_st {
    #[doc = "< Pointer to packet"]
    pub ptr_pyld: *mut u8,
    #[doc = "< Exact time in which the packet transmitted/received"]
    pub time_stamp: ral_time_st,
    #[doc = "< Variable to store the number of cycles the overflow when packet is received"]
    pub num_ov_cycles: u16,
    #[doc = "< Packet length"]
    pub pyld_len: u16,
    #[doc = "< Channel at which the packet will be transmitted"]
    pub channel: u8,
    #[doc = "< The RX channel after frame TX is done (after all frame retries - ack received, or timeout, or abort)."]
    pub rxchannelaftertxdone: u8,
    pub tx_rx_u: _ral_pkt_st__bindgen_ty_1,
}
#[doc = " @brief Union represents TX/RX info\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _ral_pkt_st__bindgen_ty_1 {
    pub tx_info: _ral_pkt_st__bindgen_ty_1__bindgen_ty_1,
    pub rx_info: _ral_pkt_st__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_pkt_st__bindgen_ty_1__bindgen_ty_1 {
    #[doc = "< Source of transmitted packet"]
    pub pkt_src: ral_pkt_src_enum_t,
    #[doc = "< Pointer to the security key used in security processing"]
    pub sec_key: *mut u8,
    #[doc = "< Flag to mark if security processed by radio or not"]
    pub is_sec_proc_by_radio: u8,
    #[doc = "< Power of transmitted packet"]
    pub tx_power: i8,
    #[doc = "< Last transmitted packet flag"]
    pub last_tx_pkt: u8,
    #[doc = "< True only if the current TX frame is a CSL frame"]
    pub csl_frame: u8,
    #[doc = "< True only if the current TX frame is a poll request frame"]
    pub is_poll_req: u8,
}
impl Default for _ral_pkt_st__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ral_pkt_st__bindgen_ty_1__bindgen_ty_2 {
    #[doc = "< frame counter used for secured Enhanced ACK"]
    pub ack_frm_cntr: u32,
    #[doc = "< key index used for secured Enhanced ACK"]
    pub ack_key_id: u8,
    #[doc = "< flag to mark usage of secured Enhanced ACK"]
    pub is_sec_enh_ack: u8,
    #[doc = "< received signal strength indicator"]
    pub rssi: i8,
    #[doc = "< link quality indicator"]
    pub lqi: u8,
    #[doc = "< This indicates if this frame was acknowledged with frame pending set"]
    pub ackFrmPending: u8,
}
impl Default for _ral_pkt_st__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for _ral_pkt_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents RAL packet and contains the parameters of the sent/received RAL packet\n"]
pub type ral_pkt_st = _ral_pkt_st;
#[doc = " @brief Structure represents RAL event information and contains the full information of the transmitted/received event\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_evnt_info_st {
    #[doc = "< Pointer to the next event, used in case of FIFO transmission"]
    pub ptr_nxt_evnt: *mut _ral_evnt_info_st,
    pub ral_evnt_info_u: _ral_evnt_info_st__bindgen_ty_1,
    #[doc = "< HW error mask of the received packet"]
    pub ral_status_mask: u32,
    #[doc = "< RAL identifier of this event"]
    pub ral_instance: ral_instance_t,
    #[doc = "< Event type"]
    pub event_state: ral_event_state_enum_t,
    #[doc = "< Event error passed to higher layers"]
    pub evnt_error: ral_error_enum_t,
    #[doc = "< Either the PTA TX or RX reject counter depending on the packet type"]
    pub tx_pta_counter: u8,
    #[doc = "< Either the PTA TX or RX reject counter depending on the packet type"]
    pub rx_pta_counter: u8,
}
#[doc = " @brief Union represents TX/RX and ED event info\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _ral_evnt_info_st__bindgen_ty_1 {
    pub pkt_info: _ral_evnt_info_st__bindgen_ty_1__bindgen_ty_1,
    pub ed_info: _ral_evnt_info_st__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_evnt_info_st__bindgen_ty_1__bindgen_ty_1 {
    #[doc = "< Pointer to data packet"]
    pub ptr_pkt: *mut ral_pkt_st,
    #[doc = "< Pointer to the ACK packet used by this event"]
    pub ptr_ack_pkt: *mut ral_pkt_st,
}
impl Default for _ral_evnt_info_st__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ral_evnt_info_st__bindgen_ty_1__bindgen_ty_2 {
    #[doc = "< Energy detection scan duration"]
    pub ed_scn_durn: u32,
    #[doc = "< Energy detection max RSSI value"]
    pub ed_max_rssi: i8,
}
impl Default for _ral_evnt_info_st__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for _ral_evnt_info_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents RAL event information and contains the full information of the transmitted/received event\n"]
pub type ral_evnt_info_st = _ral_evnt_info_st;
#[doc = " @brief Structure represents MAC filtration parameters used by HW to filter received packets\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_mac_fltr_confg_st {
    #[doc = "< The device extended address to compare the received address with"]
    pub ext_addr: [u8; 8usize],
    #[doc = "< The device short address to compare the received address with"]
    pub short_addr: u16,
    #[doc = "< The device PAN ID to compare the received PAN ID in the packet with"]
    pub pan_id: u16,
    #[doc = "< MAC filter state, if disabled so the promiscuous mode is enabled"]
    pub mac_fltr_state: ral_state_enum_t,
    #[doc = "< MAC ImplictBoradcast PIB that is set in MAC layer"]
    pub mac_implicit_broadcast: u8,
    #[doc = " The first bit used to determine if the device is pan coordinator or not.\n The second bit is indicating if this instance is currently performing scanning or not."]
    pub is_pan_coord: u8,
}
impl Default for _ral_mac_fltr_confg_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MAC filtration parameters used by HW to filter received packets\n"]
pub type ral_mac_fltr_confg_st = _ral_mac_fltr_confg_st;
#[doc = " @brief Structure represents configurations of filtration before sending ACK.\n\n \t\t  This structure contains the parameters used to filter the received packet\n \t\t  and respond to this packet with ack or not\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_ack_rspnd_fltr_confg_st {
    #[doc = "< Pointer to value to be compared with received data"]
    pub ptr_comp_value: *mut u8,
    #[doc = "< Position of the byte start to be compared in the received data"]
    pub byte_index: u8,
    #[doc = "< Number of bytes to be compared in the received data"]
    pub byte_len: u8,
    #[doc = "< Enable/Disable packet filtration before sending ack"]
    pub ack_fltr_state: ral_state_enum_t,
}
impl Default for _ral_ack_rspnd_fltr_confg_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents configurations of filtration before sending ACK.\n\n \t\t  This structure contains the parameters used to filter the received packet\n \t\t  and respond to this packet with ack or not\n"]
pub type ral_ack_rspnd_fltr_confg_st = _ral_ack_rspnd_fltr_confg_st;
#[doc = " @brief Structure represents ACK request bit configuration\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_ack_req_confg_st {
    #[doc = "< Enable/Disable ACK request bit check"]
    pub ack_req_bit_state: ral_state_enum_t,
    #[doc = "< Byte index of ACK request bit in TX/RX packet"]
    pub byte_index: u8,
    #[doc = "< Bit index of ACK request bit in TX/RX packet"]
    pub bit_index: u8,
}
impl Default for _ral_ack_req_confg_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents ACK request bit configuration\n"]
pub type ral_ack_req_confg_st = _ral_ack_req_confg_st;
#[doc = " @brief Structure represents RAL ACK data and contains the parameters of the acknowledgment configuration\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_auto_ack_confg_st {
    #[doc = "< Contains configured filters applied to the received packet to determine whether to send ACK or not"]
    pub rspnd_fltr_confg: ral_ack_rspnd_fltr_confg_st,
    #[doc = "< Contains configured ACK request bit configuration"]
    pub ack_req_confg: ral_ack_req_confg_st,
    #[doc = "< Time in microseconds between the received packet and TX ACK"]
    pub auto_tx_ack_turnaround: u16,
    #[doc = "< Time in microseconds between the transmitted packet and RX ACK"]
    pub auto_rx_ack_turnaround: u16,
    #[doc = "< Timeout in microseconds to wait for receiving immediate ACK"]
    pub auto_rx_ack_timeout: u16,
    #[doc = "< Timeout in microseconds to wait for receiving enhanced ACK"]
    pub auto_rx_enh_ack_timeout: u16,
    #[doc = "< ACK type"]
    pub ack_type: ral_ack_type_enum_t,
    #[doc = "< Enable/Disable automatic ACK transmission"]
    pub auto_tx_ack_state: ral_state_enum_t,
    #[doc = "< Enable/Disable automatic ACK reception"]
    pub auto_rx_ack_state: ral_state_enum_t,
}
impl Default for _ral_auto_ack_confg_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents RAL ACK data and contains the parameters of the acknowledgment configuration\n"]
pub type ral_auto_ack_confg_st = _ral_auto_ack_confg_st;
#[doc = " @brief Structure represents AMAC parameters\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_a_mac_params_st {
    #[doc = "< ACK configurations"]
    pub auto_ack_config: ral_auto_ack_confg_st,
    #[doc = "< Interframe spacing value"]
    pub ifs: u16,
    #[doc = "< PHY rate"]
    pub phy_rate: ral_phy_rate_enum_t,
}
impl Default for _ral_a_mac_params_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents AMAC parameters\n"]
pub type ral_a_mac_params_st = _ral_a_mac_params_st;
#[doc = " @brief Structure represents RAL event info in case of coexistence\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ral_coex_info_st {
    #[doc = "< Pointer to RAL event handle given from the event scheduler after registration"]
    pub evnt_hndl: *mut ::core::ffi::c_void,
    #[doc = "< End time in sleep timer steps of grant given from the event scheduler"]
    pub grant_end_time: ble_time_t,
}
impl Default for _ral_coex_info_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents RAL event info in case of coexistence\n"]
pub type ral_coex_info_st = _ral_coex_info_st;
#[doc = " @brief Structure represents MAC address\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_address_ {
    #[doc = "< Pointer to the address"]
    pub ptr_address: *mut u8,
    #[doc = "< Address mode"]
    pub address_mode: mac_addrs_mode_enum_t,
}
impl Default for mac_address_ {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents MAC address\n"]
pub type mac_address_st = mac_address_;
#[doc = " @brief Structure represents RAL Copy of radio otLinkMetrics structure\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct link_metrics_info_ {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl link_metrics_info_ {
    #[inline]
    pub fn mPduCount(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mPduCount(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mPduCount_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mPduCount_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mLqi(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mLqi(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mLqi_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mLqi_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mLinkMargin(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mLinkMargin(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mLinkMargin_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mLinkMargin_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mRssi(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mRssi(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mRssi_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mRssi_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mReserved(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mReserved(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mReserved_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_mReserved_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mPduCount: u8,
        mLqi: u8,
        mLinkMargin: u8,
        mRssi: u8,
        mReserved: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mPduCount: u8 = unsafe { ::core::mem::transmute(mPduCount) };
            mPduCount as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let mLqi: u8 = unsafe { ::core::mem::transmute(mLqi) };
            mLqi as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let mLinkMargin: u8 = unsafe { ::core::mem::transmute(mLinkMargin) };
            mLinkMargin as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let mRssi: u8 = unsafe { ::core::mem::transmute(mRssi) };
            mRssi as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let mReserved: u8 = unsafe { ::core::mem::transmute(mReserved) };
            mReserved as u64
        });
        __bindgen_bitfield_unit
    }
}
#[doc = " @brief Structure represents RAL Copy of radio otLinkMetrics structure\n"]
pub type link_metrics_info_st = link_metrics_info_;
pub type p_link_metric_data_info_st = *mut link_metric_data_info_st_;
#[doc = " @brief Structure represents link metrics initiator node\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_metric_data_info_st_ {
    #[doc = "< Pointer to the next initiator node"]
    pub ptr_nxt_node: p_link_metric_data_info_st,
    pub initiator_address_field_st: link_metric_data_info_st___bindgen_ty_1,
    #[doc = "< Link metrics info"]
    pub initiator_link_metrics: link_metrics_info_st,
}
#[doc = " @brief Structure represents initiator address\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct link_metric_data_info_st___bindgen_ty_1 {
    #[doc = "< The extended address of the initiator node"]
    pub extended_address: [u8; 8usize],
    #[doc = "< The short address of the initiator node"]
    pub short_address: u16,
}
impl Default for link_metric_data_info_st_ {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents link metrics initiator node\n"]
pub type link_metric_data_info_st = link_metric_data_info_st_;
#[doc = " @brief Structure represents RAL callback functions which should be called after TX/RX/ED is done\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ral_cbk_dispatch_tbl_st {
    #[doc = " RAL TX done callback"]
    pub ral_tx_done: ::core::option::Option<
        unsafe extern "C" fn(
            ral_instance: ral_instance_t,
            ptr_tx_pkt: *mut ral_pkt_st,
            ptr_ack_pkt: *mut ral_pkt_st,
            tx_error: ral_error_enum_t,
        ),
    >,
    #[doc = " RAL RX done callback"]
    pub ral_rx_done: ::core::option::Option<
        unsafe extern "C" fn(
            ral_instance: ral_instance_t,
            ptr_rx_pkt: *mut ral_pkt_st,
            rx_error: ral_error_enum_t,
        ),
    >,
    #[doc = " RAL ED done callback"]
    pub ral_ed_scan_done: ::core::option::Option<
        unsafe extern "C" fn(ral_instance: ral_instance_t, scan_durn: u32, max_rssi: i8),
    >,
    #[doc = " This callback will be called when device receives packet that requires custom ack"]
    pub ral_configure_custom_ack: ::core::option::Option<
        unsafe extern "C" fn(
            ral_instance: ral_instance_t,
            ptr_ack_pkt: *mut u8,
            ack_len: *mut u16,
            ptr_rx_pkt: *mut u8,
        ),
    >,
}
#[doc = " @brief Structure represents RAL callback functions which should be called after TX/RX/ED is done\n"]
pub type ral_cbk_dispatch_tbl_st = _ral_cbk_dispatch_tbl_st;
pub const control_flags_shift_PHY_RATE_SHIFT: control_flags_shift = 0;
pub const control_flags_shift_PHY_TX_LOWLTNCY_SHIFT: control_flags_shift = 2;
pub const control_flags_shift_PHY_RX_LOWLTNCY_SHIFT: control_flags_shift = 3;
pub const control_flags_shift_TXPP_BYPASS_CRC_SHIFT: control_flags_shift = 4;
pub const control_flags_shift_SCAN_BCN_SHIFT: control_flags_shift = 5;
pub const control_flags_shift_DROP_ON_ERR_SHIFT: control_flags_shift = 6;
pub const control_flags_shift_PANCOORD_SHIFT: control_flags_shift = 7;
pub const control_flags_shift_MACIMPLICITBROADCAST_SHIFT: control_flags_shift = 8;
pub const control_flags_shift_MACGRPRXMODE_SHIFT: control_flags_shift = 9;
pub const control_flags_shift_MAC_PRMISCMOD_SHIFT: control_flags_shift = 10;
pub const control_flags_shift_VDDH_PA_SHIFT: control_flags_shift = 11;
pub const control_flags_shift_EPA_DISABLE_SHIFT: control_flags_shift = 15;
#[doc = " @brief Enum defines bit shift amount for control flags used to configure HW\n"]
pub type control_flags_shift = ::core::ffi::c_uint;
pub const error_flags_shift_TIMEOUT_FLAG_SHIFT: error_flags_shift = 2;
pub const error_flags_shift_ADDMODE_ERR_SHIFT: error_flags_shift = 3;
pub const error_flags_shift_RX_ERR_SHIFT: error_flags_shift = 4;
pub const error_flags_shift_PPDU_ERR_SHIFT: error_flags_shift = 5;
pub const error_flags_shift_FRMLNGTH_ERR_SHIFT: error_flags_shift = 6;
pub const error_flags_shift_FRMTYPE_ERR_SHIFT: error_flags_shift = 7;
pub const error_flags_shift_FRMVRSN_ERR_SHIFT: error_flags_shift = 8;
pub const error_flags_shift_DSTPANID_ERR_SHIFT: error_flags_shift = 9;
pub const error_flags_shift_SRCPANID_ERR_SHIFT: error_flags_shift = 10;
pub const error_flags_shift_ACK_OK_SHIFT: error_flags_shift = 11;
pub const error_flags_shift_ACK_ERR_SHIFT: error_flags_shift = 12;
pub const error_flags_shift_CRC_ERR_SHIFT: error_flags_shift = 13;
pub const error_flags_shift_DSTADDR_ERR_SHIFT: error_flags_shift = 14;
pub const error_flags_shift_SEC_ERR_SHIFT: error_flags_shift = 30;
#[doc = " @brief Enum defines bit shift amount for errors returned after event completion\n"]
pub type error_flags_shift = ::core::ffi::c_uint;
#[doc = " @brief Structure contains HW event descriptor fields that:\n \t\t  1- Required to be filled before start of the event\n \t\t  2- Need to be checked after the end of the event.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llhwc_mac_evnt_info_mem_st {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 48usize]>,
}
impl Default for llhwc_mac_evnt_info_mem_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl llhwc_mac_evnt_info_mem_st {
    #[inline]
    pub fn mac_sfd_value(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_mac_sfd_value(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_sfd_value_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_sfd_value_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_preamble_val(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(32usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_mac_preamble_val(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(32usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_preamble_val_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                32usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_preamble_val_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_panid(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(64usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_mac_panid(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(64usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_panid_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                64usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_panid_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                64usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_shortaddr(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(80usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_mac_shortaddr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(80usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_shortaddr_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                80usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_shortaddr_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                80usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn eui64add_LSW(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(96usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_eui64add_LSW(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(96usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn eui64add_LSW_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                96usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_eui64add_LSW_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                96usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn eui64add_MSW(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(128usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_eui64add_MSW(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(128usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn eui64add_MSW_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                128usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_eui64add_MSW_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                128usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_extaddr_LSW(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(160usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_mac_extaddr_LSW(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(160usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_extaddr_LSW_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                160usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_extaddr_LSW_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                160usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_extaddr_MSW(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(192usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_mac_extaddr_MSW(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(192usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_extaddr_MSW_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                192usize,
                32u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_extaddr_MSW_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                192usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn error_flags(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(224usize, 15u8) as u32) }
    }
    #[inline]
    pub fn set_error_flags(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(224usize, 15u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn error_flags_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                224usize,
                15u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_error_flags_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                224usize,
                15u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rx_frm_len(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(240usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_rx_frm_len(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(240usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rx_frm_len_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                240usize,
                7u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_rx_frm_len_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                240usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn debug_ports(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(247usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_debug_ports(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(247usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn debug_ports_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                247usize,
                5u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_debug_ports_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                247usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rssi_out(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(256usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_rssi_out(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(256usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rssi_out_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                256usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_rssi_out_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                256usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn LQI(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(272usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_LQI(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(272usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn LQI_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                272usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_LQI_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                272usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn frmlngth(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(288usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_frmlngth(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(288usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn frmlngth_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                288usize,
                7u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_frmlngth_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                288usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_sfd_len(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(296usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_mac_sfd_len(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(296usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_sfd_len_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                296usize,
                3u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_sfd_len_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                296usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn mac_preamble_len(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(299usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_mac_preamble_len(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(299usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mac_preamble_len_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                299usize,
                3u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_mac_preamble_len_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                299usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn seqnum(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(304usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_seqnum(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(304usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn seqnum_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                304usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_seqnum_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                304usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn tx_latency(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(312usize, 6u8) as u32) }
    }
    #[inline]
    pub fn set_tx_latency(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(312usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn tx_latency_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                312usize,
                6u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_tx_latency_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                312usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn control_flags(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(320usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_control_flags(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(320usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn control_flags_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                320usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_control_flags_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                320usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn phy_drv_seq_strt_addr(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(336usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_phy_drv_seq_strt_addr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(336usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn phy_drv_seq_strt_addr_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                336usize,
                7u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_phy_drv_seq_strt_addr_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                336usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn phy_drv_seq_end_addr(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(344usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_phy_drv_seq_end_addr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(344usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn phy_drv_seq_end_addr_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                344usize,
                7u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_phy_drv_seq_end_addr_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                344usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn Pointer_To_current_TX(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(352usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_Pointer_To_current_TX(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(352usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn Pointer_To_current_TX_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                352usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_Pointer_To_current_TX_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                352usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn Pointer_To_current_RX(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(368usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_Pointer_To_current_RX(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(368usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn Pointer_To_current_RX_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 48usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                368usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_Pointer_To_current_RX_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 48usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                368usize,
                16u8,
                val as u64,
            )
        }
    }
}
#[doc = " @brief Structure contains HW event descriptor fields that:\n \t\t  1- Required to be filled before start of the event\n \t\t  2- Need to be checked after the end of the event.\n"]
pub type llhwc_mac_evnt_info_mem_t = llhwc_mac_evnt_info_mem_st;
#[doc = " @brief Structure represents updated security parameters\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_update_desc_st_ {
    #[doc = "< Pointer to the security key"]
    pub ptr_key: *mut u8,
    #[doc = "< Frame counter"]
    pub frm_cntr: u32,
    #[doc = "< Header length"]
    pub hdr_len: u32,
    #[doc = "< MIC length"]
    pub mic_len: u8,
    #[doc = "< Security level"]
    pub sec_lvl: u8,
}
impl Default for sec_update_desc_st_ {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents updated security parameters\n"]
pub type sec_update_desc_st = sec_update_desc_st_;
unsafe extern "C" {
    #[doc = "< MAC event descriptor shared between HW and FW"]
    #[link_name = "\u{1}g_mac_event_info"]
    pub static mut G_MAC_EVENT_INFO: *mut llhwc_mac_evnt_info_mem_t;
}
unsafe extern "C" {
    #[doc = " @brief\tRAL initialization\n\n @param[in] ptr_cbk_dispatch_tbl : Pointer to callbacks dispatch table\n\n @retval RAL instance associated to this context that should be used with any RAL interface"]
    pub fn ral_init(ptr_cbk_dispatch_tbl: *mut ral_cbk_dispatch_tbl_st) -> ral_instance_t;
}
unsafe extern "C" {
    #[doc = " @brief\tRAL reset\n\n @param[in] ral_instance : RAL instance"]
    pub fn ral_reset(ral_instance: ral_instance_t);
}
unsafe extern "C" {
    #[doc = " @brief\tSwitch power either sleep or active\n\n @param[in] ral_instance  : RAL instance\n @param[in] power_state   : New power state\n @param[in] ptr_coex_info : Pointer to the current coexistence parameters\n\n @retval RAL_ERROR_NONE if power state changed successfully"]
    pub fn ral_power_switch(
        ral_instance: ral_instance_t,
        power_state: ral_power_state_enum_t,
        ptr_coex_info: *mut ral_coex_info_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet current power state\n\n @param[in] ral_instance : RAL instance\n\n @retval Current power state: RAL_POWER_SLEEP or RAL_POWER_ACTIVE"]
    pub fn ral_get_power_state(ral_instance: ral_instance_t) -> ral_power_state_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet current event state and current channel if not idle\n\n @param[out] curr_ral_instance : Current RAL instance if not idle\n @param[out] curr_event_channel : Current event channel if not idle\n\n @retval Current event state: RX, TX, ED, or IDLE"]
    pub fn ral_get_current_event_state(
        curr_ral_instance: *mut ral_instance_t,
        curr_event_channel: *mut u8,
    ) -> ral_event_state_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tSet antenna diversity feature parameters\n\n @param[in] ral_instance       : RAL instance\n @param[in] ptr_ant_div_params : Pointer to antenna diversity parameters\n\n @retval RAL_ERROR_NONE if antenna diversity parameters are set correctly"]
    pub fn ral_set_ant_div_params(
        ral_instance: ral_instance_t,
        ptr_ant_div_params: *mut antenna_diversity_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet current antenna diversity feature parameters\n\n @param[in] ral_instance       \t: RAL instance\n @param[out] ptr_ant_div_params \t: Pointer to current antenna diversity parameters"]
    pub fn ral_get_ant_div_params(
        ral_instance: ral_instance_t,
        ptr_ant_div_params: *mut antenna_diversity_st,
    );
}
unsafe extern "C" {
    #[doc = " @brief\tEnable/disable antenna diversity feature\n\n @param[in] ral_instance   : RAL instance\n @param[in] enable         : Enable:1 / disable:0\n\n @retval RAL_ERROR_NONE if antenna diversity is enabled/disabled correctly"]
    pub fn ral_set_ant_div_enable(ral_instance: ral_instance_t, enable: u8) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tSet the default antenna id to be used for transmission and reception\n\n @param[in] ral_instance    : RAL instance\n @param[in] default_ant_id  : The antenna id to be used as default\n\n @retval RAL_ERROR_NONE if default antenna ID is set correctly"]
    pub fn ral_set_default_ant_id(
        ral_instance: ral_instance_t,
        default_ant_id: u8,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tSet RSSI threshold for antenna diversity feature\n\n @param[in] ral_instance     : RAL instance\n @param[in] rssi_threshold   : RSSI threshold to compare with during antenna diversity measurements\n\n @retval RAL_ERROR_NONE if antenna diversity RSSI threshold is set correctly"]
    pub fn ral_set_ant_div_rssi_threshold(
        ral_instance: ral_instance_t,
        rssi_threshold: i8,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief Check whether a MAC channel is about to be calibrated.\n\n @param[in] channel : MAC channel\n @return TRUE to run the MAC channel calibration, FALSE to postpone it."]
    pub fn ral_is_about_to_clbr_mac_ch(channel: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief Set the PHY calibration state for a MAC channel to pending.\n\n @param[in] channel : MAC channel"]
    pub fn ral_set_mac_chnl_clbr_pending(channel: u8);
}
unsafe extern "C" {
    #[doc = " @brief Execute pending MAC channels calibration\n"]
    pub fn ral_exec_mac_ch_phy_clbr_pending();
}
unsafe extern "C" {
    #[doc = " @brief Get the duration of single MAC channel calibration in sleep timer steps\n\n @return Duration of single MAC channel calibration in sleep timer steps"]
    pub fn ral_get_mac_ch_clbr_durn() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief\tSet PHY rate for transmission/reception\n\n @param[in] ral_instance : RAL instance\n @param[in] phy_rate : New PHY rate [1M/2M/256K/125K]\n\n @retval RAL_ERROR_NONE if phy rate changed successfully"]
    pub fn ral_set_rate(
        ral_instance: ral_instance_t,
        phy_rate: ral_phy_rate_enum_t,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tSet minimum interframe spacing between successive transmission/reception\n\n @param[in] ral_instance \t: RAL instance\n @param[in] min_ifs \t\t: New minimum interframe spacing in microseconds"]
    pub fn ral_set_min_ifs(ral_instance: ral_instance_t, min_ifs: u16);
}
unsafe extern "C" {
    #[doc = " @brief\tSet interframe spacing between successive transmission/reception\n\n @param[in] ral_instance \t: RAL instance\n @param[in] ifs \t\t\t: New interframe spacing in microseconds\n\n @retval RAL_ERROR_NONE if interframe spacing changed successfully"]
    pub fn ral_set_ifs(ral_instance: ral_instance_t, ifs: u16) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tStart packet transmission\n \t\t\tThis function is responsible for preparation for transmission of a packet by allocating and preparing a new ral event/ral pkt to be executed by HW.\n \t\t\tAfter completion of the Transmission event or if stop operation ral_tx_done() will be called carrying the status of event.\n\n @param[in] ral_instance \t  \t: RAL instance\n @param[in] pkt_src \t \t  \t: Transmission packet source, FIFO based or Packet based\n @param[in] ptr_pkt \t \t  \t: Pointer to the transmitted packet if packet source is Packet based only\n @param[in] ptr_start_time \t: Pointer to the start time structure which contains start time of transmission (SFD of packet)\n \t\t\t\t\t\t\t\t  if NULL function will use the current time get from llhwc_slptmr_get\n @param[in] periodic_interval : Periodic interval in microseconds, 0 means not periodic\n @param[in] ptr_coex_info \t: Pointer to the current coexistence parameters\n\n @retval RAL_ERROR_NONE if transmission started successfully\n \t\t   RAL_ERROR_INVALID_PARAMETERS if the passed parameters doesn't make sense e.g. starting fifo mode but ptr_fifo_head = NULL\n \t\t   RAL_ERROR_BUSY if there is a transmission event that already started and not ended yet\n\n @note: ral_tx_done won't be called unless emngr_handle_all_events() is called to call ral_sm_done which will call the ral_tx_done call back\n"]
    pub fn ral_start_tx(
        ral_instance: ral_instance_t,
        pkt_src: ral_pkt_src_enum_t,
        ptr_pkt: *mut ral_pkt_st,
        ptr_start_time: *mut ral_time_st,
        periodic_interval: u32,
        ptr_coex_info: *mut ral_coex_info_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tabort current transmission\n \t\t\tThis function is used for testing purpose\n\n @param[in] ral_instance : RAL instance\n\n @retval RAL_ERROR_NONE if transmission aborted successfully"]
    pub fn ral_abort_tx(ral_instance: ral_instance_t) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tAdd packet to transmission FIFO\n\n @param[in] ral_instance \t: RAL instance\n @param[in] ptr_pkt \t \t: Pointer to input packet allocated by user application\n\n @retval RAL_ERROR_NONE if packet added to FIFO successfully"]
    pub fn ral_add_tx_fifo(
        ral_instance: ral_instance_t,
        ptr_pkt: *mut ral_pkt_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tFlush current transmission FIFO\n\n @param[in] ral_instance : RAL instance\n\n @retval RAL_ERROR_NONE if FIFO flushed successfully"]
    pub fn ral_flush_fifo(ral_instance: ral_instance_t) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet transmission packet to prepare data before starting transmission.\n \t\t\tIt must be used in case of packet transmission mode, the packet is allocated from TX/RX shared memory.\n\n @param   ral_instance : RAL instance\n\n @retval Pointer to allocated RAL packet"]
    pub fn ral_get_tx_buf(ral_instance: ral_instance_t) -> *mut ral_pkt_st;
}
unsafe extern "C" {
    #[doc = " @}\n/\n/**  @ingroup ral_intf_ed\n  @{\n/\n/**\n @brief\tPerform Clear Channel Assessment on selected channel\n\n @param[in] channel \t\t\t: Channel to perform CCA on.\n @param[in] ptr_coex_info \t: Pointer to the current coexistence parameters\n @param[in] energyThreshold \t: The minimum value of RSSI to mark the channel busy,\n          \t\t\t\t\t  It is used only in case of phy 2.00a_tc. defined by cca_change_threshold_seq for other PHYs.\n @param[in] ral_instance\t\t: RAL instance\n @retval RAL_ERROR_NONE if no traffic on air, RAL_ERROR_CCA_FAILURE otherwise"]
    pub fn ral_perform_cca(
        channel: u8,
        ptr_coex_info: *mut ral_coex_info_st,
        energyThreshold: i8,
        ral_instance: ral_instance_t,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tStart packet reception\n \t\t\tThis function is responsible for preparation for reception of a packet by allocating and preparing a new RAL event/ral pkt to be executed by HW.\n \t\t\tAfter completion of the Reception event or if stop operation, ral_rx_done() will be called carrying the status of event and the packet received.\n\n\n @param[in] ral_instance \t\t: RAL instance\n @param[in] rx_channel \t\t: Channel to receive on\n @param[in] ptr_start_time \t: Pointer to the start time structure which contains start time of reception\n @param[in] timeout \t\t\t: Timeout of reception in microsecond from the start of reception\n @param[in] periodic_interval : Periodic interval in microsecond, 0 means not periodic\n @param[in] ptr_coex_info \t: Pointer to the current coexistence parameters\n\n @retval RAL_ERROR_NONE if reception started successfully\n  \t   RAL_ERROR_INVALID_PARAMETERS if the passed parameters doesn't make sense e.g. if the reception channel out of MAC band\n \t\t   RAL_ERROR_BUSY if there is a transmission event that already started and not ended yet\n\n @note: ral_rx_done won't be called unless emngr_handle_all_events() is called to call ral_sm_done which will call the ral_rx_done call back"]
    pub fn ral_start_rx(
        ral_instance: ral_instance_t,
        rx_channel: u8,
        ptr_start_time: *mut ral_time_st,
        timeout: u32,
        periodic_interval: u32,
        ptr_coex_info: *mut ral_coex_info_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tAbort current reception\n \t\t\tThis function is used for tseting purpose\n\n @param[in] ral_instance : RAL instance\n\n @retval RAL_ERROR_NONE if reception aborted successfully"]
    pub fn ral_abort_rx(ral_instance: ral_instance_t) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @}\n/\n/**  @ingroup ral_intf_cmn\n  @{\n/\n/**\n @brief\tSet automatic continuous reception after each event state\n\n @param[in] ral_instance \t\t: RAL instance\n @param[in] cont_recp_state \t: Continuous reception state, Enable or Disable\n\n @retval RAL_ERROR_NONE if new state saved successfully"]
    pub fn ral_set_cont_recp_state(
        ral_instance: ral_instance_t,
        cont_recp_state: ral_state_enum_t,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tEnable/Disable automatic switching to sleep mode after finish each event.\n \t\t\tSet automatic sleep after each event state in case of continuous reception is disabled\n \t\t\tas if enabled RAL shall schedule reception event from ral_isr() using active timer.\n\n @param[in] ral_instance \t \t: RAL instance\n @param[in] auto_sleep_state \t: Automatic sleep state, Enable or Disable\n\n @retval RAL_ERROR_NONE if new state saved successfully"]
    pub fn ral_set_auto_sleep_state(
        ral_instance: ral_instance_t,
        auto_sleep_state: ral_state_enum_t,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet the state of automatic switching to sleep mode\n\n @param[in] ral_instance : RAL instance\n\n @retval Automatic sleep state, enabled or disabled"]
    pub fn ral_get_auto_sleep_state(ral_instance: ral_instance_t) -> ral_state_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tConfigure MAC filter in RTL while reception.\n \t\t\twhen filter is disabled means reception in promiscuous mode.\n\n @param[in] ral_instance \t   \t\t: RAL instance\n @param[in] ptr_mac_fltr_confg \t: Pointer to MAC filter configuration\n\n @retval RAL_ERROR_NONE if new configuration saved successfully"]
    pub fn ral_confg_mac_fltr(
        ral_instance: ral_instance_t,
        ptr_mac_fltr_confg: *mut ral_mac_fltr_confg_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tConfigure MAC filter in scan mode\n\n @param[in] ral_instance : RAL instance\n @param[in] Enable \t   : Set or clear scan mode\n\n @retval RAL_ERROR_NONE if new configuration saved successfully"]
    pub fn ral_set_scan_filter(ral_instance: ral_instance_t, Enable: u8) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tConfigure automatic ACK after packet transmission/reception\n\n @param[in] ral_instance \t   \t: RAL instance\n @param[in] ptr_auto_ack_confg: Pointer to new automatic ACK configuration\n\n @retval RAL_ERROR_NONE if new configuration saved successfully"]
    pub fn ral_confg_auto_ack(
        ral_instance: ral_instance_t,
        ptr_auto_ack_confg: *mut ral_auto_ack_confg_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tTemporary pause/resume automatic ACK state only after transmission/reception.\n\t\t\tThis function is used for testing and custom purposes.\n\n @param[in] ral_instance \t  \t: RAL instance\n @param[in] auto_tx_ack_state : New state of automatic ACK transmission\n @param[in] auto_rx_ack_state : New state of automatic ACK reception\n\n @retval RAL_ERROR_NONE if new configuration saved successfully"]
    pub fn ral_pause_auto_ack(
        ral_instance: ral_instance_t,
        auto_tx_ack_state: ral_state_enum_t,
        auto_rx_ack_state: ral_state_enum_t,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tSet enhanced ACK header IE data\n\n @param[in] ral_instance\t: RAL instance\n @param[in] ptr_hdr_ie \t: Pointer to header IE data to be included in enhanced ACK\n @param[in] hdr_ie_len \t: Header IE data length\n\n @retval RAL_ERROR_NONE if new configuration saved successfully"]
    pub fn ral_set_enh_ack_hdr_ie(
        ral_instance: ral_instance_t,
        ptr_hdr_ie: *mut u8,
        hdr_ie_len: u8,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = "  @ingroup ral_intf_cmn\n  @{\n/\n/**\n @brief\tEnable/Disable source address match feature.\n\t\t\tIf disabled, the RAL must set the \"frame pending\" on all ACKs to data request commands.\n \t\t\tIf enabled, the RAL uses the source address match table to determine whether to set or clear the\n \t\t\t\"frame pending\" bit in an ACK to a data request command.\n\n @param[in] ral_instance \t\t: RAL instance\n @param[in] src_match_state \t: Source address match state"]
    pub fn ral_set_src_match_state(ral_instance: ral_instance_t, src_match_state: ral_state_enum_t);
}
unsafe extern "C" {
    #[doc = " @brief\tAdd a short address to the source address match table\n\n @param[in] ral_instance  : RAL instance\n @param[in] short_address : Short address to be added\n\n @retval RAL_ERROR_NONE if added successfully"]
    pub fn ral_add_src_match_short(
        ral_instance: ral_instance_t,
        short_address: u16,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tAdd an extended address to the source address match table\n\n @param[in] ral_instance : RAL instance\n @param[in] ptr_ext_addr : Pointer to extended address to be added\n\n @retval RAL_ERROR_NONE if added successfully"]
    pub fn ral_add_src_match_ext(
        ral_instance: ral_instance_t,
        ptr_ext_addr: *const u8,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tRemove a short address from the source address match table\n\n @param[in] ral_instance  : RAL instance\n @param[in] short_address : Short address to be removed\n\n @retval RAL_ERROR_NONE if removed successfully"]
    pub fn ral_clr_src_match_short(
        ral_instance: ral_instance_t,
        short_address: u16,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tRemove an extended address from the source address match table\n\n @param[in] ral_instance : RAL instance\n @param[in] ptr_ext_addr : Pointer to extended address to be removed\n\n @retval RAL_ERROR_NONE if removed successfully"]
    pub fn ral_clr_src_match_ext(
        ral_instance: ral_instance_t,
        ptr_ext_addr: *const u8,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tRemove all short addresses from the source address match table\n\n @param[in] ral_instance : RAL instance"]
    pub fn ral_clr_all_src_match_short(ral_instance: ral_instance_t);
}
unsafe extern "C" {
    #[doc = " @brief\tRemove all extended addresses from the source address match table\n\n @param[in] ral_instance : RAL instance"]
    pub fn ral_clr_all_src_match_ext(ral_instance: ral_instance_t);
}
unsafe extern "C" {
    #[doc = "  @ingroup ral_intf_ed\n  @{\n/\n/**\n @brief\tED reset\n"]
    pub fn ral_reset_ed();
}
unsafe extern "C" {
    #[doc = " @brief\tStart energy detection scan on specific channel,\n \t\t\tral_ed_scan_done callback is called to notify upper layer that the energy scan is complete.\n\n @param[in] ral_instance \t: RAL instance\n @param[in] scan_channel \t: The channel to perform the energy scan on\n @param[in] scan_duration : Scan duration in microseconds\n @param[in] ptr_coex_info : Pointer to the current coexistence parameters\n\n @retval RAL_ERROR_NONE if energy detection scanning started successfully\n\n @note: ral_ed_scan_done won't be called unless emngr_handle_all_events() is called to call ral_sm_done_cbk which will call ral_ed_scan_done"]
    pub fn ral_ed_scan(
        ral_instance: ral_instance_t,
        scan_channel: u8,
        scan_duration: u32,
        ptr_coex_info: *mut ral_coex_info_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tCheck whether the RAL is about to transmit ACK or not.\n\n @retval TRUE if ACK is required and about to be transmitted, FALSE otherwise"]
    pub fn ral_is_about_to_transmit_ack() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief\tStart the triggering of pre TX sequence from sequence ram as early as possible to save time in case of transmitting ACK\n"]
    pub fn ral_early_perpare_phy_to_tx_ack();
}
unsafe extern "C" {
    #[doc = " @brief\tPHY driver ISR handler in case of MAC event.\n\t\t\tIt is used to restore the phy state after triggering of the pre interpacket sequence from sequence ram.\n"]
    pub fn ral_handle_phy_driver_isr();
}
unsafe extern "C" {
    #[doc = " @brief  Update frame counter sustained in RAL instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] instance    \t: RAL instance\n @param[in] mac_frm_cntr\t: Frame counter passed by upper layers"]
    pub fn ral_update_mac_frm_cntr(instance: ral_instance_t, mac_frm_cntr: u32);
}
unsafe extern "C" {
    #[doc = " @brief  Update frame counter sustained in RAL instance only if the new value larger than the old one.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] instance    \t: RAL instance\n @param[in] mac_frm_cntr\t: Frame counter passed by upper layers"]
    pub fn ral_update_larger_mac_frm_cntr(instance: ral_instance_t, mac_frm_cntr: u32);
}
unsafe extern "C" {
    #[doc = " @brief  Update keys and keyId sustained in RAL instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in]  instance : RAL instance\n @param[in]  aKeyId\t: Key index is used for comparison in case of keyId mode '1'\n \t\t\t\t\t\t  To select between different keys.\n \t\t\t\t\t\t  ------------\n \t\t\t\t\t\t  for example:\n \t\t\t\t\t\t  ------------\n \t\t\t\t\t\t  if keyId (key index) equivalent to the received in MHR\n \t\t\t\t\t\t  Then the used key is the current key.\n\n \t\t\t\t\t\t  But if it isn't equivalent due to any connected neighbor's key index\n \t\t\t\t\t\t  mismatch the key generator in upper layers shall be notified to take action\n \t\t\t\t\t\t  based on the relation between the different key index\n\n \t\t\t\t\t\t  ------------\n \t\t\t\t\t\t  for example:\n \t\t\t\t\t\t  -----------\n \t\t\t\t\t\t  if keyId (key index) less than the received in MHR. this means that the\n \t\t\t\t\t\t  communicating device has already generate a new key and started to rotate keys.\n \t\t\t\t\t\t  so the next key is the key that shall be used in this situation.\n\n \t\t\t\t\t\t  This kind of process synchronization beside another timing triggered events for\n \t\t\t\t\t\t  new key generation and rotation allows a difference of only one between keyIds.\n\n \t\t\t\t\t\t  see thread specification under security section subsection of \"Key Rotation\"\n\n @param[in] aPrevKey\t: Key used in case of keyid of received MHR less than the keyid sustained by ral_instance by one.\n @param[in] aCurrKey\t: Key used in case of keyid of received MHR equivalent to the keyid sustained by ral_instance.\n @param[in] aNextKey\t: Key used in case of keyid of received MHR greater than the keyid sustained by ral_instance by one."]
    pub fn ral_update_mac_keys(
        instance: ral_instance_t,
        aKeyId: u8,
        aPrevKey: *const u8,
        aCurrKey: *const u8,
        aNextKey: *const u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief  Get the current key of an instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] ral_instance : RAL instance\n\n @retval Pointer to the current key"]
    pub fn ral_get_inst_curr_key(ral_instance: ral_instance_t) -> *const otMacKeyMaterial;
}
unsafe extern "C" {
    #[doc = " @brief  Get the previous key of an instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] ral_instance : RAL instance\n\n @retval Pointer to the previous key sustained by RAL layer"]
    pub fn ral_get_inst_prev_key(ral_instance: ral_instance_t) -> *mut u8;
}
unsafe extern "C" {
    #[doc = " @brief  Get the frame counter of an instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] ral_instance : RAL instance\n\n @retval Frame counter value sustained by RAL layer"]
    pub fn ral_get_inst_frm_cntr(ral_instance: ral_instance_t) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  Get the key index of an instance.\n \t\t   This function is called only in case of radio support OT_RADIO_CAPS_TRANSMIT_SEC.\n\n @param[in] ral_instance : RAL instance\n\n @retval Key index sustained by RAL layer"]
    pub fn ral_get_inst_keyId(ral_instance: ral_instance_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Configure [remove/add/modify] initiator device\n\n @param[in] short_addr    : The address info for initiator\n @param[in] ptr_ext_addr  : Pointer to the extended address of the initiator node\n @param[in] link_metrics  : Pointer to the initiator link metrics\n\n @retval Status\n \t\t   RAL_ERROR_NONE\t\t\t\t\t\t: successfully configured.\n \t\t   RAL_ERROR_LINK_METRICS_INVALID_ARGS\t: in case of ptr_ext_addr NULL.\n \t\t   RAL_ERROR_LINK_METRICS_NOT_FOUND\t\t: in case of remove non-existing node.\n \t\t   RAL_ERROR_LINK_METRICS_NO_BUF\t\t: in case of not enough supported nodes."]
    pub fn ral_config_enh_ack_probing(
        short_addr: u16,
        ptr_ext_addr: *const u8,
        link_metrics: *mut ::core::ffi::c_void,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief Set the link metrics noise floor value needed to calculate the link margin\n\n @param[in] noise_floor : The noise floor used by link metrics. It should be set to the platform's\n                          noise floor (measured noise floor, receiver sensitivity or a constant)."]
    pub fn ral_link_metrics_set_noise_floor(noise_floor: i8);
}
unsafe extern "C" {
    #[doc = " @brief Set the value of the openthread base time.\n \t\t  This value will be subtracted from all timing values sent/received to openthread.\n\n @param[in] time : Base time value"]
    pub fn ral_set_ot_base_slp_time_value(time: u32);
}
unsafe extern "C" {
    #[doc = " @brief Get the value of the openthread base time.\n \t\t  The value may be set through @ref ral_set_ot_base_slp_time_value\n\n @return OT base time"]
    pub fn ral_get_ot_base_slp_time_value() -> u64;
}
unsafe extern "C" {
    #[doc = " @brief  Convert the value of sleep timer to openthread time.\n \t\t   If openthread is not integrated, @ref ral_ot_base_slp_time is set to zero, so no conversion will take place.\n\n @param[in] time \t\t\t\t: Sleep timer value to be converted to openthread time\n @param[in] num_of_overflow \t: Number of overflow cycles to be added\n @retval The converted time value"]
    pub fn ral_cnvert_slp_tim_to_ot_tim(time: u32, num_of_overflow: u16) -> u64;
}
unsafe extern "C" {
    #[doc = " @brief  Convert the value of openthread time to sleep timer value.\n \t\t   If openthread is not integrated, @ref ral_ot_base_slp_time is set to zero, so no conversion will take place.\n\n @param[in] time : Openthread time value to be converted to sleep timer\n @retval The converted time value"]
    pub fn ral_cnvert_ot_tim_to_slp_tim(time: u64) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  Convert the microsecond time to the proper value the should be set for the sleep timer to start the event\n\n @param[in]  curr_time \t: Current sleep timer value to be converted to openthread time\n @param[in]  base_tim  \t: Base time in microseconds to be converted\n @param[in]  delay_time \t: Microsecond delay from the base time\n @retval The converted sleep timer set point to be used in setting active timer"]
    pub fn ral_cnvrt_req_time_to_set_point(curr_time: u32, base_tim: u32, delay_time: u32) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief\tSet CSL receiver parameters to enable/disable CSL.\n\n @param[in] ral_instance     : RAL instance\n @param[in] cslPeriod        : CSL period to be included in CSL header IE, or 0 to disable CSL\n @param[in] csl_short_addr   : The short source address of CSL receiver's peer.\n @param[in] ptr_csl_ext_addr : Pointer to the parent extended address"]
    pub fn ral_set_csl_rcv_param(
        ral_instance: ral_instance_t,
        cslPeriod: u32,
        csl_short_addr: u16,
        ptr_csl_ext_addr: *mut u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief\tSet CSL receiver next sample time to be used in calculating phase. the sample time points to the time of he next sample window\n\n @param[in] ral_instance  : RAL instance\n @param[in] cslSampleTime\t: The LSB part of sample time in us"]
    pub fn ral_set_csl_sample_time(ral_instance: ral_instance_t, cslSampleTime: u32);
}
unsafe extern "C" {
    #[doc = " @brief Check whether the RAL is receiving within the CSL sample window\n\n @return TRUE 1 if the receiver is in CSL window, FALSE otherwise."]
    pub fn ral_is_rcv_in_csl_smple_wndw() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief\tCheck and add CSL header IE to any outgoing frame if CSL receiver is enabled.\n \t\t\tFor any outgoing frames except the enhanced ACK frame, it assumes that the CSL header IE is stubbed in frame.\n\n @param[in] ral_instance : RAL instance\n @param[in] frame_ptr    : Pointer the frame to be transmitted, either enhanced ACK or any other frame.\n @param[in,out] ie_index : Index of the first byte of CSL header IE. if the CSL header IE will be included , It will be incremented with the length of CSL header IE\n @param[in] data_ptr     : Data pointer to the parent address in case of enhanced ACK, and frame total length for TX frames\n @param[in] enh_ack_flag : Flag to indicate whether enhanced ACK or new TX Frame"]
    pub fn ral_hndl_csl_hdr_ie(
        ral_instance: ral_instance_t,
        frame_ptr: *mut u8,
        ie_index: *mut u8,
        data_ptr: *mut u8,
        enh_ack_flag: u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief\tSet the CCA Energy Detection threshold in PHY\n\n @param[in] threshold : CCA Energy Detection threshold value to be set\n @retval OT_ERROR_NONE if the the threshold is successfully set, OT_ERROR_FAILED if the given value is out of range."]
    pub fn ral_set_cca_ed_threshold(threshold: i8) -> otError;
}
unsafe extern "C" {
    #[doc = " @brief\tGet the CCA Energy Detection threshold in PHY\n\n @retval CCA Energy Detection threshold value"]
    pub fn ral_get_cca_ed_threshold() -> i8;
}
unsafe extern "C" {
    #[doc = " @brief\tRadio CSMA error callback.\n\n @param[in] error : Status error returned from this transmission"]
    pub fn radio_coex_tx_error_cbk(error: u32);
}
unsafe extern "C" {
    #[doc = " @brief\tRadio RX  error callback (used in CSL receive AT case).\n\n @param[in] error : Status error returned from this reception"]
    pub fn radio_coex_rx_error_cbk(error: u32);
}
unsafe extern "C" {
    #[doc = " @brief\tHandle frame pending bit in ACK of all packets (set to true) or for ACK of data request command only (set to false)\n\n @param[in] hndle_frm_pending_bit_for_acks : TRUE means handle pending frame bit in ACK for all frame types, FALSE means handle frame pending bit in ACK for data request command only"]
    pub fn ral_set_frm_pend_bit_for_acks(hndle_frm_pending_bit_for_acks: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Set CSMA enable flag\n\n @param[in] csma_en : Value for CSMA enable flag to be set"]
    pub fn radio_set_csma_en(csma_en: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tGet CSMA enable flag\n\n @retval CSMA enable flag"]
    pub fn radio_get_csma_en() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief   Set CCA enable flag\n\n @param[in] cca_en : Value for CCA enable flag to be set"]
    pub fn radio_set_cca_en(cca_en: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Get CCA enable flag\n\n @retval CCA enable flag ."]
    pub fn radio_get_cca_en() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief   Set pending TX retry flags\n\n @param[in] evnt_type     : Type of new retry (CONTINUE_CSMA_RETRY, START_NEW_FULL_TX_RETRY)\n @param[in] radio_error\t: Error returned from previous TX trial\n @param[in] is_tx_blocked\t: Flag to indicate that TX event is blocked"]
    pub fn radio_set_tx_retry_pending(
        evnt_type: tx_new_retry_enum_t,
        radio_error: otError,
        is_tx_blocked: u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief   Handle pending TX retry event\n"]
    pub fn radio_handle_pnding_tx_retry_event();
}
unsafe extern "C" {
    #[doc = " @brief   Check if there's pending TX retry waiting to be executed\n\n @retval TRUE if there is a pending TX retry, FALSE otherwise."]
    pub fn radio_is_tx_retry_event_pending() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief   Set MAC implicit broadcast PIB from MAC layer to be used in filtration\n\n @param[in] ImplicitBroadcast: Value for MAC implicit broadcast PIB to be set"]
    pub fn radio_set_implicitbroadcast(ImplicitBroadcast: u8);
}
unsafe extern "C" {
    #[doc = " @brief   Set MAC implicit broadcast PIB from radio layer to be used in filtration\n\n @param[in] ral_instance\t\t: RAL instance\n @param[in] ImplicitBroadcast\t: Value for MAC implicit broadcast PIB to be set"]
    pub fn ral_set_implicitbroadcast(ral_instance: ral_instance_t, ImplicitBroadcast: u8);
}
unsafe extern "C" {
    #[doc = " @brief Update polling time value when sleep clock source is changed\n"]
    pub fn ral_update_polling_time_on_slp_src_change();
}
unsafe extern "C" {
    #[doc = " @brief\tEnergy detection timer event handle\n\n @param[in] ptr_info : Pointer to current RAL context"]
    pub fn ed_timer_hndl(ptr_info: *mut ::core::ffi::c_void);
}
unsafe extern "C" {
    #[doc = " @brief\tSet configurable library parameters\n\n @param[in] ptr_config_lib_params : Pointer to configurable library parameters\n\n @retval RAL_ERROR_NONE if configurable library parameters are set correctly"]
    pub fn ral_set_config_lib_params(ptr_config_lib_params: *mut config_lib_st)
        -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief\tGet configurable library parameters\n\n @param[out] ptr_config_lib_params : Pointer to configurable library parameters"]
    pub fn ral_get_config_lib_params(ptr_config_lib_params: *mut config_lib_st);
}
unsafe extern "C" {
    #[doc = " @brief\tSet RTL polling time\n\n @param[in] rtl_polling_time : RTL polling time"]
    pub fn ral_set_rtl_polling_time(rtl_polling_time: u8);
}
unsafe extern "C" {
    #[doc = " @brief\tGet current RTL polling time\n\n @retval Current RTL polling time"]
    pub fn ral_get_rtl_polling_time() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief   Get parameters used in augmented MAC (IFS, phy_rate, auto_ACK_config)\n\n @param[in] ral_instance \t: RAL instance\n @param[out] a_mac_params : Current augmented MAC parameters\n @retval Status"]
    pub fn ral_get_a_mac_params(
        ral_instance: ral_instance_t,
        a_mac_params: *mut ral_a_mac_params_st,
    ) -> ral_error_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief   Set value for drop on error flag\n\n @param[in] ral_instance  : RAL instance\n @param[in] drop_on_error : Value for drop on error flag"]
    pub fn ral_set_drop_on_error(ral_instance: ral_instance_t, drop_on_error: u8);
}
pub const z_dtm_mode_Z_DTM_STOPPED: z_dtm_mode = 0;
pub const z_dtm_mode_Z_DTM_TX: z_dtm_mode = 1;
pub const z_dtm_mode_Z_DTM_RX: z_dtm_mode = 2;
pub const z_dtm_mode_Z_DTM_TX_ACK: z_dtm_mode = 3;
pub const z_dtm_mode_Z_DTM_CHECK_TX_ERROR: z_dtm_mode = 4;
#[doc = " @brief  Enum represents the current DTM State.\n"]
pub type z_dtm_mode = ::core::ffi::c_uint;
#[doc = " @brief  Enum represents the current DTM State.\n"]
pub use self::z_dtm_mode as z_dtm_mode_e;
#[doc = " @brief  Structure represents the current state of DTM TX.\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Tx_Info {
    #[doc = "< Total number of transmitted frames (with/without errors)"]
    pub TxTotal: u16,
    #[doc = "< Number of frames transmitted successfully"]
    pub NoErr: u16,
    #[doc = "< Number of frames requesting ACK and no ACK received after transmission"]
    pub ErrNoAck: u16,
    #[doc = "< Number of frames transmitted with errors excluding no ACK error"]
    pub ErrOther: u16,
}
#[doc = " @brief  Structure represents the current state of DTM TX.\n"]
pub type TxInfo_s = Tx_Info;
#[doc = " @brief Structure represents the current state of DTM RX.\n"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Rx_Info {
    #[doc = "< Total number of received frames (with/without errors)"]
    pub RxTotal: u16,
    #[doc = "< Number of frames successfully received"]
    pub NoErr: u16,
    #[doc = "< Number of frames received with invalid FCS"]
    pub ErrFcs: u16,
    #[doc = "< Number of frames received with errors excluding invalid FCS error"]
    pub ErrOther: u16,
    #[doc = "< LQI of the received frame"]
    pub lqi: u8,
}
#[doc = " @brief Structure represents the current state of DTM RX.\n"]
pub type RxInfo_s = Rx_Info;
#[doc = " @brief Structure represents all information related to DTM.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtmInfo_st {
    #[doc = "< Current State (TX, RX, Stopped)"]
    pub curr_mode: z_dtm_mode_e,
    pub tx_rx_info_u: dtmInfo_st__bindgen_ty_1,
}
#[doc = " @brief Union represents DTM TX/RX information.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union dtmInfo_st__bindgen_ty_1 {
    #[doc = "< DTM information in case of TX and TX_ACK"]
    pub TxInfo: TxInfo_s,
    #[doc = "< DTM information in case of RX"]
    pub RxInfo: RxInfo_s,
}
impl Default for dtmInfo_st__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for dtmInfo_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure represents all information related to DTM.\n"]
pub type dtmInfo_t = dtmInfo_st;
unsafe extern "C" {
    #[doc = " @brief  Initialize or reset DTM, or stop the current running DTM.\n"]
    pub fn dtmReset();
}
unsafe extern "C" {
    #[doc = " @brief  Get the state of DTM (running or stopped).\n\n @return TRUE if DTM is running, FALSE if DTM is stopped."]
    pub fn dtmIsEnabled() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Start continuous DTM transmission with the given input parameters until the dtmStop() API is called.\n\n @param[in]  mPsdu     : Pointer to MAC DTM frame including MAC header.\n @param[in]  mLength   : Length of the frame to be transmitted.\n @param[in]  mChannel  : Channel to transmit the frame on.\n @param[in]  IFS       : Interframe spacing in us.\n @return MAC error state of starting transmission."]
    pub fn dtmStartTransmit(
        mPsdu: *mut u8,
        mLength: u8,
        mChannel: u8,
        IFS: u16,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Start transmission of a single DTM frame that is requesting ACK with the given input parameters.\n\n @param[in]  mPsdu     : Pointer to MAC DTM frame including MAC header.\n @param[in]  mLength   : Length of the frame to be transmitted.\n @param[in]  mChannel  : Channel to transmit the frame on.\n @param[in]  IFS       : Interframe spacing in us.\n @return MAC error state of starting transmission."]
    pub fn dtmStartTransmitwithAck(
        mPsdu: *mut u8,
        mLength: u8,
        mChannel: u8,
        IFS: u16,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Stop the current running DTM (TX or RX).\n\n @param[out] num_rec_pckts : Pointer to number of received packets.\n @param[out] lqi           : Pointer to average LQI.\n @return MAC error state of stopping DTM."]
    pub fn dtmStop(num_rec_pckts: *mut u16, lqi: *mut u8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Start continuous DTM reception on the given channel.\n\n @param[in]  aChannel : Channel to receive on.\n @return MAC error state of starting reception."]
    pub fn dtmStartReceive(aChannel: u8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Start CCA on the given channel.\n\n @param[in]  channel : Channel to perform CCA on.\n @return MAC error state of performing CCA."]
    pub fn dtmPerformCCA(channel: u8) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Get the current LQI of the last received packet.\n\n @return LQI value."]
    pub fn dtmGetLQIValue() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  get the current RSSI of the last received packet\n @retval int8_t. RSSI value"]
    pub fn dtmGetRSSIValue() -> i8;
}
unsafe extern "C" {
    #[doc = " @}\n/\n/**\n @brief  Check if there are more DTM frames to transmit.\n\n @return TRUE to continue DTM transmission, FALSE to stop DTM transmission."]
    pub fn dtmCheckMoreTx() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Check if DTM is in reception state.\n\n @return TRUE if DTM is in reception mode, FALSE otherwise."]
    pub fn dtmCheckRxState() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Done function called after each DTM event.\n\n @param[in] aFrame : Pointer to MAC transmitted/received frame.\n @param[in] aError : Status error of overall transmission (success, No_ack, and other errors)."]
    pub fn dtmRadioDone(aFrame: *mut otRadioFrame, aError: otError);
}
unsafe extern "C" {
    #[doc = " @brief  Check if in Z_DTM_CHECK_TX_ERROR transmission mode or not.\n\n @return TRUE if in Z_DTM_CHECK_TX_ERROR transmission mode, FALSE otherwise."]
    pub fn dtmCheckTxErrorState() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Count errors that happen in single TX in Z_DTM_CHECK_TX_ERROR transmission mode.\n\n @param[in] tx_error : Type of error to be counted for this trial."]
    pub fn dtmCheckTxErrorCount(tx_error: otError);
}
unsafe extern "C" {
    #[doc = " @brief  Gives stored errors and their count that happen in single TX in Z_DTM_CHECK_TX_ERROR transmission mode.\n\n @param[out] errors      : Pointer to array of stored errors in Z_DTM_CHECK_TX_ERROR transmission mode.\n @param[out] error_count : Pointer to number of errors stored in errors array."]
    pub fn dtmTxCheckErrorCountDone(errors: *mut *mut u8, error_count: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief  Start transmission in Z_DTM_CHECK_TX_ERROR transmission mode.\n\n @param[in]  mac_hndl           : MAC instance handle.\n @param[in]  mPsdu              : Pointer to TX packet.\n @param[in]  mLength            : Length of TX packet.\n @param[in]  mChannel           : Channel that packet will be sent on.\n @param[in]  IFS                : Used inter-frame-spacing.\n @param[in]  csma_en            : Flag to indicate to apply CSMA in this transmission if max_csma_retry_count != 0.\n @param[in]  backoff_count_max  : Maximum number of TX trials in every full CSMA trial.\n @param[in]  frame_retry        : Maximum number of trials after failure not related to CCA failure.\n @param[in]  ack_req            : Indicate if this packet requires ACK or not.\n @return Status."]
    pub fn dtmTxSpecificErrorCount(
        mac_hndl: u32,
        mPsdu: *mut u8,
        mLength: u8,
        mChannel: u8,
        IFS: u16,
        csma_en: u8,
        backoff_count_max: u8,
        frame_retry: u8,
        ack_req: u8,
    ) -> mac_status_enum_t;
}
unsafe extern "C" {
    #[doc = " @brief  Reset tx_check_err_arr array and its counter for Z_DTM_CHECK_TX_ERROR mode.\n"]
    pub fn dtmTxErrorCountReset();
}
pub type conditional_cbk = ::core::option::Option<
    unsafe extern "C" fn(
        em_data: *mut ::core::ffi::c_void,
        caller_data: *mut ::core::ffi::c_void,
    ) -> u8,
>;
pub const handler_t_LLHWC_EVENT: handler_t = 0;
pub const handler_t_PRDC_CLBR_EVENT: handler_t = 1;
pub const handler_t_HCI_HANDLER: handler_t = 2;
pub const handler_t_RADIO_MAC_PENDING_DONE_EVENT: handler_t = 3;
pub const handler_t_RAL_SM_DONE_EVENT: handler_t = 4;
pub const handler_t_MAC_SM_DONE_EVENT: handler_t = 5;
pub const handler_t_ED_TMR_EVENT: handler_t = 6;
pub const handler_t_HCI_TRANSPORT_HANDLER: handler_t = 7;
pub const handler_t_GENERIC_EVENT: handler_t = 8;
pub const handler_t_HCI_RADIO_ACTIVITY_EVENT: handler_t = 9;
pub const handler_t_MLME_TIMER_EVENT: handler_t = 10;
pub const handler_t_DIRECT_DATA_TX_EVENT: handler_t = 11;
pub const handler_t_INDIRECT_DATA_TIMEOUT_EVENT: handler_t = 12;
pub const handler_t_MAX_EM_HANDLE: handler_t = 13;
#[doc = " @brief Enum event manager handler types."]
pub type handler_t = ::core::ffi::c_uint;
unsafe extern "C" {
    #[doc = " @brief  Used to initialize the event manager component.\n\n @param  None.\n\n @retval None."]
    pub fn emngr_init();
}
unsafe extern "C" {
    #[doc = " @brief  Used to reset the event manager structure.\n\n @param  None.\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_reset() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @ingroup event_manager_functions\n @{\n/\n/**\n @brief  Used to initialize a certain handle in the event manager.\n\n @param  id  : [in] ID of the handle to be initialized. [Range: 0 to 255].\n @param  max : [in] Maximum number of events that can be added to this handle. [Range: 0 to 255].\n @param  call_back_fun : [in] Pointer to the function that will process the events of that handle.\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_handle_init(
        id: ::core::ffi::c_uchar,
        max: ::core::ffi::c_uchar,
        call_back_fun: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to remove a certain handle from the event manager.\n\n @param  id : [in] ID of the handle to be removed. [Range: 0 to 255].\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_handle_remove(id: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to post an event to a certain event manager handle that is identified by the handle ID.\n\n @param  id \t\t: [in] ID of the handle to which the event will be posted. [Range: 0 to 255].\n @param  event \t: [in] Void pointer pointing to the data related to the posted event.\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_post_event(
        id: ::core::ffi::c_uchar,
        event: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to post an event to a certain event manager handle that is identified by the handle ID.\n \t\t   This Event will be handled first\n\n @param  id \t\t: [in] ID of the handle to which the event will be posted. [Range: 0 to 255].\n @param  event \t: [in] Void pointer pointing to the data related to the posted event.\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_post_event_first(
        id: ::core::ffi::c_uchar,
        event: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to get an event from a certain event manager handle that is identified by the handle ID.\n\n @param  id : [in] ID of the handle whose events are to be gotten. [Range: 0 to 255].\n\n @retval event handle : Void pointer pointing to the returned event."]
    pub fn emngr_get_event(id: ::core::ffi::c_uchar) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  Used to process an event from a a certain event manager handle that is identified by the handle ID, by first getting the event (in case the handle contains any) then processing it through the associated callback function.\n\n @param  id : [in] ID of the handle whose events are to be processed. [Range: 0 to 255].\n\n @retval Status : 0: SUCCESS. Otherwise: FAILED."]
    pub fn emngr_process_event(id: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to return the number of events in the event manager.\n\n @param  None.\n\n @retval Events Number."]
    pub fn emngr_get_total_events() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to return the number of events in a certain handle that is identified by the handle ID.\n\n @param  id : [in] ID of the handle whose events number is to be returned. [Range: 0 to 255].\n\n @retval Events Number per handle."]
    pub fn emngr_get_handle_events(id: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to set a flag \"busy_flag\" to determine whether a certain handle is busy processing an event.\n\n @param  id \t\t\t: [in] ID of the handle whose \"busy_flag\" parameter value is to be set. [Range: 0 to 255].\n @param  busy_flag  \t: [in] Value to be set to the \"busy_flag\" parameter. 0: EVENT_NOT_BUSY. 1: EVENT_BUSY.\n\n @retval None."]
    pub fn emngr_set_event_handle_busy(id: ::core::ffi::c_uchar, busy_flag: ::core::ffi::c_uchar);
}
unsafe extern "C" {
    #[doc = " @brief  Used to return the state of the event \"busy_flag\", which indicates whether the events of a certain handle, identified by the handle ID, are being currently processed.\n\n @param  id : [in] ID of the handle whose events are being currently processed. [Range: 0 to 255].\n\n @retval busy_flag state : 0: EVENT_NOT_BUSY. 1: EVENT_BUSY."]
    pub fn emngr_is_event_busy(id: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief  Used to loop through all the registered handles in the event manager and process their events, if any exists.\n\n @param None.\n\n @retval None."]
    pub fn emngr_handle_all_events();
}
unsafe extern "C" {
    #[doc = " @brief  Used to process an event of a certain event manager handle that is identified by the handle ID.\n\n @param  id : [in] ID of the handle whose event is to be processed. [Range: 0 to 255].\n\n @retval None."]
    pub fn emngr_handle_event(id: handler_t);
}
unsafe extern "C" {
    #[doc = " @brief  Used to return a pointer to the first event in a certain event manager handle, identified by the handle ID, without dequeuing that event.\n\n @param  id : [in] ID of the handle whose first event is to be returned without being removed from the handle queue. [Range: 0 to 255].\n\n @retval : void Pointer to the first event of the specified handle."]
    pub fn emngr_peak_frst_event(id: ::core::ffi::c_uchar) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief  Used to delete an event from  a certain event manager handle, identified by the handle ID.\n\n @param  id \t: [in] ID of the handle whose one of its events is to be deleted. [Range: 0 to 255].\n @param  data : [in] Pointer to the event data to be used for searching for the event to be deleted out of all the events in the specified handle.\n\n @retval handle : Void pointer to the event data of the event node that is being deleted."]
    pub fn emngr_del_event(
        id: ::core::ffi::c_uchar,
        data: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn emngr_remove_conditional_event(
        id: u8,
        only_one_event: u8,
        conditional_data: *mut ::core::ffi::c_void,
        cbk: conditional_cbk,
    ) -> u8;
}
unsafe extern "C" {
    pub fn emngr_can_mcu_sleep() -> u8;
}
unsafe extern "C" {
    pub fn emngr_process_conditional_event(
        id: u8,
        only_one_event: u8,
        conditional_data: *mut ::core::ffi::c_void,
        cbk: conditional_cbk,
    ) -> u8;
}
pub type ext_evnt_hndl_t = *mut ::core::ffi::c_void;
#[doc = " @brief Generic External Event node that holds the the external event information.\n @note All parameters that are related to time assumed to be in microseconds"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _extrnl_evnt_st_t {
    #[doc = " Event must end before this point in time (us)  it is  an optional parameter and\n it should set with zero if not used by stack."]
    pub deadline: u64,
    #[doc = " Earliest time the event can start (us) , it is  an optional parameter and\n it should set with zero if not used by stack.it is not logical to have periodic event with max start time.\n  So, it will be neglected and never checked in case of periodic event."]
    pub strt_min: u64,
    #[doc = " Latest time the event can start (us), it is  an optional parameter and\n it should set with zero if not used by stack."]
    pub strt_max: u64,
    #[doc = " Minimum amount of time that must be allocated to the event (us) ,"]
    pub durn_min: u32,
    #[doc = " Maximum amount of time that the event is requesting (us), it is  an optional parameter and\n it should set with zero if not used by stack."]
    pub durn_max: u32,
    #[doc = " Periodicity of the event (us), 0: Not Periodic\n if the value is not zero , it should be multiple of 1250 us to be aligned with BLE events."]
    pub prdc_intrvl: u32,
    #[doc = " Priority of the event  from @ref _extrnl_evnt_priority_e"]
    pub priority: extrnl_evnt_priority_e,
    #[doc = " Event blocked or not, and reason if blocked from @ref _extrnl_evnt_state_e"]
    pub blocked: extrnl_evnt_state_e,
    #[doc = " Pointer to private data that should passed with the call back function"]
    pub ptr_priv: *mut ::core::ffi::c_void,
    #[doc = " Event Started Callback Function , this function will be called at the allocated time to do the job related to\n  executing the event, it must not be NULL.\n  At the end of given grant, @ref evnt_schdlr_gnrc_evnt_cmplt function should be called to inform scheduler\n  that the event is finished, and PHY should not be accessed after that call until a new grant is given\n   from event scheduler. @ref evnt_strtd_cbk should handle calling of @ref  evnt_schdlr_gnrc_evnt_cmplt  via timer or\n   at the end of execution if the execution is blocking\n\n   if @ref slot_durn is set to zero \"0\" , event scheduler has given the on idle event unlimited grant.\n    It will be aborted later with a calling to evnt_abortd_cbk function."]
    pub evnt_strtd_cbk: ::core::option::Option<
        unsafe extern "C" fn(
            evnt_hndl: ext_evnt_hndl_t,
            slot_durn: u32,
            priv_data_ptr: *mut ::core::ffi::c_void,
        ) -> u32,
    >,
    #[doc = " Event Blocked Callback Function , called if the event is blocked due deadline or other reasons as\n  in @ref blocked_state .it could be NULL if not used by stack\n  if it is not NULL ,it will be called when event exceeded the given maximum start time or the given deadline while scheduling the event"]
    pub evnt_blckd_cbk:
        ::core::option::Option<unsafe extern "C" fn(blocked_state: extrnl_evnt_state_e) -> u32>,
    #[doc = " Event Aborted Callback Function. it could be NULL if not used by stack.  it will be called when event execution is aborted.\n the event @ref EXTRNL_ON_IDLE is the only event type that can be aborted so it must not be NULL for this event type\n when it is  called from event scheduler, the stack should stop all running operation that access phy,\n  no need to call @ref evnt_schdlr_gnrc_evnt_cmpltafter calling this callback as it is called from scheduler itself."]
    pub evnt_abortd_cbk: ::core::option::Option<unsafe extern "C" fn() -> u32>,
    #[doc = " Event coexistence error Callback Function. it will be called when @ref EXTRNL_GNRC event execution returned error.\n  when  @ref evnt_strtd_cbk of @ref EXTRNL_GNRC failed at execution for any reason, this callback will be  called from event scheduler,\n  it'll send the returned error to ral_tx_done to check if there will retransmission of failed packet or send the error to upper layers,"]
    pub coex_error_cbk: ::core::option::Option<unsafe extern "C" fn(error: u32)>,
}
impl Default for _extrnl_evnt_st_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Generic External Event node that holds the the external event information.\n @note All parameters that are related to time assumed to be in microseconds"]
pub type extrnl_evnt_st_t = _extrnl_evnt_st_t;
unsafe extern "C" {
    #[doc = " @brief  request duration extension of previous registered external generic event.\n\n @param  evnt_hndl_t [in]\t\t\t\t: Event handle of registered event.\n @param  updated_priority [in]\t\t: Updated priority of the event during new extend request.\n @param  flxbl_req [in]\t\t\t\t: Flexible request flag,\n \t\t\t\t\t\t\t\t\t\t  True: scheduler will try to extend duration with available duration up to requested deadline\n \t\t\t\t\t\t\t\t\t\t  False: scheduler will try to extend duration with requested duration only (full requested duration or not)\n @param  reqstd_deadline [in and out]\t: Pointer to new end time that the event request to extend the grant to it.\n It is also will be used as an output to report the given grant end time if the request is successfully accepted..\n @retval Status (0: SUCCESS, 0xXX: ERROR_CODE from @ref ble_stat_t)."]
    pub fn evnt_schdlr_extend_gnrc_evnt_durn(
        evnt_hndl: ext_evnt_hndl_t,
        updated_priority: extrnl_evnt_priority_e,
        flxbl_req: u8,
        reqstd_deadline: *mut u64,
    ) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  register external generic event.\n This function is used to register external generic event in event scheduler.all required information to schedule this event are contained in @ref p_extrnl_evnt_st parameter\n @param  p_extrnl_evnt_st [in]\t   : Pointer to external event structure.\n @retval evnt_hndl_t                 : The event handle of registered event.\n @retval NULL                        : means that the event is not registered."]
    pub fn evnt_schdlr_rgstr_gnrc_evnt(p_extrnl_evnt_st: *mut extrnl_evnt_st_t) -> ext_evnt_hndl_t;
}
unsafe extern "C" {
    #[doc = " @brief  un-register external generic/ onidle  event.\n This function is used to remove external generic event from event scheduler.\n @param  evnt_hndl [in]\t: Event handle of generic event to be removed from scheduler.\n @retval Status (0: SUCCESS, 0xXX: ERROR_CODE from @ref ble_stat_t)."]
    pub fn evnt_schdlr_unrgstr_gnrc_evnt(evnt_hndl: ext_evnt_hndl_t) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  register external on idle event.\n\n the external on idle event is low priority event that will be executed in scheduler idle time.\n if multiple on idle events are registered the scheduler will divide the idle on them in round robin manner.\n @note All parameters in in @ ref _extrnl_evnt_st_t will be ignored  except ptr_priv parameter,\n  as it will be passed to event started call back\n @note @ref evnt_strtd_cbk and @ref evnt_abortd_cbk pointers must not be NULL,\n as event scheduler will call evnt_abortd_cbk to stop current running on idle event\n if other higher event wants to access air while on idle event has grant to access it.\n\n @note There is no need to call this function more than once as the event scheduler\n stores the event and will call it every time it has a window to start on idle event\n\n @param  p_extrnl_evnt_st [in]\t   : Pointer to external event structure.\n @retval evnt_hndl_t                 : The event handle of registered event.\n @retval NULL                        : means that the event is not registered."]
    pub fn evnt_schdlr_rgstr_on_idle_evnt(
        p_extrnl_evnt_st: *mut extrnl_evnt_st_t,
    ) -> ext_evnt_hndl_t;
}
unsafe extern "C" {
    #[doc = " @brief  Disable aborting the on idle event given by the handle @ref on_idle_evnt_hdl.  the user should call @ref evnt_schdlr_gnrc_evnt_cmplt after the event is completed\n\n @note the  usage of this function should  be limited to the cases where the event can't be aborted like MAC DTM. and the user should return the grant to scheduler as fast as possible.\n if this function is called where the event is granted, scheduler will ignore all other event till the grant is return to scheduler. it is not recommended to use this function\n\n @param  on_idle_evnt_hdl [in]\t   : handle of the on the idle event to disable aborting it\n @retval None"]
    pub fn evnt_schdlr_disable_idle_abort(on_idle_evnt_hdl: ext_evnt_hndl_t);
}
unsafe extern "C" {
    #[doc = " @brief  external generic event complete.\n\n This function is used to return the grant back to event scheduler\n at the end of duration given at the started function from event scheduler @ref _extrnl_evnt_st_t for more info\n\n @param  evnt_hndl_t [in]\t: Event handle of registered event.\n @retval Status (0: SUCCESS, 0xXX: ERROR_CODE from @ref ble_stat_t)."]
    pub fn evnt_schdlr_gnrc_evnt_cmplt(evnt_hndl: ext_evnt_hndl_t) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  Confirm the event currently has grant to access phy from event scheduler.\n\n @param  evnt_hndl [in]\t: pointer of event scheduler handle.\n @retval 1: grant given.\n @retval 0: no grant given."]
    pub fn evnt_schdlr_confrm_grant(evnt_hndl: *mut ::core::ffi::c_void) -> u8;
}
pub const pta_state_PTA_DISABLED: pta_state = 0;
pub const pta_state_PTA_ENABLED: pta_state = 1;
#[doc = " PUBLIC ENUMERATIONS *****************************/\n/**\n @brief Enumeration holding the PTA enable and disable."]
pub type pta_state = ::core::ffi::c_uint;
pub const pta_error_PTA_ERROR_SUCCESS: pta_error = 0;
pub const pta_error_PTA_ERROR_PTA_NOT_ENABLED: pta_error = 1;
pub const pta_error_PTA_ERROR_INVALID_PRIORITY_CONF: pta_error = 2;
pub const pta_error_PTA_ERROR_UNKNOWN_CONN_HANDLE: pta_error = 3;
pub const pta_error_PTA_ERROR_UNKNOWN_PRDC_SYNC_HANDLE: pta_error = 4;
pub const pta_error_PTA_ERROR_UNKNOWN_CIG_HANDLE: pta_error = 5;
pub const pta_error_PTA_ERROR_UNKNOWN_BIG_HANDLE: pta_error = 6;
pub const pta_error_PTA_ERROR_INVALID_NBR_OF_PKTS: pta_error = 7;
pub const pta_error_PTA_ERROR_INVALID_TIMEOUT: pta_error = 8;
pub const pta_error_PTA_ERROR_INVALID_REQUEST_TO_EVENT_TIME: pta_error = 9;
pub const pta_error_PTA_ERROR_INVALID_PTA_STATE: pta_error = 10;
pub const pta_error_PTA_ERROR_INIT_ALREADY_CALLED: pta_error = 11;
pub const pta_error_PTA_ERROR_PTA_ENABLED_IN_INIT: pta_error = 12;
pub const pta_error_PTA_ERROR_PTA_INIT_NOT_CALLED: pta_error = 13;
#[doc = " @brief Enumeration holding all the error codes for the PTA interfaces"]
pub type pta_error = ::core::ffi::c_uint;
unsafe extern "C" {
    #[doc = " @ingroup  pta_functions\n  @{\n/\n/**\n @brief Used to enable and disable the PTA.\n @note This API can be called directly to enable/disable the PTA feature. In case the BLE controller is supported, beside this API,\n \t\t\tthere is an option to enable/disable the PTA feature through the \"HCI_CMD_OCF_PTA_ENABLE\" custom HCI command.\n\n @param enable: [in] 0: Disable. 1: Enable.\n\n @retval pta_error : \tPTA_ERROR_INVALID_PTA_STATE:\n \t\t\t\t\t\t\t- If enable is passed while the PTA is already enabled.\n \t\t\t\t\t\t\t- If disabled is passed while the PTA is already disabled.\n \t\t\t\t\t\t\t- If a value outside of the pta_state enumeration is passed.\n \t\t\t\t\t\tPTA_ERROR_SUCCESS : Otherwise."]
    pub fn pta_enable(enable: pta_state) -> pta_error;
}
unsafe extern "C" {
    #[doc = " @brief Used to initialize the PTA feature. The PHY sequences are configured with\n \t\t  respect to the \"request_to_event_time\" parameter.\n @note This API can be called directly to initialize the PTA feature. In case the BLE controller is supported, beside this API,\n \t\t\tthere is an option to initialize the PTA feature through the \"HCI_CMD_OCF_PTA_INIT\" custom HCI command.\n\n @param request_to_event_time : [in] Time between the request signal assertion\n \t\t\t\t\t\t\t\t\t  and beginning of event on air.\n\n @retval pta_error : \tPTA_DISABLED: If PTA is disabled.\n \t\t\t\t\t   \tPTA_ERROR_INVALID_REQUEST_TO_EVENT_TIME: If the request_to_event_time is greater than the minimum time of the TX/RX interpacket time.\n \t\t\t\t\t   \tPTA_ERROR_SUCCESS : Otherwise."]
    pub fn pta_init(request_to_event_time: u8) -> pta_error;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the priority of the generic BLE events.\n @note This API can be called directly to configure the priority of the generic BLE events. In case the BLE controller is supported, beside this API,\n \t\t\tthere is an option to configure the priority of the generic BLE events through the \"HCI_CMD_OCF_PTA_BLE_SetCoexPriority\" custom HCI command.\n\n @param priority\t\t: [in] Determines the state of each priority mode.\n @param priority_mask\t: [in] Determines which priorities are in effect in\n \t\t\t\t\t\t\t  the priority variable.\n\n @retval \tpta_error : PTA_DISABLED: If PTA is disabled.\n \t\t\t\t\t\tPTA_ERROR_INVALID_PRIORITY_CONF:\n \t\t\t\t\t\t\t- If Forced configuration = 11\n \t\t\t\t\t\t\t- priority_mask did not mask away the reserved bits in priority\n \t\t\t\t\t\t\t- priority_mask masked one bit only of the first two bits in the priority.\n \t\t\t\t\t\tPTA_ERROR_SUCCESS : Otherwise."]
    pub fn pta_set_coex_priority(priority: u32, priority_mask: u32) -> pta_error;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the priority of the MAC Packets.\n @note This API can be called directly to configure the priority of the MAC Packets. Beside this API,\n \t\t\tthere is an option to configure the priority of the MAC Packets through the \"PTA_SET_PRIORITY\" MAC custom HCI command.\n\n @param priority\t\t: [in] Determines the state of each priority mode.\n @param priority_mask\t: [in] Determines which priorities are in effect in\n \t\t\t\t\t\t\t  the priority variable.\n\n @retval pta_error :\tPTA_DISABLED: If PTA is disabled.\n \t\t\t\t\t\tPTA_ERROR_INVALID_PRIORITY_CONF:\n \t\t\t\t\t\t\t- If Forced configuration = 11\n \t\t\t\t\t\t\t- priority_mask did not mask away the reserved bits in priority\n \t\t\t\t\t\t\t- priority_mask masked one bit only of the first two bits in the priority.\n \t\t\t \t\t\tPTA_ERROR_SUCCESS : Otherwise."]
    pub fn pta_set_mac_coex_priority(priority: u32, priority_mask: u32) -> pta_error;
}
unsafe extern "C" {
    #[link_name = "\u{1}p_dis_tbl"]
    pub static mut P_DIS_TBL: *const hci_dispatch_tbl;
}
pub const _ble_conn_role_e_BLE_ROLE_MASTER: _ble_conn_role_e = 0;
pub const _ble_conn_role_e_BLE_ROLE_SLAVE: _ble_conn_role_e = 1;
#[doc = " @brief BLE  role enumeration"]
pub type _ble_conn_role_e = ::core::ffi::c_uint;
#[doc = " @brief BLE  role enumeration"]
pub use self::_ble_conn_role_e as ble_conn_role_e;
pub const _ble_adv_event_type_e_ADV_IND_EVENT: _ble_adv_event_type_e = 0;
pub const _ble_adv_event_type_e_ADV_DIRECT_IND_EVENT: _ble_adv_event_type_e = 1;
pub const _ble_adv_event_type_e_ADV_SCAN_IND_EVENT: _ble_adv_event_type_e = 2;
pub const _ble_adv_event_type_e_ADV_NONCONN_IND_EVENT: _ble_adv_event_type_e = 3;
pub const _ble_adv_event_type_e_SCAN_RSP_EVENT: _ble_adv_event_type_e = 4;
#[doc = " @brief Advertising event type enumeration"]
pub type _ble_adv_event_type_e = ::core::ffi::c_uint;
#[doc = " @brief Advertising event type enumeration"]
pub use self::_ble_adv_event_type_e as ble_adv_event_type_e;
pub const dev_addr_type_PUBLIC: dev_addr_type = 0;
pub const dev_addr_type_RANDOM: dev_addr_type = 1;
pub const dev_addr_type_PUBLIC_ID: dev_addr_type = 2;
pub const dev_addr_type_RANDOM_STATIC_ID: dev_addr_type = 3;
pub const dev_addr_type_INVALID_TYPE: dev_addr_type = 4;
pub const dev_addr_type_ANONYMOUS: dev_addr_type = 255;
#[doc = " @brief Enum device address type."]
pub type dev_addr_type = ::core::ffi::c_uint;
#[doc = " @brief Enum device address type."]
pub use self::dev_addr_type as dev_addr_type_e;
pub const _rec_adv_stat_e_ADV_EXT_STAT: _rec_adv_stat_e = 0;
pub const _rec_adv_stat_e_AUX_ADV_STAT: _rec_adv_stat_e = 1;
pub const _rec_adv_stat_e_AUX_CHAIN_STAT: _rec_adv_stat_e = 2;
#[doc = " @brief Enum device address type."]
pub type _rec_adv_stat_e = ::core::ffi::c_uint;
#[doc = " @brief Enum device address type."]
pub use self::_rec_adv_stat_e as rec_adv_stat_e;
pub const _ble_prvcy_mod_e_NETWORK_MODE: _ble_prvcy_mod_e = 0;
pub const _ble_prvcy_mod_e_DEVICE_MODE: _ble_prvcy_mod_e = 1;
pub type _ble_prvcy_mod_e = ::core::ffi::c_uint;
pub use self::_ble_prvcy_mod_e as ble_prvcy_mod_e;
pub const _sm_status_e_SM_STATUS_IDLE: _sm_status_e = 0;
pub const _sm_status_e_SM_STATUS_ADV_EXTADV: _sm_status_e = 1;
pub const _sm_status_e_SM_STATUS_PERI_CONN: _sm_status_e = 2;
pub const _sm_status_e_SM_STATUS_SCN_EXTSCN: _sm_status_e = 3;
pub const _sm_status_e_SM_STATUS_CENT_CONN: _sm_status_e = 5;
pub const _sm_status_e_SM_STATUS_DTM_TX: _sm_status_e = 6;
pub const _sm_status_e_SM_STATUS_DTM_RX: _sm_status_e = 7;
pub const _sm_status_e_SM_STATUS_PRDC_ADV: _sm_status_e = 9;
pub const _sm_status_e_SM_STATUS_PRDC_SYNC: _sm_status_e = 10;
pub const _sm_status_e_SM_STATUS_BIG_ADV: _sm_status_e = 11;
pub const _sm_status_e_SM_STATUS_BIG_SYNC: _sm_status_e = 12;
pub const _sm_status_e_SM_STATUS_PERI_CIG: _sm_status_e = 13;
pub const _sm_status_e_SM_STATUS_CENT_CIG: _sm_status_e = 14;
#[doc = " @brief State machine status for Get Link Status Command, it holds the\n  values for different state machines based on its running event type"]
pub type _sm_status_e = ::core::ffi::c_uint;
#[doc = " @brief The data in one advertising report from the non-connection manager to the LL interface\n\n This structure contains the parameters should be sent from the link layer to the host per report."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_intf_adv_report_data_st {
    pub adv_addr: [u8; 8usize],
    pub adv_rprt_data: ble_buff_hdr_t,
    pub rssi: i8,
    pub evnt_type: ble_adv_event_type_e,
    pub adv_addr_type: dev_addr_type_e,
}
impl Default for _ble_intf_adv_report_data_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief The data in one advertising report from the non-connection manager to the LL interface\n\n This structure contains the parameters should be sent from the link layer to the host per report."]
pub type ble_intf_adv_report_data_st = _ble_intf_adv_report_data_st;
#[doc = " @brief Structure containing the advertising report data to be reported to host"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_intf_extended_adv_rprt_data_st {
    pub adv_stat: rec_adv_stat_e,
    pub event_type: u16,
    pub address_type: u8,
    pub ptr_address: *mut u8,
    pub primary_phy: u8,
    pub secondary_phy: u8,
    pub advertising_sid: u8,
    pub adv_data_id: u16,
    pub TX_power: u8,
    pub rssi: i8,
    pub periodic_advertisng_interval: u16,
    pub direct_addresses_type: u8,
    pub ptr_direct_address: *mut u8,
    pub data_length: u8,
    pub ptr_data: *mut ble_buff_hdr_t,
    pub rmv_adv_rprt: u8,
    pub address: [u8; 6usize],
}
impl Default for _ble_intf_extended_adv_rprt_data_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure containing the advertising report data to be reported to host"]
pub type ble_intf_extended_adv_rprt_data_st = _ble_intf_extended_adv_rprt_data_st;
#[doc = " @brief The data in one direct advertising report from the non-connection manager to the LL interface\n\n This structure contains the parameters should be sent from the link layer to the host per direct advertising report."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_intf_dir_adv_report_data_st {
    pub addr: [u8; 8usize],
    pub dir_addr: [u8; 8usize],
    pub rssi: i8,
    pub evnt_type: ble_adv_event_type_e,
    pub addr_type: dev_addr_type_e,
    pub dir_addr_type: dev_addr_type_e,
}
impl Default for _ble_intf_dir_adv_report_data_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief The data in one direct advertising report from the non-connection manager to the LL interface\n\n This structure contains the parameters should be sent from the link layer to the host per direct advertising report."]
pub type ble_intf_dir_adv_report_data_st = _ble_intf_dir_adv_report_data_st;
#[doc = " @brief Data contained in extended advertising enable command for each advertising handle.\n\n This structure contains the parameters that are passed by the Host in extended advertising enable command for each advertising handle .\n we divided the duration into two octets to avoid the structure padding"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct st_ble_intf_ext_adv_enable_params {
    pub advertising_handle: u8,
    pub duration_LSB: u8,
    pub duration_MSB: u8,
    pub max_extended_advertising_events: u8,
}
#[doc = " @brief Data contained in extended create connection command for each PHY.\n\n This structure contains the parameters that are passed by the Host in set extended scan parameters command for each PHY."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct st_ble_intf_ext_scn_params {
    pub scan_type: u8,
    pub scan_interval: u16,
    pub scan_window: u16,
}
pub const _enum_ext_create_conn_verison_EXT_CREATE_CONN_VER_1: _enum_ext_create_conn_verison = 0;
pub const _enum_ext_create_conn_verison_EXT_CREATE_CONN_VER_2: _enum_ext_create_conn_verison = 1;
pub type _enum_ext_create_conn_verison = ::core::ffi::c_uint;
pub use self::_enum_ext_create_conn_verison as enum_ext_create_conn_ver;
pub const _enum_prdc_adv_param_ver_PRDC_ADV_PARAM_VER_1: _enum_prdc_adv_param_ver = 0;
pub const _enum_prdc_adv_param_ver_PRDC_ADV_PARAM_VER_2: _enum_prdc_adv_param_ver = 1;
pub type _enum_prdc_adv_param_ver = ::core::ffi::c_uint;
pub use self::_enum_prdc_adv_param_ver as enum_prdc_adv_param_ver;
#[doc = " @brief Data contained in extended create connection command for each PHY.\n\n This structure contains the parameters that are passed by the Host in extended create connection command for each PHY."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ble_intf_ext_create_conn_st {
    pub scan_interval: u16,
    pub scan_window: u16,
    pub conn_interval_min: u16,
    pub conn_interval_max: u16,
    pub conn_latency: u16,
    pub supervision_timeout: u16,
    pub minimum_ce_length: u16,
    pub maximum_ce_length: u16,
}
#[doc = " @brief Data contained in extended create connection command for each PHY.\n\n This structure contains the parameters that are passed by the Host in extended create connection command for each PHY."]
pub type st_ble_intf_ext_create_conn = _ble_intf_ext_create_conn_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_intf_ext_create_conn_cmd_st {
    pub ptr_ext_create_conn: *mut st_ble_intf_ext_create_conn,
    pub ptr_peer_address: *mut u8,
    pub initiator_filter_policy: u8,
    pub own_address_type: u8,
    pub peer_address_type: u8,
    pub initiating_phys: u8,
}
impl Default for _ble_intf_ext_create_conn_cmd_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ble_intf_ext_create_conn_cmd_st = _ble_intf_ext_create_conn_cmd_st;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ble_set_prdc_adv_param_st {
    pub prdc_adv_intrvl_min: u16,
    pub prdc_adv_intrvl_max: u16,
    pub prdc_adv_prpts: u16,
}
pub type ble_set_prdc_adv_param_st = _ble_set_prdc_adv_param_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_enhanced_conn_cmplt_evnt_st {
    pub status: ble_stat_t,
    pub ptr_peer_addr: *mut u8,
    pub ptr_local_resolvable_prvt_addr: *mut u8,
    pub ptr_peer_resolvable_prvt_addr: *mut u8,
    pub role: ble_conn_role_e,
    pub conn_handle_id: u16,
    pub conn_intrvl: u16,
    pub slave_ltncy: u16,
    pub suprvsn_tout: u16,
    pub peer_addr_type: u8,
    pub master_clk_accurcy: u8,
}
impl Default for _ble_enhanced_conn_cmplt_evnt_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ble_enhanced_conn_cmplt_evnt_st = _ble_enhanced_conn_cmplt_evnt_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_prdc_adv_sync_estblshd_st {
    pub ptr_adv_addrs: *mut u8,
    pub status: ble_stat_t,
    pub sync_handle: u16,
    pub prdc_adv_intrvl: u16,
    pub adv_sid: u8,
    pub adv_addrs_type: u8,
    pub adv_phy: u8,
    pub adv_clk_accuracy: u8,
}
impl Default for _ble_prdc_adv_sync_estblshd_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ble_prdc_adv_sync_estblshd_st = _ble_prdc_adv_sync_estblshd_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ble_prdc_adv_rprt_st {
    pub ptr_data: *mut ble_buff_hdr_t,
    pub tx_power: u8,
    pub rssi: i8,
    pub cte_type: u8,
    pub data_status: u8,
    pub data_length: u8,
}
impl Default for _ble_prdc_adv_rprt_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ble_prdc_adv_rprt_st = _ble_prdc_adv_rprt_st;
#[doc = " @brief LE Set ADV parameters"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _le_set_adv_params_cmd_st {
    pub adv_intrv_min: u16,
    pub adv_intrv_max: u16,
    pub adv_type: u8,
    pub own_addr_type: u8,
    pub peer_addr_type: u8,
    pub peer_addr: [u8; 6usize],
    pub adv_chnl_map: u8,
    pub adv_filter_policy: u8,
}
#[doc = " @brief LE Set ADV parameters"]
pub type le_set_adv_params_cmd_st = _le_set_adv_params_cmd_st;
#[doc = " @brief LE Create Connection Command"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _le_set_scn_params_cmd_st {
    pub scn_type: u8,
    pub scn_interv: u16,
    pub scn_wndw: u16,
    pub own_addr_type: u8,
    #[doc = "<filter policy type saved for fw purposes. range 0:3"]
    pub scanning_filter_policy: u8,
}
#[doc = " @brief LE Create Connection Command"]
pub type le_set_scn_params_cmd_st = _le_set_scn_params_cmd_st;
#[doc = " @brief LE Set Scan Command"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _le_create_conn_cmd_st {
    pub le_scan_intrv: u16,
    pub le_scan_wndw: u16,
    pub init_filter_policy: u8,
    pub peer_addr_type: u8,
    pub peer_addr: [u8; 6usize],
    pub own_addr_type: u8,
    pub conn_interval_min: u16,
    pub conn_interval_max: u16,
    pub host_slave_latency: u16,
    pub sv_timeout: u16,
    pub min_ce_length: u16,
    pub max_ce_length: u16,
}
#[doc = " @brief LE Set Scan Command"]
pub type le_create_conn_cmd_st = _le_create_conn_cmd_st;
#[doc = " @brief LE Remote Connection Parameters REQ reply"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _le_rmt_conn_param_req_rply_cmd_st {
    pub conn_interval_min: u16,
    pub conn_interval_max: u16,
    pub slave_latency: u16,
    pub sv_timeout: u16,
    pub min_ce_length: u16,
    pub max_ce_length: u16,
}
#[doc = " @brief LE Remote Connection Parameters REQ reply"]
pub type le_rmt_conn_param_req_rply_cmd_st = _le_rmt_conn_param_req_rply_cmd_st;
#[doc = " @brief Control RX Data Throughput parameters"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _ctrl_rx_dtp_st {
    pub pckt_count: u8,
    pub rx_octets: u8,
}
#[doc = " @brief Control RX Data Throughput parameters"]
pub type ctrl_rx_dtp_st = _ctrl_rx_dtp_st;
#[doc = " @brief HCI Dispatch table containing callback event functions"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct hci_dispatch_tbl {
    #[doc = " @brief  Used to notify the Host that a hardware failure has occurred in the Controller\n\n @param  hw_code  : [in]  code values that indicate various hardware problems."]
    pub ll_intf_hw_error_evnt: ::core::option::Option<unsafe extern "C" fn(hw_code: u8)>,
    #[doc = " @brief  is used to suggest maximum packet sizes to the Controller.\n\n @param  conn_handle_id \t: [in] Connection Handle Id to be used to identify a connection.\n @param  max_tx_octets\t: [in] The maximum number of payload octets in a Link Layer Data Channel PDU that the local Controller will send on this connection.\n @param  max_tx_time\t\t: [in] The maximum time that the local Controller will take to send a Link Layer Data Channel PDU on this connection.\n @param  max_rx_octets\t: [in] The maximum number of payload octets in a Link Layer Data Channel PDU that the local controller expects to receive on this connection.\n @param  max_rx_time\t\t: [in] The maximum time that the local Controller expects to take to receive a Link Layer Data Channel PDU on this connection."]
    pub ll_intf_le_data_length_chg_event: ::core::option::Option<
        unsafe extern "C" fn(
            conn_handle_id: u16,
            max_tx_octets: u16,
            max_tx_time: u16,
            max_rx_octets: u16,
            max_rx_time: u16,
        ),
    >,
    #[doc = " @brief  Send the end of radio activity event to host.\n\n @param  curr_state  \t: [in] the state of the current event\n @param  nxt_state  \t: [in] the state of the next event\n\n @retval None"]
    pub ll_intf_end_of_activity_evnt:
        ::core::option::Option<unsafe extern "C" fn(curr_state: u16, nxt_state: u16)>,
}
#[doc = " @brief Union containing the set parameters commands"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union _hci_cmds_params_un {
    pub le_set_adv_params_cmd: le_set_adv_params_cmd_st,
    pub le_create_conn_cmd: le_create_conn_cmd_st,
    pub le_set_scn_params_cmd: le_set_scn_params_cmd_st,
    pub le_rmt_conn_param_req_rply_cmd: le_rmt_conn_param_req_rply_cmd_st,
}
impl Default for _hci_cmds_params_un {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Union containing the set parameters commands"]
pub type hci_cmds_params_un = _hci_cmds_params_un;
pub type hst_cbk =
    ::core::option::Option<unsafe extern "C" fn(ptr_evnt_hdr: *mut ble_buff_hdr_t) -> u8>;
pub type hst_cbk_queue_full =
    ::core::option::Option<unsafe extern "C" fn(ptr_evnt_hdr: *mut ble_buff_hdr_t)>;
unsafe extern "C" {
    #[doc = " @brief Initializes the LL stack\n\n @param p_dispatch_tbl : [in] Dispatch table for HCI events\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_init(p_dispatch_tbl: *const hci_dispatch_tbl) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Reset the controller and the Link Layer on an LE controller .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_reset() -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the values of the version information of the local controller .\n\n @param  hci_version    \t : [out] Defines the version information of the HCI layer .\n @param  hci_revision\t : [out] Revision of the Current HCI in the BE/EDR Controller .\n @param  lmp_version\t\t : [out] Version of the Current LMP or PAL in the Controller .\n @param  manfacturer_name     : [out] Manufacturer Name of the BR/EDR Controller .\n @param  lmp_subversion\t : [out] Subversion of the Current LMP or PAL in the Controller .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_read_local_ver_info(
        hci_version: *mut u8,
        hci_revision: *mut u8,
        lmp_version: *mut u8,
        manfacturer_name: *mut u8,
        lmp_subversion: *mut u8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the version information of the controller .\n\n @param  ptr_vrsn  : [out] Defines the controller version information.\n @param  length     : [in] the length of the sent array to be written.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_read_cntrlr_ver_info(ptr_vrsn: *mut u8, length: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the list of HCI commands supported for the local controller\n \t\t   (It is implied that if a command is listed as supported, the feature underlying that command is also supported) .\n\n @param  supported_cmds : [out] A bit mask for each HCI command, where:\n \t\t\t     If the controller sets a bit to 1, then the controller supports the corresponding command and the features required for the command, and\n \t\t\t     If the controller sets a bit to 0, then this command is unsupported or undefined command .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_read_local_supported_cmds(supported_cmds: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read a list of the supported features for the local BR/EDR Controller including the LE Supported feature .\n\n @param  lmp_features : [out] Bit Mask List of LMP features .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_read_local_supported_features(lmp_features: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the list of the supported LE features for the Controller .\n\n @param  le_features : [out] Bit Mask List of supported LE features .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_read_local_supported_features_page_0(le_features: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the Public Device Address of the LE controller.\n @param  bd_addr : [out] Public address of the LE controller .\n\n @retval ble_stat_t : Command status to be sent to the Host ."]
    pub fn ll_intf_read_bd_addr(bd_addr: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  write the Public Device Address of the LE controller .\n\n @param  bd_addr : [in] Public address of the LE controller .\n\n @retval ble_stat_t : Command status to be sent to the Host ."]
    pub fn ll_intf_write_bd_addr(bd_addr: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the states and state combinations that the link layer supports .\n\n @param  le_states : [out] Bit Mask List of supported LE states and state combinations .\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_read_supported_states(le_states: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @ingroup  controller_info\n  @{\n/\n/**\n @brief  Set the LE Random Device Address in the Controller. This address is sent by the Host.\n\n @param  random_addr : [in] Random Device Address sent by the Host to the Controller.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_set_random_addr(random_addr: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @ingroup  white_list_cfg White list Commands\n  @{\n/\n/**\n @brief  Add a single device to the white list stored in the Controller.\n\n @param  addr_type\t: [in] Address type of the device to be added to the white list of the controller.\n @param  addr\t\t: [in] Public Device Address or Random Device Address of the device to be added to the white list.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_add_device_white_list(addr_type: u8, addr: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Remove a single device from the white list stored in the Controller.\n\n @param  addr_type \t : [in] Address type of the device to be added to the white list of the controller.\n @param  addr\t\t : [in] Public Device Address or Random Device Address of the device to be removed from the white list.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_remove_device_white_list(addr_type: u8, addr: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Clear the white list stored in the Controller.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_clear_white_list() -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Read the total number of white list entries that can be stored in the Controller.\n\n @param  white_list_size : [out] Total number of white list entries that can be stored in the Controller.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_read_white_list_size(white_list_size: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @ingroup  dtm_cfg  DTM Commands\n  @{\n/\n/**\n @brief run the LE receiver test.\n\n @param  rx_channel\t\t: [in] rx channel value:\n\n @retval ble_stat_t \t\t: Command status to be sent to the Host."]
    pub fn ll_intf_le_receiver_test(rx_channel: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief run the LE transmitter test.\n\n @param  tx_channel\t\t: [in] tx channel value.\n @param  length_of_test_data\t: [in] Length in bytes of payload data in each packet.\n @param  packet_payload\t: [in] Packet payload type in each packet as in (7.8.29).\n\n @retval ble_stat_t \t\t: Command status to be sent to the Host."]
    pub fn ll_intf_le_transmitter_test(
        tx_channel: u8,
        length_of_test_data: u8,
        packet_payload: u8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief end LE test.\n\n @param  number_of_packets\t: Pointer to number of packets received.\n\n @retval ble_stat_t \t\t: Command status to be sent to the Host."]
    pub fn ll_intf_le_test_end(number_of_packets: *mut u16) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  The LE Set PHY command is used to request a change to the transmitter PHY and receiver PHY for a connection.\n\n @param rx_channel\t\t: [in] Frequency Range 0x00 to 0x27\n @param phy\t\t\t\t: [in] Receiver set to 1M/2M/coded PHY\n @param modulation_index\t: [in] Modulation index type (standard/stable)\n\n @retval status        \t: [out] 0:SUCCESS, 0xXX:ERROR_CODE.\n"]
    pub fn ll_intf_le_enhanced_receiver_test(
        rx_channel: u8,
        phy: u8,
        modulation_index: u8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  The LE Set PHY command is used to request a change to the transmitter PHY and receiver PHY for a connection.\n\n @param tx_channel \t\t: [in] Frequency Range 0x00 to 0x27\n @param length_of_test_data \t: [in] Length in bytes of payload data in each packet\n @param packet_payload  \t: [in] Sequence\n @param phy\t\t\t\t: [in] Transmitter set to 1M/2M/coded PHY\n\n @retval status        \t: [out] 0:SUCCESS, 0xXX:ERROR_CODE."]
    pub fn ll_intf_le_enhanced_transmitter_test(
        tx_channel: u8,
        length_of_test_data: u8,
        packet_payload: u8,
        phy: u8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @ingroup  tx_pwr_cfg  Tx Power Commands\n @{\n/\n/**\n @brief  Used to read the minimum and maximum transmit powers supported by the Controller.\n\n @param  ptr_min_tx_pwr\t: [out] A pointer to the min TX power value to be set by the controller [Range: -127 dB to 20 dB]\n @param  ptr_max_tx_pwr\t: [out] A pointer to the max TX power compensation value to be set by the controller [Range: -127 dB to 20.0 dB]\n\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_le_read_tx_pwr(ptr_min_tx_pwr: *mut i8, ptr_max_tx_pwr: *mut i8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Used to read the RF Path Compensation Values parameter used in the Tx Power Level and RSSI calculation.\n\n @param  ptr_rf_tx_path_compnstn\t: [out] A pointer to the RF TX path compensation value to be set by the controller [Range: -128.0 dB (0xFB00) -> 128.0 dB (0x0500)]\n @param  ptr_rf_rx_path_compnstn\t: [out] A pointer to the RF RX path compensation value to be set by the controller [Range: -128.0 dB (0xFB00) -> 128.0 dB (0x0500)]\n\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_le_read_rf_path_compensation(
        ptr_rf_tx_path_compnstn: *mut i16,
        ptr_rf_rx_path_compnstn: *mut i16,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Used to indicate the RF path gain or loss between the RF transceiver and the antenna contributed by intermediate components.\n\n @param  ptr_rf_tx_path_compnstn\t: [in] RF TX path compensation value sent by host [Range: -128.0 dB (0xFB00) -> 128.0 dB (0x0500)]\n @param  ptr_rf_rx_path_compnstn\t: [in] RF RX path compensation value sent by host [Range: -128.0 dB (0xFB00) -> 128.0 dB (0x0500)]\n\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_le_write_rf_path_compensation(
        ptr_rf_tx_path_compnstn: i16,
        ptr_rf_rx_path_compnstn: i16,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Set the minimum and maximum TX Power values to be supported by the controller.\n\n @param  tx_pwr     : [in] transmit power sent by host to be used by the controller\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_write_tx_pwr(tx_pwr: i8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Set Sleep Clock Acuuracy.\n\n @param  slp_clk_acc    : [in] sleep clock accuracy\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_set_sleep_clock_accuracy(slp_clk_acc: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure linklayer to go to/back from deep sleep mode.\n\n @param  dp_slp_mode : [in] 1 enable deep sleep mode , 0 go back to sleep mode\n\n @retval Status."]
    pub fn ll_intf_le_set_dp_slp_mode(dp_slp_mode: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the PHY calibration event parameters.\n\n @param  phy_clbr_evnt_period : [in] Indicate the periodicity of the PHY calibration event. Periodicity = phy_clbr_evnt_period * 1s.\n @param  phy_clbr_evnt_count \t: [in] Indicate the number of the PHY calibration events to be executed."]
    pub fn ll_intf_le_set_phy_clbr_params(phy_clbr_evnt_period: u32, phy_clbr_evnt_count: u32);
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the execution context for PHY calibration initialization.\n\n @param  phy_clbr_context: [in] Specifies the context in which the PHY\n         calibration initialization is executed.\n\n @retval None."]
    pub fn ll_intf_le_set_phy_clbr_context(phy_clbr_context: u8);
}
unsafe extern "C" {
    #[doc = " @brief Used to Get Remaining Time For Next Event.\n\n @param  remaing_time : [out] the value of remaining time  for the next event in us.\n\n @retval Status."]
    pub fn ll_intf_le_get_remaining_time_for_next_event(remaing_time: *mut u32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to Set SETUP_TIME Time For Next Event.\n\n @param  setup_time : [in]  the value of setup time in us to be used be the link layer scheduler  .\n\n @retval Status."]
    pub fn ll_intf_le_set_scheduler_setup_time(setup_time: u32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to select the source that drives the sleep clock, whether to use an external crystal oscillator or an integrated RC oscillator (RCO).\n\n @param  slp_clk_src \t\t\t\t: [in] Indicate which source to drive the sleep clock. 0: Crystal Oscillator (default). 1: RC0\n @param  ptr_slp_clk_freq_value \t: [out] Indicate the nominal frequency value of the sleep clock.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_select_slp_clk_src(
        slp_clk_src: u8,
        ptr_slp_clk_freq_value: *mut u16,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the runtime RCO calibration event parameters.\n\n @param  rco_clbr_event_duration : [in] Indicate the number of sleep clock cycles for performing the RCO calibration process.\n @param  rco_clbr_event_interval : [in] Indicate the periodicity of running the runtime RCO calibration event.\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_le_set_rco_clbr_evnt_params(
        rco_clbr_event_duration: u8,
        rco_clbr_event_interval: u32,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to specify the used power table and its size based on the selected TX_Power table ID.\n\n @param  tx_power_table_id : [in] Selected TX_Power table ID.\n\n @retval Status \t\t : 0: SUCCESS. Otherwise: Error code."]
    pub fn ll_intf_select_tx_power_table(tx_power_table_id: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  This function shall be called on any packets that has been passed from the LL to the host.\n\n @param  pkt  : [in] Pointer to the raw LL packet"]
    pub fn ll_intf_free_ll_pkt(pkt: *mut ::core::ffi::c_void);
}
unsafe extern "C" {
    #[doc = " @brief  This function free both the LL packet and the handler associated with it.\n\n @param  pkt  : [in] Pointer to ble_buff_hdr_t that points to the LL packet"]
    pub fn ll_intf_free_ll_pkt_hndlr(pkt: *mut ble_buff_hdr_t);
}
unsafe extern "C" {
    #[doc = " @brief  : TheSet EcoSystem Base Interval Command is used by the Host to hint the controller with the best radio period\n\n @param  interval     \t: hinted interval from the host\n\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_set_ecosystem_base_interval(interval: u16) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Forwards to llhwc_cmn to take decision whether to prevent device form\n \t\t\tentering sleep state\n\n @param state [in]: Enable/Disable preventing sleep state mode\n\n @retval ble_state_t : Command status"]
    pub fn ll_intf_curb_sleep_state(state: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Used to configure the LL contexts, where:\n \t\t\t1. For bare-metal:\n \t\t\t- High ISR is executed in the ISR context\n \t\t\t- Low ISR can be executed in the high ISR context, or switched to low ISR context\n \t\t\t2. For RTOS:\n \t\t\t- High ISR is executed in the ISR context\n \t\t\t- Low ISR is executed in the thread of the \"linkLayerHighPrioTask\"\n\n @param allow_low_isr : [in] Configuration parameter for the context of the low ISR in the bare-metal model. Range is [0,1].\n \t\t\t\t\t\t\t\t0: Low ISR code is executed in the same context of the high ISR.\n \t\t\t\t\t\t\t\t1: Low ISR code is executed in the context of the low ISR (by configuring a low priority interrupt that is triggered by FW).\n @param run_post_evnt_frm_isr : [in] Configuration parameter to decide whether the scheduling of the next BLE event is done in the low ISR context or to be handled by the LL main thread. Range is [0,1].\n \t\t\t\t\t\t\t\t0: BLE next event scheduling is handled in the LL main thread.\n \t\t\t\t\t\t\t\t1: BLE next event scheduling is handled in the low ISR context.\n\n @retval ble_state_t : Command status"]
    pub fn ll_intf_config_ll_ctx_params(allow_low_isr: u8, run_post_evnt_frm_isr: u8)
        -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Get the value of link layer timer in microsecond aligned with sleep timer clock edge\n Microsecond timing can be calculated as Return value (steps) * Multiplier /divider taking into consideration to implement calculation in good accuracy\n\n @param  multiplier : Value that should be multiplied by the return steps\n @param  divider    : the product of the steps and multiplier should be divided by this value\n\n @retval number of steps : Read number of steps.\n @note Caller should call it in a critical section to make sure the timing is not drifted by interrupt serving"]
    pub fn ll_intf_get_aligned_us_now(multiplier: *mut u32, divider: *mut u32) -> u32;
}
unsafe extern "C" {
    #[doc = " @ingroup  pta_ll_intf\n  @{\n/\n/**\n @brief Initializes the PTA init\n\n @param request_to_event_time :[IN] Time between the request signal assertion\n \t\t\t\t\t\t\t\t\t  and beginning of event on air.\n\n @retval INVALID_HCI_COMMAND_PARAMETERS:\n \t\t\tIf request to event time is not in range 20us to MIN(Tx Config / Rx Config)\n @retval COMMAND_DISALLOWED: All other PTA error codes\n @retval SUCCESS: Otherwise"]
    pub fn ll_intf_pta_init(request_to_event_time: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Enables the PTA in the hardware\n\n @param enable: [IN] Enable/Disable Indicator\n\n @retval COMMAND_DISALLOWED: If events exist in the system or the pta_enable\n \t\t\t\treturns any error code.\n @retval SUCCESS: Otherwise"]
    pub fn ll_intf_pta_enable(enable: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Generic Priority configuration function.\n\n @param priority\t\t:[IN] Determines the state of each priority mode.\n @param priority_mask\t:[IN] Determines which priorities are in effect in\n \t\t\t\t\t\t\t  the priority variable.\n\n @retval COMMAND_DISALLOWED: If the PTA is not enabled.\n @retval INVALID_HCI_COMMAND_PARAMETERS: For all the other PTA error codes.\n @retval SUCCESS: Otherwise"]
    pub fn ll_intf_pta_ble_set_coex_priority(priority: u32, priority_mask: u32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @ingroup vendor_cfg Vendor Specific Commands\n  @{\n/\n/**\n @brief this function is used to Start unmodulated carrier  Mode\n @param channel : input to selected channel from 0 to 39\n @param offset  : offset will have a step of 244 hz\n \t\t\t\t\tfrom -8196 steps(around -2Mhz) to 8196 steps(around 2Mhz)\n @param phy     : rate to start unmodulated carrier mode on ( LE_1M , LE_2M )\n @param  Tx_power_level\t\t: indicate TX Power level.\n @retval status."]
    pub fn ll_init_start_unmod_carrier(
        channel: u8,
        offset: i16,
        phy: u8,
        tx_power_level: i8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief function to check and stop the running phy mode in  case of a new event is\n  started while the continuous modulation mode or unmodulated carrier mode is running\n @retval status."]
    pub fn ll_init_stop_unmod_carrier() -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief this function is used to Start continuous DTM  Mode\n @param  ch_index\t\t\t\t: Logical channel index of DTM .\n @param  packet_payload\t\t: DTM pay-load type.\n @param  phy\t\t\t\t\t: PHY type, 1M/2M/coded PHY.\n @param  Tx_power_level\t\t: indicate TX Power level.\n @note the Tx_power_level input is applicable only if SUPPORT_LE_POWER_CONTROL is supported\n @retval status."]
    pub fn ll_init_start_cont_dtm(
        ch_index: u8,
        phy: u8,
        packet_payload: u8,
        tx_power_level: i8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief this function is used to Read Register from Phy\n @param  phy_reg\t\t\t\t: Address of register .\n @param  value\t\t\t\t: Pointer to store value of register in it.\n @retval status.\n @note this APi should be called after curb sleep to have a proper\n \t\t functionality as it should be called after PHY is started\n \t\t ,otherwise, it will return COMMAND_DISALLOWED"]
    pub fn ll_intf_le_read_phy_reg(phy_reg: u8, value: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief this function is used to Write value in Register of Phy\n @param  phy_reg\t\t\t\t: Address of register .\n @param  value\t\t\t\t: value to be stored in register.\n @retval status.\n @note this APi should be called after curb sleep to have a proper\n \t\t functionality as it should be called after PHY is started\n \t\t ,otherwise, it will return COMMAND_DISALLOWED"]
    pub fn ll_intf_le_write_phy_reg(phy_reg: u8, value: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief flag to the LL the existence of a temperature sensor\n @retval status"]
    pub fn ll_intf_set_temperature_sensor_state();
}
unsafe extern "C" {
    #[doc = " @brief set the current temperature\n @param temperature\t\t:\tThe current temperature\n @retval status"]
    pub fn ll_intf_set_temperature_value(temperature: u32) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief This function returns the number of packets sent in Direct Test Mode.\n @param[out] packet_number\t: number of packets\n @retval ble_stat_t\t: Command status.\n @note the value will not be cleared until the next Direct TX test starts."]
    pub fn ll_intf_le_tx_test_packet_number(packet_number: *mut u32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief This function returns the value of rssi.\n @param[out] rssi    \t: rssi value\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_read_raw_rssi(rssi: *mut i32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Set Tx free carrier mode .\n This function is used ot disable or enable Transmit Free carrier on a given channel\n @note this API should only be called if there is no events registered ( for example , Advertising , Connection, etc..)\n\n @param  enable     : [in] input argument to control TX free carrier mode\n \t\t\t\t\t\tTrue --> start transmission of free carrier on specific channel\n \t\t\t\t\t\tFalse --> Stop transmission of free carrier if it is already started\n\n @param  channel_idx     : [in] RF channel index of the used channel\n\n @retval ble_stat_t : Command status to be sent to the Host."]
    pub fn ll_intf_set_tx_free_carrier(enable: u8, channel_idx: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief This function sets the bitmask associated to END_OF_RADIO_ACTIVITY_EVENT.\n \t\tOnly the radio activities enabled in the mask will be reported to application by\n \t\tEND_OF_RADIO_ACTIVITY_EVENT.\n\n @param[in] mask    \t: bitmask of the events, the mask can take one of the following\n \t\t\t\t\t \tvalues (or a bitwise OR of them in case of a mask for multiple events)\n \t\t\t(0x0001) idle\n \t\t\t(0x0002) advertising and extended advertising events\n \t\t\t(0x0004) peripeheral in connection state event\n \t\t\t(0x0008) scanning and extended scanning events\n \t\t\t(0x0020) central in connection state event\n \t\t\t(0x0200) periodic advertising event\n \t\t\t(0x0400) periodic scanning event\n \t\t\t(0x0800) isochronous broadcast advertising event\n \t\t\t(0x1000) isochronous broadcast scanning event\n \t\t\t(0x2000) peripheral in isochronous connection state event\n \t\t\t(0x4000) central in isochronous connection state event\n \t\tnote that the following values are reserved and will be ignored upon reception\n \t\t\t(0x0010, 0x0040, 0x0080, 0x0100, 0x8000)\n\n @retval ble_stat_t\t     : Command status."]
    pub fn ll_intf_set_end_of_activity_mask(mask: u16) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief This function returns the status of the 8 BLE links managed by the device.\n @param[out] sm_status    \t\t: pointer to array of per running state machine status.\n @param[out] link_conn_handle\t\t: pointer to array of per running state machine handle.\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_get_link_status(sm_status: *mut u8, link_conn_handle: *mut u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  This function disables or enables the Peripheral latency feature\n         during a connection.\n\n @param  enable [in]\t: If set to 0, disable Peripheral latency\n                        if set to 1, enable Peripheral latency.\n @retval ble_stat_t\t: Command status."]
    pub fn ll_intf_set_peripheral_latency(enable: u8) -> ble_stat_t;
}
unsafe extern "C" {
    pub fn ll_intf_rgstr_hst_cbk_ll_queue_full(cbk: hst_cbk_queue_full);
}
unsafe extern "C" {
    pub fn ll_intf_rgstr_hst_cbk(upper_layer_cbk: hst_cbk);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _change_state_options_t {
    pub combined_value: u8,
    pub bitfield: _change_state_options_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _change_state_options_t__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl _change_state_options_t__bindgen_ty_1 {
    #[inline]
    pub fn allow_generic_event(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_generic_event(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_generic_event_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_generic_event_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn allow_acl_data(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_acl_data(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_acl_data_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_acl_data_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn allow_iso_data(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_iso_data(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_iso_data_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_iso_data_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn allow_reports(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_reports(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_reports_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                3usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_reports_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                3usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn allow_sync_event(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_sync_event(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_sync_event_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_sync_event_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn allow_eoa_event(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_allow_eoa_event(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn allow_eoa_event_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                5usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_allow_eoa_event_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rfu(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_rfu(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rfu_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                6usize,
                2u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_rfu_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                6usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        allow_generic_event: u8,
        allow_acl_data: u8,
        allow_iso_data: u8,
        allow_reports: u8,
        allow_sync_event: u8,
        allow_eoa_event: u8,
        rfu: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let allow_generic_event: u8 = unsafe { ::core::mem::transmute(allow_generic_event) };
            allow_generic_event as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let allow_acl_data: u8 = unsafe { ::core::mem::transmute(allow_acl_data) };
            allow_acl_data as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let allow_iso_data: u8 = unsafe { ::core::mem::transmute(allow_iso_data) };
            allow_iso_data as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let allow_reports: u8 = unsafe { ::core::mem::transmute(allow_reports) };
            allow_reports as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let allow_sync_event: u8 = unsafe { ::core::mem::transmute(allow_sync_event) };
            allow_sync_event as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let allow_eoa_event: u8 = unsafe { ::core::mem::transmute(allow_eoa_event) };
            allow_eoa_event as u64
        });
        __bindgen_bitfield_unit.set(6usize, 2u8, {
            let rfu: u8 = unsafe { ::core::mem::transmute(rfu) };
            rfu as u64
        });
        __bindgen_bitfield_unit
    }
}
impl Default for _change_state_options_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type change_state_options_t = _change_state_options_t;
unsafe extern "C" {
    #[doc = " @brief This function is used to indicate to the LL that the host\n \t\t  is ready to receive events as indicated by options parameter\n @param options: [In] bit-field to set specific events on\n @retval None"]
    pub fn ll_intf_chng_evnt_hndlr_state(options: change_state_options_t);
}
unsafe extern "C" {
    pub fn ll_intf_set_event_mask(event_mask: *mut u8);
}
unsafe extern "C" {
    pub fn ll_intf_set_event_mask_page2(event_mask: *mut u8);
}
unsafe extern "C" {
    pub fn ll_intf_set_le_event_mask(event_mask: *mut u8);
}
unsafe extern "C" {
    pub fn ll_intf_clear_event(conn_Handle: u16) -> ble_stat_t;
}
unsafe extern "C" {
    pub fn ll_intf_set_custom_event_mask(cstm_evnt_mask: u8);
}
unsafe extern "C" {
    #[doc = " @brief \tSet number of packets to be transmitted on DTM mode.\n\n @param\tpckt_count: [in] number of packets to be transmitter\n\n @note   for non-zero values of pckt_count, DTM start on TX mode will trigger sending packets with the specified\n \t\tnumber (pckt_count), if the value of pckt_count is Zero, DTM start on TX mode will trigger sending\n  \t\tindefinite number of packets until subsequent DTM stop is called or HCI reset is sent.\n\n @retval status  : [out] 0:SUCCESS, 0xXX:ERROR_CODE."]
    pub fn ll_intf_set_dtm_with_spcfc_pckt_count(pckt_count: u16) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  used to update the event timing.\n\n @param  p_evnt_timing[in]: pointer to structure containing the new Event timing requested from the Upper layer. All the values passed within should be the worst case timings. The actual time for each timing is calculated by the link layer.\n @param  effective_exec_time[out]: Execution time calculated by the controller.\n\n @retval None"]
    pub fn ll_intf_config_schdling_time(
        p_evnt_timing: *mut Evnt_timing_t,
        effective_exec_time: *mut u32,
    );
}
unsafe extern "C" {
    #[doc = " @brief  Set the rx data length throughput parameters.\n \t\tthe first rx_pckt_count will have a payload size of rx_pckt_len and the remaining rx slot (if any) will have a payload size of\n \t\tconnEffectiveMaxRxOctets of the current connection, if rx_pckt_count is set to a value greater than the PACKETS_PER_EVENT_MAX,\n \t\tthe PACKETS_PER_EVENT_MAX will be used, if rx_pckt_len is set to a value greater than the connEffectiveMaxRxOctets of the\n \t\tcurrent connection, the connEffectiveMaxRxOctets will be used.\n\n @param  rx_pckt_count \t: [in] number of rx packets expected to be received with a payload size of rx_pckt_len octets,\n  \t\t\t\tthe remaining rx slots will be calculated with the connEffectiveMaxRxOctets of the current connection.\n @param  rx_pckt_len\t\t: [in] length of rx packets expected to be received on the first rx_pckt_count slots.\n\n @retval ble_stat_t: Command status to be sent to the Host."]
    pub fn ll_intf_ctrl_rx_dtp(rx_pckt_count: u8, rx_pckt_len: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Proprietary commands to enable the ADV 5ms support\n @param enable  [in]: Enable/Disable proprietary feature\n @param randomization [in]: Max Randomization time in 625us unit. Range allowed 0-15"]
    pub fn ll_intf_ADV5ms_support(enable: u8, randomization: u32);
}
unsafe extern "C" {
    #[doc = " @brief Proprietary commands to enable the Connection 5ms support\n @param enable  [in]: Enable/ Disable proprietary feature.\n              (0 : disable min at 5ms, 1 : enable min Conn Interval at 5ms)"]
    pub fn ll_intf_BLE5ms_interval_support(enable: u8);
}
pub const hci_return_command_type_HCI_RETURN_COMMAND_TYPE_COMPLETE: hci_return_command_type = 0;
pub const hci_return_command_type_HCI_RETURN_COMMAND_TYPE_STATUS: hci_return_command_type = 1;
#[doc = " @brief Enumeration holding the types of the returned commands in response\n \t\t  to a received HCI command."]
pub type hci_return_command_type = ::core::ffi::c_uint;
pub type hci_trnsprt_cbk =
    ::core::option::Option<unsafe extern "C" fn(ptr_evnt_hdr: *mut ble_buff_hdr_t) -> u8>;
#[doc = " @ingroup  ext_hci_cmds External HCI Commands\n @{"]
pub type ble_ext_custm_cb_t = ::core::option::Option<
    unsafe extern "C" fn(
        ocf: u16,
        pckt_p: *mut u8,
        evnt_pckt_p: *mut u8,
        params_length: *mut u8,
        return_command_type: *mut hci_return_command_type,
    ) -> ble_stat_t,
>;
unsafe extern "C" {
    #[doc = " @brief Initialize the HCI layer and Registers a callback function to the upper layer\n\n @param p_trnsprt_cbk : [in] callback function\n\n @retval always returns SUCCESS"]
    pub fn ll_hci_init(p_trnsprt_cbk: hci_trnsprt_cbk) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  get a pointer to the HCI dispatch table.\n\n @param  p_p_dispatch_tbl : [out] pointer to be filled by the address of the HCI dispatch table."]
    #[link_name = "HCI_GET_DIS_TBL"]
    pub fn hci_get_dis_tbl(p_p_dispatch_tbl: *mut *const hci_dispatch_tbl);
}
unsafe extern "C" {
    #[doc = " @brief  allocate a message structure to be sent to the HCI layer through event manager.\n\n @retval ble_buff_hdr_t* : pointer to the buffer header allocated."]
    #[link_name = "HCI_ALLOC_MSG"]
    pub fn hci_alloc_msg() -> *mut ble_buff_hdr_t;
}
unsafe extern "C" {
    #[doc = " @brief  free an allocated message structure\n\n @param  ptr_hci_msg  : [in] Pointer to the message to be freed."]
    #[link_name = "HCI_FREE_MSG"]
    pub fn hci_free_msg(ptr_hci_msg: *mut ble_buff_hdr_t);
}
unsafe extern "C" {
    #[doc = " @ingroup  ext_hci_cmds External HCI Commands\n @{\n/\n/**\n @brief Registers a custom callback function to be called when the\n \t\t  hci_cstm_pckt_hndlr() cannot resolve the OCF.\n\n @param ext_custm_cbk :[IN] Pointer to the callback function.\n\n @retval False if the cbk is null, True otherwise."]
    #[link_name = "HCI_RGSTR_BLE_EXTERNAL_CUSTOM_CBK"]
    pub fn hci_rgstr_ble_external_custom_cbk(ext_custm_cbk: ble_ext_custm_cb_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Initialize event manager data/events queue call backs\n\n @retval ble_stat_t  None"]
    #[link_name = "HCI_INIT_EVENTS_QUEUES"]
    pub fn hci_init_events_queues();
}
unsafe extern "C" {
    #[doc = " @brief  Post data/event to appropriate queue. This function is called\n \t\t   in case of sending data/events to host\n\n @param  ptr_evnt_hdr\tdata/event ble_buff_hdr to send\n\n @retval ble_stat_t  None"]
    #[link_name = "HCI_QUEUE_SEND_PCKT"]
    pub fn hci_queue_send_pckt(ptr_evnt_hdr: *mut ble_buff_hdr_t) -> u8;
}
unsafe extern "C" {
    #[link_name = "HCI_RGSTR_HST_CBK"]
    pub fn hci_rgstr_hst_cbk(cbk: hst_cbk);
}
unsafe extern "C" {
    #[link_name = "HCI_RGSTR_HST_CBK_LL_QUEUE_FULL"]
    pub fn hci_rgstr_hst_cbk_ll_queue_full(cbk: hst_cbk_queue_full);
}
unsafe extern "C" {
    #[link_name = "HCI_LL_SET_LE_EVENT_MASK"]
    pub fn hci_ll_set_le_event_mask(event_mask: *mut u8);
}
unsafe extern "C" {
    #[link_name = "HCI_LL_SET_EVENT_MASK"]
    pub fn hci_ll_set_event_mask(event_mask: *mut u8);
}
unsafe extern "C" {
    #[link_name = "HCI_LL_SET_EVENT_MASK_PAGE2"]
    pub fn hci_ll_set_event_mask_page2(event_mask: *mut u8);
}
unsafe extern "C" {
    #[link_name = "HCI_LL_SET_CUSTOM_EVENT_MASK"]
    pub fn hci_ll_set_custom_event_mask(cstm_evnt_mask: u8);
}
pub const _HW_ERROR_CODES_HW_ERROR_CODE_UNDEFINED_ERROR: _HW_ERROR_CODES = 0;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_BL_SEARCH: _HW_ERROR_CODES = 1;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_RL_SEARCH: _HW_ERROR_CODES = 2;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_WL_SEARCH: _HW_ERROR_CODES = 3;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_PDU_PARSING: _HW_ERROR_CODES = 4;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_PDU_RECEIVING: _HW_ERROR_CODES = 5;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_SM_UPDATE: _HW_ERROR_CODES = 6;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_UNREGISTERING_EVENT: _HW_ERROR_CODES = 7;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_STARTING_CONNECTION: _HW_ERROR_CODES = 8;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_MEMORY_EXCEEDED: _HW_ERROR_CODES = 9;
pub const _HW_ERROR_CODES_HW_ERROR_CODE_UNEXPECTED_ADV_STATE: _HW_ERROR_CODES = 10;
pub type _HW_ERROR_CODES = ::core::ffi::c_uint;
pub use self::_HW_ERROR_CODES as HW_ERROR_CODES;
unsafe extern "C" {
    #[doc = " @ingroup  ll_common_interface_functions\n  @{\n/\n/**\n @brief Configure the LL contexts, where:\n \t\t\t  1. For bare-metal:\n \t\t\t    - High ISR is executed in the ISR context\n \t\t\t    - Low ISR can be executed in the high ISR context, or switched to low ISR context\n \t\t\t  2. For RTOS:\n \t\t\t    - High ISR is executed in the ISR context\n \t\t\t    - Low ISR is executed in the thread of the \"linkLayerHighPrioTask\"\n\n @param[in] allow_low_isr         : Configuration parameter for the context of the low ISR in the bare-metal model. Range is [0,1].\n \t\t\t\t\t\t\t\t                    0: Low ISR code is executed in the same context of the high ISR.\n \t\t\t\t\t\t\t\t                    1: Low ISR code is executed in the context of the low ISR (by configuring a low priority interrupt that is triggered by FW).\n @param[in] run_post_evnt_frm_isr : Configuration parameter to decide whether the scheduling of the next BLE event is done in the low ISR context or to be handled by the LL main thread. Range is [0,1].\n \t\t\t\t\t\t\t\t                    0: BLE next event scheduling is handled in the LL main thread.\n \t\t\t\t\t\t\t\t                    1: BLE next event scheduling is handled in the low ISR context.\n @return Command status"]
    pub fn ll_intf_cmn_config_ll_ctx_params(
        allow_low_isr: u8,
        run_post_evnt_frm_isr: u8,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Select the source that drives the sleep clock, whether to use an external crystal oscillator or an integrated RC oscillator (RCO).\n\n @param[in] slp_clk_src \t\t\t\t    : Indicate which source to drive the sleep clock. 0: Crystal Oscillator (default). 1: RC0\n @param[out] ptr_slp_clk_freq_value : Indicate the nominal frequency value of the sleep clock.\n @return Command status to be sent to the Host."]
    pub fn ll_intf_cmn_le_select_slp_clk_src(
        slp_clk_src: u8,
        ptr_slp_clk_freq_value: *mut u16,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Configure the runtime RCO calibration event parameters.\n\n @param[in] rco_clbr_event_duration : Indicate the number of sleep clock cycles for performing the RCO calibration process.\n @param[in] rco_clbr_event_interval : Indicate the periodicity of running the runtime RCO calibration event.\n @return Command status to be sent to the Host."]
    pub fn ll_intf_cmn_le_set_rco_clbr_evnt_params(
        rco_clbr_event_duration: u8,
        rco_clbr_event_interval: u32,
    ) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief Specify the used power table and its size based on the selected TX_Power table ID.\n\n @param[in] tx_power_table_id : Selected TX_Power table ID.\n @return Status \t\t : 0: SUCCESS. Otherwise: Error code."]
    pub fn ll_intf_cmn_select_tx_power_table(tx_power_table_id: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief flag to the LL the existence of a temperature sensor\n"]
    pub fn ll_intf_cmn_set_temperature_sensor_state();
}
unsafe extern "C" {
    #[doc = " @brief Set the current temperature\n\n @param[in] temperature :\tThe current temperature\n @return Command status"]
    pub fn ll_intf_cmn_set_temperature_value(temperature: u32) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Request new random number.\n\n @param[out] ptr_rnd\t: Pointer to the output random bytes .\n @param[in] len\t \t  : Number of required random bytes.\n\n @return Status."]
    pub fn ll_intf_cmn_gen_rnd_num(ptr_rnd: *mut u8, len: u32) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief A common wrapper for BLE-ECB and MAC-CCM security modes\n\n @param[in]\tptr_pckt\t\t  : Pointer to the data buffer (variable length in case of CCM mode, 16 bytes in case of ECB mode).\n                            The resulting Encrypted/Decrypted data overwrites this buffer.\n @param[in]\tptr_key\t\t    : Pointer to the security key buffer (16 bytes).\n @param[in]\tptr_nonce\t    : Pointer to the security nonce buffer (13 bytes in case of CCM mode, a Null pointer in case of ECB mode).\n @param[in]\tmic_len\t\t    : Length of MIC, supported values are 0, 4, 6, 8, 10, 12, 14, and 16 in case of CCM, 0 only in case of ECB.\n @param[in]\tad_len\t\t    : Length of Data to be authenticated.\n @param[in]\tmd_len\t\t    : Length of Data to be encrypted.\n @param[in]\tkey_endian\t  : Represents the format of the security key.\n @param[in]\tdata_endian\t  : Represents the endian format of the data.\n @param[in]\tsecurity_mode : Hardware security mode.\n @return Status"]
    pub fn ll_intf_cmn_crypto(
        ptr_pckt: *mut u8,
        ptr_key: *const u8,
        ptr_nonce: *mut u8,
        mic_len: u32,
        ad_len: u32,
        md_len: u32,
        key_endian: crypto_endian_enum_t,
        data_endian: crypto_endian_enum_t,
        security_mode: security_mode_enum_t,
    ) -> u32;
}
unsafe extern "C" {
    #[doc = " @brief Switches the controller to and from Deep Sleep mode.\n\n @param[in]  dp_slp_mode  : Input according to dpslp_state_e.\n                            DEEP_SLEEP_ENABLE  -> Activate Deep Sleep.\n                            DEEP_SLEEP_DISABLE -> Deactivate Deep Sleep.\n @return Status."]
    pub fn ll_intf_cmn_le_set_dp_slp_mode(dp_slp_mode: u8) -> ble_stat_t;
}
unsafe extern "C" {
    #[doc = " @brief  Set PHY calibration period.\n\n @param[in] phy_clbr_evnt_period  : Indicate the periodicity of the PHY calibration event. Periodicity = phy_clbr_evnt_period * 1s.\n @param[in] phy_clbr_evnt_count \t: Indicate the number of the PHY calibration events to be executed."]
    pub fn ll_intf_cmn_set_phy_clbr_period(phy_clbr_evnt_period: u32, phy_clbr_evnt_count: u32);
}
pub type epa_enable_cb_t = ::core::option::Option<unsafe extern "C" fn(epa_enable: u8) -> u8>;
#[doc = " @brief Structure of one element of the power table."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct power_table_entry {
    pub vddh_pa: u8,
    pub internal_pa_code: u8,
    pub epa_bypass: u8,
    pub tx_pwr: i8,
}
#[doc = " @brief Structure of the power table ID."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _power_table_id_t {
    pub ptr_tx_power_table: *const power_table_entry,
    pub tx_power_levels_count: u8,
    pub g_vdd_ldo_value_1: u8,
    pub g_vdd_ldo_value_2: u8,
    pub power_table_id: u8,
}
impl Default for _power_table_id_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Structure of the power table ID."]
pub type power_table_id_t = _power_table_id_t;
unsafe extern "C" {
    #[link_name = "\u{1}ll_tx_power_tables"]
    pub static LL_TX_POWER_TABLES: [power_table_id_t; 0usize];
}
unsafe extern "C" {
    #[link_name = "\u{1}num_of_supported_power_tables"]
    pub static NUM_OF_SUPPORTED_POWER_TABLES: u8;
}
unsafe extern "C" {
    #[doc = " @ingroup power_control_functions\n @{\n/\n/**\n @brief\tUsed to initialize the EPA parameters. Called during the initialization.\n \t\t\tNote: If an EPA is in use, then a valid callback function shall be passed to LL FW, otherwise the LL FW returns error status.\n\n @param  use_epa : [in] 1: External PA exists. 0: There is no External PA.\n @param  cbk\t   : [in] Callback function to actually enable and disable the EPA.\n\n @retval\tStatus : FW returns an error code, in case:\n \t\t\t\t\t 1. \"use_epa\" has wrong value\n \t\t\t\t\t 2. An EPA is in use and its enable callback function is set to NULL."]
    pub fn ll_tx_pwr_if_epa_init(use_epa: u8, cbk: epa_enable_cb_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief Used to specify the used power table and its size based on the selected power mode.\n\n @param  tx_power_mode : [in] Parameter indicating the selected TX_Power mode\n\n @retval Status \t\t : 0: SUCCESS. Otherwise: Error code."]
    pub fn ll_tx_pwr_if_select_tx_power_mode(tx_power_table_id: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Configure the necessary clock sources for the radio.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_ClockInit();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer active waiting loop.\n @param  delay: delay in us\n @retval None"]
    pub fn LINKLAYER_PLAT_DelayUs(delay: u32);
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer assertion API\n @param  condition: conditional statement to be checked.\n @retval None"]
    pub fn LINKLAYER_PLAT_Assert(condition: u8);
}
unsafe extern "C" {
    #[doc = " @brief  Enable/disable the Link Layer active clock (baseband clock).\n @param  enable: boolean value to enable (1) or disable (0) the clock.\n @retval None"]
    pub fn LINKLAYER_PLAT_AclkCtrl(enable: u8);
}
unsafe extern "C" {
    #[doc = " @brief  Notify the Link Layer platform layer the system will enter in WFI\n         and AHB5 clock may be turned of regarding the 2.4Ghz radio state.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_NotifyWFIEnter();
}
unsafe extern "C" {
    #[doc = " @brief  Notify the Link Layer platform layer the system exited WFI and AHB5\n         clock may be resynchronized as is may have been turned of during\n         low power mode entry.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_NotifyWFIExit();
}
unsafe extern "C" {
    #[doc = " @brief  Active wait on bus clock readiness.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_WaitHclkRdy();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer RNG request.\n @param  ptr_rnd: pointer to the variable that hosts the number.\n @param  len: number of byte of anthropy to get.\n @retval None"]
    pub fn LINKLAYER_PLAT_GetRNG(ptr_rnd: *mut u8, len: u32);
}
unsafe extern "C" {
    #[doc = " @brief  Initialize Link Layer radio high priority interrupt.\n @param  intr_cb: function pointer to assign for the radio high priority ISR routine.\n @retval None"]
    pub fn LINKLAYER_PLAT_SetupRadioIT(intr_cb: ::core::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    #[doc = " @brief  Initialize Link Layer SW low priority interrupt.\n @param  intr_cb: function pointer to assign for the SW low priority ISR routine.\n @retval None"]
    pub fn LINKLAYER_PLAT_SetupSwLowIT(intr_cb: ::core::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    #[doc = " @brief  Trigger the link layer SW low interrupt.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_TriggerSwLowIT(priority: u8);
}
unsafe extern "C" {
    #[doc = " @brief  Enable interrupts.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_EnableIRQ();
}
unsafe extern "C" {
    #[doc = " @brief  Disable interrupts.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_DisableIRQ();
}
unsafe extern "C" {
    #[doc = " @brief  Enable specific interrupt group.\n @param  isr_type: mask for interrupt group to enable.\n         This parameter can be one of the following:\n         @arg LL_HIGH_ISR_ONLY: enable link layer high priority ISR.\n         @arg LL_LOW_ISR_ONLY: enable link layer SW low priority ISR.\n         @arg SYS_LOW_ISR: unmask interrupts for all the other system ISR with\n              lower priority that link layer SW low interrupt.\n @retval None"]
    pub fn LINKLAYER_PLAT_EnableSpecificIRQ(isr_type: u8);
}
unsafe extern "C" {
    #[doc = " @brief  Disable specific interrupt group.\n @param  isr_type: mask for interrupt group to disable.\n         This parameter can be one of the following:\n         @arg LL_HIGH_ISR_ONLY: disable link layer high priority ISR.\n         @arg LL_LOW_ISR_ONLY: disable link layer SW low priority ISR.\n         @arg SYS_LOW_ISR: mask interrupts for all the other system ISR with\n              lower priority that link layer SW low interrupt.\n @retval None"]
    pub fn LINKLAYER_PLAT_DisableSpecificIRQ(isr_type: u8);
}
unsafe extern "C" {
    #[doc = " @brief  Enable link layer high priority ISR only.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_EnableRadioIT();
}
unsafe extern "C" {
    #[doc = " @brief  Disable link layer high priority ISR only.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_DisableRadioIT();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer notification for radio activity start.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_StartRadioEvt();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer notification for radio activity end.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_StopRadioEvt();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer notification for RCO calibration start.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_RCOStartClbr();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer notification for RCO calibration end.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_RCOStopClbr();
}
unsafe extern "C" {
    #[doc = " @brief  Link Layer requests temperature.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_RequestTemperature();
}
unsafe extern "C" {
    #[doc = " @brief  Enable RTOS context switch.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_PhyStartClbr();
}
unsafe extern "C" {
    #[doc = " @brief  Disable RTOS context switch.\n @param  None\n @retval None"]
    pub fn LINKLAYER_PLAT_PhyStopClbr();
}
unsafe extern "C" {
    #[doc = " @brief Notify the upper layer that new Link Layer timings have been applied.\n @param evnt_timing[in]: Evnt_timing_t pointer to structure contains drift time , execution time and scheduling time\n @retval None."]
    pub fn LINKLAYER_PLAT_SCHLDR_TIMING_UPDATE_NOT(p_evnt_timing: *mut Evnt_timing_t);
}
unsafe extern "C" {
    #[doc = " @brief  Get the ST company ID.\n @param  None\n @retval Company ID"]
    pub fn LINKLAYER_PLAT_GetSTCompanyID() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief  Get the Unique Device Number (UDN).\n @param  None\n @retval UDN"]
    pub fn LINKLAYER_PLAT_GetUDN() -> u32;
}
unsafe extern "C" {
    pub fn ll_sys_sequencer_bg_process_init();
}
unsafe extern "C" {
    pub fn ll_sys_sequencer_schedule_bg_process();
}
unsafe extern "C" {
    pub fn ll_sys_ble_cntrl_init(hostCallback: hst_cbk);
}
unsafe extern "C" {
    pub fn ll_sys_mac_cntrl_init();
}
unsafe extern "C" {
    pub fn ll_sys_thread_init();
}
unsafe extern "C" {
    pub fn ll_sys_handle_missed_event_cb(length: u16, data: *mut u8);
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32 {
    pub v: u32,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT16_WRITE {
    pub v: u16,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT16_READ {
    pub v: u16,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32_WRITE {
    pub v: u32,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32_READ {
    pub v: u32,
}
pub const ll_sys_status_t_LL_SYS_OK: ll_sys_status_t = 0;
pub const ll_sys_status_t_LL_SYS_ERROR: ll_sys_status_t = 1;
pub const ll_sys_status_t_LL_SYS_BUSY: ll_sys_status_t = 2;
#[doc = " @brief  Link Layer Status structure definition"]
pub type ll_sys_status_t = ::core::ffi::c_uint;
pub const ll_sys_radio_hclk_client_t_LL_SYS_RADIO_HCLK_RADIO_ISR: ll_sys_radio_hclk_client_t = 0;
pub const ll_sys_radio_hclk_client_t_LL_SYS_RADIO_HCLK_LL_EVT: ll_sys_radio_hclk_client_t = 1;
pub const ll_sys_radio_hclk_client_t_LL_SYS_RADIO_HCLK_LL_BG: ll_sys_radio_hclk_client_t = 2;
pub const ll_sys_radio_hclk_client_t_LL_SYS_RADIO_HCLK_BLEHOST_BG: ll_sys_radio_hclk_client_t = 3;
pub const ll_sys_radio_hclk_client_t_LL_SYS_RADIO_HCLK_PREIDLE: ll_sys_radio_hclk_client_t = 4;
#[doc = " @brief  Link Layer radio bus clock clients definition"]
pub type ll_sys_radio_hclk_client_t = ::core::ffi::c_uint;
pub const ll_sys_dp_slp_state_t_LL_SYS_DP_SLP_DISABLED: ll_sys_dp_slp_state_t = 0;
pub const ll_sys_dp_slp_state_t_LL_SYS_DP_SLP_ENABLED: ll_sys_dp_slp_state_t = 1;
#[doc = " @brief  Link Layer deep sleep state"]
pub type ll_sys_dp_slp_state_t = ::core::ffi::c_uint;
unsafe extern "C" {
    pub fn ll_sys_init();
}
unsafe extern "C" {
    pub fn ll_sys_reset();
}
unsafe extern "C" {
    pub fn ll_sys_delay_us(delay: u32);
}
unsafe extern "C" {
    pub fn ll_sys_assert(condition: u8);
}
unsafe extern "C" {
    pub fn ll_sys_get_rng(ptr_rnd: *mut u8, len: u32);
}
unsafe extern "C" {
    pub fn ll_sys_radio_ack_ctrl(enable: u8);
}
unsafe extern "C" {
    pub fn ll_sys_radio_wait_for_busclkrdy();
}
unsafe extern "C" {
    pub fn ll_sys_setup_radio_intr(intr_cb: ::core::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn ll_sys_setup_radio_sw_low_intr(intr_cb: ::core::option::Option<unsafe extern "C" fn()>);
}
unsafe extern "C" {
    pub fn ll_sys_radio_sw_low_intr_trigger(priority: u8);
}
unsafe extern "C" {
    pub fn ll_sys_radio_evt_not(start: u8);
}
unsafe extern "C" {
    pub fn ll_sys_rco_clbr_not(start: u8);
}
unsafe extern "C" {
    pub fn ll_sys_request_temperature();
}
unsafe extern "C" {
    pub fn ll_sys_schldr_timing_update_not(p_evnt_timing: *mut Evnt_timing_t);
}
unsafe extern "C" {
    pub fn ll_intf_is_ptr_in_ble_mem(inp_ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn HostStack_Process();
}
unsafe extern "C" {
    pub fn ll_sys_bg_process();
}
unsafe extern "C" {
    pub fn ll_sys_bg_process_init();
}
unsafe extern "C" {
    pub fn ll_sys_schedule_bg_process();
}
unsafe extern "C" {
    pub fn ll_sys_schedule_bg_process_isr();
}
unsafe extern "C" {
    pub fn ll_sys_config_params();
}
unsafe extern "C" {
    pub fn ll_sys_enable_irq();
}
unsafe extern "C" {
    pub fn ll_sys_disable_irq();
}
unsafe extern "C" {
    pub fn ll_sys_enable_specific_irq(isr_type: u8);
}
unsafe extern "C" {
    pub fn ll_sys_disable_specific_irq(isr_type: u8);
}
unsafe extern "C" {
    pub fn ll_sys_phy_start_clbr();
}
unsafe extern "C" {
    pub fn ll_sys_phy_stop_clbr();
}
unsafe extern "C" {
    pub fn ll_sys_dp_slp_init() -> ll_sys_status_t;
}
unsafe extern "C" {
    pub fn ll_sys_dp_slp_enter(dp_slp_duration: u32) -> ll_sys_status_t;
}
unsafe extern "C" {
    pub fn ll_sys_dp_slp_exit() -> ll_sys_status_t;
}
unsafe extern "C" {
    pub fn ll_sys_dp_slp_get_state() -> ll_sys_dp_slp_state_t;
}
unsafe extern "C" {
    pub fn ll_sys_dp_slp_wakeup_evt_clbk(ptr_arg: *const ::core::ffi::c_void);
}
unsafe extern "C" {
    #[doc = " @brief  Get the number of concurrent state machines for the Link Layer\n @param  None\n @retval Supported number of concurrent state machines"]
    pub fn ll_sys_get_concurrent_state_machines_num() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Updating Link Layer BLE timings\n @param  drift_time[in]: number of Link Layer sleep timer cycles (1 cycle = 31us) for the DRIFT TIME timing.\n @param  exec_time[in]: number of Link Layer sleep timer cycles (1 cycle = 31us)  for the EXEC TIME timing.\n @note   This interface needs to be called after system initialization\n         and before starting any radio activity.\n @retval uint32_t : Effective EXEC_Time value computed from the exec_time value profiled and given in parameter."]
    pub fn ll_sys_config_BLE_schldr_timings(drift_time: u8, exec_time: u8) -> u32;
}
unsafe extern "C" {
    pub fn ll_intf_cmn_get_slptmr_value() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief Get the brief Link Layer firmware version\n\n @details This function returns the brief firmware version of the Link Layer.\n          The version is represented as a single byte and follow this organization:\n          - Bits 7-6: Major version\n          - Bits 5-2: Minor version\n          - Bits 1-0: Patch version"]
    pub fn ll_sys_get_brief_fw_version() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief Get the system firmware version\n\n @return Short hash of the system firmware commit."]
    pub fn ll_sys_get_system_fw_version() -> u32;
}
unsafe extern "C" {
    #[doc = " @brief Get the source firmware version\n\n @return Short hash of the source firmware commit."]
    pub fn ll_sys_get_source_fw_version() -> u32;
}
