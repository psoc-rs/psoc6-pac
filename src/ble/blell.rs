#[doc = "COMMAND_REGISTER register accessor: an alias for `Reg<COMMAND_REGISTER_SPEC>`"]
pub type COMMAND_REGISTER = crate::Reg<command_register::COMMAND_REGISTER_SPEC>;
#[doc = "Instruction Register"]
pub mod command_register;
#[doc = "EVENT_INTR register accessor: an alias for `Reg<EVENT_INTR_SPEC>`"]
pub type EVENT_INTR = crate::Reg<event_intr::EVENT_INTR_SPEC>;
#[doc = "Event(Interrupt) status and Clear register"]
pub mod event_intr;
#[doc = "EVENT_ENABLE register accessor: an alias for `Reg<EVENT_ENABLE_SPEC>`"]
pub type EVENT_ENABLE = crate::Reg<event_enable::EVENT_ENABLE_SPEC>;
#[doc = "Event indications enable."]
pub mod event_enable;
#[doc = "ADV_PARAMS register accessor: an alias for `Reg<ADV_PARAMS_SPEC>`"]
pub type ADV_PARAMS = crate::Reg<adv_params::ADV_PARAMS_SPEC>;
#[doc = "Advertising parameters register."]
pub mod adv_params;
#[doc = "ADV_INTERVAL_TIMEOUT register accessor: an alias for `Reg<ADV_INTERVAL_TIMEOUT_SPEC>`"]
pub type ADV_INTERVAL_TIMEOUT = crate::Reg<adv_interval_timeout::ADV_INTERVAL_TIMEOUT_SPEC>;
#[doc = "Advertising interval register."]
pub mod adv_interval_timeout;
#[doc = "ADV_INTR register accessor: an alias for `Reg<ADV_INTR_SPEC>`"]
pub type ADV_INTR = crate::Reg<adv_intr::ADV_INTR_SPEC>;
#[doc = "Advertising interrupt status and Clear register"]
pub mod adv_intr;
#[doc = "ADV_NEXT_INSTANT register accessor: an alias for `Reg<ADV_NEXT_INSTANT_SPEC>`"]
pub type ADV_NEXT_INSTANT = crate::Reg<adv_next_instant::ADV_NEXT_INSTANT_SPEC>;
#[doc = "Advertising next instant."]
pub mod adv_next_instant;
#[doc = "SCAN_INTERVAL register accessor: an alias for `Reg<SCAN_INTERVAL_SPEC>`"]
pub type SCAN_INTERVAL = crate::Reg<scan_interval::SCAN_INTERVAL_SPEC>;
#[doc = "Scan Interval Register"]
pub mod scan_interval;
#[doc = "SCAN_WINDOW register accessor: an alias for `Reg<SCAN_WINDOW_SPEC>`"]
pub type SCAN_WINDOW = crate::Reg<scan_window::SCAN_WINDOW_SPEC>;
#[doc = "Scan window Register"]
pub mod scan_window;
#[doc = "SCAN_PARAM register accessor: an alias for `Reg<SCAN_PARAM_SPEC>`"]
pub type SCAN_PARAM = crate::Reg<scan_param::SCAN_PARAM_SPEC>;
#[doc = "Scanning parameters register"]
pub mod scan_param;
#[doc = "SCAN_INTR register accessor: an alias for `Reg<SCAN_INTR_SPEC>`"]
pub type SCAN_INTR = crate::Reg<scan_intr::SCAN_INTR_SPEC>;
#[doc = "Scan interrupt status and Clear register"]
pub mod scan_intr;
#[doc = "SCAN_NEXT_INSTANT register accessor: an alias for `Reg<SCAN_NEXT_INSTANT_SPEC>`"]
pub type SCAN_NEXT_INSTANT = crate::Reg<scan_next_instant::SCAN_NEXT_INSTANT_SPEC>;
#[doc = "Advertising next instant."]
pub mod scan_next_instant;
#[doc = "INIT_INTERVAL register accessor: an alias for `Reg<INIT_INTERVAL_SPEC>`"]
pub type INIT_INTERVAL = crate::Reg<init_interval::INIT_INTERVAL_SPEC>;
#[doc = "Initiator Interval Register"]
pub mod init_interval;
#[doc = "INIT_WINDOW register accessor: an alias for `Reg<INIT_WINDOW_SPEC>`"]
pub type INIT_WINDOW = crate::Reg<init_window::INIT_WINDOW_SPEC>;
#[doc = "Initiator window Register"]
pub mod init_window;
#[doc = "INIT_PARAM register accessor: an alias for `Reg<INIT_PARAM_SPEC>`"]
pub type INIT_PARAM = crate::Reg<init_param::INIT_PARAM_SPEC>;
#[doc = "Initiator parameters register"]
pub mod init_param;
#[doc = "INIT_INTR register accessor: an alias for `Reg<INIT_INTR_SPEC>`"]
pub type INIT_INTR = crate::Reg<init_intr::INIT_INTR_SPEC>;
#[doc = "Scan interrupt status and Clear register"]
pub mod init_intr;
#[doc = "INIT_NEXT_INSTANT register accessor: an alias for `Reg<INIT_NEXT_INSTANT_SPEC>`"]
pub type INIT_NEXT_INSTANT = crate::Reg<init_next_instant::INIT_NEXT_INSTANT_SPEC>;
#[doc = "Initiator next instant."]
pub mod init_next_instant;
#[doc = "DEVICE_RAND_ADDR_L register accessor: an alias for `Reg<DEVICE_RAND_ADDR_L_SPEC>`"]
pub type DEVICE_RAND_ADDR_L = crate::Reg<device_rand_addr_l::DEVICE_RAND_ADDR_L_SPEC>;
#[doc = "Lower 16 bit random address of the device."]
pub mod device_rand_addr_l;
#[doc = "DEVICE_RAND_ADDR_M register accessor: an alias for `Reg<DEVICE_RAND_ADDR_M_SPEC>`"]
pub type DEVICE_RAND_ADDR_M = crate::Reg<device_rand_addr_m::DEVICE_RAND_ADDR_M_SPEC>;
#[doc = "Middle 16 bit random address of the device."]
pub mod device_rand_addr_m;
#[doc = "DEVICE_RAND_ADDR_H register accessor: an alias for `Reg<DEVICE_RAND_ADDR_H_SPEC>`"]
pub type DEVICE_RAND_ADDR_H = crate::Reg<device_rand_addr_h::DEVICE_RAND_ADDR_H_SPEC>;
#[doc = "Higher 16 bit random address of the device."]
pub mod device_rand_addr_h;
#[doc = "PEER_ADDR_L register accessor: an alias for `Reg<PEER_ADDR_L_SPEC>`"]
pub type PEER_ADDR_L = crate::Reg<peer_addr_l::PEER_ADDR_L_SPEC>;
#[doc = "Lower 16 bit address of the peer device."]
pub mod peer_addr_l;
#[doc = "PEER_ADDR_M register accessor: an alias for `Reg<PEER_ADDR_M_SPEC>`"]
pub type PEER_ADDR_M = crate::Reg<peer_addr_m::PEER_ADDR_M_SPEC>;
#[doc = "Middle 16 bit address of the peer device."]
pub mod peer_addr_m;
#[doc = "PEER_ADDR_H register accessor: an alias for `Reg<PEER_ADDR_H_SPEC>`"]
pub type PEER_ADDR_H = crate::Reg<peer_addr_h::PEER_ADDR_H_SPEC>;
#[doc = "Higher 16 bit address of the peer device."]
pub mod peer_addr_h;
#[doc = "WL_ADDR_TYPE register accessor: an alias for `Reg<WL_ADDR_TYPE_SPEC>`"]
pub type WL_ADDR_TYPE = crate::Reg<wl_addr_type::WL_ADDR_TYPE_SPEC>;
#[doc = "whitelist address type"]
pub mod wl_addr_type;
#[doc = "WL_ENABLE register accessor: an alias for `Reg<WL_ENABLE_SPEC>`"]
pub type WL_ENABLE = crate::Reg<wl_enable::WL_ENABLE_SPEC>;
#[doc = "whitelist valid entry bit"]
pub mod wl_enable;
#[doc = "TRANSMIT_WINDOW_OFFSET register accessor: an alias for `Reg<TRANSMIT_WINDOW_OFFSET_SPEC>`"]
pub type TRANSMIT_WINDOW_OFFSET = crate::Reg<transmit_window_offset::TRANSMIT_WINDOW_OFFSET_SPEC>;
#[doc = "Transmit window offset"]
pub mod transmit_window_offset;
#[doc = "TRANSMIT_WINDOW_SIZE register accessor: an alias for `Reg<TRANSMIT_WINDOW_SIZE_SPEC>`"]
pub type TRANSMIT_WINDOW_SIZE = crate::Reg<transmit_window_size::TRANSMIT_WINDOW_SIZE_SPEC>;
#[doc = "Transmit window size"]
pub mod transmit_window_size;
#[doc = "DATA_CHANNELS_L0 register accessor: an alias for `Reg<DATA_CHANNELS_L0_SPEC>`"]
pub type DATA_CHANNELS_L0 = crate::Reg<data_channels_l0::DATA_CHANNELS_L0_SPEC>;
#[doc = "Data channel map 0 (lower word)"]
pub mod data_channels_l0;
#[doc = "DATA_CHANNELS_M0 register accessor: an alias for `Reg<DATA_CHANNELS_M0_SPEC>`"]
pub type DATA_CHANNELS_M0 = crate::Reg<data_channels_m0::DATA_CHANNELS_M0_SPEC>;
#[doc = "Data channel map 0 (middle word)"]
pub mod data_channels_m0;
#[doc = "DATA_CHANNELS_H0 register accessor: an alias for `Reg<DATA_CHANNELS_H0_SPEC>`"]
pub type DATA_CHANNELS_H0 = crate::Reg<data_channels_h0::DATA_CHANNELS_H0_SPEC>;
#[doc = "Data channel map 0 (upper word)"]
pub mod data_channels_h0;
#[doc = "DATA_CHANNELS_L1 register accessor: an alias for `Reg<DATA_CHANNELS_L1_SPEC>`"]
pub type DATA_CHANNELS_L1 = crate::Reg<data_channels_l1::DATA_CHANNELS_L1_SPEC>;
#[doc = "Data channel map 1 (lower word)"]
pub mod data_channels_l1;
#[doc = "DATA_CHANNELS_M1 register accessor: an alias for `Reg<DATA_CHANNELS_M1_SPEC>`"]
pub type DATA_CHANNELS_M1 = crate::Reg<data_channels_m1::DATA_CHANNELS_M1_SPEC>;
#[doc = "Data channel map 1 (middle word)"]
pub mod data_channels_m1;
#[doc = "DATA_CHANNELS_H1 register accessor: an alias for `Reg<DATA_CHANNELS_H1_SPEC>`"]
pub type DATA_CHANNELS_H1 = crate::Reg<data_channels_h1::DATA_CHANNELS_H1_SPEC>;
#[doc = "Data channel map 1 (upper word)"]
pub mod data_channels_h1;
#[doc = "CONN_INTR register accessor: an alias for `Reg<CONN_INTR_SPEC>`"]
pub type CONN_INTR = crate::Reg<conn_intr::CONN_INTR_SPEC>;
#[doc = "Connection interrupt status and Clear register"]
pub mod conn_intr;
#[doc = "CONN_STATUS register accessor: an alias for `Reg<CONN_STATUS_SPEC>`"]
pub type CONN_STATUS = crate::Reg<conn_status::CONN_STATUS_SPEC>;
#[doc = "Connection channel status"]
pub mod conn_status;
#[doc = "CONN_INDEX register accessor: an alias for `Reg<CONN_INDEX_SPEC>`"]
pub type CONN_INDEX = crate::Reg<conn_index::CONN_INDEX_SPEC>;
#[doc = "Connection Index register"]
pub mod conn_index;
#[doc = "WAKEUP_CONFIG register accessor: an alias for `Reg<WAKEUP_CONFIG_SPEC>`"]
pub type WAKEUP_CONFIG = crate::Reg<wakeup_config::WAKEUP_CONFIG_SPEC>;
#[doc = "Wakeup configuration"]
pub mod wakeup_config;
#[doc = "WAKEUP_CONTROL register accessor: an alias for `Reg<WAKEUP_CONTROL_SPEC>`"]
pub type WAKEUP_CONTROL = crate::Reg<wakeup_control::WAKEUP_CONTROL_SPEC>;
#[doc = "Wakeup control"]
pub mod wakeup_control;
#[doc = "CLOCK_CONFIG register accessor: an alias for `Reg<CLOCK_CONFIG_SPEC>`"]
pub type CLOCK_CONFIG = crate::Reg<clock_config::CLOCK_CONFIG_SPEC>;
#[doc = "Clock control"]
pub mod clock_config;
#[doc = "TIM_COUNTER_L register accessor: an alias for `Reg<TIM_COUNTER_L_SPEC>`"]
pub type TIM_COUNTER_L = crate::Reg<tim_counter_l::TIM_COUNTER_L_SPEC>;
#[doc = "Reference Clock"]
pub mod tim_counter_l;
#[doc = "WAKEUP_CONFIG_EXTD register accessor: an alias for `Reg<WAKEUP_CONFIG_EXTD_SPEC>`"]
pub type WAKEUP_CONFIG_EXTD = crate::Reg<wakeup_config_extd::WAKEUP_CONFIG_EXTD_SPEC>;
#[doc = "Wakeup configuration extended"]
pub mod wakeup_config_extd;
#[doc = "POC_REG__TIM_CONTROL register accessor: an alias for `Reg<POC_REG__TIM_CONTROL_SPEC>`"]
pub type POC_REG__TIM_CONTROL = crate::Reg<poc_reg__tim_control::POC_REG__TIM_CONTROL_SPEC>;
#[doc = "BLE Time Control"]
pub mod poc_reg__tim_control;
#[doc = "ADV_TX_DATA_FIFO register accessor: an alias for `Reg<ADV_TX_DATA_FIFO_SPEC>`"]
pub type ADV_TX_DATA_FIFO = crate::Reg<adv_tx_data_fifo::ADV_TX_DATA_FIFO_SPEC>;
#[doc = "Advertising data transmit FIFO. Access ADVCH_TX_FIFO."]
pub mod adv_tx_data_fifo;
#[doc = "ADV_SCN_RSP_TX_FIFO register accessor: an alias for `Reg<ADV_SCN_RSP_TX_FIFO_SPEC>`"]
pub type ADV_SCN_RSP_TX_FIFO = crate::Reg<adv_scn_rsp_tx_fifo::ADV_SCN_RSP_TX_FIFO_SPEC>;
#[doc = "Advertising scan response data transmit FIFO. Access ADVCH_TX_FIFO."]
pub mod adv_scn_rsp_tx_fifo;
#[doc = "INIT_SCN_ADV_RX_FIFO register accessor: an alias for `Reg<INIT_SCN_ADV_RX_FIFO_SPEC>`"]
pub type INIT_SCN_ADV_RX_FIFO = crate::Reg<init_scn_adv_rx_fifo::INIT_SCN_ADV_RX_FIFO_SPEC>;
#[doc = "advertising scan response data receive data FIFO. Access ADVRX_FIFO."]
pub mod init_scn_adv_rx_fifo;
#[doc = "CONN_INTERVAL register accessor: an alias for `Reg<CONN_INTERVAL_SPEC>`"]
pub type CONN_INTERVAL = crate::Reg<conn_interval::CONN_INTERVAL_SPEC>;
#[doc = "Connection Interval"]
pub mod conn_interval;
#[doc = "SUP_TIMEOUT register accessor: an alias for `Reg<SUP_TIMEOUT_SPEC>`"]
pub type SUP_TIMEOUT = crate::Reg<sup_timeout::SUP_TIMEOUT_SPEC>;
#[doc = "Supervision timeout"]
pub mod sup_timeout;
#[doc = "SLAVE_LATENCY register accessor: an alias for `Reg<SLAVE_LATENCY_SPEC>`"]
pub type SLAVE_LATENCY = crate::Reg<slave_latency::SLAVE_LATENCY_SPEC>;
#[doc = "Slave Latency"]
pub mod slave_latency;
#[doc = "CE_LENGTH register accessor: an alias for `Reg<CE_LENGTH_SPEC>`"]
pub type CE_LENGTH = crate::Reg<ce_length::CE_LENGTH_SPEC>;
#[doc = "Connection event length"]
pub mod ce_length;
#[doc = "PDU_ACCESS_ADDR_L_REGISTER register accessor: an alias for `Reg<PDU_ACCESS_ADDR_L_REGISTER_SPEC>`"]
pub type PDU_ACCESS_ADDR_L_REGISTER =
    crate::Reg<pdu_access_addr_l_register::PDU_ACCESS_ADDR_L_REGISTER_SPEC>;
#[doc = "Access address (lower)"]
pub mod pdu_access_addr_l_register;
#[doc = "PDU_ACCESS_ADDR_H_REGISTER register accessor: an alias for `Reg<PDU_ACCESS_ADDR_H_REGISTER_SPEC>`"]
pub type PDU_ACCESS_ADDR_H_REGISTER =
    crate::Reg<pdu_access_addr_h_register::PDU_ACCESS_ADDR_H_REGISTER_SPEC>;
#[doc = "Access address (upper)"]
pub mod pdu_access_addr_h_register;
#[doc = "CONN_CE_INSTANT register accessor: an alias for `Reg<CONN_CE_INSTANT_SPEC>`"]
pub type CONN_CE_INSTANT = crate::Reg<conn_ce_instant::CONN_CE_INSTANT_SPEC>;
#[doc = "Connection event instant"]
pub mod conn_ce_instant;
#[doc = "CE_CNFG_STS_REGISTER register accessor: an alias for `Reg<CE_CNFG_STS_REGISTER_SPEC>`"]
pub type CE_CNFG_STS_REGISTER = crate::Reg<ce_cnfg_sts_register::CE_CNFG_STS_REGISTER_SPEC>;
#[doc = "connection configuration & status register"]
pub mod ce_cnfg_sts_register;
#[doc = "NEXT_CE_INSTANT register accessor: an alias for `Reg<NEXT_CE_INSTANT_SPEC>`"]
pub type NEXT_CE_INSTANT = crate::Reg<next_ce_instant::NEXT_CE_INSTANT_SPEC>;
#[doc = "Next connection event instant"]
pub mod next_ce_instant;
#[doc = "CONN_CE_COUNTER register accessor: an alias for `Reg<CONN_CE_COUNTER_SPEC>`"]
pub type CONN_CE_COUNTER = crate::Reg<conn_ce_counter::CONN_CE_COUNTER_SPEC>;
#[doc = "connection event counter"]
pub mod conn_ce_counter;
#[doc = "DATA_LIST_SENT_UPDATE__STATUS register accessor: an alias for `Reg<DATA_LIST_SENT_UPDATE__STATUS_SPEC>`"]
pub type DATA_LIST_SENT_UPDATE__STATUS =
    crate::Reg<data_list_sent_update__status::DATA_LIST_SENT_UPDATE__STATUS_SPEC>;
#[doc = "data list sent update and status"]
pub mod data_list_sent_update__status;
#[doc = "DATA_LIST_ACK_UPDATE__STATUS register accessor: an alias for `Reg<DATA_LIST_ACK_UPDATE__STATUS_SPEC>`"]
pub type DATA_LIST_ACK_UPDATE__STATUS =
    crate::Reg<data_list_ack_update__status::DATA_LIST_ACK_UPDATE__STATUS_SPEC>;
#[doc = "data list ack update and status"]
pub mod data_list_ack_update__status;
#[doc = "CE_CNFG_STS_REGISTER_EXT register accessor: an alias for `Reg<CE_CNFG_STS_REGISTER_EXT_SPEC>`"]
pub type CE_CNFG_STS_REGISTER_EXT =
    crate::Reg<ce_cnfg_sts_register_ext::CE_CNFG_STS_REGISTER_EXT_SPEC>;
#[doc = "connection configuration & status register"]
pub mod ce_cnfg_sts_register_ext;
#[doc = "CONN_EXT_INTR register accessor: an alias for `Reg<CONN_EXT_INTR_SPEC>`"]
pub type CONN_EXT_INTR = crate::Reg<conn_ext_intr::CONN_EXT_INTR_SPEC>;
#[doc = "Connection extended interrupt status and Clear register"]
pub mod conn_ext_intr;
#[doc = "CONN_EXT_INTR_MASK register accessor: an alias for `Reg<CONN_EXT_INTR_MASK_SPEC>`"]
pub type CONN_EXT_INTR_MASK = crate::Reg<conn_ext_intr_mask::CONN_EXT_INTR_MASK_SPEC>;
#[doc = "Connection Extended Interrupt mask"]
pub mod conn_ext_intr_mask;
#[doc = "DATA_MEM_DESCRIPTOR register accessor: an alias for `Reg<DATA_MEM_DESCRIPTOR_SPEC>`"]
pub type DATA_MEM_DESCRIPTOR = crate::Reg<data_mem_descriptor::DATA_MEM_DESCRIPTOR_SPEC>;
#[doc = "Data buffer descriptor 0 to 4"]
pub mod data_mem_descriptor;
#[doc = "WINDOW_WIDEN_INTVL register accessor: an alias for `Reg<WINDOW_WIDEN_INTVL_SPEC>`"]
pub type WINDOW_WIDEN_INTVL = crate::Reg<window_widen_intvl::WINDOW_WIDEN_INTVL_SPEC>;
#[doc = "Window widen for interval"]
pub mod window_widen_intvl;
#[doc = "WINDOW_WIDEN_WINOFF register accessor: an alias for `Reg<WINDOW_WIDEN_WINOFF_SPEC>`"]
pub type WINDOW_WIDEN_WINOFF = crate::Reg<window_widen_winoff::WINDOW_WIDEN_WINOFF_SPEC>;
#[doc = "Window widen for offset"]
pub mod window_widen_winoff;
#[doc = "LE_RF_TEST_MODE register accessor: an alias for `Reg<LE_RF_TEST_MODE_SPEC>`"]
pub type LE_RF_TEST_MODE = crate::Reg<le_rf_test_mode::LE_RF_TEST_MODE_SPEC>;
#[doc = "Direct Test Mode control"]
pub mod le_rf_test_mode;
#[doc = "DTM_RX_PKT_COUNT register accessor: an alias for `Reg<DTM_RX_PKT_COUNT_SPEC>`"]
pub type DTM_RX_PKT_COUNT = crate::Reg<dtm_rx_pkt_count::DTM_RX_PKT_COUNT_SPEC>;
#[doc = "Direct Test Mode receive packet count"]
pub mod dtm_rx_pkt_count;
#[doc = "LE_RF_TEST_MODE_EXT register accessor: an alias for `Reg<LE_RF_TEST_MODE_EXT_SPEC>`"]
pub type LE_RF_TEST_MODE_EXT = crate::Reg<le_rf_test_mode_ext::LE_RF_TEST_MODE_EXT_SPEC>;
#[doc = "Direct Test Mode control"]
pub mod le_rf_test_mode_ext;
#[doc = "TXRX_HOP register accessor: an alias for `Reg<TXRX_HOP_SPEC>`"]
pub type TXRX_HOP = crate::Reg<txrx_hop::TXRX_HOP_SPEC>;
#[doc = "Channel Address register"]
pub mod txrx_hop;
#[doc = "TX_RX_ON_DELAY register accessor: an alias for `Reg<TX_RX_ON_DELAY_SPEC>`"]
pub type TX_RX_ON_DELAY = crate::Reg<tx_rx_on_delay::TX_RX_ON_DELAY_SPEC>;
#[doc = "Transmit/Receive data delay"]
pub mod tx_rx_on_delay;
#[doc = "ADV_ACCADDR_L register accessor: an alias for `Reg<ADV_ACCADDR_L_SPEC>`"]
pub type ADV_ACCADDR_L = crate::Reg<adv_accaddr_l::ADV_ACCADDR_L_SPEC>;
#[doc = "ADV packet access code low word"]
pub mod adv_accaddr_l;
#[doc = "ADV_ACCADDR_H register accessor: an alias for `Reg<ADV_ACCADDR_H_SPEC>`"]
pub type ADV_ACCADDR_H = crate::Reg<adv_accaddr_h::ADV_ACCADDR_H_SPEC>;
#[doc = "ADV packet access code high word"]
pub mod adv_accaddr_h;
#[doc = "ADV_CH_TX_POWER_LVL_LS register accessor: an alias for `Reg<ADV_CH_TX_POWER_LVL_LS_SPEC>`"]
pub type ADV_CH_TX_POWER_LVL_LS = crate::Reg<adv_ch_tx_power_lvl_ls::ADV_CH_TX_POWER_LVL_LS_SPEC>;
#[doc = "Advertising channel transmit power setting"]
pub mod adv_ch_tx_power_lvl_ls;
#[doc = "ADV_CH_TX_POWER_LVL_MS register accessor: an alias for `Reg<ADV_CH_TX_POWER_LVL_MS_SPEC>`"]
pub type ADV_CH_TX_POWER_LVL_MS = crate::Reg<adv_ch_tx_power_lvl_ms::ADV_CH_TX_POWER_LVL_MS_SPEC>;
#[doc = "Advertising channel transmit power setting extension"]
pub mod adv_ch_tx_power_lvl_ms;
#[doc = "CONN_CH_TX_POWER_LVL_LS register accessor: an alias for `Reg<CONN_CH_TX_POWER_LVL_LS_SPEC>`"]
pub type CONN_CH_TX_POWER_LVL_LS =
    crate::Reg<conn_ch_tx_power_lvl_ls::CONN_CH_TX_POWER_LVL_LS_SPEC>;
#[doc = "Connection channel transmit power setting"]
pub mod conn_ch_tx_power_lvl_ls;
#[doc = "CONN_CH_TX_POWER_LVL_MS register accessor: an alias for `Reg<CONN_CH_TX_POWER_LVL_MS_SPEC>`"]
pub type CONN_CH_TX_POWER_LVL_MS =
    crate::Reg<conn_ch_tx_power_lvl_ms::CONN_CH_TX_POWER_LVL_MS_SPEC>;
#[doc = "Connection channel transmit power setting extension"]
pub mod conn_ch_tx_power_lvl_ms;
#[doc = "DEV_PUB_ADDR_L register accessor: an alias for `Reg<DEV_PUB_ADDR_L_SPEC>`"]
pub type DEV_PUB_ADDR_L = crate::Reg<dev_pub_addr_l::DEV_PUB_ADDR_L_SPEC>;
#[doc = "Device public address lower register"]
pub mod dev_pub_addr_l;
#[doc = "DEV_PUB_ADDR_M register accessor: an alias for `Reg<DEV_PUB_ADDR_M_SPEC>`"]
pub type DEV_PUB_ADDR_M = crate::Reg<dev_pub_addr_m::DEV_PUB_ADDR_M_SPEC>;
#[doc = "Device public address middle register"]
pub mod dev_pub_addr_m;
#[doc = "DEV_PUB_ADDR_H register accessor: an alias for `Reg<DEV_PUB_ADDR_H_SPEC>`"]
pub type DEV_PUB_ADDR_H = crate::Reg<dev_pub_addr_h::DEV_PUB_ADDR_H_SPEC>;
#[doc = "Device public address higher register"]
pub mod dev_pub_addr_h;
#[doc = "OFFSET_TO_FIRST_INSTANT register accessor: an alias for `Reg<OFFSET_TO_FIRST_INSTANT_SPEC>`"]
pub type OFFSET_TO_FIRST_INSTANT =
    crate::Reg<offset_to_first_instant::OFFSET_TO_FIRST_INSTANT_SPEC>;
#[doc = "Offset to first instant"]
pub mod offset_to_first_instant;
#[doc = "ADV_CONFIG register accessor: an alias for `Reg<ADV_CONFIG_SPEC>`"]
pub type ADV_CONFIG = crate::Reg<adv_config::ADV_CONFIG_SPEC>;
#[doc = "Advertiser configuration register"]
pub mod adv_config;
#[doc = "SCAN_CONFIG register accessor: an alias for `Reg<SCAN_CONFIG_SPEC>`"]
pub type SCAN_CONFIG = crate::Reg<scan_config::SCAN_CONFIG_SPEC>;
#[doc = "Scan configuration register"]
pub mod scan_config;
#[doc = "INIT_CONFIG register accessor: an alias for `Reg<INIT_CONFIG_SPEC>`"]
pub type INIT_CONFIG = crate::Reg<init_config::INIT_CONFIG_SPEC>;
#[doc = "Initiator configuration register"]
pub mod init_config;
#[doc = "CONN_CONFIG register accessor: an alias for `Reg<CONN_CONFIG_SPEC>`"]
pub type CONN_CONFIG = crate::Reg<conn_config::CONN_CONFIG_SPEC>;
#[doc = "Connection configuration register"]
pub mod conn_config;
#[doc = "CONN_PARAM1 register accessor: an alias for `Reg<CONN_PARAM1_SPEC>`"]
pub type CONN_PARAM1 = crate::Reg<conn_param1::CONN_PARAM1_SPEC>;
#[doc = "Connection parameter 1"]
pub mod conn_param1;
#[doc = "CONN_PARAM2 register accessor: an alias for `Reg<CONN_PARAM2_SPEC>`"]
pub type CONN_PARAM2 = crate::Reg<conn_param2::CONN_PARAM2_SPEC>;
#[doc = "Connection parameter 2"]
pub mod conn_param2;
#[doc = "CONN_INTR_MASK register accessor: an alias for `Reg<CONN_INTR_MASK_SPEC>`"]
pub type CONN_INTR_MASK = crate::Reg<conn_intr_mask::CONN_INTR_MASK_SPEC>;
#[doc = "Connection Interrupt mask"]
pub mod conn_intr_mask;
#[doc = "SLAVE_TIMING_CONTROL register accessor: an alias for `Reg<SLAVE_TIMING_CONTROL_SPEC>`"]
pub type SLAVE_TIMING_CONTROL = crate::Reg<slave_timing_control::SLAVE_TIMING_CONTROL_SPEC>;
#[doc = "slave timing control"]
pub mod slave_timing_control;
#[doc = "RECEIVE_TRIG_CTRL register accessor: an alias for `Reg<RECEIVE_TRIG_CTRL_SPEC>`"]
pub type RECEIVE_TRIG_CTRL = crate::Reg<receive_trig_ctrl::RECEIVE_TRIG_CTRL_SPEC>;
#[doc = "Receive trigger control"]
pub mod receive_trig_ctrl;
#[doc = "LL_DBG_1 register accessor: an alias for `Reg<LL_DBG_1_SPEC>`"]
pub type LL_DBG_1 = crate::Reg<ll_dbg_1::LL_DBG_1_SPEC>;
#[doc = "LL debug register 1"]
pub mod ll_dbg_1;
#[doc = "LL_DBG_2 register accessor: an alias for `Reg<LL_DBG_2_SPEC>`"]
pub type LL_DBG_2 = crate::Reg<ll_dbg_2::LL_DBG_2_SPEC>;
#[doc = "LL debug register 2"]
pub mod ll_dbg_2;
#[doc = "LL_DBG_3 register accessor: an alias for `Reg<LL_DBG_3_SPEC>`"]
pub type LL_DBG_3 = crate::Reg<ll_dbg_3::LL_DBG_3_SPEC>;
#[doc = "LL debug register 3"]
pub mod ll_dbg_3;
#[doc = "LL_DBG_4 register accessor: an alias for `Reg<LL_DBG_4_SPEC>`"]
pub type LL_DBG_4 = crate::Reg<ll_dbg_4::LL_DBG_4_SPEC>;
#[doc = "LL debug register 4"]
pub mod ll_dbg_4;
#[doc = "LL_DBG_5 register accessor: an alias for `Reg<LL_DBG_5_SPEC>`"]
pub type LL_DBG_5 = crate::Reg<ll_dbg_5::LL_DBG_5_SPEC>;
#[doc = "LL debug register 5"]
pub mod ll_dbg_5;
#[doc = "LL_DBG_6 register accessor: an alias for `Reg<LL_DBG_6_SPEC>`"]
pub type LL_DBG_6 = crate::Reg<ll_dbg_6::LL_DBG_6_SPEC>;
#[doc = "LL debug register 6"]
pub mod ll_dbg_6;
#[doc = "LL_DBG_7 register accessor: an alias for `Reg<LL_DBG_7_SPEC>`"]
pub type LL_DBG_7 = crate::Reg<ll_dbg_7::LL_DBG_7_SPEC>;
#[doc = "LL debug register 7"]
pub mod ll_dbg_7;
#[doc = "LL_DBG_8 register accessor: an alias for `Reg<LL_DBG_8_SPEC>`"]
pub type LL_DBG_8 = crate::Reg<ll_dbg_8::LL_DBG_8_SPEC>;
#[doc = "LL debug register 8"]
pub mod ll_dbg_8;
#[doc = "LL_DBG_9 register accessor: an alias for `Reg<LL_DBG_9_SPEC>`"]
pub type LL_DBG_9 = crate::Reg<ll_dbg_9::LL_DBG_9_SPEC>;
#[doc = "LL debug register 9"]
pub mod ll_dbg_9;
#[doc = "LL_DBG_10 register accessor: an alias for `Reg<LL_DBG_10_SPEC>`"]
pub type LL_DBG_10 = crate::Reg<ll_dbg_10::LL_DBG_10_SPEC>;
#[doc = "LL debug register 10"]
pub mod ll_dbg_10;
#[doc = "PEER_ADDR_INIT_L register accessor: an alias for `Reg<PEER_ADDR_INIT_L_SPEC>`"]
pub type PEER_ADDR_INIT_L = crate::Reg<peer_addr_init_l::PEER_ADDR_INIT_L_SPEC>;
#[doc = "Lower 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_l;
#[doc = "PEER_ADDR_INIT_M register accessor: an alias for `Reg<PEER_ADDR_INIT_M_SPEC>`"]
pub type PEER_ADDR_INIT_M = crate::Reg<peer_addr_init_m::PEER_ADDR_INIT_M_SPEC>;
#[doc = "Middle 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_m;
#[doc = "PEER_ADDR_INIT_H register accessor: an alias for `Reg<PEER_ADDR_INIT_H_SPEC>`"]
pub type PEER_ADDR_INIT_H = crate::Reg<peer_addr_init_h::PEER_ADDR_INIT_H_SPEC>;
#[doc = "Higher 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_h;
#[doc = "PEER_SEC_ADDR_ADV_L register accessor: an alias for `Reg<PEER_SEC_ADDR_ADV_L_SPEC>`"]
pub type PEER_SEC_ADDR_ADV_L = crate::Reg<peer_sec_addr_adv_l::PEER_SEC_ADDR_ADV_L_SPEC>;
#[doc = "Lower 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_l;
#[doc = "PEER_SEC_ADDR_ADV_M register accessor: an alias for `Reg<PEER_SEC_ADDR_ADV_M_SPEC>`"]
pub type PEER_SEC_ADDR_ADV_M = crate::Reg<peer_sec_addr_adv_m::PEER_SEC_ADDR_ADV_M_SPEC>;
#[doc = "Middle 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_m;
#[doc = "PEER_SEC_ADDR_ADV_H register accessor: an alias for `Reg<PEER_SEC_ADDR_ADV_H_SPEC>`"]
pub type PEER_SEC_ADDR_ADV_H = crate::Reg<peer_sec_addr_adv_h::PEER_SEC_ADDR_ADV_H_SPEC>;
#[doc = "Higher 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_h;
#[doc = "INIT_WINDOW_TIMER_CTRL register accessor: an alias for `Reg<INIT_WINDOW_TIMER_CTRL_SPEC>`"]
pub type INIT_WINDOW_TIMER_CTRL = crate::Reg<init_window_timer_ctrl::INIT_WINDOW_TIMER_CTRL_SPEC>;
#[doc = "Initiator Window NI timer control"]
pub mod init_window_timer_ctrl;
#[doc = "CONN_CONFIG_EXT register accessor: an alias for `Reg<CONN_CONFIG_EXT_SPEC>`"]
pub type CONN_CONFIG_EXT = crate::Reg<conn_config_ext::CONN_CONFIG_EXT_SPEC>;
#[doc = "Connection extended configuration register"]
pub mod conn_config_ext;
#[doc = "DPLL_CONFIG register accessor: an alias for `Reg<DPLL_CONFIG_SPEC>`"]
pub type DPLL_CONFIG = crate::Reg<dpll_config::DPLL_CONFIG_SPEC>;
#[doc = "DPLL & CY Correlator configuration register"]
pub mod dpll_config;
#[doc = "INIT_NI_VAL register accessor: an alias for `Reg<INIT_NI_VAL_SPEC>`"]
pub type INIT_NI_VAL = crate::Reg<init_ni_val::INIT_NI_VAL_SPEC>;
#[doc = "Initiator Window NI instant"]
pub mod init_ni_val;
#[doc = "INIT_WINDOW_OFFSET register accessor: an alias for `Reg<INIT_WINDOW_OFFSET_SPEC>`"]
pub type INIT_WINDOW_OFFSET = crate::Reg<init_window_offset::INIT_WINDOW_OFFSET_SPEC>;
#[doc = "Initiator Window offset captured at conn request"]
pub mod init_window_offset;
#[doc = "INIT_WINDOW_NI_ANCHOR_PT register accessor: an alias for `Reg<INIT_WINDOW_NI_ANCHOR_PT_SPEC>`"]
pub type INIT_WINDOW_NI_ANCHOR_PT =
    crate::Reg<init_window_ni_anchor_pt::INIT_WINDOW_NI_ANCHOR_PT_SPEC>;
#[doc = "Initiator Window NI anchor point captured at conn request"]
pub mod init_window_ni_anchor_pt;
#[doc = "CONN_UPDATE_NEW_INTERVAL register accessor: an alias for `Reg<CONN_UPDATE_NEW_INTERVAL_SPEC>`"]
pub type CONN_UPDATE_NEW_INTERVAL =
    crate::Reg<conn_update_new_interval::CONN_UPDATE_NEW_INTERVAL_SPEC>;
#[doc = "Connection update new interval"]
pub mod conn_update_new_interval;
#[doc = "CONN_UPDATE_NEW_LATENCY register accessor: an alias for `Reg<CONN_UPDATE_NEW_LATENCY_SPEC>`"]
pub type CONN_UPDATE_NEW_LATENCY =
    crate::Reg<conn_update_new_latency::CONN_UPDATE_NEW_LATENCY_SPEC>;
#[doc = "Connection update new latency"]
pub mod conn_update_new_latency;
#[doc = "CONN_UPDATE_NEW_SUP_TO register accessor: an alias for `Reg<CONN_UPDATE_NEW_SUP_TO_SPEC>`"]
pub type CONN_UPDATE_NEW_SUP_TO = crate::Reg<conn_update_new_sup_to::CONN_UPDATE_NEW_SUP_TO_SPEC>;
#[doc = "Connection update new supervision timeout"]
pub mod conn_update_new_sup_to;
#[doc = "CONN_UPDATE_NEW_SL_INTERVAL register accessor: an alias for `Reg<CONN_UPDATE_NEW_SL_INTERVAL_SPEC>`"]
pub type CONN_UPDATE_NEW_SL_INTERVAL =
    crate::Reg<conn_update_new_sl_interval::CONN_UPDATE_NEW_SL_INTERVAL_SPEC>;
#[doc = "Connection update new Slave Latency X Conn interval Value"]
pub mod conn_update_new_sl_interval;
#[doc = "CONN_REQ_WORD0 register accessor: an alias for `Reg<CONN_REQ_WORD0_SPEC>`"]
pub type CONN_REQ_WORD0 = crate::Reg<conn_req_word0::CONN_REQ_WORD0_SPEC>;
#[doc = "Connection request address word 0"]
pub mod conn_req_word0;
#[doc = "CONN_REQ_WORD1 register accessor: an alias for `Reg<CONN_REQ_WORD1_SPEC>`"]
pub type CONN_REQ_WORD1 = crate::Reg<conn_req_word1::CONN_REQ_WORD1_SPEC>;
#[doc = "Connection request address word 1"]
pub mod conn_req_word1;
#[doc = "CONN_REQ_WORD2 register accessor: an alias for `Reg<CONN_REQ_WORD2_SPEC>`"]
pub type CONN_REQ_WORD2 = crate::Reg<conn_req_word2::CONN_REQ_WORD2_SPEC>;
#[doc = "Connection request address word 2"]
pub mod conn_req_word2;
#[doc = "CONN_REQ_WORD3 register accessor: an alias for `Reg<CONN_REQ_WORD3_SPEC>`"]
pub type CONN_REQ_WORD3 = crate::Reg<conn_req_word3::CONN_REQ_WORD3_SPEC>;
#[doc = "Connection request address word 3"]
pub mod conn_req_word3;
#[doc = "CONN_REQ_WORD4 register accessor: an alias for `Reg<CONN_REQ_WORD4_SPEC>`"]
pub type CONN_REQ_WORD4 = crate::Reg<conn_req_word4::CONN_REQ_WORD4_SPEC>;
#[doc = "Connection request address word 4"]
pub mod conn_req_word4;
#[doc = "CONN_REQ_WORD5 register accessor: an alias for `Reg<CONN_REQ_WORD5_SPEC>`"]
pub type CONN_REQ_WORD5 = crate::Reg<conn_req_word5::CONN_REQ_WORD5_SPEC>;
#[doc = "Connection request address word 5"]
pub mod conn_req_word5;
#[doc = "CONN_REQ_WORD6 register accessor: an alias for `Reg<CONN_REQ_WORD6_SPEC>`"]
pub type CONN_REQ_WORD6 = crate::Reg<conn_req_word6::CONN_REQ_WORD6_SPEC>;
#[doc = "Connection request address word 6"]
pub mod conn_req_word6;
#[doc = "CONN_REQ_WORD7 register accessor: an alias for `Reg<CONN_REQ_WORD7_SPEC>`"]
pub type CONN_REQ_WORD7 = crate::Reg<conn_req_word7::CONN_REQ_WORD7_SPEC>;
#[doc = "Connection request address word 7"]
pub mod conn_req_word7;
#[doc = "CONN_REQ_WORD8 register accessor: an alias for `Reg<CONN_REQ_WORD8_SPEC>`"]
pub type CONN_REQ_WORD8 = crate::Reg<conn_req_word8::CONN_REQ_WORD8_SPEC>;
#[doc = "Connection request address word 8"]
pub mod conn_req_word8;
#[doc = "CONN_REQ_WORD9 register accessor: an alias for `Reg<CONN_REQ_WORD9_SPEC>`"]
pub type CONN_REQ_WORD9 = crate::Reg<conn_req_word9::CONN_REQ_WORD9_SPEC>;
#[doc = "Connection request address word 9"]
pub mod conn_req_word9;
#[doc = "CONN_REQ_WORD10 register accessor: an alias for `Reg<CONN_REQ_WORD10_SPEC>`"]
pub type CONN_REQ_WORD10 = crate::Reg<conn_req_word10::CONN_REQ_WORD10_SPEC>;
#[doc = "Connection request address word 10"]
pub mod conn_req_word10;
#[doc = "CONN_REQ_WORD11 register accessor: an alias for `Reg<CONN_REQ_WORD11_SPEC>`"]
pub type CONN_REQ_WORD11 = crate::Reg<conn_req_word11::CONN_REQ_WORD11_SPEC>;
#[doc = "Connection request address word 11"]
pub mod conn_req_word11;
#[doc = "PDU_RESP_TIMER register accessor: an alias for `Reg<PDU_RESP_TIMER_SPEC>`"]
pub type PDU_RESP_TIMER = crate::Reg<pdu_resp_timer::PDU_RESP_TIMER_SPEC>;
#[doc = "PDU response timer/Generic Timer (MMMS mode)"]
pub mod pdu_resp_timer;
#[doc = "NEXT_RESP_TIMER_EXP register accessor: an alias for `Reg<NEXT_RESP_TIMER_EXP_SPEC>`"]
pub type NEXT_RESP_TIMER_EXP = crate::Reg<next_resp_timer_exp::NEXT_RESP_TIMER_EXP_SPEC>;
#[doc = "Next response timeout instant"]
pub mod next_resp_timer_exp;
#[doc = "NEXT_SUP_TO register accessor: an alias for `Reg<NEXT_SUP_TO_SPEC>`"]
pub type NEXT_SUP_TO = crate::Reg<next_sup_to::NEXT_SUP_TO_SPEC>;
#[doc = "Next supervision timeout instant"]
pub mod next_sup_to;
#[doc = "LLH_FEATURE_CONFIG register accessor: an alias for `Reg<LLH_FEATURE_CONFIG_SPEC>`"]
pub type LLH_FEATURE_CONFIG = crate::Reg<llh_feature_config::LLH_FEATURE_CONFIG_SPEC>;
#[doc = "Feature enable"]
pub mod llh_feature_config;
#[doc = "WIN_MIN_STEP_SIZE register accessor: an alias for `Reg<WIN_MIN_STEP_SIZE_SPEC>`"]
pub type WIN_MIN_STEP_SIZE = crate::Reg<win_min_step_size::WIN_MIN_STEP_SIZE_SPEC>;
#[doc = "Window minimum step size"]
pub mod win_min_step_size;
#[doc = "SLV_WIN_ADJ register accessor: an alias for `Reg<SLV_WIN_ADJ_SPEC>`"]
pub type SLV_WIN_ADJ = crate::Reg<slv_win_adj::SLV_WIN_ADJ_SPEC>;
#[doc = "Slave window adjustment"]
pub mod slv_win_adj;
#[doc = "SL_CONN_INTERVAL register accessor: an alias for `Reg<SL_CONN_INTERVAL_SPEC>`"]
pub type SL_CONN_INTERVAL = crate::Reg<sl_conn_interval::SL_CONN_INTERVAL_SPEC>;
#[doc = "Slave Latency X Conn Interval Value"]
pub mod sl_conn_interval;
#[doc = "LE_PING_TIMER_ADDR register accessor: an alias for `Reg<LE_PING_TIMER_ADDR_SPEC>`"]
pub type LE_PING_TIMER_ADDR = crate::Reg<le_ping_timer_addr::LE_PING_TIMER_ADDR_SPEC>;
#[doc = "LE Ping connection timer address"]
pub mod le_ping_timer_addr;
#[doc = "LE_PING_TIMER_OFFSET register accessor: an alias for `Reg<LE_PING_TIMER_OFFSET_SPEC>`"]
pub type LE_PING_TIMER_OFFSET = crate::Reg<le_ping_timer_offset::LE_PING_TIMER_OFFSET_SPEC>;
#[doc = "LE Ping connection timer offset"]
pub mod le_ping_timer_offset;
#[doc = "LE_PING_TIMER_NEXT_EXP register accessor: an alias for `Reg<LE_PING_TIMER_NEXT_EXP_SPEC>`"]
pub type LE_PING_TIMER_NEXT_EXP = crate::Reg<le_ping_timer_next_exp::LE_PING_TIMER_NEXT_EXP_SPEC>;
#[doc = "LE Ping timer next expiry instant"]
pub mod le_ping_timer_next_exp;
#[doc = "LE_PING_TIMER_WRAP_COUNT register accessor: an alias for `Reg<LE_PING_TIMER_WRAP_COUNT_SPEC>`"]
pub type LE_PING_TIMER_WRAP_COUNT =
    crate::Reg<le_ping_timer_wrap_count::LE_PING_TIMER_WRAP_COUNT_SPEC>;
#[doc = "LE Ping Timer wrap count"]
pub mod le_ping_timer_wrap_count;
#[doc = "TX_EN_EXT_DELAY register accessor: an alias for `Reg<TX_EN_EXT_DELAY_SPEC>`"]
pub type TX_EN_EXT_DELAY = crate::Reg<tx_en_ext_delay::TX_EN_EXT_DELAY_SPEC>;
#[doc = "Transmit enable extension delay"]
pub mod tx_en_ext_delay;
#[doc = "TX_RX_SYNTH_DELAY register accessor: an alias for `Reg<TX_RX_SYNTH_DELAY_SPEC>`"]
pub type TX_RX_SYNTH_DELAY = crate::Reg<tx_rx_synth_delay::TX_RX_SYNTH_DELAY_SPEC>;
#[doc = "Transmit/Receive enable delay"]
pub mod tx_rx_synth_delay;
#[doc = "EXT_PA_LNA_DLY_CNFG register accessor: an alias for `Reg<EXT_PA_LNA_DLY_CNFG_SPEC>`"]
pub type EXT_PA_LNA_DLY_CNFG = crate::Reg<ext_pa_lna_dly_cnfg::EXT_PA_LNA_DLY_CNFG_SPEC>;
#[doc = "External TX PA and RX LNA delay configuration"]
pub mod ext_pa_lna_dly_cnfg;
#[doc = "LL_CONFIG register accessor: an alias for `Reg<LL_CONFIG_SPEC>`"]
pub type LL_CONFIG = crate::Reg<ll_config::LL_CONFIG_SPEC>;
#[doc = "Link Layer additional configuration"]
pub mod ll_config;
#[doc = "LL_CONTROL register accessor: an alias for `Reg<LL_CONTROL_SPEC>`"]
pub type LL_CONTROL = crate::Reg<ll_control::LL_CONTROL_SPEC>;
#[doc = "LL Backward compatibility"]
pub mod ll_control;
#[doc = "DEV_PA_ADDR_L register accessor: an alias for `Reg<DEV_PA_ADDR_L_SPEC>`"]
pub type DEV_PA_ADDR_L = crate::Reg<dev_pa_addr_l::DEV_PA_ADDR_L_SPEC>;
#[doc = "Device Resolvable/Non-Resolvable Private address lower register"]
pub mod dev_pa_addr_l;
#[doc = "DEV_PA_ADDR_M register accessor: an alias for `Reg<DEV_PA_ADDR_M_SPEC>`"]
pub type DEV_PA_ADDR_M = crate::Reg<dev_pa_addr_m::DEV_PA_ADDR_M_SPEC>;
#[doc = "Device Resolvable/Non-Resolvable Private address middle register"]
pub mod dev_pa_addr_m;
#[doc = "DEV_PA_ADDR_H register accessor: an alias for `Reg<DEV_PA_ADDR_H_SPEC>`"]
pub type DEV_PA_ADDR_H = crate::Reg<dev_pa_addr_h::DEV_PA_ADDR_H_SPEC>;
#[doc = "Device Resolvable/Non-Resolvable Private address higher register"]
pub mod dev_pa_addr_h;
#[doc = "RSLV_LIST_ENABLE register accessor: an alias for `Reg<RSLV_LIST_ENABLE_SPEC>`"]
pub type RSLV_LIST_ENABLE = crate::Reg<rslv_list_enable::RSLV_LIST_ENABLE_SPEC>;
#[doc = "Resolving list entry control bit"]
pub mod rslv_list_enable;
#[doc = "WL_CONNECTION_STATUS register accessor: an alias for `Reg<WL_CONNECTION_STATUS_SPEC>`"]
pub type WL_CONNECTION_STATUS = crate::Reg<wl_connection_status::WL_CONNECTION_STATUS_SPEC>;
#[doc = "whitelist valid entry bit"]
pub mod wl_connection_status;
#[doc = "CONN_RXMEM_BASE_ADDR_DLE register accessor: an alias for `Reg<CONN_RXMEM_BASE_ADDR_DLE_SPEC>`"]
pub type CONN_RXMEM_BASE_ADDR_DLE =
    crate::Reg<conn_rxmem_base_addr_dle::CONN_RXMEM_BASE_ADDR_DLE_SPEC>;
#[doc = "DLE Connection RX memory base address"]
pub mod conn_rxmem_base_addr_dle;
#[doc = "CONN_TXMEM_BASE_ADDR_DLE register accessor: an alias for `Reg<CONN_TXMEM_BASE_ADDR_DLE_SPEC>`"]
pub type CONN_TXMEM_BASE_ADDR_DLE =
    crate::Reg<conn_txmem_base_addr_dle::CONN_TXMEM_BASE_ADDR_DLE_SPEC>;
#[doc = "DLE Connection TX memory base address"]
pub mod conn_txmem_base_addr_dle;
#[doc = "CONN_1_PARAM_MEM_BASE_ADDR register accessor: an alias for `Reg<CONN_1_PARAM_MEM_BASE_ADDR_SPEC>`"]
pub type CONN_1_PARAM_MEM_BASE_ADDR =
    crate::Reg<conn_1_param_mem_base_addr::CONN_1_PARAM_MEM_BASE_ADDR_SPEC>;
#[doc = "Connection Parameter memory base address for connection 1"]
pub mod conn_1_param_mem_base_addr;
#[doc = "CONN_2_PARAM_MEM_BASE_ADDR register accessor: an alias for `Reg<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>`"]
pub type CONN_2_PARAM_MEM_BASE_ADDR =
    crate::Reg<conn_2_param_mem_base_addr::CONN_2_PARAM_MEM_BASE_ADDR_SPEC>;
#[doc = "Connection Parameter memory base address for connection 2"]
pub mod conn_2_param_mem_base_addr;
#[doc = "CONN_3_PARAM_MEM_BASE_ADDR register accessor: an alias for `Reg<CONN_3_PARAM_MEM_BASE_ADDR_SPEC>`"]
pub type CONN_3_PARAM_MEM_BASE_ADDR =
    crate::Reg<conn_3_param_mem_base_addr::CONN_3_PARAM_MEM_BASE_ADDR_SPEC>;
#[doc = "Connection Parameter memory base address for connection 3"]
pub mod conn_3_param_mem_base_addr;
#[doc = "CONN_4_PARAM_MEM_BASE_ADDR register accessor: an alias for `Reg<CONN_4_PARAM_MEM_BASE_ADDR_SPEC>`"]
pub type CONN_4_PARAM_MEM_BASE_ADDR =
    crate::Reg<conn_4_param_mem_base_addr::CONN_4_PARAM_MEM_BASE_ADDR_SPEC>;
#[doc = "Connection Parameter memory base address for connection 4"]
pub mod conn_4_param_mem_base_addr;
#[doc = "NI_TIMER register accessor: an alias for `Reg<NI_TIMER_SPEC>`"]
pub type NI_TIMER = crate::Reg<ni_timer::NI_TIMER_SPEC>;
#[doc = "Next Instant Timer"]
pub mod ni_timer;
#[doc = "US_OFFSET register accessor: an alias for `Reg<US_OFFSET_SPEC>`"]
pub type US_OFFSET = crate::Reg<us_offset::US_OFFSET_SPEC>;
#[doc = "Micro-second Offset"]
pub mod us_offset;
#[doc = "NEXT_CONN register accessor: an alias for `Reg<NEXT_CONN_SPEC>`"]
pub type NEXT_CONN = crate::Reg<next_conn::NEXT_CONN_SPEC>;
#[doc = "Next Connection"]
pub mod next_conn;
#[doc = "NI_ABORT register accessor: an alias for `Reg<NI_ABORT_SPEC>`"]
pub type NI_ABORT = crate::Reg<ni_abort::NI_ABORT_SPEC>;
#[doc = "Abort next scheduled connection"]
pub mod ni_abort;
#[doc = "CONN_NI_STATUS register accessor: an alias for `Reg<CONN_NI_STATUS_SPEC>`"]
pub type CONN_NI_STATUS = crate::Reg<conn_ni_status::CONN_NI_STATUS_SPEC>;
#[doc = "Connection NI Status"]
pub mod conn_ni_status;
#[doc = "NEXT_SUP_TO_STATUS register accessor: an alias for `Reg<NEXT_SUP_TO_STATUS_SPEC>`"]
pub type NEXT_SUP_TO_STATUS = crate::Reg<next_sup_to_status::NEXT_SUP_TO_STATUS_SPEC>;
#[doc = "Next Supervision timeout Status"]
pub mod next_sup_to_status;
#[doc = "MMMS_CONN_STATUS register accessor: an alias for `Reg<MMMS_CONN_STATUS_SPEC>`"]
pub type MMMS_CONN_STATUS = crate::Reg<mmms_conn_status::MMMS_CONN_STATUS_SPEC>;
#[doc = "Connection Status"]
pub mod mmms_conn_status;
#[doc = "BT_SLOT_CAPT_STATUS register accessor: an alias for `Reg<BT_SLOT_CAPT_STATUS_SPEC>`"]
pub type BT_SLOT_CAPT_STATUS = crate::Reg<bt_slot_capt_status::BT_SLOT_CAPT_STATUS_SPEC>;
#[doc = "BT Slot Captured Status"]
pub mod bt_slot_capt_status;
#[doc = "US_CAPT_STATUS register accessor: an alias for `Reg<US_CAPT_STATUS_SPEC>`"]
pub type US_CAPT_STATUS = crate::Reg<us_capt_status::US_CAPT_STATUS_SPEC>;
#[doc = "Micro-second Capture Status"]
pub mod us_capt_status;
#[doc = "US_OFFSET_STATUS register accessor: an alias for `Reg<US_OFFSET_STATUS_SPEC>`"]
pub type US_OFFSET_STATUS = crate::Reg<us_offset_status::US_OFFSET_STATUS_SPEC>;
#[doc = "Micro-second Offset Status"]
pub mod us_offset_status;
#[doc = "ACCU_WINDOW_WIDEN_STATUS register accessor: an alias for `Reg<ACCU_WINDOW_WIDEN_STATUS_SPEC>`"]
pub type ACCU_WINDOW_WIDEN_STATUS =
    crate::Reg<accu_window_widen_status::ACCU_WINDOW_WIDEN_STATUS_SPEC>;
#[doc = "Accumulated Window Widen Status"]
pub mod accu_window_widen_status;
#[doc = "EARLY_INTR_STATUS register accessor: an alias for `Reg<EARLY_INTR_STATUS_SPEC>`"]
pub type EARLY_INTR_STATUS = crate::Reg<early_intr_status::EARLY_INTR_STATUS_SPEC>;
#[doc = "Status when early interrupt is raised"]
pub mod early_intr_status;
#[doc = "MMMS_CONFIG register accessor: an alias for `Reg<MMMS_CONFIG_SPEC>`"]
pub type MMMS_CONFIG = crate::Reg<mmms_config::MMMS_CONFIG_SPEC>;
#[doc = "Multi-Master Multi-Slave Config"]
pub mod mmms_config;
#[doc = "US_COUNTER register accessor: an alias for `Reg<US_COUNTER_SPEC>`"]
pub type US_COUNTER = crate::Reg<us_counter::US_COUNTER_SPEC>;
#[doc = "Running US of the current BT Slot"]
pub mod us_counter;
#[doc = "US_CAPT_PREV register accessor: an alias for `Reg<US_CAPT_PREV_SPEC>`"]
pub type US_CAPT_PREV = crate::Reg<us_capt_prev::US_CAPT_PREV_SPEC>;
#[doc = "Previous captured US of the BT Slot"]
pub mod us_capt_prev;
#[doc = "EARLY_INTR_NI register accessor: an alias for `Reg<EARLY_INTR_NI_SPEC>`"]
pub type EARLY_INTR_NI = crate::Reg<early_intr_ni::EARLY_INTR_NI_SPEC>;
#[doc = "NI at early interrupt"]
pub mod early_intr_ni;
#[doc = "MMMS_MASTER_CREATE_BT_CAPT register accessor: an alias for `Reg<MMMS_MASTER_CREATE_BT_CAPT_SPEC>`"]
pub type MMMS_MASTER_CREATE_BT_CAPT =
    crate::Reg<mmms_master_create_bt_capt::MMMS_MASTER_CREATE_BT_CAPT_SPEC>;
#[doc = "BT slot capture for master connection creation"]
pub mod mmms_master_create_bt_capt;
#[doc = "MMMS_SLAVE_CREATE_BT_CAPT register accessor: an alias for `Reg<MMMS_SLAVE_CREATE_BT_CAPT_SPEC>`"]
pub type MMMS_SLAVE_CREATE_BT_CAPT =
    crate::Reg<mmms_slave_create_bt_capt::MMMS_SLAVE_CREATE_BT_CAPT_SPEC>;
#[doc = "BT slot capture for slave connection creation"]
pub mod mmms_slave_create_bt_capt;
#[doc = "MMMS_SLAVE_CREATE_US_CAPT register accessor: an alias for `Reg<MMMS_SLAVE_CREATE_US_CAPT_SPEC>`"]
pub type MMMS_SLAVE_CREATE_US_CAPT =
    crate::Reg<mmms_slave_create_us_capt::MMMS_SLAVE_CREATE_US_CAPT_SPEC>;
#[doc = "Micro second capture for slave connection creation"]
pub mod mmms_slave_create_us_capt;
#[doc = "MMMS_DATA_MEM_DESCRIPTOR register accessor: an alias for `Reg<MMMS_DATA_MEM_DESCRIPTOR_SPEC>`"]
pub type MMMS_DATA_MEM_DESCRIPTOR =
    crate::Reg<mmms_data_mem_descriptor::MMMS_DATA_MEM_DESCRIPTOR_SPEC>;
#[doc = "Data buffer descriptor 0 to 15"]
pub mod mmms_data_mem_descriptor;
#[doc = "CONN_1_DATA_LIST_SENT register accessor: an alias for `Reg<CONN_1_DATA_LIST_SENT_SPEC>`"]
pub type CONN_1_DATA_LIST_SENT = crate::Reg<conn_1_data_list_sent::CONN_1_DATA_LIST_SENT_SPEC>;
#[doc = "data list sent update and status for connection 1"]
pub mod conn_1_data_list_sent;
#[doc = "CONN_1_DATA_LIST_ACK register accessor: an alias for `Reg<CONN_1_DATA_LIST_ACK_SPEC>`"]
pub type CONN_1_DATA_LIST_ACK = crate::Reg<conn_1_data_list_ack::CONN_1_DATA_LIST_ACK_SPEC>;
#[doc = "data list ack update and status for connection 1"]
pub mod conn_1_data_list_ack;
#[doc = "CONN_1_CE_DATA_LIST_CFG register accessor: an alias for `Reg<CONN_1_CE_DATA_LIST_CFG_SPEC>`"]
pub type CONN_1_CE_DATA_LIST_CFG =
    crate::Reg<conn_1_ce_data_list_cfg::CONN_1_CE_DATA_LIST_CFG_SPEC>;
#[doc = "Connection specific pause resume for connection 1"]
pub mod conn_1_ce_data_list_cfg;
#[doc = "CONN_2_DATA_LIST_SENT register accessor: an alias for `Reg<CONN_2_DATA_LIST_SENT_SPEC>`"]
pub type CONN_2_DATA_LIST_SENT = crate::Reg<conn_2_data_list_sent::CONN_2_DATA_LIST_SENT_SPEC>;
#[doc = "data list sent update and status for connection 2"]
pub mod conn_2_data_list_sent;
#[doc = "CONN_2_DATA_LIST_ACK register accessor: an alias for `Reg<CONN_2_DATA_LIST_ACK_SPEC>`"]
pub type CONN_2_DATA_LIST_ACK = crate::Reg<conn_2_data_list_ack::CONN_2_DATA_LIST_ACK_SPEC>;
#[doc = "data list ack update and status for connection 2"]
pub mod conn_2_data_list_ack;
#[doc = "CONN_2_CE_DATA_LIST_CFG register accessor: an alias for `Reg<CONN_2_CE_DATA_LIST_CFG_SPEC>`"]
pub type CONN_2_CE_DATA_LIST_CFG =
    crate::Reg<conn_2_ce_data_list_cfg::CONN_2_CE_DATA_LIST_CFG_SPEC>;
#[doc = "Connection specific pause resume for connection 2"]
pub mod conn_2_ce_data_list_cfg;
#[doc = "CONN_3_DATA_LIST_SENT register accessor: an alias for `Reg<CONN_3_DATA_LIST_SENT_SPEC>`"]
pub type CONN_3_DATA_LIST_SENT = crate::Reg<conn_3_data_list_sent::CONN_3_DATA_LIST_SENT_SPEC>;
#[doc = "data list sent update and status for connection 3"]
pub mod conn_3_data_list_sent;
#[doc = "CONN_3_DATA_LIST_ACK register accessor: an alias for `Reg<CONN_3_DATA_LIST_ACK_SPEC>`"]
pub type CONN_3_DATA_LIST_ACK = crate::Reg<conn_3_data_list_ack::CONN_3_DATA_LIST_ACK_SPEC>;
#[doc = "data list ack update and status for connection 3"]
pub mod conn_3_data_list_ack;
#[doc = "CONN_3_CE_DATA_LIST_CFG register accessor: an alias for `Reg<CONN_3_CE_DATA_LIST_CFG_SPEC>`"]
pub type CONN_3_CE_DATA_LIST_CFG =
    crate::Reg<conn_3_ce_data_list_cfg::CONN_3_CE_DATA_LIST_CFG_SPEC>;
#[doc = "Connection specific pause resume for connection 3"]
pub mod conn_3_ce_data_list_cfg;
#[doc = "CONN_4_DATA_LIST_SENT register accessor: an alias for `Reg<CONN_4_DATA_LIST_SENT_SPEC>`"]
pub type CONN_4_DATA_LIST_SENT = crate::Reg<conn_4_data_list_sent::CONN_4_DATA_LIST_SENT_SPEC>;
#[doc = "data list sent update and status for connection 4"]
pub mod conn_4_data_list_sent;
#[doc = "CONN_4_DATA_LIST_ACK register accessor: an alias for `Reg<CONN_4_DATA_LIST_ACK_SPEC>`"]
pub type CONN_4_DATA_LIST_ACK = crate::Reg<conn_4_data_list_ack::CONN_4_DATA_LIST_ACK_SPEC>;
#[doc = "data list ack update and status for connection 4"]
pub mod conn_4_data_list_ack;
#[doc = "CONN_4_CE_DATA_LIST_CFG register accessor: an alias for `Reg<CONN_4_CE_DATA_LIST_CFG_SPEC>`"]
pub type CONN_4_CE_DATA_LIST_CFG =
    crate::Reg<conn_4_ce_data_list_cfg::CONN_4_CE_DATA_LIST_CFG_SPEC>;
#[doc = "Connection specific pause resume for connection 4"]
pub mod conn_4_ce_data_list_cfg;
#[doc = "MMMS_ADVCH_NI_ENABLE register accessor: an alias for `Reg<MMMS_ADVCH_NI_ENABLE_SPEC>`"]
pub type MMMS_ADVCH_NI_ENABLE = crate::Reg<mmms_advch_ni_enable::MMMS_ADVCH_NI_ENABLE_SPEC>;
#[doc = "Enable bits for ADV_NI, SCAN_NI and INIT_NI"]
pub mod mmms_advch_ni_enable;
#[doc = "MMMS_ADVCH_NI_VALID register accessor: an alias for `Reg<MMMS_ADVCH_NI_VALID_SPEC>`"]
pub type MMMS_ADVCH_NI_VALID = crate::Reg<mmms_advch_ni_valid::MMMS_ADVCH_NI_VALID_SPEC>;
#[doc = "Next instant valid for ADV, SCAN, INIT"]
pub mod mmms_advch_ni_valid;
#[doc = "MMMS_ADVCH_NI_ABORT register accessor: an alias for `Reg<MMMS_ADVCH_NI_ABORT_SPEC>`"]
pub type MMMS_ADVCH_NI_ABORT = crate::Reg<mmms_advch_ni_abort::MMMS_ADVCH_NI_ABORT_SPEC>;
#[doc = "Abort the next instant of ADV, SCAN, INIT"]
pub mod mmms_advch_ni_abort;
#[doc = "CONN_PARAM_NEXT_SUP_TO register accessor: an alias for `Reg<CONN_PARAM_NEXT_SUP_TO_SPEC>`"]
pub type CONN_PARAM_NEXT_SUP_TO = crate::Reg<conn_param_next_sup_to::CONN_PARAM_NEXT_SUP_TO_SPEC>;
#[doc = "Register to configure the supervision timeout for next scheduled connection"]
pub mod conn_param_next_sup_to;
#[doc = "CONN_PARAM_ACC_WIN_WIDEN register accessor: an alias for `Reg<CONN_PARAM_ACC_WIN_WIDEN_SPEC>`"]
pub type CONN_PARAM_ACC_WIN_WIDEN =
    crate::Reg<conn_param_acc_win_widen::CONN_PARAM_ACC_WIN_WIDEN_SPEC>;
#[doc = "Register to configure Accumulated window widening for next scheduled connection"]
pub mod conn_param_acc_win_widen;
#[doc = "HW_LOAD_OFFSET register accessor: an alias for `Reg<HW_LOAD_OFFSET_SPEC>`"]
pub type HW_LOAD_OFFSET = crate::Reg<hw_load_offset::HW_LOAD_OFFSET_SPEC>;
#[doc = "Register to configure offset from connection anchor point at which connection parameter memory should be read"]
pub mod hw_load_offset;
#[doc = "ADV_RAND register accessor: an alias for `Reg<ADV_RAND_SPEC>`"]
pub type ADV_RAND = crate::Reg<adv_rand::ADV_RAND_SPEC>;
#[doc = "Random number generated by Hardware for ADV NI calculation"]
pub mod adv_rand;
#[doc = "MMMS_RX_PKT_CNTR register accessor: an alias for `Reg<MMMS_RX_PKT_CNTR_SPEC>`"]
pub type MMMS_RX_PKT_CNTR = crate::Reg<mmms_rx_pkt_cntr::MMMS_RX_PKT_CNTR_SPEC>;
#[doc = "Packet Counter of packets in RX FIFO in MMMS mode"]
pub mod mmms_rx_pkt_cntr;
#[doc = "CONN_RX_PKT_CNTR register accessor: an alias for `Reg<CONN_RX_PKT_CNTR_SPEC>`"]
pub type CONN_RX_PKT_CNTR = crate::Reg<conn_rx_pkt_cntr::CONN_RX_PKT_CNTR_SPEC>;
#[doc = "Packet Counter for Individual connection index"]
pub mod conn_rx_pkt_cntr;
#[doc = "WHITELIST_BASE_ADDR register accessor: an alias for `Reg<WHITELIST_BASE_ADDR_SPEC>`"]
pub type WHITELIST_BASE_ADDR = crate::Reg<whitelist_base_addr::WHITELIST_BASE_ADDR_SPEC>;
#[doc = "Whitelist base address"]
pub mod whitelist_base_addr;
#[doc = "RSLV_LIST_PEER_IDNTT_BASE_ADDR register accessor: an alias for `Reg<RSLV_LIST_PEER_IDNTT_BASE_ADDR_SPEC>`"]
pub type RSLV_LIST_PEER_IDNTT_BASE_ADDR =
    crate::Reg<rslv_list_peer_idntt_base_addr::RSLV_LIST_PEER_IDNTT_BASE_ADDR_SPEC>;
#[doc = "Resolving list base address for storing Peer Identity address"]
pub mod rslv_list_peer_idntt_base_addr;
#[doc = "RSLV_LIST_PEER_RPA_BASE_ADDR register accessor: an alias for `Reg<RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>`"]
pub type RSLV_LIST_PEER_RPA_BASE_ADDR =
    crate::Reg<rslv_list_peer_rpa_base_addr::RSLV_LIST_PEER_RPA_BASE_ADDR_SPEC>;
#[doc = "Resolving list base address for storing resolved Peer RPA address"]
pub mod rslv_list_peer_rpa_base_addr;
#[doc = "RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR register accessor: an alias for `Reg<RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR_SPEC>`"]
pub type RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR =
    crate::Reg<rslv_list_rcvd_init_rpa_base_addr::RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR_SPEC>;
#[doc = "Resolving list base address for storing Resolved received INITA RPA"]
pub mod rslv_list_rcvd_init_rpa_base_addr;
#[doc = "RSLV_LIST_TX_INIT_RPA_BASE_ADDR register accessor: an alias for `Reg<RSLV_LIST_TX_INIT_RPA_BASE_ADDR_SPEC>`"]
pub type RSLV_LIST_TX_INIT_RPA_BASE_ADDR =
    crate::Reg<rslv_list_tx_init_rpa_base_addr::RSLV_LIST_TX_INIT_RPA_BASE_ADDR_SPEC>;
#[doc = "Resolving list base address for storing generated TX INITA RPA"]
pub mod rslv_list_tx_init_rpa_base_addr;
