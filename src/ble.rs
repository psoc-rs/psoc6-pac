#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x148 - Radio Control Bus (RCB) controller"]
    pub rcb: RCB,
    _reserved1: [u8; 0x0eb8],
    #[doc = "0x1000..0x15b04 - Bluetooth Low Energy Link Layer"]
    pub blell: BLELL,
    _reserved2: [u8; 0x94fc],
    #[doc = "0x1f000..0x1ff38 - Bluetooth Low Energy Subsystem Miscellaneous"]
    pub bless: BLESS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RCB {
    #[doc = "0x00 - RCB control register."]
    pub ctrl: crate::Reg<self::rcb::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - RCB status register."]
    pub status: crate::Reg<self::rcb::status::STATUS_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Transmitter control register."]
    pub tx_ctrl: crate::Reg<self::rcb::tx_ctrl::TX_CTRL_SPEC>,
    #[doc = "0x14 - Transmitter FIFO control register."]
    pub tx_fifo_ctrl: crate::Reg<self::rcb::tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>,
    #[doc = "0x18 - Transmitter FIFO status register."]
    pub tx_fifo_status: crate::Reg<self::rcb::tx_fifo_status::TX_FIFO_STATUS_SPEC>,
    #[doc = "0x1c - Transmitter FIFO write register."]
    pub tx_fifo_wr: crate::Reg<self::rcb::tx_fifo_wr::TX_FIFO_WR_SPEC>,
    #[doc = "0x20 - Receiver control register."]
    pub rx_ctrl: crate::Reg<self::rcb::rx_ctrl::RX_CTRL_SPEC>,
    #[doc = "0x24 - Receiver FIFO control register."]
    pub rx_fifo_ctrl: crate::Reg<self::rcb::rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>,
    #[doc = "0x28 - Receiver FIFO status register."]
    pub rx_fifo_status: crate::Reg<self::rcb::rx_fifo_status::RX_FIFO_STATUS_SPEC>,
    #[doc = "0x2c - Receiver FIFO read register."]
    pub rx_fifo_rd: crate::Reg<self::rcb::rx_fifo_rd::RX_FIFO_RD_SPEC>,
    #[doc = "0x30 - Receiver FIFO read register."]
    pub rx_fifo_rd_silent: crate::Reg<self::rcb::rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - Master interrupt request register."]
    pub intr: crate::Reg<self::rcb::intr::INTR_SPEC>,
    #[doc = "0x44 - Master interrupt set request register"]
    pub intr_set: crate::Reg<self::rcb::intr_set::INTR_SET_SPEC>,
    #[doc = "0x48 - Master interrupt mask register."]
    pub intr_mask: crate::Reg<self::rcb::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x4c - Master interrupt masked request register"]
    pub intr_masked: crate::Reg<self::rcb::intr_masked::INTR_MASKED_SPEC>,
    _reserved15: [u8; 0xb0],
    #[doc = "0x100..0x148 - Radio Control Bus (RCB) & Link Layer controller"]
    pub rcbll: self::rcb::RCBLL,
}
#[doc = r"Register block"]
#[doc = "Radio Control Bus (RCB) controller"]
pub mod rcb;
#[doc = r"Register block"]
#[repr(C)]
pub struct BLELL {
    #[doc = "0x00 - Instruction Register"]
    pub command_register: crate::Reg<self::blell::command_register::COMMAND_REGISTER_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Event(Interrupt) status and Clear register"]
    pub event_intr: crate::Reg<self::blell::event_intr::EVENT_INTR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Event indications enable."]
    pub event_enable: crate::Reg<self::blell::event_enable::EVENT_ENABLE_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Advertising parameters register."]
    pub adv_params: crate::Reg<self::blell::adv_params::ADV_PARAMS_SPEC>,
    #[doc = "0x1c - Advertising interval register."]
    pub adv_interval_timeout:
        crate::Reg<self::blell::adv_interval_timeout::ADV_INTERVAL_TIMEOUT_SPEC>,
    #[doc = "0x20 - Advertising interrupt status and Clear register"]
    pub adv_intr: crate::Reg<self::blell::adv_intr::ADV_INTR_SPEC>,
    #[doc = "0x24 - Advertising next instant."]
    pub adv_next_instant: crate::Reg<self::blell::adv_next_instant::ADV_NEXT_INSTANT_SPEC>,
    #[doc = "0x28 - Scan Interval Register"]
    pub scan_interval: crate::Reg<self::blell::scan_interval::SCAN_INTERVAL_SPEC>,
    #[doc = "0x2c - Scan window Register"]
    pub scan_window: crate::Reg<self::blell::scan_window::SCAN_WINDOW_SPEC>,
    #[doc = "0x30 - Scanning parameters register"]
    pub scan_param: crate::Reg<self::blell::scan_param::SCAN_PARAM_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x38 - Scan interrupt status and Clear register"]
    pub scan_intr: crate::Reg<self::blell::scan_intr::SCAN_INTR_SPEC>,
    #[doc = "0x3c - Advertising next instant."]
    pub scan_next_instant: crate::Reg<self::blell::scan_next_instant::SCAN_NEXT_INSTANT_SPEC>,
    #[doc = "0x40 - Initiator Interval Register"]
    pub init_interval: crate::Reg<self::blell::init_interval::INIT_INTERVAL_SPEC>,
    #[doc = "0x44 - Initiator window Register"]
    pub init_window: crate::Reg<self::blell::init_window::INIT_WINDOW_SPEC>,
    #[doc = "0x48 - Initiator parameters register"]
    pub init_param: crate::Reg<self::blell::init_param::INIT_PARAM_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Scan interrupt status and Clear register"]
    pub init_intr: crate::Reg<self::blell::init_intr::INIT_INTR_SPEC>,
    #[doc = "0x54 - Initiator next instant."]
    pub init_next_instant: crate::Reg<self::blell::init_next_instant::INIT_NEXT_INSTANT_SPEC>,
    #[doc = "0x58 - Lower 16 bit random address of the device."]
    pub device_rand_addr_l: crate::Reg<self::blell::device_rand_addr_l::DEVICE_RAND_ADDR_L_SPEC>,
    #[doc = "0x5c - Middle 16 bit random address of the device."]
    pub device_rand_addr_m: crate::Reg<self::blell::device_rand_addr_m::DEVICE_RAND_ADDR_M_SPEC>,
    #[doc = "0x60 - Higher 16 bit random address of the device."]
    pub device_rand_addr_h: crate::Reg<self::blell::device_rand_addr_h::DEVICE_RAND_ADDR_H_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x68 - Lower 16 bit address of the peer device."]
    pub peer_addr_l: crate::Reg<self::blell::peer_addr_l::PEER_ADDR_L_SPEC>,
    #[doc = "0x6c - Middle 16 bit address of the peer device."]
    pub peer_addr_m: crate::Reg<self::blell::peer_addr_m::PEER_ADDR_M_SPEC>,
    #[doc = "0x70 - Higher 16 bit address of the peer device."]
    pub peer_addr_h: crate::Reg<self::blell::peer_addr_h::PEER_ADDR_H_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x78 - whitelist address type"]
    pub wl_addr_type: crate::Reg<self::blell::wl_addr_type::WL_ADDR_TYPE_SPEC>,
    #[doc = "0x7c - whitelist valid entry bit"]
    pub wl_enable: crate::Reg<self::blell::wl_enable::WL_ENABLE_SPEC>,
    #[doc = "0x80 - Transmit window offset"]
    pub transmit_window_offset:
        crate::Reg<self::blell::transmit_window_offset::TRANSMIT_WINDOW_OFFSET_SPEC>,
    #[doc = "0x84 - Transmit window size"]
    pub transmit_window_size:
        crate::Reg<self::blell::transmit_window_size::TRANSMIT_WINDOW_SIZE_SPEC>,
    #[doc = "0x88 - Data channel map 0 (lower word)"]
    pub data_channels_l0: crate::Reg<self::blell::data_channels_l0::DATA_CHANNELS_L0_SPEC>,
    #[doc = "0x8c - Data channel map 0 (middle word)"]
    pub data_channels_m0: crate::Reg<self::blell::data_channels_m0::DATA_CHANNELS_M0_SPEC>,
    #[doc = "0x90 - Data channel map 0 (upper word)"]
    pub data_channels_h0: crate::Reg<self::blell::data_channels_h0::DATA_CHANNELS_H0_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x98 - Data channel map 1 (lower word)"]
    pub data_channels_l1: crate::Reg<self::blell::data_channels_l1::DATA_CHANNELS_L1_SPEC>,
    #[doc = "0x9c - Data channel map 1 (middle word)"]
    pub data_channels_m1: crate::Reg<self::blell::data_channels_m1::DATA_CHANNELS_M1_SPEC>,
    #[doc = "0xa0 - Data channel map 1 (upper word)"]
    pub data_channels_h1: crate::Reg<self::blell::data_channels_h1::DATA_CHANNELS_H1_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xa8 - Connection interrupt status and Clear register"]
    pub conn_intr: crate::Reg<self::blell::conn_intr::CONN_INTR_SPEC>,
    #[doc = "0xac - Connection channel status"]
    pub conn_status: crate::Reg<self::blell::conn_status::CONN_STATUS_SPEC>,
    #[doc = "0xb0 - Connection Index register"]
    pub conn_index: crate::Reg<self::blell::conn_index::CONN_INDEX_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0xb8 - Wakeup configuration"]
    pub wakeup_config: crate::Reg<self::blell::wakeup_config::WAKEUP_CONFIG_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0xc0 - Wakeup control"]
    pub wakeup_control: crate::Reg<self::blell::wakeup_control::WAKEUP_CONTROL_SPEC>,
    #[doc = "0xc4 - Clock control"]
    pub clock_config: crate::Reg<self::blell::clock_config::CLOCK_CONFIG_SPEC>,
    #[doc = "0xc8 - Reference Clock"]
    pub tim_counter_l: crate::Reg<self::blell::tim_counter_l::TIM_COUNTER_L_SPEC>,
    #[doc = "0xcc - Wakeup configuration extended"]
    pub wakeup_config_extd: crate::Reg<self::blell::wakeup_config_extd::WAKEUP_CONFIG_EXTD_SPEC>,
    _reserved41: [u8; 0x08],
    #[doc = "0xd8 - BLE Time Control"]
    pub poc_reg__tim_control:
        crate::Reg<self::blell::poc_reg__tim_control::POC_REG__TIM_CONTROL_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0xe0 - Advertising data transmit FIFO. Access ADVCH_TX_FIFO."]
    pub adv_tx_data_fifo: crate::Reg<self::blell::adv_tx_data_fifo::ADV_TX_DATA_FIFO_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0xe8 - Advertising scan response data transmit FIFO. Access ADVCH_TX_FIFO."]
    pub adv_scn_rsp_tx_fifo: crate::Reg<self::blell::adv_scn_rsp_tx_fifo::ADV_SCN_RSP_TX_FIFO_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xf8 - advertising scan response data receive data FIFO. Access ADVRX_FIFO."]
    pub init_scn_adv_rx_fifo:
        crate::Reg<self::blell::init_scn_adv_rx_fifo::INIT_SCN_ADV_RX_FIFO_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x100 - Connection Interval"]
    pub conn_interval: crate::Reg<self::blell::conn_interval::CONN_INTERVAL_SPEC>,
    #[doc = "0x104 - Supervision timeout"]
    pub sup_timeout: crate::Reg<self::blell::sup_timeout::SUP_TIMEOUT_SPEC>,
    #[doc = "0x108 - Slave Latency"]
    pub slave_latency: crate::Reg<self::blell::slave_latency::SLAVE_LATENCY_SPEC>,
    #[doc = "0x10c - Connection event length"]
    pub ce_length: crate::Reg<self::blell::ce_length::CE_LENGTH_SPEC>,
    #[doc = "0x110 - Access address (lower)"]
    pub pdu_access_addr_l_register:
        crate::Reg<self::blell::pdu_access_addr_l_register::PDU_ACCESS_ADDR_L_REGISTER_SPEC>,
    #[doc = "0x114 - Access address (upper)"]
    pub pdu_access_addr_h_register:
        crate::Reg<self::blell::pdu_access_addr_h_register::PDU_ACCESS_ADDR_H_REGISTER_SPEC>,
    #[doc = "0x118 - Connection event instant"]
    pub conn_ce_instant: crate::Reg<self::blell::conn_ce_instant::CONN_CE_INSTANT_SPEC>,
    #[doc = "0x11c - connection configuration & status register"]
    pub ce_cnfg_sts_register:
        crate::Reg<self::blell::ce_cnfg_sts_register::CE_CNFG_STS_REGISTER_SPEC>,
    #[doc = "0x120 - Next connection event instant"]
    pub next_ce_instant: crate::Reg<self::blell::next_ce_instant::NEXT_CE_INSTANT_SPEC>,
    #[doc = "0x124 - connection event counter"]
    pub conn_ce_counter: crate::Reg<self::blell::conn_ce_counter::CONN_CE_COUNTER_SPEC>,
    #[doc = "0x128 - data list sent update and status"]
    pub data_list_sent_update__status:
        crate::Reg<self::blell::data_list_sent_update__status::DATA_LIST_SENT_UPDATE__STATUS_SPEC>,
    #[doc = "0x12c - data list ack update and status"]
    pub data_list_ack_update__status:
        crate::Reg<self::blell::data_list_ack_update__status::DATA_LIST_ACK_UPDATE__STATUS_SPEC>,
    #[doc = "0x130 - connection configuration & status register"]
    pub ce_cnfg_sts_register_ext:
        crate::Reg<self::blell::ce_cnfg_sts_register_ext::CE_CNFG_STS_REGISTER_EXT_SPEC>,
    #[doc = "0x134 - Connection extended interrupt status and Clear register"]
    pub conn_ext_intr: crate::Reg<self::blell::conn_ext_intr::CONN_EXT_INTR_SPEC>,
    #[doc = "0x138 - Connection Extended Interrupt mask"]
    pub conn_ext_intr_mask: crate::Reg<self::blell::conn_ext_intr_mask::CONN_EXT_INTR_MASK_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x140..0x154 - Data buffer descriptor 0 to 4"]
    pub data_mem_descriptor:
        [crate::Reg<self::blell::data_mem_descriptor::DATA_MEM_DESCRIPTOR_SPEC>; 5],
    _reserved61: [u8; 0x0c],
    #[doc = "0x160 - Window widen for interval"]
    pub window_widen_intvl: crate::Reg<self::blell::window_widen_intvl::WINDOW_WIDEN_INTVL_SPEC>,
    #[doc = "0x164 - Window widen for offset"]
    pub window_widen_winoff: crate::Reg<self::blell::window_widen_winoff::WINDOW_WIDEN_WINOFF_SPEC>,
    _reserved63: [u8; 0x08],
    #[doc = "0x170 - Direct Test Mode control"]
    pub le_rf_test_mode: crate::Reg<self::blell::le_rf_test_mode::LE_RF_TEST_MODE_SPEC>,
    #[doc = "0x174 - Direct Test Mode receive packet count"]
    pub dtm_rx_pkt_count: crate::Reg<self::blell::dtm_rx_pkt_count::DTM_RX_PKT_COUNT_SPEC>,
    #[doc = "0x178 - Direct Test Mode control"]
    pub le_rf_test_mode_ext: crate::Reg<self::blell::le_rf_test_mode_ext::LE_RF_TEST_MODE_EXT_SPEC>,
    _reserved66: [u8; 0x0c],
    #[doc = "0x188 - Channel Address register"]
    pub txrx_hop: crate::Reg<self::blell::txrx_hop::TXRX_HOP_SPEC>,
    _reserved67: [u8; 0x04],
    #[doc = "0x190 - Transmit/Receive data delay"]
    pub tx_rx_on_delay: crate::Reg<self::blell::tx_rx_on_delay::TX_RX_ON_DELAY_SPEC>,
    _reserved68: [u8; 0x14],
    #[doc = "0x1a8 - ADV packet access code low word"]
    pub adv_accaddr_l: crate::Reg<self::blell::adv_accaddr_l::ADV_ACCADDR_L_SPEC>,
    #[doc = "0x1ac - ADV packet access code high word"]
    pub adv_accaddr_h: crate::Reg<self::blell::adv_accaddr_h::ADV_ACCADDR_H_SPEC>,
    #[doc = "0x1b0 - Advertising channel transmit power setting"]
    pub adv_ch_tx_power_lvl_ls:
        crate::Reg<self::blell::adv_ch_tx_power_lvl_ls::ADV_CH_TX_POWER_LVL_LS_SPEC>,
    #[doc = "0x1b4 - Advertising channel transmit power setting extension"]
    pub adv_ch_tx_power_lvl_ms:
        crate::Reg<self::blell::adv_ch_tx_power_lvl_ms::ADV_CH_TX_POWER_LVL_MS_SPEC>,
    #[doc = "0x1b8 - Connection channel transmit power setting"]
    pub conn_ch_tx_power_lvl_ls:
        crate::Reg<self::blell::conn_ch_tx_power_lvl_ls::CONN_CH_TX_POWER_LVL_LS_SPEC>,
    #[doc = "0x1bc - Connection channel transmit power setting extension"]
    pub conn_ch_tx_power_lvl_ms:
        crate::Reg<self::blell::conn_ch_tx_power_lvl_ms::CONN_CH_TX_POWER_LVL_MS_SPEC>,
    #[doc = "0x1c0 - Device public address lower register"]
    pub dev_pub_addr_l: crate::Reg<self::blell::dev_pub_addr_l::DEV_PUB_ADDR_L_SPEC>,
    #[doc = "0x1c4 - Device public address middle register"]
    pub dev_pub_addr_m: crate::Reg<self::blell::dev_pub_addr_m::DEV_PUB_ADDR_M_SPEC>,
    #[doc = "0x1c8 - Device public address higher register"]
    pub dev_pub_addr_h: crate::Reg<self::blell::dev_pub_addr_h::DEV_PUB_ADDR_H_SPEC>,
    _reserved77: [u8; 0x04],
    #[doc = "0x1d0 - Offset to first instant"]
    pub offset_to_first_instant:
        crate::Reg<self::blell::offset_to_first_instant::OFFSET_TO_FIRST_INSTANT_SPEC>,
    #[doc = "0x1d4 - Advertiser configuration register"]
    pub adv_config: crate::Reg<self::blell::adv_config::ADV_CONFIG_SPEC>,
    #[doc = "0x1d8 - Scan configuration register"]
    pub scan_config: crate::Reg<self::blell::scan_config::SCAN_CONFIG_SPEC>,
    #[doc = "0x1dc - Initiator configuration register"]
    pub init_config: crate::Reg<self::blell::init_config::INIT_CONFIG_SPEC>,
    #[doc = "0x1e0 - Connection configuration register"]
    pub conn_config: crate::Reg<self::blell::conn_config::CONN_CONFIG_SPEC>,
    _reserved82: [u8; 0x04],
    #[doc = "0x1e8 - Connection parameter 1"]
    pub conn_param1: crate::Reg<self::blell::conn_param1::CONN_PARAM1_SPEC>,
    #[doc = "0x1ec - Connection parameter 2"]
    pub conn_param2: crate::Reg<self::blell::conn_param2::CONN_PARAM2_SPEC>,
    #[doc = "0x1f0 - Connection Interrupt mask"]
    pub conn_intr_mask: crate::Reg<self::blell::conn_intr_mask::CONN_INTR_MASK_SPEC>,
    #[doc = "0x1f4 - slave timing control"]
    pub slave_timing_control:
        crate::Reg<self::blell::slave_timing_control::SLAVE_TIMING_CONTROL_SPEC>,
    #[doc = "0x1f8 - Receive trigger control"]
    pub receive_trig_ctrl: crate::Reg<self::blell::receive_trig_ctrl::RECEIVE_TRIG_CTRL_SPEC>,
    _reserved87: [u8; 0x04],
    #[doc = "0x200 - LL debug register 1"]
    pub ll_dbg_1: crate::Reg<self::blell::ll_dbg_1::LL_DBG_1_SPEC>,
    #[doc = "0x204 - LL debug register 2"]
    pub ll_dbg_2: crate::Reg<self::blell::ll_dbg_2::LL_DBG_2_SPEC>,
    #[doc = "0x208 - LL debug register 3"]
    pub ll_dbg_3: crate::Reg<self::blell::ll_dbg_3::LL_DBG_3_SPEC>,
    #[doc = "0x20c - LL debug register 4"]
    pub ll_dbg_4: crate::Reg<self::blell::ll_dbg_4::LL_DBG_4_SPEC>,
    #[doc = "0x210 - LL debug register 5"]
    pub ll_dbg_5: crate::Reg<self::blell::ll_dbg_5::LL_DBG_5_SPEC>,
    #[doc = "0x214 - LL debug register 6"]
    pub ll_dbg_6: crate::Reg<self::blell::ll_dbg_6::LL_DBG_6_SPEC>,
    #[doc = "0x218 - LL debug register 7"]
    pub ll_dbg_7: crate::Reg<self::blell::ll_dbg_7::LL_DBG_7_SPEC>,
    #[doc = "0x21c - LL debug register 8"]
    pub ll_dbg_8: crate::Reg<self::blell::ll_dbg_8::LL_DBG_8_SPEC>,
    #[doc = "0x220 - LL debug register 9"]
    pub ll_dbg_9: crate::Reg<self::blell::ll_dbg_9::LL_DBG_9_SPEC>,
    #[doc = "0x224 - LL debug register 10"]
    pub ll_dbg_10: crate::Reg<self::blell::ll_dbg_10::LL_DBG_10_SPEC>,
    _reserved97: [u8; 0x08],
    #[doc = "0x230 - Lower 16 bit address of the peer device for INIT."]
    pub peer_addr_init_l: crate::Reg<self::blell::peer_addr_init_l::PEER_ADDR_INIT_L_SPEC>,
    #[doc = "0x234 - Middle 16 bit address of the peer device for INIT."]
    pub peer_addr_init_m: crate::Reg<self::blell::peer_addr_init_m::PEER_ADDR_INIT_M_SPEC>,
    #[doc = "0x238 - Higher 16 bit address of the peer device for INIT."]
    pub peer_addr_init_h: crate::Reg<self::blell::peer_addr_init_h::PEER_ADDR_INIT_H_SPEC>,
    #[doc = "0x23c - Lower 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_l: crate::Reg<self::blell::peer_sec_addr_adv_l::PEER_SEC_ADDR_ADV_L_SPEC>,
    #[doc = "0x240 - Middle 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_m: crate::Reg<self::blell::peer_sec_addr_adv_m::PEER_SEC_ADDR_ADV_M_SPEC>,
    #[doc = "0x244 - Higher 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_h: crate::Reg<self::blell::peer_sec_addr_adv_h::PEER_SEC_ADDR_ADV_H_SPEC>,
    #[doc = "0x248 - Initiator Window NI timer control"]
    pub init_window_timer_ctrl:
        crate::Reg<self::blell::init_window_timer_ctrl::INIT_WINDOW_TIMER_CTRL_SPEC>,
    #[doc = "0x24c - Connection extended configuration register"]
    pub conn_config_ext: crate::Reg<self::blell::conn_config_ext::CONN_CONFIG_EXT_SPEC>,
    _reserved105: [u8; 0x08],
    #[doc = "0x258 - DPLL & CY Correlator configuration register"]
    pub dpll_config: crate::Reg<self::blell::dpll_config::DPLL_CONFIG_SPEC>,
    _reserved106: [u8; 0x04],
    #[doc = "0x260 - Initiator Window NI instant"]
    pub init_ni_val: crate::Reg<self::blell::init_ni_val::INIT_NI_VAL_SPEC>,
    #[doc = "0x264 - Initiator Window offset captured at conn request"]
    pub init_window_offset: crate::Reg<self::blell::init_window_offset::INIT_WINDOW_OFFSET_SPEC>,
    #[doc = "0x268 - Initiator Window NI anchor point captured at conn request"]
    pub init_window_ni_anchor_pt:
        crate::Reg<self::blell::init_window_ni_anchor_pt::INIT_WINDOW_NI_ANCHOR_PT_SPEC>,
    _reserved109: [u8; 0x0138],
    #[doc = "0x3a4 - Connection update new interval"]
    pub conn_update_new_interval:
        crate::Reg<self::blell::conn_update_new_interval::CONN_UPDATE_NEW_INTERVAL_SPEC>,
    #[doc = "0x3a8 - Connection update new latency"]
    pub conn_update_new_latency:
        crate::Reg<self::blell::conn_update_new_latency::CONN_UPDATE_NEW_LATENCY_SPEC>,
    #[doc = "0x3ac - Connection update new supervision timeout"]
    pub conn_update_new_sup_to:
        crate::Reg<self::blell::conn_update_new_sup_to::CONN_UPDATE_NEW_SUP_TO_SPEC>,
    #[doc = "0x3b0 - Connection update new Slave Latency X Conn interval Value"]
    pub conn_update_new_sl_interval:
        crate::Reg<self::blell::conn_update_new_sl_interval::CONN_UPDATE_NEW_SL_INTERVAL_SPEC>,
    _reserved113: [u8; 0x0c],
    #[doc = "0x3c0 - Connection request address word 0"]
    pub conn_req_word0: crate::Reg<self::blell::conn_req_word0::CONN_REQ_WORD0_SPEC>,
    #[doc = "0x3c4 - Connection request address word 1"]
    pub conn_req_word1: crate::Reg<self::blell::conn_req_word1::CONN_REQ_WORD1_SPEC>,
    #[doc = "0x3c8 - Connection request address word 2"]
    pub conn_req_word2: crate::Reg<self::blell::conn_req_word2::CONN_REQ_WORD2_SPEC>,
    #[doc = "0x3cc - Connection request address word 3"]
    pub conn_req_word3: crate::Reg<self::blell::conn_req_word3::CONN_REQ_WORD3_SPEC>,
    #[doc = "0x3d0 - Connection request address word 4"]
    pub conn_req_word4: crate::Reg<self::blell::conn_req_word4::CONN_REQ_WORD4_SPEC>,
    #[doc = "0x3d4 - Connection request address word 5"]
    pub conn_req_word5: crate::Reg<self::blell::conn_req_word5::CONN_REQ_WORD5_SPEC>,
    #[doc = "0x3d8 - Connection request address word 6"]
    pub conn_req_word6: crate::Reg<self::blell::conn_req_word6::CONN_REQ_WORD6_SPEC>,
    #[doc = "0x3dc - Connection request address word 7"]
    pub conn_req_word7: crate::Reg<self::blell::conn_req_word7::CONN_REQ_WORD7_SPEC>,
    #[doc = "0x3e0 - Connection request address word 8"]
    pub conn_req_word8: crate::Reg<self::blell::conn_req_word8::CONN_REQ_WORD8_SPEC>,
    #[doc = "0x3e4 - Connection request address word 9"]
    pub conn_req_word9: crate::Reg<self::blell::conn_req_word9::CONN_REQ_WORD9_SPEC>,
    #[doc = "0x3e8 - Connection request address word 10"]
    pub conn_req_word10: crate::Reg<self::blell::conn_req_word10::CONN_REQ_WORD10_SPEC>,
    #[doc = "0x3ec - Connection request address word 11"]
    pub conn_req_word11: crate::Reg<self::blell::conn_req_word11::CONN_REQ_WORD11_SPEC>,
    _reserved125: [u8; 0x0614],
    #[doc = "0xa04 - PDU response timer/Generic Timer (MMMS mode)"]
    pub pdu_resp_timer: crate::Reg<self::blell::pdu_resp_timer::PDU_RESP_TIMER_SPEC>,
    #[doc = "0xa08 - Next response timeout instant"]
    pub next_resp_timer_exp: crate::Reg<self::blell::next_resp_timer_exp::NEXT_RESP_TIMER_EXP_SPEC>,
    #[doc = "0xa0c - Next supervision timeout instant"]
    pub next_sup_to: crate::Reg<self::blell::next_sup_to::NEXT_SUP_TO_SPEC>,
    #[doc = "0xa10 - Feature enable"]
    pub llh_feature_config: crate::Reg<self::blell::llh_feature_config::LLH_FEATURE_CONFIG_SPEC>,
    #[doc = "0xa14 - Window minimum step size"]
    pub win_min_step_size: crate::Reg<self::blell::win_min_step_size::WIN_MIN_STEP_SIZE_SPEC>,
    #[doc = "0xa18 - Slave window adjustment"]
    pub slv_win_adj: crate::Reg<self::blell::slv_win_adj::SLV_WIN_ADJ_SPEC>,
    #[doc = "0xa1c - Slave Latency X Conn Interval Value"]
    pub sl_conn_interval: crate::Reg<self::blell::sl_conn_interval::SL_CONN_INTERVAL_SPEC>,
    #[doc = "0xa20 - LE Ping connection timer address"]
    pub le_ping_timer_addr: crate::Reg<self::blell::le_ping_timer_addr::LE_PING_TIMER_ADDR_SPEC>,
    #[doc = "0xa24 - LE Ping connection timer offset"]
    pub le_ping_timer_offset:
        crate::Reg<self::blell::le_ping_timer_offset::LE_PING_TIMER_OFFSET_SPEC>,
    #[doc = "0xa28 - LE Ping timer next expiry instant"]
    pub le_ping_timer_next_exp:
        crate::Reg<self::blell::le_ping_timer_next_exp::LE_PING_TIMER_NEXT_EXP_SPEC>,
    #[doc = "0xa2c - LE Ping Timer wrap count"]
    pub le_ping_timer_wrap_count:
        crate::Reg<self::blell::le_ping_timer_wrap_count::LE_PING_TIMER_WRAP_COUNT_SPEC>,
    _reserved136: [u8; 0x03d0],
    #[doc = "0xe00 - Transmit enable extension delay"]
    pub tx_en_ext_delay: crate::Reg<self::blell::tx_en_ext_delay::TX_EN_EXT_DELAY_SPEC>,
    #[doc = "0xe04 - Transmit/Receive enable delay"]
    pub tx_rx_synth_delay: crate::Reg<self::blell::tx_rx_synth_delay::TX_RX_SYNTH_DELAY_SPEC>,
    #[doc = "0xe08 - External TX PA and RX LNA delay configuration"]
    pub ext_pa_lna_dly_cnfg: crate::Reg<self::blell::ext_pa_lna_dly_cnfg::EXT_PA_LNA_DLY_CNFG_SPEC>,
    _reserved139: [u8; 0x04],
    #[doc = "0xe10 - Link Layer additional configuration"]
    pub ll_config: crate::Reg<self::blell::ll_config::LL_CONFIG_SPEC>,
    _reserved140: [u8; 0xec],
    #[doc = "0xf00 - LL Backward compatibility"]
    pub ll_control: crate::Reg<self::blell::ll_control::LL_CONTROL_SPEC>,
    #[doc = "0xf04 - Device Resolvable/Non-Resolvable Private address lower register"]
    pub dev_pa_addr_l: crate::Reg<self::blell::dev_pa_addr_l::DEV_PA_ADDR_L_SPEC>,
    #[doc = "0xf08 - Device Resolvable/Non-Resolvable Private address middle register"]
    pub dev_pa_addr_m: crate::Reg<self::blell::dev_pa_addr_m::DEV_PA_ADDR_M_SPEC>,
    #[doc = "0xf0c - Device Resolvable/Non-Resolvable Private address higher register"]
    pub dev_pa_addr_h: crate::Reg<self::blell::dev_pa_addr_h::DEV_PA_ADDR_H_SPEC>,
    #[doc = "0xf10..0xf50 - Resolving list entry control bit"]
    pub rslv_list_enable: [crate::Reg<self::blell::rslv_list_enable::RSLV_LIST_ENABLE_SPEC>; 16],
    _reserved145: [u8; 0x50],
    #[doc = "0xfa0 - whitelist valid entry bit"]
    pub wl_connection_status:
        crate::Reg<self::blell::wl_connection_status::WL_CONNECTION_STATUS_SPEC>,
    _reserved146: [u8; 0x085c],
    #[doc = "0x1800 - DLE Connection RX memory base address"]
    pub conn_rxmem_base_addr_dle:
        crate::Reg<self::blell::conn_rxmem_base_addr_dle::CONN_RXMEM_BASE_ADDR_DLE_SPEC>,
    _reserved147: [u8; 0x0ffc],
    #[doc = "0x2800 - DLE Connection TX memory base address"]
    pub conn_txmem_base_addr_dle:
        crate::Reg<self::blell::conn_txmem_base_addr_dle::CONN_TXMEM_BASE_ADDR_DLE_SPEC>,
    _reserved148: [u8; 0xfffc],
    #[doc = "0x12800 - Connection Parameter memory base address for connection 1"]
    pub conn_1_param_mem_base_addr:
        crate::Reg<self::blell::conn_1_param_mem_base_addr::CONN_1_PARAM_MEM_BASE_ADDR_SPEC>,
    _reserved149: [u8; 0x7c],
    #[doc = "0x12880 - Connection Parameter memory base address for connection 2"]
    pub conn_2_param_mem_base_addr:
        crate::Reg<self::blell::conn_2_param_mem_base_addr::CONN_2_PARAM_MEM_BASE_ADDR_SPEC>,
    _reserved150: [u8; 0x7c],
    #[doc = "0x12900 - Connection Parameter memory base address for connection 3"]
    pub conn_3_param_mem_base_addr:
        crate::Reg<self::blell::conn_3_param_mem_base_addr::CONN_3_PARAM_MEM_BASE_ADDR_SPEC>,
    _reserved151: [u8; 0x7c],
    #[doc = "0x12980 - Connection Parameter memory base address for connection 4"]
    pub conn_4_param_mem_base_addr:
        crate::Reg<self::blell::conn_4_param_mem_base_addr::CONN_4_PARAM_MEM_BASE_ADDR_SPEC>,
    _reserved152: [u8; 0x167c],
    #[doc = "0x14000 - Next Instant Timer"]
    pub ni_timer: crate::Reg<self::blell::ni_timer::NI_TIMER_SPEC>,
    #[doc = "0x14004 - Micro-second Offset"]
    pub us_offset: crate::Reg<self::blell::us_offset::US_OFFSET_SPEC>,
    #[doc = "0x14008 - Next Connection"]
    pub next_conn: crate::Reg<self::blell::next_conn::NEXT_CONN_SPEC>,
    #[doc = "0x1400c - Abort next scheduled connection"]
    pub ni_abort: crate::Reg<self::blell::ni_abort::NI_ABORT_SPEC>,
    _reserved156: [u8; 0x10],
    #[doc = "0x14020 - Connection NI Status"]
    pub conn_ni_status: crate::Reg<self::blell::conn_ni_status::CONN_NI_STATUS_SPEC>,
    #[doc = "0x14024 - Next Supervision timeout Status"]
    pub next_sup_to_status: crate::Reg<self::blell::next_sup_to_status::NEXT_SUP_TO_STATUS_SPEC>,
    #[doc = "0x14028 - Connection Status"]
    pub mmms_conn_status: crate::Reg<self::blell::mmms_conn_status::MMMS_CONN_STATUS_SPEC>,
    #[doc = "0x1402c - BT Slot Captured Status"]
    pub bt_slot_capt_status: crate::Reg<self::blell::bt_slot_capt_status::BT_SLOT_CAPT_STATUS_SPEC>,
    #[doc = "0x14030 - Micro-second Capture Status"]
    pub us_capt_status: crate::Reg<self::blell::us_capt_status::US_CAPT_STATUS_SPEC>,
    #[doc = "0x14034 - Micro-second Offset Status"]
    pub us_offset_status: crate::Reg<self::blell::us_offset_status::US_OFFSET_STATUS_SPEC>,
    #[doc = "0x14038 - Accumulated Window Widen Status"]
    pub accu_window_widen_status:
        crate::Reg<self::blell::accu_window_widen_status::ACCU_WINDOW_WIDEN_STATUS_SPEC>,
    #[doc = "0x1403c - Status when early interrupt is raised"]
    pub early_intr_status: crate::Reg<self::blell::early_intr_status::EARLY_INTR_STATUS_SPEC>,
    #[doc = "0x14040 - Multi-Master Multi-Slave Config"]
    pub mmms_config: crate::Reg<self::blell::mmms_config::MMMS_CONFIG_SPEC>,
    #[doc = "0x14044 - Running US of the current BT Slot"]
    pub us_counter: crate::Reg<self::blell::us_counter::US_COUNTER_SPEC>,
    #[doc = "0x14048 - Previous captured US of the BT Slot"]
    pub us_capt_prev: crate::Reg<self::blell::us_capt_prev::US_CAPT_PREV_SPEC>,
    #[doc = "0x1404c - NI at early interrupt"]
    pub early_intr_ni: crate::Reg<self::blell::early_intr_ni::EARLY_INTR_NI_SPEC>,
    _reserved168: [u8; 0x30],
    #[doc = "0x14080 - BT slot capture for master connection creation"]
    pub mmms_master_create_bt_capt:
        crate::Reg<self::blell::mmms_master_create_bt_capt::MMMS_MASTER_CREATE_BT_CAPT_SPEC>,
    #[doc = "0x14084 - BT slot capture for slave connection creation"]
    pub mmms_slave_create_bt_capt:
        crate::Reg<self::blell::mmms_slave_create_bt_capt::MMMS_SLAVE_CREATE_BT_CAPT_SPEC>,
    #[doc = "0x14088 - Micro second capture for slave connection creation"]
    pub mmms_slave_create_us_capt:
        crate::Reg<self::blell::mmms_slave_create_us_capt::MMMS_SLAVE_CREATE_US_CAPT_SPEC>,
    _reserved171: [u8; 0x74],
    #[doc = "0x14100..0x14140 - Data buffer descriptor 0 to 15"]
    pub mmms_data_mem_descriptor:
        [crate::Reg<self::blell::mmms_data_mem_descriptor::MMMS_DATA_MEM_DESCRIPTOR_SPEC>; 16],
    _reserved172: [u8; 0xc0],
    #[doc = "0x14200 - data list sent update and status for connection 1"]
    pub conn_1_data_list_sent:
        crate::Reg<self::blell::conn_1_data_list_sent::CONN_1_DATA_LIST_SENT_SPEC>,
    #[doc = "0x14204 - data list ack update and status for connection 1"]
    pub conn_1_data_list_ack:
        crate::Reg<self::blell::conn_1_data_list_ack::CONN_1_DATA_LIST_ACK_SPEC>,
    #[doc = "0x14208 - Connection specific pause resume for connection 1"]
    pub conn_1_ce_data_list_cfg:
        crate::Reg<self::blell::conn_1_ce_data_list_cfg::CONN_1_CE_DATA_LIST_CFG_SPEC>,
    _reserved175: [u8; 0x04],
    #[doc = "0x14210 - data list sent update and status for connection 2"]
    pub conn_2_data_list_sent:
        crate::Reg<self::blell::conn_2_data_list_sent::CONN_2_DATA_LIST_SENT_SPEC>,
    #[doc = "0x14214 - data list ack update and status for connection 2"]
    pub conn_2_data_list_ack:
        crate::Reg<self::blell::conn_2_data_list_ack::CONN_2_DATA_LIST_ACK_SPEC>,
    #[doc = "0x14218 - Connection specific pause resume for connection 2"]
    pub conn_2_ce_data_list_cfg:
        crate::Reg<self::blell::conn_2_ce_data_list_cfg::CONN_2_CE_DATA_LIST_CFG_SPEC>,
    _reserved178: [u8; 0x04],
    #[doc = "0x14220 - data list sent update and status for connection 3"]
    pub conn_3_data_list_sent:
        crate::Reg<self::blell::conn_3_data_list_sent::CONN_3_DATA_LIST_SENT_SPEC>,
    #[doc = "0x14224 - data list ack update and status for connection 3"]
    pub conn_3_data_list_ack:
        crate::Reg<self::blell::conn_3_data_list_ack::CONN_3_DATA_LIST_ACK_SPEC>,
    #[doc = "0x14228 - Connection specific pause resume for connection 3"]
    pub conn_3_ce_data_list_cfg:
        crate::Reg<self::blell::conn_3_ce_data_list_cfg::CONN_3_CE_DATA_LIST_CFG_SPEC>,
    _reserved181: [u8; 0x04],
    #[doc = "0x14230 - data list sent update and status for connection 4"]
    pub conn_4_data_list_sent:
        crate::Reg<self::blell::conn_4_data_list_sent::CONN_4_DATA_LIST_SENT_SPEC>,
    #[doc = "0x14234 - data list ack update and status for connection 4"]
    pub conn_4_data_list_ack:
        crate::Reg<self::blell::conn_4_data_list_ack::CONN_4_DATA_LIST_ACK_SPEC>,
    #[doc = "0x14238 - Connection specific pause resume for connection 4"]
    pub conn_4_ce_data_list_cfg:
        crate::Reg<self::blell::conn_4_ce_data_list_cfg::CONN_4_CE_DATA_LIST_CFG_SPEC>,
    _reserved184: [u8; 0x01c4],
    #[doc = "0x14400 - Enable bits for ADV_NI, SCAN_NI and INIT_NI"]
    pub mmms_advch_ni_enable:
        crate::Reg<self::blell::mmms_advch_ni_enable::MMMS_ADVCH_NI_ENABLE_SPEC>,
    #[doc = "0x14404 - Next instant valid for ADV, SCAN, INIT"]
    pub mmms_advch_ni_valid: crate::Reg<self::blell::mmms_advch_ni_valid::MMMS_ADVCH_NI_VALID_SPEC>,
    #[doc = "0x14408 - Abort the next instant of ADV, SCAN, INIT"]
    pub mmms_advch_ni_abort: crate::Reg<self::blell::mmms_advch_ni_abort::MMMS_ADVCH_NI_ABORT_SPEC>,
    _reserved187: [u8; 0x04],
    #[doc = "0x14410 - Register to configure the supervision timeout for next scheduled connection"]
    pub conn_param_next_sup_to:
        crate::Reg<self::blell::conn_param_next_sup_to::CONN_PARAM_NEXT_SUP_TO_SPEC>,
    #[doc = "0x14414 - Register to configure Accumulated window widening for next scheduled connection"]
    pub conn_param_acc_win_widen:
        crate::Reg<self::blell::conn_param_acc_win_widen::CONN_PARAM_ACC_WIN_WIDEN_SPEC>,
    _reserved189: [u8; 0x08],
    #[doc = "0x14420 - Register to configure offset from connection anchor point at which connection parameter memory should be read"]
    pub hw_load_offset: crate::Reg<self::blell::hw_load_offset::HW_LOAD_OFFSET_SPEC>,
    #[doc = "0x14424 - Random number generated by Hardware for ADV NI calculation"]
    pub adv_rand: crate::Reg<self::blell::adv_rand::ADV_RAND_SPEC>,
    #[doc = "0x14428 - Packet Counter of packets in RX FIFO in MMMS mode"]
    pub mmms_rx_pkt_cntr: crate::Reg<self::blell::mmms_rx_pkt_cntr::MMMS_RX_PKT_CNTR_SPEC>,
    _reserved192: [u8; 0x04],
    #[doc = "0x14430..0x14450 - Packet Counter for Individual connection index"]
    pub conn_rx_pkt_cntr: [crate::Reg<self::blell::conn_rx_pkt_cntr::CONN_RX_PKT_CNTR_SPEC>; 8],
    _reserved193: [u8; 0x03b0],
    #[doc = "0x14800 - Whitelist base address"]
    pub whitelist_base_addr: crate::Reg<self::blell::whitelist_base_addr::WHITELIST_BASE_ADDR_SPEC>,
    _reserved194: [u8; 0xbc],
    #[doc = "0x148c0 - Resolving list base address for storing Peer Identity address"]
    pub rslv_list_peer_idntt_base_addr: crate::Reg<
        self::blell::rslv_list_peer_idntt_base_addr::RSLV_LIST_PEER_IDNTT_BASE_ADDR_SPEC,
    >,
    _reserved195: [u8; 0xbc],
    #[doc = "0x14980 - Resolving list base address for storing resolved Peer RPA address"]
    pub rslv_list_peer_rpa_base_addr:
        crate::Reg<self::blell::rslv_list_peer_rpa_base_addr::RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>,
    _reserved196: [u8; 0xbc],
    #[doc = "0x14a40 - Resolving list base address for storing Resolved received INITA RPA"]
    pub rslv_list_rcvd_init_rpa_base_addr: crate::Reg<
        self::blell::rslv_list_rcvd_init_rpa_base_addr::RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR_SPEC,
    >,
    _reserved197: [u8; 0xbc],
    #[doc = "0x14b00 - Resolving list base address for storing generated TX INITA RPA"]
    pub rslv_list_tx_init_rpa_base_addr: crate::Reg<
        self::blell::rslv_list_tx_init_rpa_base_addr::RSLV_LIST_TX_INIT_RPA_BASE_ADDR_SPEC,
    >,
}
#[doc = r"Register block"]
#[doc = "Bluetooth Low Energy Link Layer"]
pub mod blell;
#[doc = r"Register block"]
#[repr(C)]
pub struct BLESS {
    _reserved0: [u8; 0x60],
    #[doc = "0x60 - BLESS DDFT configuration register"]
    pub ddft_config: crate::Reg<self::bless::ddft_config::DDFT_CONFIG_SPEC>,
    #[doc = "0x64 - Crystal clock divider configuration register"]
    pub xtal_clk_div_config: crate::Reg<self::bless::xtal_clk_div_config::XTAL_CLK_DIV_CONFIG_SPEC>,
    #[doc = "0x68 - Link Layer interrupt status register"]
    pub intr_stat: crate::Reg<self::bless::intr_stat::INTR_STAT_SPEC>,
    #[doc = "0x6c - Link Layer interrupt mask register"]
    pub intr_mask: crate::Reg<self::bless::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x70 - Link Layer primary clock enable"]
    pub ll_clk_en: crate::Reg<self::bless::ll_clk_en::LL_CLK_EN_SPEC>,
    #[doc = "0x74 - BLESS LF clock control and BLESS revision ID indicator"]
    pub lf_clk_ctrl: crate::Reg<self::bless::lf_clk_ctrl::LF_CLK_CTRL_SPEC>,
    #[doc = "0x78 - External TX PA and RX LNA control"]
    pub ext_pa_lna_ctrl: crate::Reg<self::bless::ext_pa_lna_ctrl::EXT_PA_LNA_CTRL_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x80 - Link Layer Last Received packet RSSI/Channel energy and channel number"]
    pub ll_pkt_rssi_ch_energy:
        crate::Reg<self::bless::ll_pkt_rssi_ch_energy::LL_PKT_RSSI_CH_ENERGY_SPEC>,
    #[doc = "0x84 - BT clock captured on an LL DSM exit"]
    pub bt_clock_capt: crate::Reg<self::bless::bt_clock_capt::BT_CLOCK_CAPT_SPEC>,
    _reserved9: [u8; 0x18],
    #[doc = "0xa0 - MT Configuration Register"]
    pub mt_cfg: crate::Reg<self::bless::mt_cfg::MT_CFG_SPEC>,
    #[doc = "0xa4 - MT Delay configuration for state transitions"]
    pub mt_delay_cfg: crate::Reg<self::bless::mt_delay_cfg::MT_DELAY_CFG_SPEC>,
    #[doc = "0xa8 - MT Delay configuration for state transitions"]
    pub mt_delay_cfg2: crate::Reg<self::bless::mt_delay_cfg2::MT_DELAY_CFG2_SPEC>,
    #[doc = "0xac - MT Delay configuration for state transitions"]
    pub mt_delay_cfg3: crate::Reg<self::bless::mt_delay_cfg3::MT_DELAY_CFG3_SPEC>,
    #[doc = "0xb0 - MT Configuration Register to control VIO switches"]
    pub mt_vio_ctrl: crate::Reg<self::bless::mt_vio_ctrl::MT_VIO_CTRL_SPEC>,
    #[doc = "0xb4 - MT Status Register"]
    pub mt_status: crate::Reg<self::bless::mt_status::MT_STATUS_SPEC>,
    #[doc = "0xb8 - Link Layer Power Control FSM Status Register"]
    pub pwr_ctrl_sm_st: crate::Reg<self::bless::pwr_ctrl_sm_st::PWR_CTRL_SM_ST_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0xc0 - HVLDO Configuration register"]
    pub hvldo_ctrl: crate::Reg<self::bless::hvldo_ctrl::HVLDO_CTRL_SPEC>,
    #[doc = "0xc4 - Radio Buck and Active regulator enable control"]
    pub misc_en_ctrl: crate::Reg<self::bless::misc_en_ctrl::MISC_EN_CTRL_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0xd0 - EFUSE mode configuration register"]
    pub efuse_config: crate::Reg<self::bless::efuse_config::EFUSE_CONFIG_SPEC>,
    #[doc = "0xd4 - EFUSE timing control register (common for Program and Read modes)"]
    pub efuse_tim_ctrl1: crate::Reg<self::bless::efuse_tim_ctrl1::EFUSE_TIM_CTRL1_SPEC>,
    #[doc = "0xd8 - EFUSE timing control Register (for Read)"]
    pub efuse_tim_ctrl2: crate::Reg<self::bless::efuse_tim_ctrl2::EFUSE_TIM_CTRL2_SPEC>,
    #[doc = "0xdc - EFUSE timing control Register (for Program)"]
    pub efuse_tim_ctrl3: crate::Reg<self::bless::efuse_tim_ctrl3::EFUSE_TIM_CTRL3_SPEC>,
    #[doc = "0xe0 - EFUSE Lower read data"]
    pub efuse_rdata_l: crate::Reg<self::bless::efuse_rdata_l::EFUSE_RDATA_L_SPEC>,
    #[doc = "0xe4 - EFUSE higher read data"]
    pub efuse_rdata_h: crate::Reg<self::bless::efuse_rdata_h::EFUSE_RDATA_H_SPEC>,
    #[doc = "0xe8 - EFUSE lower write word"]
    pub efuse_wdata_l: crate::Reg<self::bless::efuse_wdata_l::EFUSE_WDATA_L_SPEC>,
    #[doc = "0xec - EFUSE higher write word"]
    pub efuse_wdata_h: crate::Reg<self::bless::efuse_wdata_h::EFUSE_WDATA_H_SPEC>,
    #[doc = "0xf0 - Divide by 625 for FW Use"]
    pub div_by_625_cfg: crate::Reg<self::bless::div_by_625_cfg::DIV_BY_625_CFG_SPEC>,
    #[doc = "0xf4 - Output of divide by 625 divider"]
    pub div_by_625_sts: crate::Reg<self::bless::div_by_625_sts::DIV_BY_625_STS_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x100 - Packet counter 0"]
    pub packet_counter0: crate::Reg<self::bless::packet_counter0::PACKET_COUNTER0_SPEC>,
    #[doc = "0x104 - Packet counter 2"]
    pub packet_counter2: crate::Reg<self::bless::packet_counter2::PACKET_COUNTER2_SPEC>,
    #[doc = "0x108 - Master Initialization Vector 0"]
    pub iv_master0: crate::Reg<self::bless::iv_master0::IV_MASTER0_SPEC>,
    #[doc = "0x10c - Slave Initialization Vector 0"]
    pub iv_slave0: crate::Reg<self::bless::iv_slave0::IV_SLAVE0_SPEC>,
    #[doc = "0x110..0x120 - Encryption Key register 0-3"]
    pub enc_key: [crate::Reg<self::bless::enc_key::ENC_KEY_SPEC>; 4],
    #[doc = "0x120 - MIC input register"]
    pub mic_in0: crate::Reg<self::bless::mic_in0::MIC_IN0_SPEC>,
    #[doc = "0x124 - MIC output register"]
    pub mic_out0: crate::Reg<self::bless::mic_out0::MIC_OUT0_SPEC>,
    #[doc = "0x128 - Encryption Parameter register"]
    pub enc_params: crate::Reg<self::bless::enc_params::ENC_PARAMS_SPEC>,
    #[doc = "0x12c - Encryption Configuration"]
    pub enc_config: crate::Reg<self::bless::enc_config::ENC_CONFIG_SPEC>,
    #[doc = "0x130 - Encryption Interrupt enable"]
    pub enc_intr_en: crate::Reg<self::bless::enc_intr_en::ENC_INTR_EN_SPEC>,
    #[doc = "0x134 - Encryption Interrupt status and clear register"]
    pub enc_intr: crate::Reg<self::bless::enc_intr::ENC_INTR_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x140..0x150 - Programmable B1 Data register (0-3)"]
    pub b1_data_reg: [crate::Reg<self::bless::b1_data_reg::B1_DATA_REG_SPEC>; 4],
    #[doc = "0x150 - Encryption memory base address"]
    pub enc_mem_base_addr: crate::Reg<self::bless::enc_mem_base_addr::ENC_MEM_BASE_ADDR_SPEC>,
    _reserved41: [u8; 0x0dac],
    #[doc = "0xf00 - LDO Trim register 0"]
    pub trim_ldo_0: crate::Reg<self::bless::trim_ldo_0::TRIM_LDO_0_SPEC>,
    #[doc = "0xf04 - LDO Trim register 1"]
    pub trim_ldo_1: crate::Reg<self::bless::trim_ldo_1::TRIM_LDO_1_SPEC>,
    #[doc = "0xf08 - LDO Trim register 2"]
    pub trim_ldo_2: crate::Reg<self::bless::trim_ldo_2::TRIM_LDO_2_SPEC>,
    #[doc = "0xf0c - LDO Trim register 3"]
    pub trim_ldo_3: crate::Reg<self::bless::trim_ldo_3::TRIM_LDO_3_SPEC>,
    #[doc = "0xf10..0xf20 - MXD die Trim registers"]
    pub trim_mxd: [crate::Reg<self::bless::trim_mxd::TRIM_MXD_SPEC>; 4],
    _reserved46: [u8; 0x10],
    #[doc = "0xf30 - LDO Trim register 4"]
    pub trim_ldo_4: crate::Reg<self::bless::trim_ldo_4::TRIM_LDO_4_SPEC>,
    #[doc = "0xf34 - LDO Trim register 5"]
    pub trim_ldo_5: crate::Reg<self::bless::trim_ldo_5::TRIM_LDO_5_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Bluetooth Low Energy Subsystem Miscellaneous"]
pub mod bless;
