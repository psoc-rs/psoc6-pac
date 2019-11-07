#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Radio Control Bus (RCB) controller"]
    pub rcb: RCB,
    _reserved1: [u8; 3768usize],
    #[doc = "0x1000 - Bluetooth Low Energy Link Layer"]
    pub blell: BLELL,
    _reserved2: [u8; 38140usize],
    #[doc = "0x1f000 - Bluetooth Low Energy Subsystem Miscellaneous"]
    pub bless: BLESS,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RCB {
    #[doc = "0x00 - RCB control register."]
    pub ctrl: self::rcb::CTRL,
    #[doc = "0x04 - RCB status register."]
    pub status: self::rcb::STATUS,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Transmitter control register."]
    pub tx_ctrl: self::rcb::TX_CTRL,
    #[doc = "0x14 - Transmitter FIFO control register."]
    pub tx_fifo_ctrl: self::rcb::TX_FIFO_CTRL,
    #[doc = "0x18 - Transmitter FIFO status register."]
    pub tx_fifo_status: self::rcb::TX_FIFO_STATUS,
    #[doc = "0x1c - Transmitter FIFO write register."]
    pub tx_fifo_wr: self::rcb::TX_FIFO_WR,
    #[doc = "0x20 - Receiver control register."]
    pub rx_ctrl: self::rcb::RX_CTRL,
    #[doc = "0x24 - Receiver FIFO control register."]
    pub rx_fifo_ctrl: self::rcb::RX_FIFO_CTRL,
    #[doc = "0x28 - Receiver FIFO status register."]
    pub rx_fifo_status: self::rcb::RX_FIFO_STATUS,
    #[doc = "0x2c - Receiver FIFO read register."]
    pub rx_fifo_rd: self::rcb::RX_FIFO_RD,
    #[doc = "0x30 - Receiver FIFO read register."]
    pub rx_fifo_rd_silent: self::rcb::RX_FIFO_RD_SILENT,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - Master interrupt request register."]
    pub intr: self::rcb::INTR,
    #[doc = "0x44 - Master interrupt set request register"]
    pub intr_set: self::rcb::INTR_SET,
    #[doc = "0x48 - Master interrupt mask register."]
    pub intr_mask: self::rcb::INTR_MASK,
    #[doc = "0x4c - Master interrupt masked request register"]
    pub intr_masked: self::rcb::INTR_MASKED,
    _reserved15: [u8; 176usize],
    #[doc = "0x100 - Radio Control Bus (RCB) & Link Layer controller"]
    pub rcbll: RCBLL,
}
#[doc = r"Register block"]
#[doc = "Radio Control Bus (RCB) controller"]
pub mod rcb;
#[doc = r"Register block"]
#[repr(C)]
pub struct BLELL {
    #[doc = "0x00 - Instruction Register"]
    pub command_register: self::blell::COMMAND_REGISTER,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Event(Interrupt) status and Clear register"]
    pub event_intr: self::blell::EVENT_INTR,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Event indications enable."]
    pub event_enable: self::blell::EVENT_ENABLE,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - Advertising parameters register."]
    pub adv_params: self::blell::ADV_PARAMS,
    #[doc = "0x1c - Advertising interval register."]
    pub adv_interval_timeout: self::blell::ADV_INTERVAL_TIMEOUT,
    #[doc = "0x20 - Advertising interrupt status and Clear register"]
    pub adv_intr: self::blell::ADV_INTR,
    #[doc = "0x24 - Advertising next instant."]
    pub adv_next_instant: self::blell::ADV_NEXT_INSTANT,
    #[doc = "0x28 - Scan Interval Register"]
    pub scan_interval: self::blell::SCAN_INTERVAL,
    #[doc = "0x2c - Scan window Register"]
    pub scan_window: self::blell::SCAN_WINDOW,
    #[doc = "0x30 - Scanning parameters register"]
    pub scan_param: self::blell::SCAN_PARAM,
    _reserved10: [u8; 4usize],
    #[doc = "0x38 - Scan interrupt status and Clear register"]
    pub scan_intr: self::blell::SCAN_INTR,
    #[doc = "0x3c - Advertising next instant."]
    pub scan_next_instant: self::blell::SCAN_NEXT_INSTANT,
    #[doc = "0x40 - Initiator Interval Register"]
    pub init_interval: self::blell::INIT_INTERVAL,
    #[doc = "0x44 - Initiator window Register"]
    pub init_window: self::blell::INIT_WINDOW,
    #[doc = "0x48 - Initiator parameters register"]
    pub init_param: self::blell::INIT_PARAM,
    _reserved15: [u8; 4usize],
    #[doc = "0x50 - Scan interrupt status and Clear register"]
    pub init_intr: self::blell::INIT_INTR,
    #[doc = "0x54 - Initiator next instant."]
    pub init_next_instant: self::blell::INIT_NEXT_INSTANT,
    #[doc = "0x58 - Lower 16 bit random address of the device."]
    pub device_rand_addr_l: self::blell::DEVICE_RAND_ADDR_L,
    #[doc = "0x5c - Middle 16 bit random address of the device."]
    pub device_rand_addr_m: self::blell::DEVICE_RAND_ADDR_M,
    #[doc = "0x60 - Higher 16 bit random address of the device."]
    pub device_rand_addr_h: self::blell::DEVICE_RAND_ADDR_H,
    _reserved20: [u8; 4usize],
    #[doc = "0x68 - Lower 16 bit address of the peer device."]
    pub peer_addr_l: self::blell::PEER_ADDR_L,
    #[doc = "0x6c - Middle 16 bit address of the peer device."]
    pub peer_addr_m: self::blell::PEER_ADDR_M,
    #[doc = "0x70 - Higher 16 bit address of the peer device."]
    pub peer_addr_h: self::blell::PEER_ADDR_H,
    _reserved23: [u8; 4usize],
    #[doc = "0x78 - whitelist address type"]
    pub wl_addr_type: self::blell::WL_ADDR_TYPE,
    #[doc = "0x7c - whitelist valid entry bit"]
    pub wl_enable: self::blell::WL_ENABLE,
    #[doc = "0x80 - Transmit window offset"]
    pub transmit_window_offset: self::blell::TRANSMIT_WINDOW_OFFSET,
    #[doc = "0x84 - Transmit window size"]
    pub transmit_window_size: self::blell::TRANSMIT_WINDOW_SIZE,
    #[doc = "0x88 - Data channel map 0 (lower word)"]
    pub data_channels_l0: self::blell::DATA_CHANNELS_L0,
    #[doc = "0x8c - Data channel map 0 (middle word)"]
    pub data_channels_m0: self::blell::DATA_CHANNELS_M0,
    #[doc = "0x90 - Data channel map 0 (upper word)"]
    pub data_channels_h0: self::blell::DATA_CHANNELS_H0,
    _reserved30: [u8; 4usize],
    #[doc = "0x98 - Data channel map 1 (lower word)"]
    pub data_channels_l1: self::blell::DATA_CHANNELS_L1,
    #[doc = "0x9c - Data channel map 1 (middle word)"]
    pub data_channels_m1: self::blell::DATA_CHANNELS_M1,
    #[doc = "0xa0 - Data channel map 1 (upper word)"]
    pub data_channels_h1: self::blell::DATA_CHANNELS_H1,
    _reserved33: [u8; 4usize],
    #[doc = "0xa8 - Connection interrupt status and Clear register"]
    pub conn_intr: self::blell::CONN_INTR,
    #[doc = "0xac - Connection channel status"]
    pub conn_status: self::blell::CONN_STATUS,
    #[doc = "0xb0 - Connection Index register"]
    pub conn_index: self::blell::CONN_INDEX,
    _reserved36: [u8; 4usize],
    #[doc = "0xb8 - Wakeup configuration"]
    pub wakeup_config: self::blell::WAKEUP_CONFIG,
    _reserved37: [u8; 4usize],
    #[doc = "0xc0 - Wakeup control"]
    pub wakeup_control: self::blell::WAKEUP_CONTROL,
    #[doc = "0xc4 - Clock control"]
    pub clock_config: self::blell::CLOCK_CONFIG,
    #[doc = "0xc8 - Reference Clock"]
    pub tim_counter_l: self::blell::TIM_COUNTER_L,
    #[doc = "0xcc - Wakeup configuration extended"]
    pub wakeup_config_extd: self::blell::WAKEUP_CONFIG_EXTD,
    _reserved41: [u8; 8usize],
    #[doc = "0xd8 - BLE Time Control"]
    pub poc_reg__tim_control: self::blell::POC_REG__TIM_CONTROL,
    _reserved42: [u8; 4usize],
    #[doc = "0xe0 - Advertising data transmit FIFO. Access ADVCH_TX_FIFO."]
    pub adv_tx_data_fifo: self::blell::ADV_TX_DATA_FIFO,
    _reserved43: [u8; 4usize],
    #[doc = "0xe8 - Advertising scan response data transmit FIFO. Access ADVCH_TX_FIFO."]
    pub adv_scn_rsp_tx_fifo: self::blell::ADV_SCN_RSP_TX_FIFO,
    _reserved44: [u8; 12usize],
    #[doc = "0xf8 - advertising scan response data receive data FIFO. Access ADVRX_FIFO."]
    pub init_scn_adv_rx_fifo: self::blell::INIT_SCN_ADV_RX_FIFO,
    _reserved45: [u8; 4usize],
    #[doc = "0x100 - Connection Interval"]
    pub conn_interval: self::blell::CONN_INTERVAL,
    #[doc = "0x104 - Supervision timeout"]
    pub sup_timeout: self::blell::SUP_TIMEOUT,
    #[doc = "0x108 - Slave Latency"]
    pub slave_latency: self::blell::SLAVE_LATENCY,
    #[doc = "0x10c - Connection event length"]
    pub ce_length: self::blell::CE_LENGTH,
    #[doc = "0x110 - Access address (lower)"]
    pub pdu_access_addr_l_register: self::blell::PDU_ACCESS_ADDR_L_REGISTER,
    #[doc = "0x114 - Access address (upper)"]
    pub pdu_access_addr_h_register: self::blell::PDU_ACCESS_ADDR_H_REGISTER,
    #[doc = "0x118 - Connection event instant"]
    pub conn_ce_instant: self::blell::CONN_CE_INSTANT,
    #[doc = "0x11c - connection configuration & status register"]
    pub ce_cnfg_sts_register: self::blell::CE_CNFG_STS_REGISTER,
    #[doc = "0x120 - Next connection event instant"]
    pub next_ce_instant: self::blell::NEXT_CE_INSTANT,
    #[doc = "0x124 - connection event counter"]
    pub conn_ce_counter: self::blell::CONN_CE_COUNTER,
    #[doc = "0x128 - data list sent update and status"]
    pub data_list_sent_update__status: self::blell::DATA_LIST_SENT_UPDATE__STATUS,
    #[doc = "0x12c - data list ack update and status"]
    pub data_list_ack_update__status: self::blell::DATA_LIST_ACK_UPDATE__STATUS,
    #[doc = "0x130 - connection configuration & status register"]
    pub ce_cnfg_sts_register_ext: self::blell::CE_CNFG_STS_REGISTER_EXT,
    #[doc = "0x134 - Connection extended interrupt status and Clear register"]
    pub conn_ext_intr: self::blell::CONN_EXT_INTR,
    #[doc = "0x138 - Connection Extended Interrupt mask"]
    pub conn_ext_intr_mask: self::blell::CONN_EXT_INTR_MASK,
    _reserved60: [u8; 4usize],
    #[doc = "0x140 - Data buffer descriptor 0 to 4"]
    pub data_mem_descriptor: [self::blell::DATA_MEM_DESCRIPTOR; 5],
    _reserved61: [u8; 12usize],
    #[doc = "0x160 - Window widen for interval"]
    pub window_widen_intvl: self::blell::WINDOW_WIDEN_INTVL,
    #[doc = "0x164 - Window widen for offset"]
    pub window_widen_winoff: self::blell::WINDOW_WIDEN_WINOFF,
    _reserved63: [u8; 8usize],
    #[doc = "0x170 - Direct Test Mode control"]
    pub le_rf_test_mode: self::blell::LE_RF_TEST_MODE,
    #[doc = "0x174 - Direct Test Mode receive packet count"]
    pub dtm_rx_pkt_count: self::blell::DTM_RX_PKT_COUNT,
    #[doc = "0x178 - Direct Test Mode control"]
    pub le_rf_test_mode_ext: self::blell::LE_RF_TEST_MODE_EXT,
    _reserved66: [u8; 12usize],
    #[doc = "0x188 - Channel Address register"]
    pub txrx_hop: self::blell::TXRX_HOP,
    _reserved67: [u8; 4usize],
    #[doc = "0x190 - Transmit/Receive data delay"]
    pub tx_rx_on_delay: self::blell::TX_RX_ON_DELAY,
    _reserved68: [u8; 20usize],
    #[doc = "0x1a8 - ADV packet access code low word"]
    pub adv_accaddr_l: self::blell::ADV_ACCADDR_L,
    #[doc = "0x1ac - ADV packet access code high word"]
    pub adv_accaddr_h: self::blell::ADV_ACCADDR_H,
    #[doc = "0x1b0 - Advertising channel transmit power setting"]
    pub adv_ch_tx_power_lvl_ls: self::blell::ADV_CH_TX_POWER_LVL_LS,
    #[doc = "0x1b4 - Advertising channel transmit power setting extension"]
    pub adv_ch_tx_power_lvl_ms: self::blell::ADV_CH_TX_POWER_LVL_MS,
    #[doc = "0x1b8 - Connection channel transmit power setting"]
    pub conn_ch_tx_power_lvl_ls: self::blell::CONN_CH_TX_POWER_LVL_LS,
    #[doc = "0x1bc - Connection channel transmit power setting extension"]
    pub conn_ch_tx_power_lvl_ms: self::blell::CONN_CH_TX_POWER_LVL_MS,
    #[doc = "0x1c0 - Device public address lower register"]
    pub dev_pub_addr_l: self::blell::DEV_PUB_ADDR_L,
    #[doc = "0x1c4 - Device public address middle register"]
    pub dev_pub_addr_m: self::blell::DEV_PUB_ADDR_M,
    #[doc = "0x1c8 - Device public address higher register"]
    pub dev_pub_addr_h: self::blell::DEV_PUB_ADDR_H,
    _reserved77: [u8; 4usize],
    #[doc = "0x1d0 - Offset to first instant"]
    pub offset_to_first_instant: self::blell::OFFSET_TO_FIRST_INSTANT,
    #[doc = "0x1d4 - Advertiser configuration register"]
    pub adv_config: self::blell::ADV_CONFIG,
    #[doc = "0x1d8 - Scan configuration register"]
    pub scan_config: self::blell::SCAN_CONFIG,
    #[doc = "0x1dc - Initiator configuration register"]
    pub init_config: self::blell::INIT_CONFIG,
    #[doc = "0x1e0 - Connection configuration register"]
    pub conn_config: self::blell::CONN_CONFIG,
    _reserved82: [u8; 4usize],
    #[doc = "0x1e8 - Connection parameter 1"]
    pub conn_param1: self::blell::CONN_PARAM1,
    #[doc = "0x1ec - Connection parameter 2"]
    pub conn_param2: self::blell::CONN_PARAM2,
    #[doc = "0x1f0 - Connection Interrupt mask"]
    pub conn_intr_mask: self::blell::CONN_INTR_MASK,
    #[doc = "0x1f4 - slave timing control"]
    pub slave_timing_control: self::blell::SLAVE_TIMING_CONTROL,
    #[doc = "0x1f8 - Receive trigger control"]
    pub receive_trig_ctrl: self::blell::RECEIVE_TRIG_CTRL,
    _reserved87: [u8; 4usize],
    #[doc = "0x200 - LL debug register 1"]
    pub ll_dbg_1: self::blell::LL_DBG_1,
    #[doc = "0x204 - LL debug register 2"]
    pub ll_dbg_2: self::blell::LL_DBG_2,
    #[doc = "0x208 - LL debug register 3"]
    pub ll_dbg_3: self::blell::LL_DBG_3,
    #[doc = "0x20c - LL debug register 4"]
    pub ll_dbg_4: self::blell::LL_DBG_4,
    #[doc = "0x210 - LL debug register 5"]
    pub ll_dbg_5: self::blell::LL_DBG_5,
    #[doc = "0x214 - LL debug register 6"]
    pub ll_dbg_6: self::blell::LL_DBG_6,
    #[doc = "0x218 - LL debug register 7"]
    pub ll_dbg_7: self::blell::LL_DBG_7,
    #[doc = "0x21c - LL debug register 8"]
    pub ll_dbg_8: self::blell::LL_DBG_8,
    #[doc = "0x220 - LL debug register 9"]
    pub ll_dbg_9: self::blell::LL_DBG_9,
    #[doc = "0x224 - LL debug register 10"]
    pub ll_dbg_10: self::blell::LL_DBG_10,
    _reserved97: [u8; 8usize],
    #[doc = "0x230 - Lower 16 bit address of the peer device for INIT."]
    pub peer_addr_init_l: self::blell::PEER_ADDR_INIT_L,
    #[doc = "0x234 - Middle 16 bit address of the peer device for INIT."]
    pub peer_addr_init_m: self::blell::PEER_ADDR_INIT_M,
    #[doc = "0x238 - Higher 16 bit address of the peer device for INIT."]
    pub peer_addr_init_h: self::blell::PEER_ADDR_INIT_H,
    #[doc = "0x23c - Lower 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_l: self::blell::PEER_SEC_ADDR_ADV_L,
    #[doc = "0x240 - Middle 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_m: self::blell::PEER_SEC_ADDR_ADV_M,
    #[doc = "0x244 - Higher 16 bits of the secondary address of the peer device for ADV_DIR."]
    pub peer_sec_addr_adv_h: self::blell::PEER_SEC_ADDR_ADV_H,
    #[doc = "0x248 - Initiator Window NI timer control"]
    pub init_window_timer_ctrl: self::blell::INIT_WINDOW_TIMER_CTRL,
    #[doc = "0x24c - Connection extended configuration register"]
    pub conn_config_ext: self::blell::CONN_CONFIG_EXT,
    _reserved105: [u8; 8usize],
    #[doc = "0x258 - DPLL & CY Correlator configuration register"]
    pub dpll_config: self::blell::DPLL_CONFIG,
    _reserved106: [u8; 4usize],
    #[doc = "0x260 - Initiator Window NI instant"]
    pub init_ni_val: self::blell::INIT_NI_VAL,
    #[doc = "0x264 - Initiator Window offset captured at conn request"]
    pub init_window_offset: self::blell::INIT_WINDOW_OFFSET,
    #[doc = "0x268 - Initiator Window NI anchor point captured at conn request"]
    pub init_window_ni_anchor_pt: self::blell::INIT_WINDOW_NI_ANCHOR_PT,
    _reserved109: [u8; 312usize],
    #[doc = "0x3a4 - Connection update new interval"]
    pub conn_update_new_interval: self::blell::CONN_UPDATE_NEW_INTERVAL,
    #[doc = "0x3a8 - Connection update new latency"]
    pub conn_update_new_latency: self::blell::CONN_UPDATE_NEW_LATENCY,
    #[doc = "0x3ac - Connection update new supervision timeout"]
    pub conn_update_new_sup_to: self::blell::CONN_UPDATE_NEW_SUP_TO,
    #[doc = "0x3b0 - Connection update new Slave Latency X Conn interval Value"]
    pub conn_update_new_sl_interval: self::blell::CONN_UPDATE_NEW_SL_INTERVAL,
    _reserved113: [u8; 12usize],
    #[doc = "0x3c0 - Connection request address word 0"]
    pub conn_req_word0: self::blell::CONN_REQ_WORD0,
    #[doc = "0x3c4 - Connection request address word 1"]
    pub conn_req_word1: self::blell::CONN_REQ_WORD1,
    #[doc = "0x3c8 - Connection request address word 2"]
    pub conn_req_word2: self::blell::CONN_REQ_WORD2,
    #[doc = "0x3cc - Connection request address word 3"]
    pub conn_req_word3: self::blell::CONN_REQ_WORD3,
    #[doc = "0x3d0 - Connection request address word 4"]
    pub conn_req_word4: self::blell::CONN_REQ_WORD4,
    #[doc = "0x3d4 - Connection request address word 5"]
    pub conn_req_word5: self::blell::CONN_REQ_WORD5,
    #[doc = "0x3d8 - Connection request address word 6"]
    pub conn_req_word6: self::blell::CONN_REQ_WORD6,
    #[doc = "0x3dc - Connection request address word 7"]
    pub conn_req_word7: self::blell::CONN_REQ_WORD7,
    #[doc = "0x3e0 - Connection request address word 8"]
    pub conn_req_word8: self::blell::CONN_REQ_WORD8,
    #[doc = "0x3e4 - Connection request address word 9"]
    pub conn_req_word9: self::blell::CONN_REQ_WORD9,
    #[doc = "0x3e8 - Connection request address word 10"]
    pub conn_req_word10: self::blell::CONN_REQ_WORD10,
    #[doc = "0x3ec - Connection request address word 11"]
    pub conn_req_word11: self::blell::CONN_REQ_WORD11,
    _reserved125: [u8; 1556usize],
    #[doc = "0xa04 - PDU response timer/Generic Timer (MMMS mode)"]
    pub pdu_resp_timer: self::blell::PDU_RESP_TIMER,
    #[doc = "0xa08 - Next response timeout instant"]
    pub next_resp_timer_exp: self::blell::NEXT_RESP_TIMER_EXP,
    #[doc = "0xa0c - Next supervision timeout instant"]
    pub next_sup_to: self::blell::NEXT_SUP_TO,
    #[doc = "0xa10 - Feature enable"]
    pub llh_feature_config: self::blell::LLH_FEATURE_CONFIG,
    #[doc = "0xa14 - Window minimum step size"]
    pub win_min_step_size: self::blell::WIN_MIN_STEP_SIZE,
    #[doc = "0xa18 - Slave window adjustment"]
    pub slv_win_adj: self::blell::SLV_WIN_ADJ,
    #[doc = "0xa1c - Slave Latency X Conn Interval Value"]
    pub sl_conn_interval: self::blell::SL_CONN_INTERVAL,
    #[doc = "0xa20 - LE Ping connection timer address"]
    pub le_ping_timer_addr: self::blell::LE_PING_TIMER_ADDR,
    #[doc = "0xa24 - LE Ping connection timer offset"]
    pub le_ping_timer_offset: self::blell::LE_PING_TIMER_OFFSET,
    #[doc = "0xa28 - LE Ping timer next expiry instant"]
    pub le_ping_timer_next_exp: self::blell::LE_PING_TIMER_NEXT_EXP,
    #[doc = "0xa2c - LE Ping Timer wrap count"]
    pub le_ping_timer_wrap_count: self::blell::LE_PING_TIMER_WRAP_COUNT,
    _reserved136: [u8; 976usize],
    #[doc = "0xe00 - Transmit enable extension delay"]
    pub tx_en_ext_delay: self::blell::TX_EN_EXT_DELAY,
    #[doc = "0xe04 - Transmit/Receive enable delay"]
    pub tx_rx_synth_delay: self::blell::TX_RX_SYNTH_DELAY,
    #[doc = "0xe08 - External TX PA and RX LNA delay configuration"]
    pub ext_pa_lna_dly_cnfg: self::blell::EXT_PA_LNA_DLY_CNFG,
    _reserved139: [u8; 4usize],
    #[doc = "0xe10 - Link Layer additional configuration"]
    pub ll_config: self::blell::LL_CONFIG,
    _reserved140: [u8; 236usize],
    #[doc = "0xf00 - LL Backward compatibility"]
    pub ll_control: self::blell::LL_CONTROL,
    #[doc = "0xf04 - Device Resolvable/Non-Resolvable Private address lower register"]
    pub dev_pa_addr_l: self::blell::DEV_PA_ADDR_L,
    #[doc = "0xf08 - Device Resolvable/Non-Resolvable Private address middle register"]
    pub dev_pa_addr_m: self::blell::DEV_PA_ADDR_M,
    #[doc = "0xf0c - Device Resolvable/Non-Resolvable Private address higher register"]
    pub dev_pa_addr_h: self::blell::DEV_PA_ADDR_H,
    #[doc = "0xf10 - Resolving list entry control bit"]
    pub rslv_list_enable: [self::blell::RSLV_LIST_ENABLE; 16],
    _reserved145: [u8; 80usize],
    #[doc = "0xfa0 - whitelist valid entry bit"]
    pub wl_connection_status: self::blell::WL_CONNECTION_STATUS,
    _reserved146: [u8; 2140usize],
    #[doc = "0x1800 - DLE Connection RX memory base address"]
    pub conn_rxmem_base_addr_dle: self::blell::CONN_RXMEM_BASE_ADDR_DLE,
    _reserved147: [u8; 4092usize],
    #[doc = "0x2800 - DLE Connection TX memory base address"]
    pub conn_txmem_base_addr_dle: self::blell::CONN_TXMEM_BASE_ADDR_DLE,
    _reserved148: [u8; 65532usize],
    #[doc = "0x12800 - Connection Parameter memory base address for connection 1"]
    pub conn_1_param_mem_base_addr: self::blell::CONN_1_PARAM_MEM_BASE_ADDR,
    _reserved149: [u8; 124usize],
    #[doc = "0x12880 - Connection Parameter memory base address for connection 2"]
    pub conn_2_param_mem_base_addr: self::blell::CONN_2_PARAM_MEM_BASE_ADDR,
    _reserved150: [u8; 124usize],
    #[doc = "0x12900 - Connection Parameter memory base address for connection 3"]
    pub conn_3_param_mem_base_addr: self::blell::CONN_3_PARAM_MEM_BASE_ADDR,
    _reserved151: [u8; 124usize],
    #[doc = "0x12980 - Connection Parameter memory base address for connection 4"]
    pub conn_4_param_mem_base_addr: self::blell::CONN_4_PARAM_MEM_BASE_ADDR,
    _reserved152: [u8; 5756usize],
    #[doc = "0x14000 - Next Instant Timer"]
    pub ni_timer: self::blell::NI_TIMER,
    #[doc = "0x14004 - Micro-second Offset"]
    pub us_offset: self::blell::US_OFFSET,
    #[doc = "0x14008 - Next Connection"]
    pub next_conn: self::blell::NEXT_CONN,
    #[doc = "0x1400c - Abort next scheduled connection"]
    pub ni_abort: self::blell::NI_ABORT,
    _reserved156: [u8; 16usize],
    #[doc = "0x14020 - Connection NI Status"]
    pub conn_ni_status: self::blell::CONN_NI_STATUS,
    #[doc = "0x14024 - Next Supervision timeout Status"]
    pub next_sup_to_status: self::blell::NEXT_SUP_TO_STATUS,
    #[doc = "0x14028 - Connection Status"]
    pub mmms_conn_status: self::blell::MMMS_CONN_STATUS,
    #[doc = "0x1402c - BT Slot Captured Status"]
    pub bt_slot_capt_status: self::blell::BT_SLOT_CAPT_STATUS,
    #[doc = "0x14030 - Micro-second Capture Status"]
    pub us_capt_status: self::blell::US_CAPT_STATUS,
    #[doc = "0x14034 - Micro-second Offset Status"]
    pub us_offset_status: self::blell::US_OFFSET_STATUS,
    #[doc = "0x14038 - Accumulated Window Widen Status"]
    pub accu_window_widen_status: self::blell::ACCU_WINDOW_WIDEN_STATUS,
    #[doc = "0x1403c - Status when early interrupt is raised"]
    pub early_intr_status: self::blell::EARLY_INTR_STATUS,
    #[doc = "0x14040 - Multi-Master Multi-Slave Config"]
    pub mmms_config: self::blell::MMMS_CONFIG,
    #[doc = "0x14044 - Running US of the current BT Slot"]
    pub us_counter: self::blell::US_COUNTER,
    #[doc = "0x14048 - Previous captured US of the BT Slot"]
    pub us_capt_prev: self::blell::US_CAPT_PREV,
    #[doc = "0x1404c - NI at early interrupt"]
    pub early_intr_ni: self::blell::EARLY_INTR_NI,
    _reserved168: [u8; 48usize],
    #[doc = "0x14080 - BT slot capture for master connection creation"]
    pub mmms_master_create_bt_capt: self::blell::MMMS_MASTER_CREATE_BT_CAPT,
    #[doc = "0x14084 - BT slot capture for slave connection creation"]
    pub mmms_slave_create_bt_capt: self::blell::MMMS_SLAVE_CREATE_BT_CAPT,
    #[doc = "0x14088 - Micro second capture for slave connection creation"]
    pub mmms_slave_create_us_capt: self::blell::MMMS_SLAVE_CREATE_US_CAPT,
    _reserved171: [u8; 116usize],
    #[doc = "0x14100 - Data buffer descriptor 0 to 15"]
    pub mmms_data_mem_descriptor: [self::blell::MMMS_DATA_MEM_DESCRIPTOR; 16],
    _reserved172: [u8; 192usize],
    #[doc = "0x14200 - data list sent update and status for connection 1"]
    pub conn_1_data_list_sent: self::blell::CONN_1_DATA_LIST_SENT,
    #[doc = "0x14204 - data list ack update and status for connection 1"]
    pub conn_1_data_list_ack: self::blell::CONN_1_DATA_LIST_ACK,
    #[doc = "0x14208 - Connection specific pause resume for connection 1"]
    pub conn_1_ce_data_list_cfg: self::blell::CONN_1_CE_DATA_LIST_CFG,
    _reserved175: [u8; 4usize],
    #[doc = "0x14210 - data list sent update and status for connection 2"]
    pub conn_2_data_list_sent: self::blell::CONN_2_DATA_LIST_SENT,
    #[doc = "0x14214 - data list ack update and status for connection 2"]
    pub conn_2_data_list_ack: self::blell::CONN_2_DATA_LIST_ACK,
    #[doc = "0x14218 - Connection specific pause resume for connection 2"]
    pub conn_2_ce_data_list_cfg: self::blell::CONN_2_CE_DATA_LIST_CFG,
    _reserved178: [u8; 4usize],
    #[doc = "0x14220 - data list sent update and status for connection 3"]
    pub conn_3_data_list_sent: self::blell::CONN_3_DATA_LIST_SENT,
    #[doc = "0x14224 - data list ack update and status for connection 3"]
    pub conn_3_data_list_ack: self::blell::CONN_3_DATA_LIST_ACK,
    #[doc = "0x14228 - Connection specific pause resume for connection 3"]
    pub conn_3_ce_data_list_cfg: self::blell::CONN_3_CE_DATA_LIST_CFG,
    _reserved181: [u8; 4usize],
    #[doc = "0x14230 - data list sent update and status for connection 4"]
    pub conn_4_data_list_sent: self::blell::CONN_4_DATA_LIST_SENT,
    #[doc = "0x14234 - data list ack update and status for connection 4"]
    pub conn_4_data_list_ack: self::blell::CONN_4_DATA_LIST_ACK,
    #[doc = "0x14238 - Connection specific pause resume for connection 4"]
    pub conn_4_ce_data_list_cfg: self::blell::CONN_4_CE_DATA_LIST_CFG,
    _reserved184: [u8; 452usize],
    #[doc = "0x14400 - Enable bits for ADV_NI, SCAN_NI and INIT_NI"]
    pub mmms_advch_ni_enable: self::blell::MMMS_ADVCH_NI_ENABLE,
    #[doc = "0x14404 - Next instant valid for ADV, SCAN, INIT"]
    pub mmms_advch_ni_valid: self::blell::MMMS_ADVCH_NI_VALID,
    #[doc = "0x14408 - Abort the next instant of ADV, SCAN, INIT"]
    pub mmms_advch_ni_abort: self::blell::MMMS_ADVCH_NI_ABORT,
    _reserved187: [u8; 4usize],
    #[doc = "0x14410 - Register to configure the supervision timeout for next scheduled connection"]
    pub conn_param_next_sup_to: self::blell::CONN_PARAM_NEXT_SUP_TO,
    #[doc = "0x14414 - Register to configure Accumulated window widening for next scheduled connection"]
    pub conn_param_acc_win_widen: self::blell::CONN_PARAM_ACC_WIN_WIDEN,
    _reserved189: [u8; 8usize],
    #[doc = "0x14420 - Register to configure offset from connection anchor point at which connection parameter memory should be read"]
    pub hw_load_offset: self::blell::HW_LOAD_OFFSET,
    #[doc = "0x14424 - Random number generated by Hardware for ADV NI calculation"]
    pub adv_rand: self::blell::ADV_RAND,
    #[doc = "0x14428 - Packet Counter of packets in RX FIFO in MMMS mode"]
    pub mmms_rx_pkt_cntr: self::blell::MMMS_RX_PKT_CNTR,
    _reserved192: [u8; 4usize],
    #[doc = "0x14430 - Packet Counter for Individual connection index"]
    pub conn_rx_pkt_cntr: [self::blell::CONN_RX_PKT_CNTR; 8],
    _reserved193: [u8; 944usize],
    #[doc = "0x14800 - Whitelist base address"]
    pub whitelist_base_addr: self::blell::WHITELIST_BASE_ADDR,
    _reserved194: [u8; 188usize],
    #[doc = "0x148c0 - Resolving list base address for storing Peer Identity address"]
    pub rslv_list_peer_idntt_base_addr: self::blell::RSLV_LIST_PEER_IDNTT_BASE_ADDR,
    _reserved195: [u8; 188usize],
    #[doc = "0x14980 - Resolving list base address for storing resolved Peer RPA address"]
    pub rslv_list_peer_rpa_base_addr: self::blell::RSLV_LIST_PEER_RPA_BASE_ADDR,
    _reserved196: [u8; 188usize],
    #[doc = "0x14a40 - Resolving list base address for storing Resolved received INITA RPA"]
    pub rslv_list_rcvd_init_rpa_base_addr: self::blell::RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR,
    _reserved197: [u8; 188usize],
    #[doc = "0x14b00 - Resolving list base address for storing generated TX INITA RPA"]
    pub rslv_list_tx_init_rpa_base_addr: self::blell::RSLV_LIST_TX_INIT_RPA_BASE_ADDR,
}
#[doc = r"Register block"]
#[doc = "Bluetooth Low Energy Link Layer"]
pub mod blell;
#[doc = r"Register block"]
#[repr(C)]
pub struct BLESS {
    _reserved0: [u8; 96usize],
    #[doc = "0x60 - BLESS DDFT configuration register"]
    pub ddft_config: self::bless::DDFT_CONFIG,
    #[doc = "0x64 - Crystal clock divider configuration register"]
    pub xtal_clk_div_config: self::bless::XTAL_CLK_DIV_CONFIG,
    #[doc = "0x68 - Link Layer interrupt status register"]
    pub intr_stat: self::bless::INTR_STAT,
    #[doc = "0x6c - Link Layer interrupt mask register"]
    pub intr_mask: self::bless::INTR_MASK,
    #[doc = "0x70 - Link Layer primary clock enable"]
    pub ll_clk_en: self::bless::LL_CLK_EN,
    #[doc = "0x74 - BLESS LF clock control and BLESS revision ID indicator"]
    pub lf_clk_ctrl: self::bless::LF_CLK_CTRL,
    #[doc = "0x78 - External TX PA and RX LNA control"]
    pub ext_pa_lna_ctrl: self::bless::EXT_PA_LNA_CTRL,
    _reserved7: [u8; 4usize],
    #[doc = "0x80 - Link Layer Last Received packet RSSI/Channel energy and channel number"]
    pub ll_pkt_rssi_ch_energy: self::bless::LL_PKT_RSSI_CH_ENERGY,
    #[doc = "0x84 - BT clock captured on an LL DSM exit"]
    pub bt_clock_capt: self::bless::BT_CLOCK_CAPT,
    _reserved9: [u8; 24usize],
    #[doc = "0xa0 - MT Configuration Register"]
    pub mt_cfg: self::bless::MT_CFG,
    #[doc = "0xa4 - MT Delay configuration for state transitions"]
    pub mt_delay_cfg: self::bless::MT_DELAY_CFG,
    #[doc = "0xa8 - MT Delay configuration for state transitions"]
    pub mt_delay_cfg2: self::bless::MT_DELAY_CFG2,
    #[doc = "0xac - MT Delay configuration for state transitions"]
    pub mt_delay_cfg3: self::bless::MT_DELAY_CFG3,
    #[doc = "0xb0 - MT Configuration Register to control VIO switches"]
    pub mt_vio_ctrl: self::bless::MT_VIO_CTRL,
    #[doc = "0xb4 - MT Status Register"]
    pub mt_status: self::bless::MT_STATUS,
    #[doc = "0xb8 - Link Layer Power Control FSM Status Register"]
    pub pwr_ctrl_sm_st: self::bless::PWR_CTRL_SM_ST,
    _reserved16: [u8; 4usize],
    #[doc = "0xc0 - HVLDO Configuration register"]
    pub hvldo_ctrl: self::bless::HVLDO_CTRL,
    #[doc = "0xc4 - Radio Buck and Active regulator enable control"]
    pub misc_en_ctrl: self::bless::MISC_EN_CTRL,
    _reserved18: [u8; 8usize],
    #[doc = "0xd0 - EFUSE mode configuration register"]
    pub efuse_config: self::bless::EFUSE_CONFIG,
    #[doc = "0xd4 - EFUSE timing control register (common for Program and Read modes)"]
    pub efuse_tim_ctrl1: self::bless::EFUSE_TIM_CTRL1,
    #[doc = "0xd8 - EFUSE timing control Register (for Read)"]
    pub efuse_tim_ctrl2: self::bless::EFUSE_TIM_CTRL2,
    #[doc = "0xdc - EFUSE timing control Register (for Program)"]
    pub efuse_tim_ctrl3: self::bless::EFUSE_TIM_CTRL3,
    #[doc = "0xe0 - EFUSE Lower read data"]
    pub efuse_rdata_l: self::bless::EFUSE_RDATA_L,
    #[doc = "0xe4 - EFUSE higher read data"]
    pub efuse_rdata_h: self::bless::EFUSE_RDATA_H,
    #[doc = "0xe8 - EFUSE lower write word"]
    pub efuse_wdata_l: self::bless::EFUSE_WDATA_L,
    #[doc = "0xec - EFUSE higher write word"]
    pub efuse_wdata_h: self::bless::EFUSE_WDATA_H,
    #[doc = "0xf0 - Divide by 625 for FW Use"]
    pub div_by_625_cfg: self::bless::DIV_BY_625_CFG,
    #[doc = "0xf4 - Output of divide by 625 divider"]
    pub div_by_625_sts: self::bless::DIV_BY_625_STS,
    _reserved28: [u8; 8usize],
    #[doc = "0x100 - Packet counter 0"]
    pub packet_counter0: self::bless::PACKET_COUNTER0,
    #[doc = "0x104 - Packet counter 2"]
    pub packet_counter2: self::bless::PACKET_COUNTER2,
    #[doc = "0x108 - Master Initialization Vector 0"]
    pub iv_master0: self::bless::IV_MASTER0,
    #[doc = "0x10c - Slave Initialization Vector 0"]
    pub iv_slave0: self::bless::IV_SLAVE0,
    #[doc = "0x110 - Encryption Key register 0-3"]
    pub enc_key: [self::bless::ENC_KEY; 4],
    #[doc = "0x120 - MIC input register"]
    pub mic_in0: self::bless::MIC_IN0,
    #[doc = "0x124 - MIC output register"]
    pub mic_out0: self::bless::MIC_OUT0,
    #[doc = "0x128 - Encryption Parameter register"]
    pub enc_params: self::bless::ENC_PARAMS,
    #[doc = "0x12c - Encryption Configuration"]
    pub enc_config: self::bless::ENC_CONFIG,
    #[doc = "0x130 - Encryption Interrupt enable"]
    pub enc_intr_en: self::bless::ENC_INTR_EN,
    #[doc = "0x134 - Encryption Interrupt status and clear register"]
    pub enc_intr: self::bless::ENC_INTR,
    _reserved39: [u8; 8usize],
    #[doc = "0x140 - Programmable B1 Data register (0-3)"]
    pub b1_data_reg: [self::bless::B1_DATA_REG; 4],
    #[doc = "0x150 - Encryption memory base address"]
    pub enc_mem_base_addr: self::bless::ENC_MEM_BASE_ADDR,
    _reserved41: [u8; 3500usize],
    #[doc = "0xf00 - LDO Trim register 0"]
    pub trim_ldo_0: self::bless::TRIM_LDO_0,
    #[doc = "0xf04 - LDO Trim register 1"]
    pub trim_ldo_1: self::bless::TRIM_LDO_1,
    #[doc = "0xf08 - LDO Trim register 2"]
    pub trim_ldo_2: self::bless::TRIM_LDO_2,
    #[doc = "0xf0c - LDO Trim register 3"]
    pub trim_ldo_3: self::bless::TRIM_LDO_3,
    #[doc = "0xf10 - MXD die Trim registers"]
    pub trim_mxd: [self::bless::TRIM_MXD; 4],
    _reserved46: [u8; 16usize],
    #[doc = "0xf30 - LDO Trim register 4"]
    pub trim_ldo_4: self::bless::TRIM_LDO_4,
    #[doc = "0xf34 - LDO Trim register 5"]
    pub trim_ldo_5: self::bless::TRIM_LDO_5,
}
#[doc = r"Register block"]
#[doc = "Bluetooth Low Energy Subsystem Miscellaneous"]
pub mod bless;
