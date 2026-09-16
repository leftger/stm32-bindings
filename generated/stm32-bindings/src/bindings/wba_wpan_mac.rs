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
pub const G_MAX_PAN_DESC_SUPPORTED_C: u32 = 6;
pub const G_MAX_SOUNDING_LIST_SUPPORTED_C: u32 = 1;
pub const G_MAX_PENDING_ADDRESS_C: u32 = 1;
pub const G_MAX_ED_SCAN_RESULTS_SUPPORTED_C: u32 = 16;
pub const AMAXSIFSFRAMESIZE: u32 = 18;
pub const MAX_HDR_IE_CONTENT: u32 = 6;
pub const MAX_PYLD_IE_CONTENT: u32 = 28;
pub const NB_HDR_IES: u32 = 3;
pub const NB_PYLD_IES: u32 = 2;
pub const MASK_HDR_IE_SIZE: u32 = 127;
pub const MASK_HDR_IE_ELEMENT_TYPE: u32 = 65408;
pub const MASK_PYLD_IE_SIZE: u32 = 2047;
pub const MASK_PYLD_IE_GROUP_TYPE: u32 = 63488;
pub const MASK_IE_TYPE: u32 = 32768;
pub const G_SIZE_LONG_INT_C: u32 = 4;
pub const NOBUFFER: u32 = 255;
pub const G_LONG_ADDRESS_LENGTH_C: u32 = 8;
pub const G_TX_OPTION_DIRECT_C: u32 = 0;
pub const G_TX_OPTION_ACK_C: u32 = 1;
pub const G_TX_OPTION_INDIRECT_C: u32 = 4;
pub const G_TX_OPTION_INDIRECT_WITH_ACK_C: u32 = 5;
pub const G_ARESPONSE_TIME_C: u32 = 491;
pub const G_SWITCH_OFF_RECIEVER_C: u32 = 0;
pub const G_SWITCH_ON_RECIEVER_C: u32 = 1;
pub const G_TRUE: u32 = 1;
pub const G_FALSE: u32 = 0;
pub const ST_PIB_REVISION: u32 = 16777472;
pub const G_NULL_C: u32 = 0;
pub const G_RESET_C: u32 = 0;
pub const G_EXTENDED_ADDRESS_LENGTH_C: u32 = 8;
pub const G_SHORT_ADDRESS_LENGTH_C: u32 = 2;
pub const G_SHORT_PAN_ID_LENGTH_C: u32 = 2;
pub const G_EIGHT_BYTE_LENGTH_C: u32 = 8;
pub const G_BROADCAST_PAN_ID_C: u32 = 65535;
pub const G_BROADCAST_ADDRESS_C: u32 = 65535;
pub const G_INVALID_ADDRESS_C: u32 = 65535;
pub const G_INVALID_PAN_ID_C: u32 = 65535;
pub const G_INVALID_INDEX_C: u32 = 255;
pub const G_INVALID_VALUE_C: u32 = 255;
pub const G_ADDRESS_NOT_ALLOCATED_C: u32 = 65534;
pub const G_IDX_TO_CMD_ID_C: u32 = 0;
pub const NB_PAN_BEACON_RECEIVED: u32 = 10;
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
pub const SUPPORT_OPENTHREAD_1_2: u32 = 1;
pub const SUPPORT_SEC: u32 = 1;
pub const RADIO_CSMA: u32 = 1;
pub const ENHANCED_RX_WHILE_CSMA_BACKOFF_DELAY: u32 = 1;
pub const SUPPORT_ANT_DIV: u32 = 1;
pub const SUPPORT_A_MAC: u32 = 1;
pub const SUPPORT_CONFIG_LIB: u32 = 1;
pub const SMPL_PRTCL_TEST_ENABLE: u32 = 0;
pub const IEEE_EUI64_VENDOR_SPECIFIC_FUNC: u32 = 1;
pub const SUPPORT_ZIGBEE_PHY_CERTIFICATION: u32 = 0;
pub const POOL_BLOCK_SIZE: u32 = 16;
pub const POOL_TOTAL_BLOCKS_SIZE: u32 = 10;
pub const POOL_INDEX_SIZE: u32 = 6;
pub const SUPPORT_BLE: u32 = 0;
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
pub const SUPPORT_HCI_EVENT_ONLY: u32 = 0;
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
pub const MAC_CMD_MSG_ID_OFFSET: u32 = 0;
pub const MAC_CMD_MSG_HANDLE_ID_OFFSET: u32 = 1;
pub const MAC_CMD_MSG_STRUCT_LEN_OFFSET: u32 = 2;
pub const MAC_CMD_MSG_STUFF_OFFSET: u32 = 3;
pub const MAC_CMD_MSG_HEADER_OFFSET: u32 = 4;
pub const M_DEFAULT_PHY_CHANNELS_SUPPORTED_C: u32 = 134215680;
pub const QUEUE_MAX_LEN: u32 = 10;
pub const CCA_THRESHOLD: i32 = -75;
pub const ST_MAC_MAX_TX_POWER: u32 = 10;
pub const ST_MAC_MIN_TX_POWER: i32 = -20;
pub const ST_MAC_ASSOCIATION_REQ_INT_CMD_ID: u32 = 0;
pub const ST_MAC_DISASSOCIATION_REQ_INT_CMD_ID: u32 = 1;
pub const ST_MAC_GET_REQ_INT_CMD_ID: u32 = 2;
pub const ST_MAC_RESET_REQ_INT_CMD_ID: u32 = 3;
pub const ST_MAC_RX_ENABLE_REQ_INT_CMD_ID: u32 = 4;
pub const ST_MAC_SCAN_REQ_INT_CMD_ID: u32 = 5;
pub const ST_MAC_SET_REQ_INT_CMD_ID: u32 = 6;
pub const ST_MAC_START_REQ_INT_CMD_ID: u32 = 7;
pub const ST_MAC_POLL_REQ_INT_CMD_ID: u32 = 8;
pub const ST_MAC_ASSOCIATE_RES_INT_CMD_ID: u32 = 9;
pub const ST_MAC_ORPHAN_RES_INT_CMD_ID: u32 = 10;
pub const ST_MAC_DATA_REQ_INT_CMD_ID: u32 = 11;
pub const ST_MAC_PURGE_REQ_INT_CMD_ID: u32 = 12;
pub const ST_MAC_BEACON_REQ_INT_CMD_ID: u32 = 13;
pub const ST_MAC_GET_PWR_INFO_TABLE_REQ_INT_CMD_ID: u32 = 14;
pub const ST_MAC_SET_PWR_INFO_TABLE_REQ_INT_CMD_ID: u32 = 15;
pub const ST_MAC_MAX_INT_IN_CMD_ID: u32 = 16;
pub const ST_MAC_RESET_CNF_INT_MSG_ID: u32 = 0;
pub const ST_MAC_SET_CNF_INT_MSG_ID: u32 = 1;
pub const ST_MAC_SCAN_CNF_INT_MSG_ID: u32 = 2;
pub const ST_MAC_BEACON_IND_INT_MSG_ID: u32 = 3;
pub const ST_MAC_START_CNF_INT_MSG_ID: u32 = 4;
pub const ST_MAC_ASSOCIATION_IND_INT_MSG_ID: u32 = 5;
pub const ST_MAC_GET_CNF_INT_MSG_ID: u32 = 6;
pub const ST_MAC_DATA_CNF_INT_MSG_ID: u32 = 7;
pub const ST_MAC_DATA_IND_INT_MSG_ID: u32 = 8;
pub const ST_MAC_ASSOCIATION_CNF_INT_MSG_ID: u32 = 9;
pub const ST_MAC_COM_STATUS_IND_INT_MSG_ID: u32 = 10;
pub const ST_MAC_POLL_CNF_INT_MSG_ID: u32 = 11;
pub const ST_MAC_POLL_IND_INT_MSG_ID: u32 = 12;
pub const ST_MAC_PURGE_CNF_INT_MSG_ID: u32 = 13;
pub const ST_MAC_ORPHAN_IND_INT_MSG_ID: u32 = 14;
pub const ST_MAC_RX_ENABLE_CNF_INT_CMD_ID: u32 = 15;
pub const ST_MAC_DISASSOCIATION_CNF_INT_CMD_ID: u32 = 16;
pub const ST_MAC_DISASSOCIATION_IND_INT_CMD_ID: u32 = 17;
pub const ST_MAC_BEACONREQ_IND_INT_CMD_ID: u32 = 18;
pub const ST_MAC_BEACON_CNF_INT_MSG_ID: u32 = 19;
pub const ST_MAC_GET_PWR_INFO_TABLE_CNF_INT_CMD_ID: u32 = 20;
pub const ST_MAC_SET_PWR_INFO_TABLE_CNF_INT_CMD_ID: u32 = 21;
pub const ST_MAC_SYNC_LOSS_IND_INT_MSG_ID: u32 = 22;
pub const MAX_SIZE_PIB: u32 = 127;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const USE_CLANG_STDDEF: u32 = 0;
pub const SEVERITY_LOW: u32 = 0;
pub const SEVERITY_HIGH: u32 = 1;
pub const ST_MAC_HANDLE_INCOMING_MAC_CMD: u32 = 0;
pub const ST_MAC_HANDLE_OUTGOING_MAC_MSG: u32 = 1;
pub const EXT_TX_US_FEATURE: u32 = 0;
pub const EXT_MIN_TX_LEN: u32 = 5;
pub const EXT_MAX_TX_LEN: u32 = 127;
pub const EXT_MIN_ACK_LEN: u32 = 5;
pub const EXT_MAX_ACK_LEN: u32 = 127;
pub const EXT_CRC_LEN: u32 = 2;
pub const TXFIFO_SIZE: u32 = 5;
pub const RXFIFO_SIZE: u32 = 5;
pub const EXT_MAX_CHANNEL: u32 = 40;
pub const EXT_SCAN_ALL_CHANNEL_KPBS: i64 = -281474976710656;
pub const EXT_SCAN_ALL_CHANNEL_MPBS: i32 = -16777216;
pub const ST_MAC_RAW_RX_PAYLOAD_MAX_SIZE: u32 = 125;
pub const ST_MAC_RAW_RX_FRAME_MAX_SIZE: u32 = 127;
pub const QUEUE_FALSE: u32 = 1;
pub const QUEUE_TRUE: u32 = 0;
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
pub const USE_CLANG_STDARG: u32 = 0;
pub const RENAME_SECLUDE: u32 = 1;
pub const RENAME_SWAP: u32 = 2;
pub const RENAME_EXCL: u32 = 4;
pub const RENAME_RESERVED1: u32 = 8;
pub const RENAME_NOFOLLOW_ANY: u32 = 16;
pub const RENAME_RESOLVE_BENEATH: u32 = 32;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_HOLE: u32 = 3;
pub const SEEK_DATA: u32 = 4;
pub const __SLBF: u32 = 1;
pub const __SNBF: u32 = 2;
pub const __SRD: u32 = 4;
pub const __SWR: u32 = 8;
pub const __SRW: u32 = 16;
pub const __SEOF: u32 = 32;
pub const __SERR: u32 = 64;
pub const __SMBF: u32 = 128;
pub const __SAPP: u32 = 256;
pub const __SSTR: u32 = 512;
pub const __SOPT: u32 = 1024;
pub const __SNPT: u32 = 2048;
pub const __SOFF: u32 = 4096;
pub const __SMOD: u32 = 8192;
pub const __SALC: u32 = 16384;
pub const __SIGN: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 1024;
pub const EOF: i32 = -1;
pub const FOPEN_MAX: u32 = 20;
pub const FILENAME_MAX: u32 = 1024;
pub const P_TMPDIR: &[u8; 10] = b"/var/tmp/\0";
pub const L_TMPNAM: u32 = 1024;
pub const TMP_MAX: u32 = 308915776;
pub const L_CTERMID: u32 = 1024;
pub const _USE_FORTIFY_LEVEL: u32 = 2;
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
pub const NULL: u32 = 0;
pub const CIRCULAR_QUEUE_NO_FLAG: u32 = 0;
pub const CIRCULAR_QUEUE_NO_WRAP_FLAG: u32 = 1;
pub const CIRCULAR_QUEUE_SPLIT_IF_WRAPPING_FLAG: u32 = 2;
pub const LOG_LEVEL_INFO: u32 = 0;
pub const CFG_LOG_SUPPORTED: u32 = 0;
pub const PWR_LDO_SUPPLY: u32 = 0;
pub const RADIO_INTR_NUM: u32 = 0;
pub const RADIO_INTR_PRIO_HIGH: u32 = 0;
pub const RADIO_INTR_PRIO_LOW: u32 = 0;
pub const RADIO_SW_LOW_INTR_NUM: u32 = 0;
pub const RADIO_SW_LOW_INTR_PRIO: u32 = 0;
pub const RCC_INTR_PRIO: u32 = 0;
pub const USE_RADIO_LOW_ISR: u32 = 1;
pub const NEXT_EVENT_SCHEDULING_FROM_ISR: u32 = 1;
pub const CFG_SCM_SUPPORTED: u32 = 1;
pub const CFG_DEBUGGER_LEVEL: u32 = 0;
pub const CFG_RT_DEBUG_GPIO_MODULE: u32 = 0;
pub const CFG_RT_DEBUG_DTB: u32 = 0;
pub const CFG_LPM_LEVEL: u32 = 0;
pub const CFG_LPM_STDBY_SUPPORTED: u32 = 0;
pub const CFG_LPM_WAKEUP_TIME_PROFILING: u32 = 1;
pub const CFG_LPM_STDBY_WAKEUP_TIME: u32 = 1500;
pub const CFG_CORE_SUPPLY: u32 = 0;
pub const CFG_LOG_INSERT_COLOR_INSIDE_THE_TRACE: u32 = 0;
pub const CFG_LOG_INSERT_TIME_STAMP_INSIDE_THE_TRACE: u32 = 0;
pub const CFG_LOG_INSERT_EOL_INSIDE_THE_TRACE: u32 = 0;
pub const APPLI_CONFIG_LOG_LEVEL: u32 = 0;
pub const APPLI_PRINT_FILE_FUNC_LINE: u32 = 0;
pub const RADIO_NUM_OF_ANTENNAS: u32 = 4;
pub const CFG_RF_TX_POWER_TABLE_ID: u32 = 0;
pub const CFG_HW_RNG_POOL_SIZE: u32 = 32;
pub const CFG_HW_RNG_POOL_THRESHOLD: u32 = 16;
pub const CFG_LED_SUPPORTED: u32 = 1;
pub const CFG_BUTTON_SUPPORTED: u32 = 1;
pub const MAC_RETRY_SCAN: u32 = 3;
pub const BASE_SCAN_DURATION: u32 = 5;
pub const BEACON_PAYLOAD: &[u8; 4] = b"BZH\0";
pub const BEACON_PAYLOAD_SIZE: u32 = 3;
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
#[doc = " -0x00 - No address mode specified"]
pub const Addr_Mode_Tag_g_NO_ADDR_MODE_c: Addr_Mode_Tag = 0;
#[doc = " -0x01 - Reserved"]
pub const Addr_Mode_Tag_g_RESERVED_MODE_c: Addr_Mode_Tag = 1;
#[doc = " -0x02 - Short address mode"]
pub const Addr_Mode_Tag_g_SHORT_ADDR_MODE_c: Addr_Mode_Tag = 2;
#[doc = " -0x03 - Long or extended address mode"]
pub const Addr_Mode_Tag_g_EXTENDED_ADDR_MODE_c: Addr_Mode_Tag = 3;
#[doc = " @brief Address Mode values"]
pub type Addr_Mode_Tag = ::core::ffi::c_uint;
#[doc = " @brief Address Mode values"]
pub use self::Addr_Mode_Tag as Addr_Mode_t;
#[doc = " -False is defined as 0x00"]
pub const Bool_Tag_g_FALSE_c: Bool_Tag = 0;
#[doc = " -True is defined as 0x01"]
pub const Bool_Tag_g_TRUE_c: Bool_Tag = 1;
#[doc = " @brief Boolean values for true and false"]
pub type Bool_Tag = ::core::ffi::c_uint;
#[doc = " @brief Boolean values for true and false"]
pub use self::Bool_Tag as Bool_t;
#[doc = " -MCPS Data Request"]
pub const MAC_Message_ID_Tag_g_MCPS_DATA_REQUEST_c: MAC_Message_ID_Tag = 64;
#[doc = " -MCPS Data Confirm"]
pub const MAC_Message_ID_Tag_g_MCPS_DATA_CONFIRM_c: MAC_Message_ID_Tag = 65;
#[doc = " -MCPS Data Indication"]
pub const MAC_Message_ID_Tag_g_MCPS_DATA_INDICATION_c: MAC_Message_ID_Tag = 66;
#[doc = " -MCPS Purge request"]
pub const MAC_Message_ID_Tag_g_MCPS_PURGE_REQUEST_c: MAC_Message_ID_Tag = 67;
#[doc = " -MCPS Purge Confirm"]
pub const MAC_Message_ID_Tag_g_MCPS_PURGE_CONFIRM_c: MAC_Message_ID_Tag = 68;
#[doc = " -MLME Associate Request"]
pub const MAC_Message_ID_Tag_g_MLME_ASSOCIATE_REQUEST_c: MAC_Message_ID_Tag = 69;
#[doc = " -MLME Associate Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_ASSOCIATE_CONFIRM_c: MAC_Message_ID_Tag = 70;
#[doc = " -MLME Associate Indication"]
pub const MAC_Message_ID_Tag_g_MLME_ASSOCIATE_INDICATION_c: MAC_Message_ID_Tag = 71;
#[doc = " -MLME Associate Response"]
pub const MAC_Message_ID_Tag_g_MLME_ASSOCIATE_RESPONSE_c: MAC_Message_ID_Tag = 72;
#[doc = " -MLME Disassociate Request"]
pub const MAC_Message_ID_Tag_g_MLME_DISASSOCIATE_REQUEST_c: MAC_Message_ID_Tag = 73;
#[doc = " -MLME Disassociate Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_DISASSOCIATE_CONFIRM_c: MAC_Message_ID_Tag = 74;
#[doc = " -MLME Disassociate Indication"]
pub const MAC_Message_ID_Tag_g_MLME_DISASSOCIATE_INDICATION_c: MAC_Message_ID_Tag = 75;
#[doc = " -MLME Beacon Notify Indication"]
pub const MAC_Message_ID_Tag_g_MLME_BEACON_NOTIFY_INDICATION_c: MAC_Message_ID_Tag = 76;
#[doc = " -MLME Get Request"]
pub const MAC_Message_ID_Tag_g_MLME_GET_REQUEST_c: MAC_Message_ID_Tag = 77;
#[doc = " -MLME Get Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_GET_CONFIRM_c: MAC_Message_ID_Tag = 78;
#[doc = " -MLME Orphan Indication"]
pub const MAC_Message_ID_Tag_g_MLME_ORPHAN_INDICATION_c: MAC_Message_ID_Tag = 79;
#[doc = " -MLME Orphan Response"]
pub const MAC_Message_ID_Tag_g_MLME_ORPHAN_RESPONSE_c: MAC_Message_ID_Tag = 80;
#[doc = " -MLME Reset Request"]
pub const MAC_Message_ID_Tag_g_MLME_RESET_REQUEST_c: MAC_Message_ID_Tag = 81;
#[doc = " -MLME Reset Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_RESET_CONFIRM_c: MAC_Message_ID_Tag = 82;
#[doc = " -MLME RX Enable Request"]
pub const MAC_Message_ID_Tag_g_MLME_RX_ENABLE_REQUEST_c: MAC_Message_ID_Tag = 83;
#[doc = " -MLME RX Enable Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_RX_ENABLE_CONFIRM_c: MAC_Message_ID_Tag = 84;
#[doc = " -MLME Scan Request"]
pub const MAC_Message_ID_Tag_g_MLME_SCAN_REQUEST_c: MAC_Message_ID_Tag = 85;
#[doc = " -MLME Scan Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_SCAN_CONFIRM_c: MAC_Message_ID_Tag = 86;
#[doc = " -MLME Comm Status Indication"]
pub const MAC_Message_ID_Tag_g_MLME_COMM_STATUS_INDICATION_c: MAC_Message_ID_Tag = 87;
#[doc = " -MLME Set Request"]
pub const MAC_Message_ID_Tag_g_MLME_SET_REQUEST_c: MAC_Message_ID_Tag = 88;
#[doc = " -MLME Set Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_SET_CONFIRM_c: MAC_Message_ID_Tag = 89;
#[doc = " -MLME Start Request"]
pub const MAC_Message_ID_Tag_g_MLME_START_REQUEST_c: MAC_Message_ID_Tag = 90;
#[doc = " -MLME Start Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_START_CONFIRM_c: MAC_Message_ID_Tag = 91;
#[doc = " -MLME Sync Loss Indication"]
pub const MAC_Message_ID_Tag_g_MLME_SYNC_LOSS_INDICATION_c: MAC_Message_ID_Tag = 92;
#[doc = " -MLME Poll Request"]
pub const MAC_Message_ID_Tag_g_MLME_POLL_REQUEST_c: MAC_Message_ID_Tag = 93;
#[doc = " -MLME Poll Confirm"]
pub const MAC_Message_ID_Tag_g_MLME_POLL_CONFIRM_c: MAC_Message_ID_Tag = 94;
#[doc = " -MLME Poll Indication"]
pub const MAC_Message_ID_Tag_g_MLME_POLL_INDICATION_c: MAC_Message_ID_Tag = 95;
#[doc = " @brief   Message ID of MAC request/response/indication/confirmation"]
pub type MAC_Message_ID_Tag = ::core::ffi::c_uint;
#[doc = " @brief   Message ID of MAC request/response/indication/confirmation"]
pub use self::MAC_Message_ID_Tag as MAC_Message_ID_t;
#[doc = " -Acknowledged Transmission"]
pub const MAC_Tx_Options_Tag_g_ACK_TX_c: MAC_Tx_Options_Tag = 1;
#[doc = " -GTS Transmission"]
pub const MAC_Tx_Options_Tag_g_GTS_TX_c: MAC_Tx_Options_Tag = 2;
#[doc = " -Indirect Transmission"]
pub const MAC_Tx_Options_Tag_g_INDIRECT_TX_c: MAC_Tx_Options_Tag = 4;
#[doc = " -Security Enabled Transmission"]
pub const MAC_Tx_Options_Tag_g_SECURED_TX_c: MAC_Tx_Options_Tag = 8;
#[doc = " @brief MAC Transmission Options"]
pub type MAC_Tx_Options_Tag = ::core::ffi::c_uint;
#[doc = " @brief MAC Transmission Options"]
pub use self::MAC_Tx_Options_Tag as MAC_Tx_Options_t;
#[doc = " -Association successful"]
pub const MAC_Association_Status_Tag_g_MAC_ASSO_SUCCESS_c: MAC_Association_Status_Tag = 0;
#[doc = " -PAN at capacity"]
pub const MAC_Association_Status_Tag_g_MAC_PAN_AT_CAPACITY_c: MAC_Association_Status_Tag = 1;
#[doc = " -PAN access denied"]
pub const MAC_Association_Status_Tag_g_MAC_PAN_ACCESS_DENIED_c: MAC_Association_Status_Tag = 2;
#[doc = "/\n/** @brief MAC Association Status"]
pub type MAC_Association_Status_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief MAC Association Status"]
pub use self::MAC_Association_Status_Tag as MAC_Association_Status_t;
#[doc = " -The coordinator wishes the device to leave the PAN."]
pub const MAC_Disassociation_Reason_Tag_g_COORD_REQUESTED_c: MAC_Disassociation_Reason_Tag = 1;
#[doc = " -The device wishes to leave the PAN."]
pub const MAC_Disassociation_Reason_Tag_g_DEVICE_REQUESTED_c: MAC_Disassociation_Reason_Tag = 2;
#[doc = "/\n/** @brief MAC Disassociation reason codes"]
pub type MAC_Disassociation_Reason_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief MAC Disassociation reason codes"]
pub use self::MAC_Disassociation_Reason_Tag as MAC_Disassociation_Reason_t;
#[doc = " - MAC Unsecured Mode Security"]
pub const MAC_Security_Mode_Tag_g_MAC_UNSECURED_MODE_c: MAC_Security_Mode_Tag = 0;
#[doc = " - MAC ACL Mode Security"]
pub const MAC_Security_Mode_Tag_g_MAC_ACL_MODE_c: MAC_Security_Mode_Tag = 1;
#[doc = " - MAC Secured Mode Security"]
pub const MAC_Security_Mode_Tag_g_MAC_SECURED_MODE_c: MAC_Security_Mode_Tag = 2;
#[doc = "/\n/** @brief MAC Security Mode"]
pub type MAC_Security_Mode_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief MAC Security Mode"]
pub use self::MAC_Security_Mode_Tag as MAC_Security_Mode_t;
#[doc = " Attribute ID of mac attribute mac_ack_wait_duration"]
pub const MAC_Pib_Ids_Tag_g_MAC_ACK_WAIT_DURATION_c: MAC_Pib_Ids_Tag = 64;
#[doc = " Attribute ID of mac attribute mac_association_permit"]
pub const MAC_Pib_Ids_Tag_g_MAC_ASSOCIATION_PERMIT_c: MAC_Pib_Ids_Tag = 65;
#[doc = " Attribute ID of mac attribute mac_auto_request"]
pub const MAC_Pib_Ids_Tag_g_MAC_AUTO_REQUEST_c: MAC_Pib_Ids_Tag = 66;
#[doc = " Attribute ID of mac attribute mac_beacon_payload"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_PAYLOAD_c: MAC_Pib_Ids_Tag = 69;
#[doc = " Attribute ID of mac attribute mac_beacon_payload_length"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_PAYLOAD_LENGTH_c: MAC_Pib_Ids_Tag = 70;
#[doc = " Attribute ID of mac attribute mac_beacon_order"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_ORDER_c: MAC_Pib_Ids_Tag = 71;
#[doc = " Attribute ID of mac attribute mac_bsn"]
pub const MAC_Pib_Ids_Tag_g_MAC_BSN_c: MAC_Pib_Ids_Tag = 73;
#[doc = " Attribute ID of mac attribute mac_coord_extended_address"]
pub const MAC_Pib_Ids_Tag_g_MAC_COORD_EXTENDED_ADDDRESS_c: MAC_Pib_Ids_Tag = 74;
#[doc = " Attribute ID of mac attribute mac_coord_short_address"]
pub const MAC_Pib_Ids_Tag_g_MAC_COORD_SHORT_ADDRESS_c: MAC_Pib_Ids_Tag = 75;
#[doc = " Attribute ID of mac attribute mac_dsn"]
pub const MAC_Pib_Ids_Tag_g_MAC_DSN_c: MAC_Pib_Ids_Tag = 76;
#[doc = " Attribute ID of mac attribute mac_max_frame_total_wait_time"]
pub const MAC_Pib_Ids_Tag_g_MAC_MAX_FRAME_TOTAL_WAIT_TIME_c: MAC_Pib_Ids_Tag = 88;
#[doc = " Attribute ID of mac attribute mac_max_frame_retries"]
pub const MAC_Pib_Ids_Tag_g_MAC_MAX_FRAME_RETRIES_c: MAC_Pib_Ids_Tag = 89;
#[doc = " Attribute ID of mac attribute mac_pan_id"]
pub const MAC_Pib_Ids_Tag_g_MAC_PAN_ID_c: MAC_Pib_Ids_Tag = 80;
#[doc = " Attribute ID of mac attribute mac_response_wait_time"]
pub const MAC_Pib_Ids_Tag_g_MAC_RESPONSE_WAIT_TIME_c: MAC_Pib_Ids_Tag = 90;
#[doc = " Attribute ID of mac attribute mac_rx_on_when_idle"]
pub const MAC_Pib_Ids_Tag_g_MAC_RX_ON_WHEN_IDLE_c: MAC_Pib_Ids_Tag = 82;
#[doc = " Attribute ID of mac attribute mac_security_enabled"]
pub const MAC_Pib_Ids_Tag_g_MAC_SECURITY_ENABLED_c: MAC_Pib_Ids_Tag = 93;
#[doc = " Attribute ID of mac attribute mac_short_address"]
pub const MAC_Pib_Ids_Tag_g_MAC_SHORT_ADDRESS_c: MAC_Pib_Ids_Tag = 83;
#[doc = " Attribute ID of mac attribute mac_superframe_order"]
pub const MAC_Pib_Ids_Tag_g_MAC_SUPERFRAME_ORDER_c: MAC_Pib_Ids_Tag = 84;
#[doc = " Attribute ID of mac attribute mac_time_stamp_supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_TIMESTAMP_SUPPORTED_c: MAC_Pib_Ids_Tag = 92;
#[doc = "  Attribute ID of mac attribute mac_transaction_persistence_time"]
pub const MAC_Pib_Ids_Tag_g_MAC_TRANSACTION_PERSISTENCE_TIME_c: MAC_Pib_Ids_Tag = 85;
#[doc = " Attribute ID of mac attribute mac_max_be"]
pub const MAC_Pib_Ids_Tag_g_MAC_MAX_BE_c: MAC_Pib_Ids_Tag = 87;
#[doc = " Attribute ID of mac attribute mac_lifs"]
pub const MAC_Pib_Ids_Tag_g_MAC_LIFS_PERIOD_c: MAC_Pib_Ids_Tag = 94;
#[doc = " Attribute ID of mac attribute mac_sifs"]
pub const MAC_Pib_Ids_Tag_g_MAC_SIFS_PERIOD_c: MAC_Pib_Ids_Tag = 95;
#[doc = " Attribute ID of mac attribute mac_max_csma_backoffs"]
pub const MAC_Pib_Ids_Tag_g_MAC_MAX_CSMA_BACKOFFS_c: MAC_Pib_Ids_Tag = 78;
#[doc = " Attribute ID of mac attribute mac_min_be"]
pub const MAC_Pib_Ids_Tag_g_MAC_MIN_BE_c: MAC_Pib_Ids_Tag = 79;
#[doc = " Attribute ID of pan coordinator"]
pub const MAC_Pib_Ids_Tag_g_MAC_PAN_COORDINATOR_c: MAC_Pib_Ids_Tag = 16;
#[doc = " Attribute ID of parent is a pan coordinator"]
pub const MAC_Pib_Ids_Tag_g_MAC_ASSOC_PAN_COORDINATOR_c: MAC_Pib_Ids_Tag = 17;
#[doc = " Attribute ID of mac extended address"]
pub const MAC_Pib_Ids_Tag_g_MAC_EXTENDED_ADDRESS_c: MAC_Pib_Ids_Tag = 111;
#[doc = " Attribute ID of MAC ACL Entry"]
pub const MAC_Pib_Ids_Tag_g_MAC_ACL_ENTRY_DESCRIPTOR_c: MAC_Pib_Ids_Tag = 112;
#[doc = " Attribute ID of No of ACL Security Descriptor Entries"]
pub const MAC_Pib_Ids_Tag_g_MAC_ACL_ENTRY_DESCRIPTOR_SIZE_c: MAC_Pib_Ids_Tag = 113;
#[doc = " Attribute ID of MAC Default Security Support"]
pub const MAC_Pib_Ids_Tag_g_MAC_DEFAULT_SECURITY_c: MAC_Pib_Ids_Tag = 114;
#[doc = " Attribute ID of MAC Default Security material length"]
pub const MAC_Pib_Ids_Tag_g_MAC_DEFAULT_SECURITY_MATERIAL_LENGTH_c: MAC_Pib_Ids_Tag = 115;
#[doc = " Attribute ID of MAC Default Security Material"]
pub const MAC_Pib_Ids_Tag_g_MAC_DEFAULT_SECURITY_MATERIAL_c: MAC_Pib_Ids_Tag = 116;
#[doc = " Attribute ID of MAC Default Security Suite"]
pub const MAC_Pib_Ids_Tag_g_MAC_DEFAULT_SECURITY_SUITE_c: MAC_Pib_Ids_Tag = 117;
#[doc = " Attribute ID of MAC Security Mode"]
pub const MAC_Pib_Ids_Tag_g_MAC_SECURITY_MODE_c: MAC_Pib_Ids_Tag = 118;
#[doc = " Attribute ID of Current number of ACL Entries"]
pub const MAC_Pib_Ids_Tag_g_MAC_CURRENT_ACL_ENTRIES_c: MAC_Pib_Ids_Tag = 128;
#[doc = " Proprietary Attribute ID of Default MAC Security Extended Address"]
pub const MAC_Pib_Ids_Tag_g_MAC_DEFAULT_SECURITY_EXTENDED_ADDRESS_c: MAC_Pib_Ids_Tag = 129;
#[doc = " Attribute ID of max csma-ca frame retry"]
pub const MAC_Pib_Ids_Tag_g_MAC_MAX_FULL_CSMA_FRAME_RETRY_ID_c: MAC_Pib_Ids_Tag = 138;
#[doc = " Attribute ID of implicit broadcast"]
pub const MAC_Pib_Ids_Tag_g_MAC_IMPLICIT_BROADCAST_ID_c: MAC_Pib_Ids_Tag = 139;
#[doc = " Proprietary Attribute ID of Default MAC Security Extended Address"]
pub const MAC_Pib_Ids_Tag_g_MAC_ASSOCIATED_PAN_COORDINATOR_c: MAC_Pib_Ids_Tag = 86;
#[doc = " Attribute ID of enabling the promiscuous mode"]
pub const MAC_Pib_Ids_Tag_g_MAC_PROMISCUOUS_MODE_c: MAC_Pib_Ids_Tag = 81;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_NOTIFY_ALL_BEACONS: MAC_Pib_Ids_Tag = 176;
#[doc = " Attribute ID for enhanced beacon order always set to 15 in our implementation"]
pub const MAC_Pib_Ids_Tag_g_MAC_ENHANCED_BEACON_ORDER: MAC_Pib_Ids_Tag = 177;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_MPMLE: MAC_Pib_Ids_Tag = 178;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_OFFSET_TIME_SLOT: MAC_Pib_Ids_Tag = 179;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_FCS_TYPE: MAC_Pib_Ids_Tag = 180;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_LE_CIM_ALOHA_UNIT_BACKOFF_PERIOD: MAC_Pib_Ids_Tag = 181;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_LE_CIM_ALOHA_BE: MAC_Pib_Ids_Tag = 182;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_PRIORITY_CHANNEL_ACCESS: MAC_Pib_Ids_Tag = 183;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_PCA_ALLOCATION_SUPER_RATE: MAC_Pib_Ids_Tag = 184;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_PCA_ALLOCATION_RATE: MAC_Pib_Ids_Tag = 185;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_CRIT_MSG_DELAY_TOY: MAC_Pib_Ids_Tag = 186;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_START_BAND_EDGE: MAC_Pib_Ids_Tag = 187;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_END_BAND_EDGE: MAC_Pib_Ids_Tag = 188;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_GROUP_RX_MODE: MAC_Pib_Ids_Tag = 189;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_TMCTP_EXTENDED_ORDER: MAC_Pib_Ids_Tag = 190;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_SEC_ENHANCED_BEACON_SECURITY_LEVEL: MAC_Pib_Ids_Tag = 191;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_SEC_ENHANCED_BEACON_KEY_ID_MODE: MAC_Pib_Ids_Tag = 192;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_SEC_ENHANCED_BEACON_KEY_SOURCE: MAC_Pib_Ids_Tag = 193;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_SEC_ENHANCED_BEACON_KEY_INDEX: MAC_Pib_Ids_Tag = 194;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_COORD_REALIGN_SECURITY_LEVEL: MAC_Pib_Ids_Tag = 195;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_COORD_REALIGN_KEY_ID_MODE: MAC_Pib_Ids_Tag = 196;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_COORD_REALIGN_KEY_SOURCE: MAC_Pib_Ids_Tag = 197;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_SECURITY_LEVEL: MAC_Pib_Ids_Tag = 198;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_KEY_ID_MODE: MAC_Pib_Ids_Tag = 199;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_KEY_SOURCE: MAC_Pib_Ids_Tag = 200;
#[doc = " Attribute ID not supported"]
pub const MAC_Pib_Ids_Tag_g_MAC_BEACON_KEY_INDEX: MAC_Pib_Ids_Tag = 201;
#[doc = " Attribute ID to know which PIB version is used"]
pub const MAC_Pib_Ids_Tag_g_MAC_PROP_PIB_REVISION: MAC_Pib_Ids_Tag = 252;
#[doc = "/\n/** @brief This enum contains all the mac pib Ids"]
pub type MAC_Pib_Ids_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief This enum contains all the mac pib Ids"]
pub use self::MAC_Pib_Ids_Tag as MAC_Pib_Ids_t;
#[doc = " Attribute ID for current channel"]
pub const PHY_Pib_Id_Tag_g_PHY_CURRENT_CHANNEL_c: PHY_Pib_Id_Tag = 0;
#[doc = " Attribute ID for channels supported"]
pub const PHY_Pib_Id_Tag_g_PHY_CHANNELS_SUPPORTED_c: PHY_Pib_Id_Tag = 1;
#[doc = " Attribute ID for transmit power"]
pub const PHY_Pib_Id_Tag_g_PHY_TRANSMIT_POWER_c: PHY_Pib_Id_Tag = 2;
#[doc = " Attribute ID for CCA Mode"]
pub const PHY_Pib_Id_Tag_g_PHY_CCA_MODE_c: PHY_Pib_Id_Tag = 3;
#[doc = " Attribute ID for current page"]
pub const PHY_Pib_Id_Tag_g_PHY_CURRENT_PAGE_c: PHY_Pib_Id_Tag = 4;
#[doc = " Attribute ID for Max Frame Duration"]
pub const PHY_Pib_Id_Tag_g_PHY_MAX_FRAME_DURATION_c: PHY_Pib_Id_Tag = 5;
#[doc = " Attribute ID for SHR duration"]
pub const PHY_Pib_Id_Tag_g_PHY_SHR_DURATION_c: PHY_Pib_Id_Tag = 6;
#[doc = " Attribute ID for symbols per octet"]
pub const PHY_Pib_Id_Tag_g_PHY_SYMBOLS_PER_OCTET_c: PHY_Pib_Id_Tag = 7;
#[doc = "/\n/** @brief This enum contains all the PHY pib Ids */\n/** @brief The PHY PIB attribute IDs"]
pub type PHY_Pib_Id_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief This enum contains all the PHY pib Ids */\n/** @brief The PHY PIB attribute IDs"]
pub use self::PHY_Pib_Id_Tag as PHY_Pib_Id_t;
#[doc = " -ED Scan"]
pub const MAC_Scan_Types_Tag_g_MAC_ED_SCAN_TYPE_c: MAC_Scan_Types_Tag = 0;
#[doc = " -Active Scan"]
pub const MAC_Scan_Types_Tag_g_MAC_ACTIVE_SCAN_TYPE_c: MAC_Scan_Types_Tag = 1;
#[doc = " -Passive Scan"]
pub const MAC_Scan_Types_Tag_g_MAC_PASSIVE_SCAN_TYPE_c: MAC_Scan_Types_Tag = 2;
#[doc = " -Orphan Scan"]
pub const MAC_Scan_Types_Tag_g_MAC_ORPHAN_SCAN_TYPE_c: MAC_Scan_Types_Tag = 3;
#[doc = " -Enhanced Active Scan"]
pub const MAC_Scan_Types_Tag_g_MAC_ENHANCED_ACTIVE_SCAN_TYPE_c: MAC_Scan_Types_Tag = 4;
#[doc = "/\n/** @brief This enum indicates the type of scan to be performed"]
pub type MAC_Scan_Types_Tag = ::core::ffi::c_uint;
#[doc = "/\n/** @brief This enum indicates the type of scan to be performed"]
pub use self::MAC_Scan_Types_Tag as MAC_Scan_Types_t;
#[doc = "/\n/** @brief This structure contains PIB attributes value for ST service"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct st_mlme_pib_t {
    pub txPwr: i8,
}
pub type MAC_Status_t = u8;
pub type MAC_handle = u8;
#[doc = " @brief  Defines the type used to handle addresses. Addresses are either short (2 bytes)\nor extended (8 bytes)"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union MAC_addr_t {
    #[doc = " Short address"]
    pub a_short_addr: [u8; 2usize],
    #[doc = " Extended address"]
    pub a_extend_addr: [u8; 8usize],
}
impl Default for MAC_addr_t {
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
pub struct MAC_GTSCharacteristics_t {
    pub GTSCharacteristics_fields: u8,
}
#[doc = " @brief Define the type used to IEs header.\nSize in 1 byte and list pointer for all elements"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_hdr_ie {
    #[doc = " length of header IEs on 7 bits + ElementID + Type of header IEs, Type=0"]
    pub hdr_ie_length_elementID_Type: u16,
    #[doc = " content of header IEs"]
    pub hdr_ie_content: [u8; 6usize],
}
#[doc = " @brief Define the type used to IEs payload.\nSize in 1 byte and list pointer for all elements"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_pyld_ie {
    #[doc = " length of payload IEs on 10 bits + GroupID + Type of payload IEs Type=1"]
    pub pyld_ie_length_groupID_Type: u16,
    #[doc = " content of payload IEs"]
    pub pyld_ie_content: [u8; 28usize],
}
#[doc = "/\n/** @brief  Defines a structure for MAC PAN Descriptor which contains the\nnetwork details of the device from which the beacon is received"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_PAN_Desc_t {
    #[doc = " PAN identifier of the coordinator"]
    pub a_coord_PAN_id: [u8; 2usize],
    #[doc = " Coordinator addressing mode"]
    pub coord_addr_mode: u8,
    #[doc = " The current logical channel occupied by the network"]
    pub logical_channel: u8,
    #[doc = " Coordinator address"]
    pub coord_addr: MAC_addr_t,
    #[doc = " The current channel page occupied by the network"]
    pub channel_page: u8,
    #[doc = " PAN coordinator is accepting GTS requests or not"]
    pub gts_permit: u8,
    #[doc = " Superframe specification as specified in the received beacon frame"]
    pub a_superframe_spec: [u8; 2usize],
    #[doc = " The time at which the beacon frame was received, in symbols"]
    pub a_time_stamp: [u8; 4usize],
    #[doc = " The LQI at which the network beacon was received"]
    pub link_quality: u8,
    #[doc = " Security level purportedly used by the received beacon frame"]
    pub security_level: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
impl Default for ST_MAC_PAN_Desc_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for  MLME ASSOCIATE Request which will be used\nby the application to request an association"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_associateReq_t {
    #[doc = " The logical channel on which to attempt association"]
    pub channel_number: u8,
    #[doc = " The channel page on which to attempt association"]
    pub channel_page: u8,
    #[doc = " Coordinator addressing mode used"]
    pub coord_addr_mode: u8,
    #[doc = " Operational capabilities of the associating device"]
    pub capability_information: u8,
    #[doc = " The identifier of the PAN with which to associate"]
    pub a_coord_PAN_id: [u8; 2usize],
    #[doc = " The security level to be used"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " Coordinator address"]
    pub coord_address: MAC_addr_t,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
}
impl Default for ST_MAC_associateReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for  MLME DISASSOCIATE Request which will be\nused by the application to request an disassociation"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_disassociateReq_t {
    #[doc = " Device addressing mode used"]
    pub device_addr_mode: u8,
    #[doc = " The identifier of the PAN of the device"]
    pub a_device_PAN_id: [u8; 2usize],
    #[doc = " The reason for the disassociation"]
    pub disassociate_reason: u8,
    #[doc = " Device address"]
    pub device_address: MAC_addr_t,
    #[doc = " TRUE if the disassociation notification command is to be sent indirectly"]
    pub tx_Indirect: u8,
    #[doc = " The security level to be used"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
}
impl Default for ST_MAC_disassociateReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for  MLME GET Request which will be\nused by the application to request a PIB Value"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_getReq_t {
    #[doc = " The name of the PIB attribute to read"]
    pub PIB_attribute: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = " @brief Defines the structure for  MLME GTS Request which will be\nused by the application to request and maintain GTSs"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_gtsReq_t {
    #[doc = " The characteristics of the GTS"]
    pub GTS_characteristics: MAC_GTSCharacteristics_t,
    #[doc = " The security level to be used"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
}
#[doc = " @brief  Defines the structure for MLME RESET Request which allows the\napplication to request that the MLME of the MAC layer performs a\nreset operation"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_resetReq_t {
    #[doc = " MAC PIB attributes are set to their default values or not during reset"]
    pub set_default_PIB: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME RX ENABLE Request which allows the\napplication to request that the receiver is either enabled\nfor a finite period of time or disabled"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_rxEnableReq_t {
    #[doc = " The requested operation can be deferred or not"]
    pub defer_permit: u8,
    #[doc = "Configure the transceiver to Rx with ranging for a\nvalue of RANGING_ON or to not enable ranging for\nRANGING_OFF"]
    pub ranging_Rx_control: u8,
    #[doc = " Number of symbols measured before the receiver is to be enabled or\ndisabled"]
    pub a_Rx_on_time: [u8; 3usize],
    #[doc = " Number of symbols for which the receiver is to be enabled"]
    pub a_Rx_on_duration: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME SCAN Request which is used to\ninitiate a channel scan over a given list of channels"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_scanReq_t {
    #[doc = " The type of scan to be performed"]
    pub scan_type: u8,
    #[doc = " The time spent on scanning each channel"]
    pub scan_duration: u8,
    #[doc = " Channel page on which to perform the scan"]
    pub channel_page: u8,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " Indicate which channels are to be scanned"]
    pub a_scan_channels: [u8; 4usize],
    #[doc = " Originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " Structure to header IE List"]
    pub hdr_ie_list: [ST_MAC_hdr_ie; 3usize],
    #[doc = " Structure to payload IE List"]
    pub pyld_ie_list: [ST_MAC_pyld_ie; 2usize],
    #[doc = " Set to TRUE if the sequence number is suppressed in the frame"]
    pub scan_seq_sup: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub key_index: u8,
}
#[doc = "/\n/** @brief  Defines the structure for MLME SET Request which is used to\nattempt to write the given value to the indicated PIB attribute"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_setReq_t {
    #[doc = " The pointer to the Value of the PIB attribute to set"]
    pub PIB_attribute_valuePtr: *mut u8,
    #[doc = " The name of the PIB attribute to set"]
    pub PIB_attribute: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
impl Default for ST_MAC_setReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief Defines the structure for MLME START Request which is used by the\nFFDs to initiate a new PAN or to begin using a new superframe\nconfiguration"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_startReq_t {
    #[doc = " PAN identifier to be used by the device"]
    pub a_PAN_id: [u8; 2usize],
    #[doc = " Logical channel on which to begin"]
    pub channel_number: u8,
    #[doc = " Channel page on which to begin"]
    pub channel_page: u8,
    #[doc = " Time at which to begin transmitting beacons"]
    pub a_start_time: [u8; 4usize],
    #[doc = " Indicates how often the beacon is to be transmitted"]
    pub beacon_order: u8,
    #[doc = " Length of the active portion of the superframe"]
    pub superframe_order: u8,
    #[doc = " Indicates whether the device is a PAN coordinator or not"]
    pub PAN_coordinator: u8,
    #[doc = " Indicates if the receiver of the beaconing device is disabled or not"]
    pub battery_life_extension: u8,
    #[doc = " Indicates if the coordinator realignment command is to be transmitted"]
    pub coord_realignment: u8,
    #[doc = " Indicates if the coordinator realignment command is to be transmitted"]
    pub coord_realign_security_level: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub coord_realign_key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub coord_realign_key_index: u8,
    #[doc = " Originator of the key to be used"]
    pub a_coord_realign_key_source: [u8; 8usize],
    #[doc = " Security level to be used for beacon frames"]
    pub beacon_security_level: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub beacon_key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub beacon_key_index: u8,
    #[doc = " Originator of the key to be used"]
    pub a_beacon_key_source: [u8; 8usize],
    pub hdr_ie_list: [ST_MAC_hdr_ie; 3usize],
    pub pyld_ie_list: [ST_MAC_pyld_ie; 2usize],
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 1usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME POLL Request which prompts the\ndevice to request data from the coordinator"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_pollReq_t {
    #[doc = " Addressing mode of the coordinator"]
    pub coord_addr_mode: u8,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub key_index: u8,
    #[doc = " Coordinator address"]
    pub coord_address: MAC_addr_t,
    #[doc = " Originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " PAN identifier of the coordinator"]
    pub a_coord_PAN_id: [u8; 2usize],
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
impl Default for ST_MAC_pollReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief  Defines the structure for MLME DPS Request which allows the next\nhigher layer to request that the PHY utilize a given pair of preamble codes\nfor a single use pending expiration of the DPSIndexDuration"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_dpsReq_t {
    #[doc = " The index value for the transmitter"]
    pub Tx_DPS_index: u8,
    #[doc = " The index value for the receiver"]
    pub Rx_DPS_index: u8,
    #[doc = " The number of symbols for which the transmitter and\nreceiver will utilize the respective DPS indices"]
    pub DPS_index_duration: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 1usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME SOUNDING request primitive which is\nused by the next higher layer to request that the PHY respond with channel\nsounding information"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_soundingReq_t {
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 4usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME CALIBRATE request primitive which\nused  to obtain the results of a ranging calibration request from an RDEV"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_calibrateReq_t {
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 4usize],
}
#[doc = " @brief Defines the structure for MCPS DATA Request which will be used for\nMAC data related requests from the application"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_dataReq_t {
    #[doc = " Source addressing mode used"]
    pub src_addr_mode: u8,
    #[doc = " Destination addressing mode used"]
    pub dst_addr_mode: u8,
    #[doc = " Destination PAN ID"]
    pub a_dst_PAN_id: [u8; 2usize],
    #[doc = " Destination address"]
    pub dst_address: MAC_addr_t,
    #[doc = " The number of octets contained in the MSDU"]
    pub msdu_length: u8,
    #[doc = " The handle associated with the MSDU to be transmitted"]
    pub msdu_handle: u8,
    #[doc = " The ACK transmission options for the MSDU"]
    pub ack_Tx: u8,
    #[doc = "TRUE if a GTS is to be used for transmission.\nFALSE indicates that the CAP will be used"]
    pub GTS_Tx: u8,
    #[doc = " The Pending Bit transmission options for the MSDU"]
    pub indirect_Tx: u8,
    #[doc = " The security level to be used"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " 2011 - The pulse repetition value"]
    pub UWBPRF: u8,
    #[doc = " 2011 - The ranging configuration"]
    pub ranging: u8,
    #[doc = " 2011 - The preamble symbol repetitions"]
    pub UWB_preamble_symbol_repetitions: u8,
    #[doc = " 2011 - Indicates the data rate"]
    pub data_rate: u8,
    #[doc = " The handle associated with the MSDU to be transmitted  */\n/*! Must always be at the end of this structure"]
    pub msduPtr: *mut u8,
}
impl Default for ST_MAC_dataReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief  Defines the structure for MLME Get-Power-Information-Table Request primitive\nwhich prompts the device to request the Power Control Information entry for the link pair"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_getPwrInfoTableReq_t {
    #[doc = " Short address of the link pair to transmit the packet to"]
    pub short_address: u16,
    #[doc = " Extended (IEEE) address of the link pair to transmit the packet to"]
    pub ieee_address: [u8; 8usize],
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME Set-Power-Information-Table Request primitive\nwhich prompts the device to add the Power Control Information entry for the link pair"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_setPwrInfoTableReq_t {
    #[doc = " Short address of the link pair to transmit the packet to"]
    pub short_address: u16,
    #[doc = " Extended (IEEE) address of the link pair to transmit the packet to"]
    pub ieee_address: [u8; 8usize],
    #[doc = " Tx Power Level"]
    pub tx_pwr_level: i8,
    #[doc = " Last RSSI Level"]
    pub last_rssi_level: i8,
    #[doc = " Network Negotiated"]
    pub nwk_negociated: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = " @brief Defines the structure for MCPS PURGE Request which will be used by\nthe application to purge an MSDU from the transaction\nqueue"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_purgeReq_t {
    #[doc = " The handle associated with the MSDU to be purged from the transaction\nqueue"]
    pub msdu_handle: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = " @brief Defines the structure for MLME BEACON Request primitive\nwhich send a beacon manually"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_beaconReq_t {
    #[doc = " The beacon type NormalBeacon=0, EnhancedBeacon=1"]
    pub beacon_type: u8,
    #[doc = " The logical channel on which the device lost synchronization or to which it was realigned"]
    pub channel_number: u8,
    #[doc = " The channel page on which the device lost synchronization or to which it was realigned"]
    pub channel_page: u8,
    #[doc = " Length of the active portion of the superframe"]
    pub superframe_order: u8,
    pub hdr_ie_list: [ST_MAC_hdr_ie; 3usize],
    pub pyld_ie_list: [ST_MAC_pyld_ie; 2usize],
    #[doc = " The beacon security level to be used"]
    pub beacon_security_level: u8,
    #[doc = " The mode used to identify the beacon key to be used"]
    pub beacon_key_id_mode: u8,
    #[doc = " The originator of the beacon key to be used"]
    pub beacon_key_source: [u8; 8usize],
    #[doc = " The index of the beacon key to be used"]
    pub beacon_key_index: u8,
    #[doc = " source address mode"]
    pub src_addr_mode: u8,
    #[doc = " Destination addressing mode used"]
    pub dst_addr_mode: u8,
    #[doc = " Destination address"]
    pub dst_address: MAC_addr_t,
    #[doc = " BSN Suppression"]
    pub BSN_suppression: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 1usize],
}
impl Default for ST_MAC_beaconReq_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief  Defines the structure for MLME ASSOCIATE Response which is used to\ninitiate a response to an MLME-ASSOCIATE.indication"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_associateRes_t {
    #[doc = " Extended address of the device requesting association"]
    pub a_device_address: [u8; 8usize],
    #[doc = " 16-bit short device address allocated by the coordinator on successful\nassociation"]
    pub a_assoc_short_address: [u8; 2usize],
    #[doc = " Status of the association attempt"]
    pub status: u8,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME ORPHAN Response which is used by the\napplication layer of the coordinator to respond to the MLME ORPHAN\nIndication"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_orphanRes_t {
    #[doc = " Extended address of the orphaned device"]
    pub a_orphan_address: [u8; 8usize],
    #[doc = " Short address allocated to the orphaned device"]
    pub a_short_address: [u8; 2usize],
    #[doc = " If the orphaned device is associated with the coordinator or not"]
    pub associated_member: u8,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " Originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
    #[doc = " Mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub key_index: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = " @brief Defines a structure for MLME ASSOCIATE Confirm which is used to\ninform the application of the initiating device whether its\nrequest to associate was successful or unsuccessful"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_associateCnf_t {
    #[doc = " Short address allocated by the coordinator on successful association"]
    pub a_assoc_short_address: [u8; 2usize],
    #[doc = " Status of the association attempt"]
    pub status: u8,
    #[doc = " The security level used"]
    pub security_level: u8,
    #[doc = " The originator of the key"]
    pub a_key_source: [u8; 8usize],
    #[doc = " The mode used to identify the key"]
    pub key_id_mode: u8,
    #[doc = " The index of the key"]
    pub key_index: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = " @brief Defines the structure for  MLME DISASSOCIATE Confirm which will be\nused to send disassociation Confirmation to the application."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_disassociateCnf_t {
    #[doc = " Status of the disassociation attempt"]
    pub status: u8,
    #[doc = " Device addressing mode used"]
    pub device_addr_mode: u8,
    #[doc = " The identifier of the PAN of the device"]
    pub a_device_PAN_id: [u8; 2usize],
    #[doc = " Device address"]
    pub device_address: MAC_addr_t,
}
impl Default for ST_MAC_disassociateCnf_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief Defines the structure for  MLME GET Confirm which requests information\nabout a given PIB attribute"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_getCnf_t {
    #[doc = " Status of the GET attempt"]
    pub status: u8,
    #[doc = " The name of the PIB attribute attempted to read"]
    pub PIB_attribute: u8,
    #[doc = " The length of the PIB attribute Value return"]
    pub PIB_attribute_value_len: u8,
    #[doc = " The pointer to the value of the PIB attribute attempted to read"]
    pub PIB_attribute_value: [u8; 1usize],
}
#[doc = "/\n/** @brief Defines the structure for  MLME GTS Confirm which eports the results\nof a request to allocate a new GTS or to deallocate an existing GTS"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_gtsCnf_t {
    #[doc = " The characteristics of the GTS"]
    pub GTS_characteristics: u8,
    #[doc = "The status of the GTS request"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME RESET Confirm which is used to report\nthe results of the reset operation"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_resetCnf_t {
    #[doc = " The result of the reset operation"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME RX ENABLE Confirm which is used to\nreport the results of the attempt to enable or disable the receiver"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_rxEnableCnf_t {
    #[doc = " Result of the request to enable or disable the receiver"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME SCAN Confirm which is used to report\nthe result of the channel scan request"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_scanCnf_t {
    #[doc = " Status of the scan request"]
    pub status: u8,
    #[doc = " The type of scan performed"]
    pub scan_type: u8,
    #[doc = " Channel page on which the scan was performed"]
    pub channel_page: u8,
    #[doc = " Channels given in the request which were not scanned"]
    pub a_unscanned_channels: [u8; 4usize],
    #[doc = " Number of elements returned in the appropriate result lists"]
    pub result_list_size: u8,
    #[doc = " List of energy measurements"]
    pub a_energy_detect_list: [u8; 16usize],
    #[doc = " List of PAN descriptors"]
    pub a_PAN_descriptor_list: [ST_MAC_PAN_Desc_t; 6usize],
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
impl Default for ST_MAC_scanCnf_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief  Defines the structure for MLME SET Confirm which reports the results\nof an attempt to write a value to a PIB attribute"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_setCnf_t {
    #[doc = " The result of the set operation"]
    pub status: u8,
    #[doc = " The name of the PIB attribute that was written."]
    pub PIB_attribute: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME START Confirm which is used to\nreport the results of the attempt to start using a new superframe\nconfiguration"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_startCnf_t {
    #[doc = "Result of the attempt to start using an updated superframe configuration"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME POLL Confirm which is used to report\nthe result of a request to poll the coordinator for data"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_pollCnf_t {
    #[doc = " The status of the data request"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME DPS Confirm which  reports the\nresults of the attempt to enable or disable the DPS"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_dpsCnf_t {
    #[doc = " The status of the DPS request"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME SOUNDING Confirm which  reports the\nresult of a request to the PHY to provide channel sounding information"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_soundingCnf_t {
    #[doc = " Results of the sounding measurement"]
    pub a_sounding_list: [u8; 1usize],
    #[doc = " The status of the attempt to return sounding data"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME CALIBRATE Confirm which reports the\nresult of a request to the PHY to provide internal propagation path information."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_calibrateCnf_t {
    #[doc = " The status of the attempt to return sounding data"]
    pub status: u8,
    #[doc = " A count of the propagation time from the ranging counter to\nthe transmit antenna"]
    pub cal_Tx_rmaker_offset: u32,
    #[doc = " A count of the propagation time from the\nreceive antenna to the ranging counter"]
    pub cal_Rx_rmaker_offset: u32,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief Defines the structure for MCPS DATA Confirm which will be used for\nreporting the results of MAC data related requests from the\napplication"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_dataCnf_t {
    #[doc = " The handle associated with the MSDU being confirmed"]
    pub msdu_handle: u8,
    #[doc = " The time, in symbols, at which the data were transmitted"]
    pub a_time_stamp: [u8; 4usize],
    #[doc = " ranging status"]
    pub ranging_received: u8,
    #[doc = " The status of the last MSDU transmission"]
    pub status: u8,
    #[doc = " time units corresponding to an RMARKER at the antenna at the\nbeginning of a ranging exchange"]
    pub ranging_counter_start: u32,
    #[doc = "time units corresponding to an RMARKER at the antenna at the\nend of a ranging exchange"]
    pub ranging_counter_stop: u32,
    #[doc = " time units in a message exchange over which the tracking offset\nwas measured"]
    pub ranging_tracking_interval: u32,
    #[doc = " time units slipped or advanced by the radio tracking system"]
    pub ranging_offset: u32,
    #[doc = " The FOM characterizing the ranging measurement"]
    pub ranging_FOM: u8,
}
#[doc = "/\n/** @brief Defines the structure for MCPS PURGE Confirm which will be used by\nthe  MAC to notify the application of the status of its request\nto purge an MSDU from the transaction queue"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_purgeCnf_t {
    #[doc = " Handle associated with the MSDU requested to be purged from the\ntransaction queue"]
    pub msdu_handle: u8,
    #[doc = " The status of the request"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME Get-Power-Information-Table Confirm primitive\nwhich is used to inform the application of the initiating device about the status and\ninformation requested"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_getPwrInfoTableCnf_t {
    #[doc = " Status used to indicate if an entry was found for the pair requested"]
    pub status: u8,
    #[doc = " Short address of the link pair to transmit the packet to"]
    pub short_address: u16,
    #[doc = " Extended (IEEE) address of the link pair to transmit the packet to"]
    pub ieee_address: [u8; 8usize],
    #[doc = " Tx Power Level"]
    pub tx_pwr_level: i8,
    #[doc = " Last RSSI Level"]
    pub last_rssi_level: i8,
    #[doc = " Network Negotiated"]
    pub nwk_negociated: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME Set-Power-Information-Table Confirm primitive\nwhich is used to inform the application of the initiating device about the status of\nthe REQUEST"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_setPwrInfoTableCnf_t {
    #[doc = " The result of the reset operation"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = " @brief Defines the structure for MLME BEACON Confirm which will be used for\nreporting the results of MAC beacon related requests from the\napplication"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_beaconCnf_t {
    #[doc = " The status of the request"]
    pub status: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
#[doc = "/\n/** @brief Defines the structure for MLME ASSOCIATE Indication which will be\nused by the MAC to indicate the reception of an association request\ncommand"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_associateInd_t {
    #[doc = " Extended address of the device requesting association"]
    pub a_device_address: [u8; 8usize],
    #[doc = " Operational capabilities of the device requesting association"]
    pub capability_information: u8,
    #[doc = " Security level purportedly used by the received MAC command frame"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key used by the originator of frame"]
    pub key_id_mode: u8,
    #[doc = " Index of the key used by the originator of the received frame"]
    pub key_index: u8,
    #[doc = " The originator of the key used by the originator of the received frame"]
    pub a_key_source: [u8; 8usize],
}
#[doc = "/\n/** @brief Defines the structure for  MLME DISASSOCIATE indication which will be\nused to send disassociation indication to the application."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_disassociateInd_t {
    #[doc = " Extended address of the device requesting association"]
    pub a_device_address: [u8; 8usize],
    #[doc = " The reason for the disassociation"]
    pub disassociate_reason: u8,
    #[doc = " The security level to be used"]
    pub security_level: u8,
    #[doc = " The mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " The index of the key to be used"]
    pub key_index: u8,
    #[doc = " The originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
}
#[doc = "/\n/** @brief   Defines a structure for MLME BEACON NOTIIFY Indication which is\nused to send parameters contained within a beacon frame received\nby the MAC to the application"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_beaconNotifyInd_t {
    #[doc = " The PAN Descriptor for the received beacon"]
    pub PAN_descriptor: ST_MAC_PAN_Desc_t,
    #[doc = " The list of addresses of the devices for\nwhich the beacon source has data"]
    pub a_addr_list: [MAC_addr_t; 1usize],
    #[doc = " Beacon Sequence Number"]
    pub BSN: u8,
    #[doc = " The beacon pending address specification"]
    pub pend_addr_spec: u8,
    #[doc = " The beacon type NormalBeacon=0, EnhancedBeacon=1"]
    pub beacon_type: u8,
    #[doc = " header IEs"]
    pub hdr_ie_list: [ST_MAC_hdr_ie; 3usize],
    #[doc = " Payload IEs"]
    pub pyld_ie_list: [ST_MAC_pyld_ie; 2usize],
    #[doc = " Number of octets contained in the beacon payload of the beacon frame"]
    pub sdu_length: u8,
    #[doc = " The set of octets comprising the beacon payload to be transferred\nfrom the MAC sublayer entity to the next higher layer"]
    pub sduPtr: [u8; 1usize],
}
impl Default for ST_MAC_beaconNotifyInd_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief  Defines the structure for MLME COMM STATUS Indication which is used\nby the MAC to indicate a communications status"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_commStatusInd_t {
    #[doc = " The 16-bit PAN identifier of the device from which the frame was\nreceived or to which the frame was being sent"]
    pub a_PAN_id: [u8; 2usize],
    #[doc = " Source addressing mode"]
    pub src_addr_mode: u8,
    #[doc = " Destination addressing mode"]
    pub dst_addr_mode: u8,
    #[doc = " Source address"]
    pub src_address: MAC_addr_t,
    #[doc = " Destination address"]
    pub dst_address: MAC_addr_t,
    #[doc = " The communications status"]
    pub status: u8,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub key_index: u8,
    #[doc = " Originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
}
impl Default for ST_MAC_commStatusInd_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief  Defines the structure for MLME GTS Indication indicates that a GTS\nhas been allocated or that a previously allocated GTS has been deallocated"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_GtsInd_t {
    #[doc = " The short address of the device that has been\nallocated or deallocated a GTS"]
    pub a_device_address: [u8; 2usize],
    #[doc = " The characteristics of the GTS"]
    pub GTS_characteristics: MAC_GTSCharacteristics_t,
    #[doc = " Security level to be used"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key to be used"]
    pub key_id_mode: u8,
    #[doc = " Index of the key to be used"]
    pub key_index: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
    #[doc = " Originator of the key to be used"]
    pub a_key_source: [u8; 8usize],
}
#[doc = "/\n/** @brief   Defines the structure for MLME ORPHAN Indication which is used by\nthe coordinator to notify the application of the presence of\nan orphaned device"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_orphanInd_t {
    #[doc = " Extended address of the orphaned device"]
    pub a_orphan_address: [u8; 8usize],
    #[doc = " Originator of the key used by the originator of the received frame"]
    pub a_key_source: [u8; 8usize],
    #[doc = " Security level purportedly used by the received MAC command frame"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key used by originator of received frame"]
    pub key_id_mode: u8,
    #[doc = " Index of the key used by the originator of the received frame"]
    pub key_index: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 1usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME SYNC LOSS Indication which is used\nby the MAC to indicate the loss of synchronization with the\ncoordinator"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_syncLoss_t {
    #[doc = " The PAN identifier with which the device lost synchronization or to\nwhich it was realigned"]
    pub a_PAN_id: [u8; 2usize],
    #[doc = " The reason that synchronization was lost"]
    pub loss_reason: u8,
    #[doc = " The logical channel on which the device lost synchronization or to which\nit was realigned"]
    pub channel_number: u8,
    #[doc = " The channel page on which the device lost synchronization or to which it\nwas realigned"]
    pub channel_page: u8,
    #[doc = " The security level used by the received MAC frame"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key used by originator of received frame"]
    pub key_id_mode: u8,
    #[doc = " Index of the key used by the originator of the received frame"]
    pub key_index: u8,
    #[doc = " Originator of the key used by the originator of the received frame"]
    pub a_key_source: [u8; 8usize],
}
#[doc = "/\n/** @brief  Defines the structure for MLME DPS Indication which indicates the\nexpiration of the DPSIndexDuration and the resetting of the DPS values\nin the PHY"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_dpsInd_t {
    pub a_stuffing: [u8; 4usize],
}
#[doc = "/\n/** @brief Defines the structure for MCPS DATA Indication which will be used\nfor indicating the transfer of a data packet by the MAC"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_dataInd_t {
    #[doc = " Source addressing mode used"]
    pub src_addr_mode: u8,
    #[doc = " Source PAN ID"]
    pub a_src_PAN_id: [u8; 2usize],
    #[doc = " Source address"]
    pub src_address: MAC_addr_t,
    #[doc = " Destination addressing mode used"]
    pub dst_addr_mode: u8,
    #[doc = " Destination PAN ID"]
    pub a_dst_PAN_id: [u8; 2usize],
    #[doc = " Destination address"]
    pub dst_address: MAC_addr_t,
    #[doc = " The number of octets contained in the MSDU being indicated"]
    pub msdu_length: u8,
    #[doc = "LQI value measured during reception of the MPDU"]
    pub mpdu_link_quality: u8,
    #[doc = " The data sequence number of the received data frame"]
    pub DSN: u8,
    #[doc = " The time, in symbols, at which the data were received"]
    pub a_time_stamp: [u8; 4usize],
    #[doc = " The security level purportedly used by the received data frame"]
    pub security_level: u8,
    #[doc = " Mode used to identify the key used by originator of received frame"]
    pub key_id_mode: u8,
    #[doc = " The originator of the key"]
    pub a_key_source: [u8; 8usize],
    #[doc = " The index of the key"]
    pub key_index: u8,
    #[doc = "The pulse repetition value of the received PPDU"]
    pub UWBPRF: u8,
    #[doc = " The preamble symbol repetitions of the UWB PHY frame"]
    pub UWB_preamble_symbol_repetitions: u8,
    #[doc = " Indicates the data rate"]
    pub data_rate: u8,
    pub ranging_received: u8,
    pub ranging_counter_start: u32,
    #[doc = " time units corresponding to an RMARKER at the antenna at the end of a ranging exchange,"]
    pub ranging_counter_stop: u32,
    #[doc = "time units in a message exchange over which the tracking offset was measured"]
    pub ranging_tracking_interval: u32,
    #[doc = " time units slipped or advanced by the radio tracking system"]
    pub ranging_offset: u32,
    #[doc = " The FOM characterizing the ranging measurement"]
    pub ranging_FOM: u8,
    #[doc = " The Received Signal Strength Indicator measured"]
    pub rssi: i8,
    pub Stuffing: i8,
    #[doc = " Pointer to the set of octets forming the MSDU being indicated"]
    pub msdu: [u8; 1usize],
}
impl Default for ST_MAC_dataInd_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief Defines the structure for MLME POLL Indication which will be used\nfor indicating the Data Request reception to upper layer as defined in\nZigbee r22 - D.8.2"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_pollInd_t {
    #[doc = " addressing mode used"]
    pub addr_mode: u8,
    #[doc = " Poll requester address"]
    pub request_address: MAC_addr_t,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 3usize],
}
impl Default for ST_MAC_pollInd_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for MLME BEACONREQUEST Indication which will be used\nfor indicating a beacon is received by the MAC"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_beaconReqInd_t {
    #[doc = " Source address"]
    pub src_address: [u8; 8usize],
    #[doc = " header IEs from beacon request frame"]
    pub hdr_ie_list: [ST_MAC_hdr_ie; 3usize],
    #[doc = " Payload IEs from beacon request frame"]
    pub pyld_ie_list: [ST_MAC_pyld_ie; 2usize],
    #[doc = "  Source PAN ID"]
    pub a_device_PAN_id: [u8; 2usize],
    #[doc = " Type of beacon frame"]
    pub beacon_type: u8,
    #[doc = " source address mode"]
    pub src_addr_mode: u8,
    #[doc = " Byte Stuffing to keep 32 bit alignment"]
    pub a_stuffing: [u8; 2usize],
}
#[doc = " @brief Defines the structure for list_PAN_Coord tab, allows to stock all beacons received from PAN coordinator."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_PAN_COORD_t {
    #[doc = " PAN identifier of the coordinator"]
    pub a_coord_PAN_id: [u8; 2usize],
    #[doc = " Coordinator addressing mode"]
    pub coord_addr_mode: u8,
    #[doc = " The current logical channel occupied by the network"]
    pub logical_channel: u8,
    #[doc = " Coordinator address"]
    pub coord_addr: MAC_addr_t,
}
impl Default for ST_MAC_PAN_COORD_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for regroup table of ST_MAC_PAN_COORD_t and indice_PAN_Coord"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_PAN_Coord_Table_t {
    pub list_PAN_Coord: [ST_MAC_PAN_COORD_t; 10usize],
    pub indice_PAN_Coord: u8,
}
impl Default for ST_MAC_PAN_Coord_Table_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Associate Confirmation"]
pub type ST_MAC_MLMEAssociateCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pAssociateCnf: *const ST_MAC_associateCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer for Pointer type for\nMLME Disassociate Confirmation"]
pub type ST_MAC_MLMEDisassociateCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pDisassociateCnf: *const ST_MAC_disassociateCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer for Pointer type for\nMLME Get Confirmation"]
pub type ST_MAC_MLMEGetCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pGetCnf: *const ST_MAC_getCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer for Pointer type for\nMLME GTS Confirmation"]
pub type ST_MAC_MLMEGtsCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pGtsCnf: *const ST_MAC_gtsCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function for Pointer type for\nMLME RESET Confirmation"]
pub type ST_MAC_MLMEResetCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pResetCnf: *const ST_MAC_resetCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME RXEnable Confirmation"]
pub type ST_MAC_MLMERxEnableCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pRxEnable: *const ST_MAC_rxEnableCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME SCAN Confirmation"]
pub type ST_MAC_MLMEScanCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pScanCnf: *const ST_MAC_scanCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME SET Confirmation"]
pub type ST_MAC_MLMESetCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pSetCnf: *const ST_MAC_setCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback functionPointer type for\nMLME START Confirmation"]
pub type ST_MAC_MLMEStartCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pStartCnf: *const ST_MAC_startCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME POLL Confirmation"]
pub type ST_MAC_MLMEPollCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pPollCnf: *const ST_MAC_pollCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME DPS Confirmation"]
pub type ST_MAC_MLMEDpsCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pDpsCnf: *const ST_MAC_dpsCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME SOUNDING Confirmation"]
pub type ST_MAC_MLMESoundingCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pSoudingCnf: *const ST_MAC_soundingCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME CALIBRATE Confirmation"]
pub type ST_MAC_MLMECalibrateCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pCalibrateCnf: *const ST_MAC_calibrateCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMCPS DATA Confirmation"]
pub type ST_MAC_MCPSDataCnfCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pDataCnf: *const ST_MAC_dataCnf_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMCPS PURGE Confirmation"]
pub type ST_MAC_MCPSPurgeCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pPurgeCnf: *const ST_MAC_purgeCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Beacon confirm"]
pub type ST_MAC_MLMEBeaconCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pBeaconCnf: *const ST_MAC_beaconCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME GetPwrInfoTable confirm"]
pub type ST_MAC_MLMEGetPwrInfoTableCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pGetPwrInfoTableCnf: *const ST_MAC_getPwrInfoTableCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME SetPwrInfoTable confirm"]
pub type ST_MAC_MLMESetPwrInfoTableCnfCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pSetPwrInfoTableCnf: *const ST_MAC_setPwrInfoTableCnf_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Associate Indication*/\n/"]
pub type ST_MAC_MLMEAssociateIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pAssociateInd: *const ST_MAC_associateInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Disassociate Indication"]
pub type ST_MAC_MLMEDisassociateIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pDisassociateInd: *const ST_MAC_disassociateInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Beacon Notification Indication"]
pub type ST_MAC_MLMEBeaconNotifyIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pBeaconNotifyInd: *const ST_MAC_beaconNotifyInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Communication Status Indication"]
pub type ST_MAC_MLMECommStatusIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pCommStatusInd: *const ST_MAC_commStatusInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME GTS Indication"]
pub type ST_MAC_MLMEGtsIndCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pGtsInd: *const ST_MAC_GtsInd_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME Orphan Indication"]
pub type ST_MAC_MLMEOrphanIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pOrphanInd: *const ST_MAC_orphanInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME SYNC LOSS Indication"]
pub type ST_MAC_MLMESyncLossIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pSyncLoss: *const ST_MAC_syncLoss_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME DPS Indication"]
pub type ST_MAC_MLMEDpsIndCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pDdsInd: *const ST_MAC_dpsInd_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMCPS DATA Indication"]
pub type ST_MAC_MCPSDataIndCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pDataInd: *const ST_MAC_dataInd_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME POLL Indication"]
pub type ST_MAC_MLMEPollIndCbPtr =
    ::core::option::Option<unsafe extern "C" fn(pPollInd: *const ST_MAC_pollInd_t) -> MAC_Status_t>;
#[doc = "/\n/** @brief Defines the callback function Pointer type for\nMLME BeaconReq Indication"]
pub type ST_MAC_MLMEBeaconReqIndCbPtr = ::core::option::Option<
    unsafe extern "C" fn(pBeaconReqInd: *const ST_MAC_beaconReqInd_t) -> MAC_Status_t,
>;
#[doc = "/\n/** @brief Defines callback init structure to be set by NWK upper layer before\nMAC Initialisation"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_callbacks_t {
    pub mlmeAssociateCnfCb: ST_MAC_MLMEAssociateCnfCbPtr,
    pub mlmeAssociateIndCb: ST_MAC_MLMEAssociateIndCbPtr,
    pub mlmeBeaconNotifyIndCb: ST_MAC_MLMEBeaconNotifyIndCbPtr,
    pub mlmeCalibrateCnfCb: ST_MAC_MLMECalibrateCnfCbPtr,
    pub mlmeCommStatusIndCb: ST_MAC_MLMECommStatusIndCbPtr,
    pub mlmeDisassociateCnfCb: ST_MAC_MLMEDisassociateCnfCbPtr,
    pub mlmeDisassociateIndCb: ST_MAC_MLMEDisassociateIndCbPtr,
    pub mlmeDpsCnfCb: ST_MAC_MLMEDpsCnfCbPtr,
    pub mlmeDpsIndCb: ST_MAC_MLMEDpsIndCbPtr,
    pub mlmeGetCnfCb: ST_MAC_MLMEGetCnfCbPtr,
    pub mlmeGtsCnfCb: ST_MAC_MLMEGtsCnfCbPtr,
    pub mlmeGtsIndCb: ST_MAC_MLMEGtsIndCbPtr,
    pub mlmeOrphanIndCb: ST_MAC_MLMEOrphanIndCbPtr,
    pub mlmePollCnfCb: ST_MAC_MLMEPollCnfCbPtr,
    pub mlmeResetCnfCb: ST_MAC_MLMEResetCnfCbPtr,
    pub mlmeRxEnableCnfCb: ST_MAC_MLMERxEnableCnfCbPtr,
    pub mlmeScanCnfCb: ST_MAC_MLMEScanCnfCbPtr,
    pub mlmeSetCnfCb: ST_MAC_MLMESetCnfCbPtr,
    pub mlmeSoundingCnfCb: ST_MAC_MLMESoundingCnfCbPtr,
    pub mlmeStartCnfCb: ST_MAC_MLMEStartCnfCbPtr,
    pub mlmeSyncLossIndCb: ST_MAC_MLMESyncLossIndCbPtr,
    pub mcpsDataIndCb: ST_MAC_MCPSDataIndCbPtr,
    pub mcpsDataCnfCb: ST_MAC_MCPSDataCnfCbPtr,
    pub mcpsPurgeCnfCb: ST_MAC_MCPSPurgeCnfCbPtr,
    pub mlmePollIndCb: ST_MAC_MLMEPollIndCbPtr,
    pub mlmeBeaconReqIndCb: ST_MAC_MLMEBeaconReqIndCbPtr,
    pub mlmeBeaconCnfCb: ST_MAC_MLMEBeaconCnfCbPtr,
    pub mlmeGetPwrInfoTableCnfCb: ST_MAC_MLMEGetPwrInfoTableCnfCbPtr,
    pub mlmeSetPwrInfoTableCnfCb: ST_MAC_MLMESetPwrInfoTableCnfCbPtr,
}
unsafe extern "C" {
    pub fn ST_MAC_preInit();
}
unsafe extern "C" {
    pub fn ST_MAC_init(macCallback: *mut ST_MAC_callbacks_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEAssociateReq(
        st_mac_hndl: MAC_handle,
        pAssociateReq: *const ST_MAC_associateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEDisassociateReq(
        st_mac_hndl: MAC_handle,
        pDisassiociateReq: *const ST_MAC_disassociateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEGetReq(
        st_mac_hndl: MAC_handle,
        pGetReq: *const ST_MAC_getReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEGtsReq(
        st_mac_hndl: MAC_handle,
        pGtsReq: *const ST_MAC_gtsReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEResetReq(
        st_mac_hndl: MAC_handle,
        pResetReq: *const ST_MAC_resetReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMERxEnableReq(
        st_mac_hndl: MAC_handle,
        pRxEnableReq: *const ST_MAC_rxEnableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEScanReq(
        st_mac_hndl: MAC_handle,
        pScanReq: *const ST_MAC_scanReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMESetReq(
        st_mac_hndl: MAC_handle,
        pSetReq: *const ST_MAC_setReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEStartReq(
        st_mac_hndl: MAC_handle,
        pStartReq: *const ST_MAC_startReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEPollReq(
        st_mac_hndl: MAC_handle,
        pPollReq: *const ST_MAC_pollReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEDpsReq(
        st_mac_hndl: MAC_handle,
        pDpsReq: *const ST_MAC_dpsReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMESoundingReq(
        st_mac_hndl: MAC_handle,
        pSoundingReq: *const ST_MAC_soundingReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMECalibrateReq(
        st_mac_hndl: MAC_handle,
        pCalibrateReq: *const ST_MAC_calibrateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEBeaconReq(
        st_mac_hndl: MAC_handle,
        pBeaconReq: *const ST_MAC_beaconReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEGetPwrInfoTableReq(
        st_mac_hndl: MAC_handle,
        pGetPwrInfoTableReq: *const ST_MAC_getPwrInfoTableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMESetPwrInfoTableReq(
        st_mac_hndl: MAC_handle,
        pSetPwrInfoTableReq: *const ST_MAC_setPwrInfoTableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MCPSDataReq(
        st_mac_hndl: MAC_handle,
        pDataReq: *const ST_MAC_dataReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MCPSPurgeReq(
        st_mac_hndl: MAC_handle,
        pPurgeReq: *const ST_MAC_purgeReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEAssociateRes(
        st_mac_hndl: MAC_handle,
        pAssociateRes: *const ST_MAC_associateRes_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_MLMEOrphanRes(
        st_mac_hndl: MAC_handle,
        pOrphanRes: *const ST_MAC_orphanRes_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn MacSys_Init();
}
unsafe extern "C" {
    pub fn MacSys_Resume();
}
unsafe extern "C" {
    pub fn MacSys_SemaphoreSet();
}
unsafe extern "C" {
    pub fn MacSys_SemaphoreWait();
}
unsafe extern "C" {
    pub fn MacSys_EventSet();
}
unsafe extern "C" {
    pub fn MacSys_EventWait();
}
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
unsafe extern "C" {
    #[doc = " @brief  Enqueue function used in the MAC wrapper"]
    pub fn ST_MAC_enqueue_radio_Incoming(
        command_id: u8,
        st_mac_hndl: MAC_handle,
        req_len: u8,
        reqPtr: *mut ::core::ffi::c_void,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_enqueue_radio_Incoming_with_payload(
        command_id: u8,
        st_mac_hndl: MAC_handle,
        req_len: u8,
        reqPtr: *mut ::core::ffi::c_void,
        payload: *mut u8,
        payload_len: u8,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_enqueue_req(
        command_id: u8,
        st_mac_hndl: MAC_handle,
        req_len: u8,
        reqPtr: *mut ::core::ffi::c_void,
    ) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Function allows to serialize pan description for MLME-BeaconNotify.indication"]
    pub fn mac_serialize_pan_desc_to_ST(
        pan_desc: *mut ST_MAC_PAN_Desc_t,
        pan_desc_snps: *mut pan_descr_st,
    );
}
unsafe extern "C" {
    #[doc = " @brief  Function allows to identify if a beacon is already received with these information like channel, PANID, addr mode, short/ext address"]
    pub fn mac_unique_list_PAN_beacon(MAC_PAN_Desc: ST_MAC_PAN_Desc_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief  Interface Functions"]
    pub fn is_MAC_ready(mac_command_id: u8) -> u8;
}
unsafe extern "C" {
    pub fn mac_command_validate(command_id: u8, command_len: u8) -> u8;
}
unsafe extern "C" {
    pub fn mac_get_pib_attribute_len(mac_cntx_ptr: *mut ::core::ffi::c_void, pib_attr_id: u8)
        -> u8;
}
unsafe extern "C" {
    pub fn ST_MAC_get_mac_interface(mac_cntx_ptr: *mut ::core::ffi::c_void) -> MAC_handle;
}
unsafe extern "C" {
    pub fn MAC_config() -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn MAC_reset() -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief  Main function"]
    pub fn mac_baremetal_run();
}
unsafe extern "C" {
    #[doc = " Public Variable Declaration"]
    #[link_name = "\u{1}ga_Heap"]
    pub static mut GA_HEAP: [u8; 0usize];
}
unsafe extern "C" {
    #[link_name = "\u{1}ga_Buffer_Link_Array"]
    pub static mut GA_BUFFER_LINK_ARRAY: [u8; 0usize];
}
unsafe extern "C" {
    #[doc = " @brief  Interface to access m_SMALL_BUFFER_SIZE_c from lib"]
    #[link_name = "\u{1}g_Small_Buffer_Size_c"]
    pub static G_SMALL_BUFFER_SIZE_C: u8;
}
unsafe extern "C" {
    #[doc = " @brief  Interface to access m_LARGE_BUFFER_SIZE_c from lib"]
    #[link_name = "\u{1}g_Large_Buffer_Size_c"]
    pub static G_LARGE_BUFFER_SIZE_C: u8;
}
unsafe extern "C" {
    #[doc = " @brief  Interface to access m_TOTAL_NUMBER_OF_SMALL_BUFFERS_c from lib"]
    #[link_name = "\u{1}g_Total_Number_Of_Small_Buffers_c"]
    pub static G_TOTAL_NUMBER_OF_SMALL_BUFFERS_C: u8;
}
unsafe extern "C" {
    #[doc = " @brief  Interface to access m_TOTAL_NUMBER_OF_LARGE_BUFFERS_c from lib"]
    #[link_name = "\u{1}g_Total_Number_Of_Large_Buffers_c"]
    pub static G_TOTAL_NUMBER_OF_LARGE_BUFFERS_C: u8;
}
unsafe extern "C" {
    #[doc = " @brief  Interface to access m_TOTAL_NUMBER_OF_BUFFERS_c from lib"]
    #[link_name = "\u{1}g_Total_Number_Of_Buffers_c"]
    pub static G_TOTAL_NUMBER_OF_BUFFERS_C: u8;
}
unsafe extern "C" {
    pub fn ST_MAC_sys_svc_ant_div_en(st_mac_hndl: *mut MAC_handle) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_sys_svc_ant_div_dis(st_mac_hndl: *mut MAC_handle) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_sys_svc_ant_div_init(
        st_mac_hndl: *mut MAC_handle,
        rssi_threshold: i8,
        default_ant_id: u8,
        coord_short_addr: u16,
        coord_ext_addr: *mut u8,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_sys_svc_ant_div_set_intrv_radio_evts(
        st_mac_hndl: *mut MAC_handle,
        radio_evts: u32,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_sys_svc_ant_div_set_intrv_time(
        st_mac_hndl: *mut MAC_handle,
        time_us: u32,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEAssociateReq(
        st_mac_hndl: *mut MAC_handle,
        pAssociateReq: *const ST_MAC_associateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDisassociateReq(
        st_mac_hndl: *mut MAC_handle,
        pDisassiociateReq: *const ST_MAC_disassociateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEGetReq(
        st_mac_hndl: *mut MAC_handle,
        pGetReq: *const ST_MAC_getReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEGtsReq(
        st_mac_hndl: *mut MAC_handle,
        pGtsReq: *const ST_MAC_gtsReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEResetReq(
        st_mac_hndl: *mut MAC_handle,
        pResetReq: *const ST_MAC_resetReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMERxEnableReq(
        st_mac_hndl: *mut MAC_handle,
        pRxEnableReq: *const ST_MAC_rxEnableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEScanReq(
        st_mac_hndl: *mut MAC_handle,
        pScanReq: *const ST_MAC_scanReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESetReq(
        st_mac_hndl: *mut MAC_handle,
        pSetReq: *const ST_MAC_setReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEStartReq(
        st_mac_hndl: *mut MAC_handle,
        pStartReq: *const ST_MAC_startReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEPollReq(
        st_mac_hndl: *mut MAC_handle,
        pPollReq: *const ST_MAC_pollReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDpsReq(
        st_mac_hndl: *mut MAC_handle,
        pDpsReq: *const ST_MAC_dpsReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESoundingReq(
        st_mac_hndl: *mut MAC_handle,
        pSoundingReq: *const ST_MAC_soundingReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMECalibrateReq(
        st_mac_hndl: *mut MAC_handle,
        pCalibrateReq: *const ST_MAC_calibrateReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEBeaconReq(
        st_mac_hndl: *mut MAC_handle,
        pbeaconReq: *const ST_MAC_beaconReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEGetPwrInfoTableReq(
        st_mac_hndl: *mut MAC_handle,
        pGetPwrInfoTableReq: *const ST_MAC_getPwrInfoTableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESetPwrInfoTableReq(
        st_mac_hndl: *mut MAC_handle,
        pSetPwrInfoTableReq: *const ST_MAC_setPwrInfoTableReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MCPSDataReq(
        st_mac_hndl: *mut MAC_handle,
        pDataReq: *const ST_MAC_dataReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MCPSPurgeReq(
        st_mac_hndl: *mut MAC_handle,
        pPurgeReq: *const ST_MAC_purgeReq_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEAssociateRes(
        st_mac_hndl: *mut MAC_handle,
        pAssociateRes: *const ST_MAC_associateRes_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEOrphanRes(
        st_mac_hndl: *mut MAC_handle,
        pOrphanRes: *const ST_MAC_orphanRes_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEResetCnf(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESetCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        status: u8,
        pib_attr_id: u8,
        pib_attr_indx: u8,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEScanCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        mlme_scan_cmf_params: *mut mlme_scn_cmf_param_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEAssociationCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        status: u8,
        short_addr: u16,
        ptr_sec_params: *mut prim_sec_param_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEStartCnf(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEGetCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        status: u8,
        pib_attr_id: u8,
        pib_attr_indx: u8,
        pib_attr_val: *mut attr_val_t,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMERxEnableCnf(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEBeaconCnf(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEGetPwrInfoTableCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        status: u8,
        short_addrs: u16,
        ptr_ext_addrs: *mut u8,
        tx_pwr_level: i8,
        last_rssi_level: i8,
        nwk_negotiated: u8,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESetPwrInfoTableCnf(mac_cntx_ptr: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEOrphanInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mlme_orphan_ind_params: *mut mlme_orphan_ind_st_t,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDataPollCnf(ptr_mac_cntx: *mut ::core::ffi::c_void, status: u8);
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDisassociationCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mlme_disassoc_cfm_params: *mut mlme_disassoc_cfm_params_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEBeaconReqInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_bcon_req_ind_params: *mut mlme_bcon_req_ind_params_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MCPSDataCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mcps_data_cnf: *mut mcps_data_cfm_params_st_t,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MCPSDataPurgeCnf(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        msdu_hndl: u8,
        status: u8,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEBeaconNotifyInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_bcon_notify_params: *mut mlme_bcon_notfy_params_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEComStatusInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mlme_comm_status: *mut mlme_comm_status_st_t,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDataPollInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        addr_mode: u8,
        dev_addr: *mut u8,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMESyncLossInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_st_mlme_sync_loss: *mut mlme_sync_loss_params_st_t,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEDisassociationInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mlme_disassoc_ind_params: *mut mlme_disassoc_ind_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MLMEAssociationInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mlme_assoc_ind_params: *mut mlme_assoc_ind_param_st,
    );
}
unsafe extern "C" {
    pub fn ST_MAC_handle_MCPSDataInd(
        mac_cntx_ptr: *mut ::core::ffi::c_void,
        ptr_mcps_data_ind: *mut mcps_indicate_params_st_t,
    );
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
pub type __darwin_nl_item = ::core::ffi::c_int;
pub type __darwin_wctrans_t = ::core::ffi::c_int;
pub type __darwin_wctype_t = ::core::ffi::c_ulong;
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
pub type rsize_t = __darwin_size_t;
pub type wchar_t = __darwin_wchar_t;
pub type wint_t = __darwin_wint_t;
pub type max_align_t = f64;
unsafe extern "C" {
    pub fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn memmove(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn memset(
        s: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn memcmp(
        s1: *const ::core::ffi::c_void,
        s2: *const ::core::ffi::c_void,
        n: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn strlen(s: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    pub fn strnlen(s: *const ::core::ffi::c_char, maxlen: usize) -> usize;
}
unsafe extern "C" {
    pub fn strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn strncpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn strncat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        n: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn strcmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn strncmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
        n: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
#[doc = " @brief Ext Ack Frame struct (returned in Tx success callback)"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_ExtAckFrame_s {
    pub Frame: [u8; 127usize],
    pub Len: u8,
}
impl Default for ST_MAC_ExtAckFrame_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief this structure is used by default, but user can override this struct by is own\n thanks ST_MAC_ParseRxFrame weak function\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_ExtRxFrame_s {
    pub Frame: *mut u8,
    pub Len: u8,
    pub rssi: i8,
    pub lqi: u8,
}
impl Default for ST_MAC_ExtRxFrame_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const ST_MAC_ExtBitRate_t_EXT_RATE_125K: ST_MAC_ExtBitRate_t = 0;
pub const ST_MAC_ExtBitRate_t_EXT_RATE_256K: ST_MAC_ExtBitRate_t = 1;
pub const ST_MAC_ExtBitRate_t_EXT_RATE_1M: ST_MAC_ExtBitRate_t = 2;
pub const ST_MAC_ExtBitRate_t_EXT_RATE_2M: ST_MAC_ExtBitRate_t = 3;
#[doc = " @brief Extended Rate, Extended Service is abled to work at diffenrent radio rate"]
pub type ST_MAC_ExtBitRate_t = ::core::ffi::c_uint;
pub const ST_MAC_ExtTxMode_t_EXT_TX_AUTO: ST_MAC_ExtTxMode_t = 0;
pub const ST_MAC_ExtTxMode_t_EXT_TX_MANUAL: ST_MAC_ExtTxMode_t = 1;
#[doc = " @brief Extended Transission Mode"]
pub type ST_MAC_ExtTxMode_t = ::core::ffi::c_uint;
pub const ST_MAC_ExtAckMode_t_EXT_ACK_ENABLE: ST_MAC_ExtAckMode_t = 0;
pub const ST_MAC_ExtAckMode_t_EXT_ACK_DISABLE: ST_MAC_ExtAckMode_t = 1;
#[doc = " @brief Extended Ack mode of the DRx device\n EXT_ACK_ENABLE or EXT_ACK_DISABLE"]
pub type ST_MAC_ExtAckMode_t = ::core::ffi::c_uint;
#[doc = " @brief Extended Ack config of the device Exammple :\n DEFAULT_ACK_CONFIG"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_ExtAckCfg {
    pub TurnaroundTime: u16,
    pub Timeout: u16,
}
#[doc = " @brief Extended Svc Tx config of the device Exammple :\n DEFAULT_TX_CONFIG"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_ExtTxCfg {
    pub TxMode: ST_MAC_ExtTxMode_t,
    pub TxIfs: u16,
    pub TxRetryTime: u16,
    pub TxRetryCount: u8,
}
impl Default for ST_MAC_ExtTxCfg {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Extended Svc config of the device Exammple :\n DEFAULT_EXT_CONFIG\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_ExtConfig {
    pub AckMode: ST_MAC_ExtAckMode_t,
    pub Bitrate: ST_MAC_ExtBitRate_t,
    pub Channel: u8,
    pub Power: i8,
    pub AckCfg: *mut ST_MAC_ExtAckCfg,
    pub TxCfg: *mut ST_MAC_ExtTxCfg,
}
impl Default for ST_MAC_ExtConfig {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const ST_MAC_ExtState_EXT_NOT_INIT: ST_MAC_ExtState = 0;
pub const ST_MAC_ExtState_EXT_IDLE: ST_MAC_ExtState = 1;
pub const ST_MAC_ExtState_EXT_TX: ST_MAC_ExtState = 2;
pub const ST_MAC_ExtState_EXT_RX: ST_MAC_ExtState = 3;
pub const ST_MAC_ExtState_EXT_ED: ST_MAC_ExtState = 4;
pub const ST_MAC_ExtState_EXT_TX_US: ST_MAC_ExtState = 5;
#[doc = " @brief Current state of the device"]
pub type ST_MAC_ExtState = ::core::ffi::c_uint;
pub const ST_MAC_ExtRxCont_EXT_RX_CONT_DISABLE: ST_MAC_ExtRxCont = 0;
pub const ST_MAC_ExtRxCont_EXT_RX_CONT_ENABLE: ST_MAC_ExtRxCont = 1;
#[doc = " @brief status of continuous reception"]
pub type ST_MAC_ExtRxCont = ::core::ffi::c_uint;
#[doc = " @brief Struct Callback dispatcher, user will receive this callback in case of\n Transmission success or failure\n Reception Done\n Scan done"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_ExtCallback_Dispatcher {
    pub ext_tx_success:
        ::core::option::Option<unsafe extern "C" fn(AckFrame: *mut ST_MAC_ExtAckFrame_s)>,
    pub ext_tx_failure: ::core::option::Option<unsafe extern "C" fn()>,
    pub ext_rx_done: ::core::option::Option<
        unsafe extern "C" fn(arg1: MAC_Status_t, arg2: *mut ::core::ffi::c_void),
    >,
    pub ext_scan_done:
        ::core::option::Option<unsafe extern "C" fn(arg1: u64, arg2: u8, arg3: u8, arg4: *mut u8)>,
}
unsafe extern "C" {
    #[doc = " @brief (weak) ST_MAC_ParseRxFrame defined as weak can be overridden by user to parse frame at ext service layer\n Hardware radio automatically adds CRC, in this function CRC is returned at end of InFrame\n\n @param InFrame* : RAW Frame (CRC included +2 bytes at end of frame)\n        InLen    : Total Length of the RAW Frame (CRC included = +2 bytes)\n        rssi     : receive streng signal indicator\n        lqi      : link quality indicator\n @retval void* Frame Parsed, return NULL if errors occurs during parsing"]
    pub fn ST_MAC_ParseRxFrame(
        InFrame: *mut u8,
        InLen: u8,
        rssi: i8,
        lqi: u8,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    #[doc = " @brief (weak) ST_MAC_ExtRxFilter defined as weak can be overridden by user to filter frame at ext service layer\n if ST_MAC_ParseRxFrame has been defined by user InFrameStruct has same type else : ST_MAC_ExtRxFrame_s*\n\n @param InFrameStruct* : Frame Parsed to filter\n @retval 0 = frame not filtered else frame will be filtered"]
    pub fn ST_MAC_ExtRxFilter(InFrameStruct: *mut ::core::ffi::c_void) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief (weak) ST_MAC_custom_ack_cb defined as weak can be overridden by user but need to be optimized for fast execution\n By default weak function is defined as IEEE Ack 15.4\n ACK_MEMCPY must be used to copy payload (avoid hardfault because of unaligned access)\n\n @param Ack : Ack Frame to fulfill (CRC added automatically by the HW radio)\n        AckLen : Size of Ack Frame (CRC length will be added automatically)\n        Rxframe : Frame has just received (sequence number can be retrieved from Rxframe for ex)"]
    pub fn ST_MAC_custom_ack_cb(Ack: *mut u8, AckLen: *mut u16, Rxframe: *mut u8);
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtInit\n\n @brief Init Extended Mac service with given configuration and link user callback\n Configuration example : DEFAULT_EXT_CONFIG\n\n @param   *pStExtMacCfg : [in] configuration of extended svc\n          * pExtCallbackDispatcher : [in] user callback (ext_tx_success, ext_tx_failure, ext_rx_done, ext_scan_done)\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtInit(
        pStExtMacCfg: *mut ST_MAC_ExtConfig,
        pExtCallbackDispatcher: *mut ST_MAC_ExtCallback_Dispatcher,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtSetPower\n\n @brief set Tx power in dbm\n\n @param   eExtPower : [in] Tx power in dbm (min -20dbm, max 10dbm)\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtSetPower(eExtPower: i8) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtSetRate\n\n @brief set radio Rate in Kbps\n\n @param   eExtRate : [in] radio in bps (  EXT_RATE_125K, EXT_RATE_256K, EXT_RATE_1M, EXT_RATE_2M)\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtSetRate(eExtRate: ST_MAC_ExtBitRate_t) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtSetTxIfs\n\n @brief set Tx interframe spacing\n (minimal time between 2 transmission at radio level, time between last byte of first frame and first byte of second frame)\n\n @param   TxIfs : [in] Tx  interframe spacing (us) min : 140 max 1000\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtSetTxIfs(TxIfs: u16) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtSetChannel\n\n @brief Set radio frequency channel (dependent of radio rate)\n EXT_RATE_125K, EXT_RATE_256K : min 11 max 26\n EXT_RATE_1M, EXT_RATE_2M : min 11 max 50\n Automatically resume RX if needed\n\n @param   uExtChannel : [in] channel\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtSetChannel(uExtChannel: u8) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtSetAck\n\n @brief Set ack config : Custom Ack or Mac Ack. Only custom ack supported for the moment\n\n @param   eAckMode : [in] ST_MAC_ExtAckMode_t Enable or disable\n         *pCustomAckcfg : [in] Ack config example DEFAULT_ACK_CONFIG\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtSetAck(
        eAckMode: ST_MAC_ExtAckMode_t,
        pCustomAckcfg: *mut ST_MAC_ExtAckCfg,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtStartStopAck\n\n @brief Stop or Start acknowledgement at runtime\n\n @param   eAckMode : [in] ST_MAC_ExtAckMode_t Enable or disable\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtStartStopAck(eAckMode: ST_MAC_ExtAckMode_t) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtPushTxFIFO\n\n @brief Push Frame in Tx FIFO,\n      if EXT_TX_AUTO is set then the frame will be send as soon as possible\n      else user needs to call ST_MAC_ExtStartTx()\n\n @param   *TxFrame : [in] TxFrame to send\n          Len      : [in] Size of the payloas\n          eAck     : [in] ST_MAC_ExtAckMode_t Frame must be acknowledge by DRx or not\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtPushTxFIFO(
        Payload: *mut u8,
        Len: u8,
        eAck: ST_MAC_ExtAckMode_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtStartTx\n\n @brief Use to start transmission if EXT_TX_MANUAL is set\n\n @param  void\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtStartTx() -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtStartRx\n\n @brief Start reception in continuous mode or one time mode\n\n @param  ST_MAC_ExtRxCont Continuous_Rx : Start_Rx in continuous mode or not\n if Continuous_Rx is set to EXT_RX_CONT_ENABLE, device will resume Rx after any event\n Rx done callback will be called each time correct frame is received\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtStartRx(Continuous_Rx: ST_MAC_ExtRxCont) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtStopRx\n\n @brief Stop Reception\n\n @param  void\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtStopRx() -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_GetState\n\n @brief Get state of Device\n\n @param  void\n\n @retval ST_MAC_ExtState (EXT_NOT_INIT, EXT_IDLE, EXT_TX, EXT_RX)"]
    pub fn ST_MAC_GetState() -> ST_MAC_ExtState;
}
unsafe extern "C" {
    #[doc = " @fn ST_MAC_ExtStartScan\n\n @brief Use to start scan on different channel for DRx device,\n Scan callback function will be called when scan is finished\n\n @param  channel_mask : (uint64_t) mask must significant bit correspond to channel 11\n example : 0x9010000000000000 -> channel 11, 14, 22:\n           EXT_SCAN_ALL_CHANNEL_KPBS = 0xffff000000000000 -> channel 11 to 26\n           EXT_SCAN_ALL_CHANNEL_MPBS = 0xffffffffff000000 -> channel 11 to 50\n\n @param ScanDurationChannel : Scanning time per channel (us) (increase Scanning time increase accuracy)\n\n @retval MAC_Status_t"]
    pub fn ST_MAC_ExtStartScan(channel_mask: u64, ScanDurationChannel: u32) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = "/\n/** @brief Holder for events, to be accessed through macros. In general, each events\nshould have a set, clear, is_set macros."]
    #[link_name = "\u{1}mac_radio_evts"]
    pub static mut MAC_RADIO_EVTS: u32;
}
pub const MAC_RAW_State_t_ST_MAC_UNITIALIZED: MAC_RAW_State_t = 0;
pub const MAC_RAW_State_t_ST_MAC_IDLE: MAC_RAW_State_t = 1;
pub const MAC_RAW_State_t_ST_MAC_TX: MAC_RAW_State_t = 2;
pub const MAC_RAW_State_t_ST_MAC_RX: MAC_RAW_State_t = 3;
pub const MAC_RAW_State_t_ST_MAC_ED: MAC_RAW_State_t = 4;
pub const MAC_RAW_State_t_ST_MAC_CCA: MAC_RAW_State_t = 5;
pub const MAC_RAW_State_t_ST_MAC_NOTIF: MAC_RAW_State_t = 6;
pub const MAC_RAW_State_t_ST_MAC_UNKNOWN: MAC_RAW_State_t = 7;
#[doc = " @brief Defines the current operation of the radio"]
pub type MAC_RAW_State_t = ::core::ffi::c_uint;
pub const MAC_RAW_TX_Type_t_ST_MAC_NONE_TX: MAC_RAW_TX_Type_t = 0;
pub const MAC_RAW_TX_Type_t_ST_MAC_STANDARD_TX: MAC_RAW_TX_Type_t = 1;
pub const MAC_RAW_TX_Type_t_ST_MAC_CONTINUOUS_TX: MAC_RAW_TX_Type_t = 2;
pub const MAC_RAW_TX_Type_t_ST_MAC_CONTINUOUS_WAVE: MAC_RAW_TX_Type_t = 3;
pub const MAC_RAW_TX_Type_t_ST_MAC_UNKNOWN_TX: MAC_RAW_TX_Type_t = 4;
#[doc = " @brief Defines the type of transmission"]
pub type MAC_RAW_TX_Type_t = ::core::ffi::c_uint;
pub const MAC_RAW_TX_Status_t_ST_TX_SUCCESS: MAC_RAW_TX_Status_t = 0;
pub const MAC_RAW_TX_Status_t_ST_TX_ACK_TIMEOUT: MAC_RAW_TX_Status_t = 1;
pub const MAC_RAW_TX_Status_t_ST_TX_FAILED: MAC_RAW_TX_Status_t = 2;
pub const MAC_RAW_TX_Status_t_ST_TX_CHANNEL_ACCESS_FAILURE: MAC_RAW_TX_Status_t = 3;
#[doc = " @brief TX status"]
pub type MAC_RAW_TX_Status_t = ::core::ffi::c_uint;
pub const MAC_RAW_RX_Status_t_ST_RX_SUCCESS: MAC_RAW_RX_Status_t = 0;
pub const MAC_RAW_RX_Status_t_ST_RX_NO_PAYLOAD: MAC_RAW_RX_Status_t = 1;
pub const MAC_RAW_RX_Status_t_ST_RX_REJECTED: MAC_RAW_RX_Status_t = 2;
pub const MAC_RAW_RX_Status_t_ST_RX_FILTERED: MAC_RAW_RX_Status_t = 3;
pub const MAC_RAW_RX_Status_t_ST_RX_INVALID_FCS: MAC_RAW_RX_Status_t = 4;
#[doc = " @brief TX status"]
pub type MAC_RAW_RX_Status_t = ::core::ffi::c_uint;
#[doc = " 802.15.4 PHY ONLY"]
pub const ST_MAC_Config_Mode_ST_RAW_CONFIG: ST_MAC_Config_Mode = 0;
#[doc = " 802.15.4 custom/proprietary"]
pub const ST_MAC_Config_Mode_ST_EXT_CONFIG: ST_MAC_Config_Mode = 1;
#[doc = " 802.15.4 PHY + BLE"]
pub const ST_MAC_Config_Mode_ST_CR_CONFIG: ST_MAC_Config_Mode = 2;
#[doc = " @brief Initialisation config"]
pub type ST_MAC_Config_Mode = ::core::ffi::c_uint;
#[doc = " @brief Defines the structure holding Radio capabilities"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_raw_caps_t {
    #[doc = " CCA threshold range (dbm)"]
    pub min_cca_threshold: i8,
    pub max_cca_threshold: i8,
    #[doc = " Transmission power range (dbm)"]
    pub min_tx_power: i8,
    pub max_tx_power: i8,
}
#[doc = " @brief Defines the structure for sending one or more frames regardless of the\nformat. Only the PHY preamble at the start and the FCS field at the end are added."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_raw_TX_start_t {
    #[doc = " The channel on which to send to the frame"]
    pub channel_number: u8,
    #[doc = " Frame power (dbm)"]
    pub power: i8,
    #[doc = " Type of TX"]
    pub tx_type: MAC_RAW_TX_Type_t,
    #[doc = " Payload to send (case ST_MAC_STANDARD_TX only)"]
    pub payload: [u8; 125usize],
    #[doc = " Payload length (CRC included)"]
    pub payload_len: u8,
    #[doc = " Number of frames to send"]
    pub frames_number: u16,
    #[doc = " Delay between each frames (in ms)"]
    pub delay_ms: u16,
    #[doc = " Stop next TX if current one failed (0: continue, otherwise stop)"]
    pub stopTx_if_failure: u8,
}
impl Default for ST_MAC_raw_TX_start_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for starting the radio in reception."]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_raw_RX_start_t {
    #[doc = " The channel on which to receive frames"]
    pub channel_number: u8,
    #[doc = " Reception duration in ms"]
    pub period: u32,
    #[doc = " Max number of frames to receive; once reached stop the reception"]
    pub frames_number: u16,
}
#[doc = " @brief Defines the structure to request a Clear Channel Assessement"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_raw_CCA_t {
    #[doc = " The channel on which to perform the CCA"]
    pub channel_number: u8,
    #[doc = " CCA threshold in dbm"]
    pub cca_threshold: i8,
    #[doc = " CCA result: 1: clear, 0: busy"]
    pub cca_result: u8,
}
#[doc = " @brief Defines the structure to request an ED scan"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_raw_EDscan_t {
    #[doc = " Channel"]
    pub channel: u8,
}
#[doc = " @brief Defines the structure for an ED scan result"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_rw_EDscan_result_t {
    #[doc = " ED result"]
    pub ed: u8,
}
#[doc = " @brief Defines the structure for a single RX event"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_raw_single_RX_event_t {
    #[doc = " Status of Reception"]
    pub rx_status: MAC_RAW_RX_Status_t,
    #[doc = " Pointer to payload of frame received"]
    pub payload_ptr: *mut u8,
    #[doc = " Size of frame received"]
    pub payload_len: u8,
    #[doc = " RSSI of frame received"]
    pub rssi: i8,
    #[doc = " LQI of frame received"]
    pub lqi: u8,
}
impl Default for ST_MAC_raw_single_RX_event_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines the structure for a signel TX event"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ST_MAC_raw_single_TX_event_t {
    #[doc = " Status of transmission"]
    pub tx_status: MAC_RAW_TX_Status_t,
    #[doc = " Pointer to ACK if any (NULL otherwise)"]
    pub ack_ptr: *mut u8,
    #[doc = " Size of ACK (0 if any)"]
    pub ack_length: u16,
}
impl Default for ST_MAC_raw_single_TX_event_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Defines callback init structure for optional events\nin MAC Initialisation"]
pub type ST_MAC_raw_Notif_callback =
    ::core::option::Option<unsafe extern "C" fn(state: MAC_RAW_State_t)>;
pub type ST_MAC_raw_single_RX_cbk =
    ::core::option::Option<unsafe extern "C" fn(p_RX_evt: *const ST_MAC_raw_single_RX_event_t)>;
pub type ST_MAC_raw_single_TX_cbk =
    ::core::option::Option<unsafe extern "C" fn(p_TX_evt: *const ST_MAC_raw_single_TX_event_t)>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ST_MAC_Raw_event_callbacks_t {
    pub p_Notif: ST_MAC_raw_Notif_callback,
    pub p_RX_Done: ST_MAC_raw_single_RX_cbk,
    pub p_TX_Done: ST_MAC_raw_single_TX_cbk,
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_init allow you to initialize the raw layer. there are 3 modes available.\n RAW_CONFIG allows you to send and received frame in 802.15.4 only.\n EXT_CONFIG not yet implemented. if used always return MAC_INVALID_PARAMETER.\n CR_CONFIG allows you to send and received frame in 802.15.4 and have BLE in concurrent.\n\n @param *p_callback : [in] callback for TX done, RX received, state notify.\n        *config : [in] config selected.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_init(
        p_callback: *mut ST_MAC_Raw_event_callbacks_t,
        config: ST_MAC_Config_Mode,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_get_caps allows you to get information: min_cca_threshold, max_cca_threshold, min_tx_power and max_tx_power.\n\n @param *MAC_handle : [in] MAC instance\n        *ST_MAC_raw_caps_t : [in] raw_caps structure allow you to get CCA and TxPower information.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_get_caps(
        st_mac_hndl: *mut MAC_handle,
        pRawCaps: *mut ST_MAC_raw_caps_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_start_TX allows you to send frame in 802.15.4.\n Depending on initialization mode, can also generate noise on a selected channel.\n\n @param *MAC_handle : [in] MAC instance\n        *pRawTXStartReq : [in] TxFrame to send. Check the structure to have more information.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_start_TX(
        st_mac_hndl: *mut MAC_handle,
        pRawTXStartReq: *const ST_MAC_raw_TX_start_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_stop_TX allow you to abort frame transmission in 802.15.4.\n\n @param None\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_stop_TX() -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_start_RX allows you to received frame in 802.15.4.\n\n @param *MAC_handle : [in] MAC instance\n        *pRawTXStartReq : [in] Enable the radio to receive frames. Check the structure to have more information.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_start_RX(
        st_mac_hndl: *mut MAC_handle,
        pRawRXStartReq: *const ST_MAC_raw_RX_start_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_stop_RX allow you to stop receiving frame in 802.15.4.\n\n @param None\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_stop_RX() -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_start_CCA allow you to perform a CCA.\n This feature lets you know whether the channel is noisy or not.\n\n @param *MAC_handle : [in] MAC instance\n        *pRawTXStartReq : [in] Channel and threshold. Check the structure to have more information.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_start_CCA(
        st_mac_hndl: *mut MAC_handle,
        pRawCCAStartReq: *mut ST_MAC_raw_CCA_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_EDscan allow you to perform Energy detector scan on one selected channel.\n\n @param *MAC_handle : [in] MAC instance\n        *pRawEDscanReq : [in] Channel selected.\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_EDscan(
        st_mac_hndl: *mut MAC_handle,
        pRawEDscanReq: *const ST_MAC_raw_EDscan_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    #[doc = " @brief ST_MAC_raw_EDscan allow you to get the result for the latest ED scan performed.\n\n @param *MAC_handle : [in] MAC instance\n        *pRawEDscanReq : [in] Energy on the selected channel (busy=255, clear=0).\n @retval MAC_Status_t"]
    pub fn ST_MAC_raw_EDscan_get_result(
        st_mac_hndl: *mut MAC_handle,
        pRawEDscanReq: *const ST_MAC_rw_EDscan_result_t,
    ) -> MAC_Status_t;
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct mac_ext_diagnostics_t {
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mcps-data.Request."]
    pub macDataReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mcps-data.Confirm."]
    pub macDataCnfCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mcps-data.Indication."]
    pub macDataIndCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mcps-purge.Request."]
    pub macPurgeReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mcps-purge.Confirm."]
    pub macPurgeCnfCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-scan.Request."]
    pub macScanReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-scan.Confirm."]
    pub macScanCnfCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-associate.Request."]
    pub macAssociationReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-association.Confirm."]
    pub macAssociationCnfCounter: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails an association."]
    pub macAssociationCnfFail: u32,
    #[doc = " @brief A counter that is incremented each time the MAC layer fails an association with the status=0xE9."]
    pub macAssociationCnfNoACK: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails an association with the status=0xE1."]
    pub macAssociationCnfCCAFailure: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails an association with the status=0xEB."]
    pub macAssociationCnfNoData: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-association.Indication."]
    pub macAssociationIndCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-association.Response."]
    pub macAssociationResCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-poll.Request."]
    pub macPollReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-poll.Confirm."]
    pub macPollCnfCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-poll.Indication."]
    pub macPollIndCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-beaconNotify.Indication."]
    pub macBeaconNotifyIndCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-Disassociate.Request."]
    pub macDisassociationReqCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-Disassociation.Confirm."]
    pub macDisassociationCnfCounter: u32,
    #[doc = " @brief   A counter that is incremented each time the MAC layer use the primitive mlme-Disassociation.Indication."]
    pub macDisassociationIndCounter: u32,
    #[doc = " @brief A counter that is incremented each time the MAC layer fails to send a unicast with the status=0xE9."]
    pub macDataCnfNoACK: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails to send a unicast with the status=0xE1."]
    pub macDataCnfCCAFailure: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails to send a unicast with the status=0xF1."]
    pub macDataCnfTransactionOverFlow: u32,
    #[doc = " @brief    A counter that is incremented each time the MAC layer fails to send a unicast."]
    pub macDataCnfFail: u32,
    #[doc = " @brief    This variable allows you to save the last fail status for Data.cnf."]
    pub macDataCnfLastFail: u8,
    #[doc = " @brief    This variable allows you to save the last status for Data.cnf."]
    pub macDataCnfLastStatus: u8,
    #[doc = " @brief    A counter that is incremented each time the Queue fails"]
    pub macQueueFail: u32,
    #[doc = " @brief    A counter that is incremented each time the Enqueue fails"]
    pub macEnqueueFail: u32,
    #[doc = " @brief    A counter that is incremented each time no buffer is allocated"]
    pub macQueueNoBuffer: u32,
    #[doc = " @brief    A counter that is incremented each time the buffer allocate fail"]
    pub macQueueBufferAllocateFailure: u32,
}
unsafe extern "C" {
    #[doc = " diagnostics function"]
    pub fn ST_init_mac_diagnostics();
}
unsafe extern "C" {
    pub fn ST_get_mac_diagnostics(mac_ext_diagnostics: *mut mac_ext_diagnostics_t);
}
unsafe extern "C" {
    #[doc = " MCPS function"]
    pub fn set_mac_ext_macDataReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataIndCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataIndCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macPurgeReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macPurgeReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macPurgeCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macPurgeCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    #[doc = " MLME function"]
    pub fn set_mac_ext_macScanReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macScanReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macScanCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macScanCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationIndCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationIndCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationResCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationResCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macPollReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macPollReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macPollCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macPollCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macPollIndCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macPollIndCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macBeaconNotifyIndCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macBeaconNotifyIndCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDisassociationReqCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDisassociationReqCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDisassociationCnfCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDisassociationCnfCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDisassociationIndCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDisassociationIndCounterHandler() -> u32;
}
unsafe extern "C" {
    #[doc = " Associate.Confirm Fails"]
    pub fn set_mac_ext_macAssociationCnfFailCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationCnfFailCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationCnfNoACKCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationCnfNoACKCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationCnfCCAFailureCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationCnfCCAFailureCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macAssociationCnfNoDataCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macAssociationCnfNoDataCounterHandler() -> u32;
}
unsafe extern "C" {
    #[doc = " Data.Confirm Fails"]
    pub fn set_mac_ext_macDataCnfNoACKCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfNoACKCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfCCAFailureCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfCCAFailureCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfTransactionOverFlowCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfTransactionOverFlowCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfFailCounterHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfFailCounterHandler() -> u32;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfLastFailHandler(status: u8);
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfLastFailHandler() -> u8;
}
unsafe extern "C" {
    pub fn set_mac_ext_macDataCnfLastStatusHandler(status: u8);
}
unsafe extern "C" {
    pub fn get_mac_ext_macDataCnfLastStatusHandler() -> u8;
}
unsafe extern "C" {
    #[doc = " Queue function"]
    pub fn set_mac_ext_macQueueFailHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macQueueFailHandler() -> u8;
}
unsafe extern "C" {
    pub fn set_mac_ext_macEnqueueFailHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macEnqueueFailHandler() -> u8;
}
unsafe extern "C" {
    pub fn set_mac_ext_macQueueNoBufferHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macQueueNoBufferHandler() -> u8;
}
unsafe extern "C" {
    pub fn set_mac_ext_macQueueBufferAllocateFailureHandler();
}
unsafe extern "C" {
    pub fn get_mac_ext_macQueueBufferAllocateFailureHandler() -> u8;
}
#[doc = " Public Type Definitions"]
pub type Buffer_Id_t = u8;
unsafe extern "C" {
    #[doc = " @brief This function true if it remains some available large buffer\n\n @param None\n\n @return 0 if no more data available 1 otherwise"]
    pub fn is_available_large_buffer() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief This function true if it remains some available small buffer\n\n @param None\n\n @return 0 if no more data available 1 otherwise"]
    pub fn is_available_small_buffer() -> u8;
}
unsafe extern "C" {
    #[doc = " @brief This function shall be called at system reset to initialise buffer\n          management\n\n @param None\n\n @return None"]
    pub fn buffMgmt_init();
}
unsafe extern "C" {
    #[doc = " @brief This function shall be called to reset buffer\n          management\n\n @param None\n\n @return None"]
    pub fn buffMgmt_reset();
}
unsafe extern "C" {
    #[doc = "/\n/** @brief This function shall be called by application to request for dynamic\n          memory allocation. This API is capable of allocating a buffer of size\n          c_small_buffer_size or c_large_buffer_size depending on the number of\n          bytes requested. If number of bytes requested is less than or equal\n          to c_small_buffer_size, buffer of size c_small_buffer_size will be\n          allocated else a buffer of size c_large_buffer_size will be allocated\n\n @param size: The number of bytes requested.\n\n @return The buffer_id of the buffer allocated, or 0xFF if the allocation fails"]
    pub fn buffMgmt_allocateBuffer(size: u8) -> Buffer_Id_t;
}
unsafe extern "C" {
    #[doc = "/\n/** @brief This function shall be called by application to free a buffer\n          previously allocated by buffMgmt_allocateBuffer\n\n @param buf_id: The buffer_id of the buffer to be freed\n\n @return None"]
    pub fn buffMgmt_freeBuffer(buffer_id: Buffer_Id_t, location: u8);
}
unsafe extern "C" {
    #[doc = "/\n/** @brief This function shall be called by application to get a pointer to a\n          buffer previously allocated by buffMgmt_allocateBuffer\n\n @param buf_id: The buffer_id of the buffer for which pointer is required\n\n @return Pointer to the buffer id equal to buffer_id or null pointer if\n         buffer_id is not a valid buffer_id."]
    pub fn buffMgmt_getBufferPointer(buffer_id: Buffer_Id_t) -> *mut u8;
}
unsafe extern "C" {
    #[doc = " @brief This function shall be called by application to get the number of\n          buffers. The number of small free buffer is returned if size is less\n          than or equal to c_small_buffer_size else the number of free large\n          buffers is returned. The function is meant only for debug purpose.\n\n @param size: The size of the buffer, whose availability is getting checked\n\n @return The number of free buffers"]
    pub fn buffMgmt_getNumberofFreeBuffers(size: u8) -> u8;
}
unsafe extern "C" {
    pub fn buffMgmt_isBufferIdValid(buffer_id: Buffer_Id_t) -> u8;
}
#[doc = " @brief  This is the structure to declare a queue"]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Queue_Tag {
    #[doc = " Holds the head of the queue"]
    pub head: u8,
    #[doc = " Holds the tail of the queue"]
    pub tail: u8,
    #[doc = " Holds the total capacity of the current queue"]
    pub total_capacity: u8,
    #[doc = " Holds the current number of entries in the queue"]
    pub current_capacity: u8,
}
#[doc = " @brief  This is the structure to declare a queue"]
pub type st_mac_queue_t = Queue_Tag;
unsafe extern "C" {
    #[doc = " @brief This function initializes a queue\n\n @param p_queue_name: This is a pointer to the queue to be initialized\n\n @param queue_size: Gives the size of the queue to be initialized\n\n @return None"]
    pub fn queueMgmt_Init(p_queue_name: *mut st_mac_queue_t, queue_size: u8);
}
unsafe extern "C" {
    #[doc = " @brief This function reset a queue\n\n @param p_queue_name: This is a pointer to the queue to be reset\n\n\n @return None"]
    pub fn queueMgmt_Reset(pQueueName: *mut st_mac_queue_t);
}
unsafe extern "C" {
    #[doc = " @brief This function enqueues a buffer id into the specified queue\n\n @param p_queue_name: pointer to the queue into which the buffer id is to be\n                      enqueued\n\n @param buffer_id: The id of the buffer to be enqueued into the queue\n\n @return returns g_SUCCESS_c if the buffer id was enqueued successfully else\n         returns g_QUEUE_FULL_c"]
    pub fn queueMgmt_enqueue(p_queue_name: *mut st_mac_queue_t, buffer_id: u8) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief This function dequeues the buffer id from the queue\n\n @param p_queue_name: pointer to the queue from which the buffer id is to be\n                      dequeued\n\n @return returns the current buffer id in the queue else returns\n         QUEUE_FAIL if there are no buffer id's in the queue"]
    pub fn queueMgmt_dequeue(p_queue_name: *mut st_mac_queue_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief This function is used to dequeue a specific buffer id from the queue\n\n @param p_queue_name: pointer to the queue from which the buffer id is to be\n                      dequeued\n\n @param buffer_id: ID of the buffer to be dequeued\n\n @return returns g_HEAD_c if the buffer id is dequeued successfully and\n                 if the queue head is pointing to that buffer id or\n                 g_SUCCESS_c if the buffer id is dequeued successfully or\n                 g_FAILURE_c if the buffer id id is not dequeued successfully"]
    pub fn queueMgmt_dequeueSpecBuffer(p_queue_name: *mut st_mac_queue_t, buffer_id: u8) -> u8;
}
unsafe extern "C" {
    pub fn queueMgmt_isQueueFull(pQueueName: *mut st_mac_queue_t) -> u8;
}
unsafe extern "C" {
    #[doc = " @brief This function return the queue owner of the bufferId\n\n @param bufferId: Id of the buffer belonging the searched queue.\n\n @return pointer of the queue owning the buffer ."]
    pub fn queueMgmt_getQueue(bufferId: u8) -> *mut st_mac_queue_t;
}
unsafe extern "C" {
    #[doc = " @brief This function reset the queue owner\n\n @param None\n\n @return None."]
    pub fn queueMgmt_resetQueueOwner();
}
unsafe extern "C" {
    pub fn queueMgmt_releaseBufferFromOwner(bufferId: u8);
}
unsafe extern "C" {
    #[doc = " @brief This function clean the queue *\n @param queue to clean up\n\n @return None."]
    pub fn queueMgmt_cleanUp(pQueue: *mut st_mac_queue_t);
}
pub type va_list = __darwin_va_list;
unsafe extern "C" {
    pub fn renameat(
        arg1: ::core::ffi::c_int,
        arg2: *const ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
        arg4: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn renamex_np(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn renameatx_np(
        arg1: ::core::ffi::c_int,
        arg2: *const ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
        arg4: *const ::core::ffi::c_char,
        arg5: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn printf(arg1: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
impl Default for __sbuf {
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
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Default, Copy, Clone)]
pub struct __sFILE {
    pub _bindgen_opaque_blob: [u64; 11usize],
}
pub type FILE = __sFILE;
unsafe extern "C" {
    #[link_name = "\u{1}__stdinp"]
    pub static mut __STDINP: *mut FILE;
}
unsafe extern "C" {
    #[link_name = "\u{1}__stdoutp"]
    pub static mut __STDOUTP: *mut FILE;
}
unsafe extern "C" {
    #[link_name = "\u{1}__stderrp"]
    pub static mut __STDERRP: *mut FILE;
}
unsafe extern "C" {
    pub fn clearerr(arg1: *mut FILE);
}
unsafe extern "C" {
    pub fn fclose(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn feof(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ferror(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fflush(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fgetc(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fgets(
        arg1: *mut ::core::ffi::c_char,
        __size: ::core::ffi::c_int,
        arg2: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn fprintf(arg1: *mut FILE, arg2: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fputc(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fputs(arg1: *const ::core::ffi::c_char, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: ::core::ffi::c_uint,
        __nitems: ::core::ffi::c_uint,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    pub fn freopen(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: *mut FILE,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn fscanf(arg1: *mut FILE, arg2: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fseek(
        arg1: *mut FILE,
        arg2: ::core::ffi::c_long,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ftell(arg1: *mut FILE) -> ::core::ffi::c_long;
}
unsafe extern "C" {
    pub fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: ::core::ffi::c_uint,
        __nitems: ::core::ffi::c_uint,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    pub fn getc(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getchar() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn gets(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn perror(arg1: *const ::core::ffi::c_char);
}
unsafe extern "C" {
    pub fn putc(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn putchar(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn puts(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn remove(arg1: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn rewind(arg1: *mut FILE);
}
unsafe extern "C" {
    pub fn scanf(arg1: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setbuf(arg1: *mut FILE, arg2: *mut ::core::ffi::c_char);
}
unsafe extern "C" {
    pub fn setvbuf(
        arg1: *mut FILE,
        arg2: *mut ::core::ffi::c_char,
        arg3: ::core::ffi::c_int,
        __size: usize,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sprintf(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn sscanf(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
unsafe extern "C" {
    pub fn tmpnam(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn ungetc(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vfprintf(
        arg1: *mut FILE,
        arg2: *const ::core::ffi::c_char,
        arg3: u32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vprintf(arg1: *const ::core::ffi::c_char, arg2: u32) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vsprintf(
        arg1: *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: u32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ctermid(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn fdopen(arg1: ::core::ffi::c_int, arg2: *const ::core::ffi::c_char) -> *mut FILE;
}
unsafe extern "C" {
    pub fn fileno(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn pclose(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn popen(arg1: *const ::core::ffi::c_char, arg2: *const ::core::ffi::c_char) -> *mut FILE;
}
unsafe extern "C" {
    pub fn __srget(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn __svfscanf(
        arg1: *mut FILE,
        arg2: *const ::core::ffi::c_char,
        arg3: va_list,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn __swbuf(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn flockfile(arg1: *mut FILE);
}
unsafe extern "C" {
    pub fn ftrylockfile(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn funlockfile(arg1: *mut FILE);
}
unsafe extern "C" {
    pub fn getc_unlocked(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getchar_unlocked() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn putc_unlocked(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn putchar_unlocked(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getw(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn putw(arg1: ::core::ffi::c_int, arg2: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn tempnam(
        __dir: *const ::core::ffi::c_char,
        __prefix: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type off_t = __darwin_off_t;
unsafe extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __offset: off_t,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ftello(__stream: *mut FILE) -> off_t;
}
unsafe extern "C" {
    pub fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: ::core::ffi::c_uint,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vfscanf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        arg1: u32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vscanf(__format: *const ::core::ffi::c_char, arg1: u32) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vsnprintf(
        __str: *mut ::core::ffi::c_char,
        __size: ::core::ffi::c_uint,
        __format: *const ::core::ffi::c_char,
        arg1: u32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vsscanf(
        __str: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        arg1: u32,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn dprintf(
        arg1: ::core::ffi::c_int,
        arg2: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vdprintf(
        arg1: ::core::ffi::c_int,
        arg2: *const ::core::ffi::c_char,
        arg3: va_list,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn getdelim(
        __linep: *mut *mut ::core::ffi::c_char,
        __linecapp: *mut usize,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> isize;
}
unsafe extern "C" {
    pub fn getline(
        __linep: *mut *mut ::core::ffi::c_char,
        __linecapp: *mut usize,
        __stream: *mut FILE,
    ) -> isize;
}
unsafe extern "C" {
    pub fn fmemopen(
        __buf: *mut ::core::ffi::c_void,
        __size: usize,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn open_memstream(__bufp: *mut *mut ::core::ffi::c_char, __sizep: *mut usize) -> *mut FILE;
}
unsafe extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static SYS_NERR: ::core::ffi::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static SYS_ERRLIST: [*const ::core::ffi::c_char; 0usize];
}
unsafe extern "C" {
    pub fn asprintf(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ctermid_r(arg1: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn fgetln(arg1: *mut FILE, __len: *mut usize) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn fmtcheck(
        arg1: *const ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn fpurge(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn setbuffer(arg1: *mut FILE, arg2: *mut ::core::ffi::c_char, __size: ::core::ffi::c_int);
}
unsafe extern "C" {
    pub fn setlinebuf(arg1: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn vasprintf(
        arg1: *mut *mut ::core::ffi::c_char,
        arg2: *const ::core::ffi::c_char,
        arg3: va_list,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn funopen(
        arg1: *const ::core::ffi::c_void,
        arg2: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *mut ::core::ffi::c_char,
                __n: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        arg3: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *const ::core::ffi::c_char,
                __n: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        arg4: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: fpos_t,
                arg3: ::core::ffi::c_int,
            ) -> fpos_t,
        >,
        arg5: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        >,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn __snprintf_chk(
        arg1: *mut ::core::ffi::c_char,
        __maxlen: usize,
        arg2: ::core::ffi::c_int,
        arg3: usize,
        arg4: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn __vsnprintf_chk(
        arg1: *mut ::core::ffi::c_char,
        __maxlen: usize,
        arg2: ::core::ffi::c_int,
        arg3: usize,
        arg4: *const ::core::ffi::c_char,
        arg5: va_list,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn __sprintf_chk(
        arg1: *mut ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: usize,
        arg4: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn __vsprintf_chk(
        arg1: *mut ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: usize,
        arg4: *const ::core::ffi::c_char,
        arg5: va_list,
    ) -> ::core::ffi::c_int;
}
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::core::ffi::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::core::ffi::c_int;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_wbl_queue_t {
    pub qBuff: *mut u8,
    pub queueMaxSize: u32,
    pub elementSize: u16,
    pub first: u32,
    pub last: u32,
    pub byteCount: u32,
    pub elementCount: u32,
    pub optionFlags: u8,
}
impl Default for stm_wbl_queue_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn CircularQueue_Init(
        q: *mut stm_wbl_queue_t,
        queueBuffer: *mut u8,
        queueSize: u32,
        elementSize: u16,
        optionlags: u8,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn CircularQueue_Add(
        q: *mut stm_wbl_queue_t,
        x: *mut u8,
        elementSize: u16,
        nbElements: u32,
    ) -> *mut u8;
}
unsafe extern "C" {
    pub fn CircularQueue_Remove(q: *mut stm_wbl_queue_t, elementSize: *mut u16) -> *mut u8;
}
unsafe extern "C" {
    pub fn CircularQueue_Sense(q: *mut stm_wbl_queue_t, elementSize: *mut u16) -> *mut u8;
}
unsafe extern "C" {
    pub fn CircularQueue_Empty(q: *mut stm_wbl_queue_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn CircularQueue_NbElement(q: *mut stm_wbl_queue_t) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn CircularQueue_Remove_Copy(
        q: *mut stm_wbl_queue_t,
        elementSize: *mut u16,
        buffer: *mut u8,
    ) -> *mut u8;
}
unsafe extern "C" {
    pub fn CircularQueue_Sense_Copy(
        q: *mut stm_wbl_queue_t,
        elementSize: *mut u16,
        buffer: *mut u8,
    ) -> *mut u8;
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
pub const CFG_LPM_Id_t_CFG_LPM_APP: CFG_LPM_Id_t = 0;
#[doc = " Supported requester to the MCU Low Power Manager - can be increased up  to 32\n It list a bit mapping of all user of the Low Power Manager"]
pub type CFG_LPM_Id_t = ::core::ffi::c_uint;
pub const CFG_Task_Id_t_CFG_TASK_HW_RNG: CFG_Task_Id_t = 0;
pub const CFG_Task_Id_t_CFG_TASK_LINK_LAYER: CFG_Task_Id_t = 1;
pub const CFG_Task_Id_t_CFG_TASK_LINK_LAYER_TEMP_MEAS: CFG_Task_Id_t = 2;
pub const CFG_Task_Id_t_CFG_TASK_MAC_LAYER: CFG_Task_Id_t = 3;
pub const CFG_Task_Id_t_CFG_TASK_RFD: CFG_Task_Id_t = 4;
pub const CFG_Task_Id_t_CFG_TASK_DATA_POOL: CFG_Task_Id_t = 5;
pub const CFG_Task_Id_t_CFG_TASK_BUTTON_1: CFG_Task_Id_t = 6;
pub const CFG_Task_Id_t_CFG_TASK_BUTTON_2: CFG_Task_Id_t = 7;
pub const CFG_Task_Id_t_CFG_TASK_BUTTON_3: CFG_Task_Id_t = 8;
pub const CFG_Task_Id_t_CFG_TASK_NBR: CFG_Task_Id_t = 9;
#[doc = " These are the lists of task id registered to the sequencer\n Each task id shall be in the range [0:31]"]
pub type CFG_Task_Id_t = ::core::ffi::c_uint;
pub const CFG_SEQ_Prio_Id_t_CFG_SEQ_PRIO_0: CFG_SEQ_Prio_Id_t = 0;
pub const CFG_SEQ_Prio_Id_t_CFG_SEQ_PRIO_1: CFG_SEQ_Prio_Id_t = 1;
pub const CFG_SEQ_Prio_Id_t_CFG_SEQ_PRIO_NBR: CFG_SEQ_Prio_Id_t = 2;
#[doc = " This is the list of priority required by the application\n Shall be in the range 0..31"]
pub type CFG_SEQ_Prio_Id_t = ::core::ffi::c_uint;
pub const CFG_IdleEvt_Id_t_CFG_EVT_RESET_CNF: CFG_IdleEvt_Id_t = 0;
pub const CFG_IdleEvt_Id_t_CFG_EVT_SET_CNF: CFG_IdleEvt_Id_t = 1;
pub const CFG_IdleEvt_Id_t_CFG_EVT_GET_CNF: CFG_IdleEvt_Id_t = 2;
pub const CFG_IdleEvt_Id_t_CFG_EVT_START_CNF: CFG_IdleEvt_Id_t = 3;
pub const CFG_IdleEvt_Id_t_CFG_EVT_RX_ON_WHEN_IDLE_CNF: CFG_IdleEvt_Id_t = 4;
pub const CFG_IdleEvt_Id_t_CFG_EVT_ASSOCIATE_CNF: CFG_IdleEvt_Id_t = 5;
pub const CFG_IdleEvt_Id_t_CFG_EVT_DATA_CNF: CFG_IdleEvt_Id_t = 6;
pub const CFG_IdleEvt_Id_t_CFG_EVT_SCAN_CNF: CFG_IdleEvt_Id_t = 7;
pub const CFG_IdleEvt_Id_t_CFG_EVT_POLL_CNF: CFG_IdleEvt_Id_t = 8;
pub const CFG_IdleEvt_Id_t_CFG_EVT_DISASSOCIATE_CNF: CFG_IdleEvt_Id_t = 9;
pub const CFG_IdleEvt_Id_t_CFG_EVT_PURGE_CNF: CFG_IdleEvt_Id_t = 10;
pub const CFG_IdleEvt_Id_t_CFG_EVT_BEACON_CNF: CFG_IdleEvt_Id_t = 11;
pub const CFG_IdleEvt_Id_t_CFG_EVT_GET_PWR_INFO_TABLE_CNF: CFG_IdleEvt_Id_t = 12;
pub const CFG_IdleEvt_Id_t_CFG_EVT_SET_PWR_INFO_TABLE_CNF: CFG_IdleEvt_Id_t = 13;
pub const CFG_IdleEvt_Id_t_CFG_EVENT_NBR: CFG_IdleEvt_Id_t = 14;
#[doc = " This is a bit mapping over 32bits listing all events id supported in the application"]
pub type CFG_IdleEvt_Id_t = ::core::ffi::c_uint;
pub const HW_TS_InitMode_t_hw_ts_InitMode_Full: HW_TS_InitMode_t = 0;
pub const HW_TS_InitMode_t_hw_ts_InitMode_Limited: HW_TS_InitMode_t = 1;
#[doc = " HW TimerServer\n/\n/**\n This setting is used when standby mode is supported.\n hw_ts_InitMode_Limited should be used when the device restarts from Standby Mode. In that case, the Timer Server does\n not re-initialized its context. Only the Hardware register which content has been lost is reconfigured\n Otherwise, hw_ts_InitMode_Full should be requested (Start from Power ON) and everything is re-initialized."]
pub type HW_TS_InitMode_t = ::core::ffi::c_uint;
pub const HW_TS_Mode_t_hw_ts_SingleShot: HW_TS_Mode_t = 0;
pub const HW_TS_Mode_t_hw_ts_Repeated: HW_TS_Mode_t = 1;
#[doc = " When a Timer is created as a SingleShot timer, it is not automatically restarted when the timeout occurs. However,\n the timer is kept reserved in the list and could be restarted at anytime with HW_TS_Start()\n\n When a Timer is created as a Repeated timer, it is automatically restarted when the timeout occurs."]
pub type HW_TS_Mode_t = ::core::ffi::c_uint;
pub const HW_TS_ReturnStatus_t_hw_ts_Successful: HW_TS_ReturnStatus_t = 0;
pub const HW_TS_ReturnStatus_t_hw_ts_Failed: HW_TS_ReturnStatus_t = 1;
#[doc = " hw_ts_Successful is returned when a Timer has been successfully created with HW_TS_Create(). Otherwise, hw_ts_Failed\n is returned. When hw_ts_Failed is returned, that means there are not enough free slots in the list to create a\n Timer. In that case, CFG_HW_TS_MAX_NBR_CONCURRENT_TIMER should be increased"]
pub type HW_TS_ReturnStatus_t = ::core::ffi::c_uint;
unsafe extern "C" {
    pub fn APP_MAC_mlmeAssociateCnfCb(pAssociateCnf: *const ST_MAC_associateCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeAssociateIndCb(pAssociateInd: *const ST_MAC_associateInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeBeaconNotifyIndCb(
        pBeaconNotifyInd: *const ST_MAC_beaconNotifyInd_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeCommStatusIndCb(
        pCommStatusInd: *const ST_MAC_commStatusInd_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeDisassociateCnfCb(
        pDisassociateCnf: *const ST_MAC_disassociateCnf_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeDisassociateIndCb(
        pDisassociateInd: *const ST_MAC_disassociateInd_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeGetCnfCb(pGetCnf: *const ST_MAC_getCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeOrphanIndCb(pOrphanInd: *const ST_MAC_orphanInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmePollCnfCb(pPollCnf: *const ST_MAC_pollCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeResetCnfCb(pResetCnf: *const ST_MAC_resetCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeRxEnableCnfCb(pRxEnableCnf: *const ST_MAC_rxEnableCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeScanCnfCb(pScanCnf: *const ST_MAC_scanCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeSetCnfCb(pSetCnf: *const ST_MAC_setCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeStartCnfCb(pStartCnf: *const ST_MAC_startCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mcpsDataIndCb(pDataInd: *const ST_MAC_dataInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mcpsDataCnfCb(pDataCnf: *const ST_MAC_dataCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mcpsPurgeCnfCb(pPurgeCnf: *const ST_MAC_purgeCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmePollIndCb(pPollInd: *const ST_MAC_pollInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeBeaconReqIndCb(pBeaconReqInd: *const ST_MAC_beaconReqInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeBeaconCnfCb(pBeaconCnf: *const ST_MAC_beaconCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeGetPwrInfoTableCnfCb(
        pGetPwrInfoTableCnf: *const ST_MAC_getPwrInfoTableCnf_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeSetPwrInfoTableCnfCb(
        pSetPwrInfoTableCnf: *const ST_MAC_setPwrInfoTableCnf_t,
    ) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeSyncLossIndCb(syncLossPtr: *const ST_MAC_syncLoss_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeCalibrateCnfCb(pCallibrateCnf: *const ST_MAC_calibrateCnf_t)
        -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeDpsCnfCb(pDpsCnf: *const ST_MAC_dpsCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeDpsIndCb(pDpsInd: *const ST_MAC_dpsInd_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeSoundingCnfCb(pSoudingCnf: *const ST_MAC_soundingCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeGtsCnfCb(pGtsCnf: *const ST_MAC_gtsCnf_t) -> MAC_Status_t;
}
unsafe extern "C" {
    pub fn APP_MAC_mlmeGtsIndCb(pGtsInd: *const ST_MAC_GtsInd_t) -> MAC_Status_t;
}
