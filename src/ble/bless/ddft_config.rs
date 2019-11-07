#[doc = "Reader of register DDFT_CONFIG"]
pub type R = crate::R<u32, super::DDFT_CONFIG>;
#[doc = "Writer for register DDFT_CONFIG"]
pub type W = crate::W<u32, super::DDFT_CONFIG>;
#[doc = "Register DDFT_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DDFT_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDFT_ENABLE`"]
pub type DDFT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDFT_ENABLE`"]
pub struct DDFT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BLERD_DDFT_EN`"]
pub type BLERD_DDFT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLERD_DDFT_EN`"]
pub struct BLERD_DDFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_DDFT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DDFT_MUX_CFG1`"]
pub type DDFT_MUX_CFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_MUX_CFG1`"]
pub struct DDFT_MUX_CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_MUX_CFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DDFT_MUX_CFG2`"]
pub type DDFT_MUX_CFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_MUX_CFG2`"]
pub struct DDFT_MUX_CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_MUX_CFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&self) -> DDFT_ENABLE_R {
        DDFT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&self) -> BLERD_DDFT_EN_R {
        BLERD_DDFT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\] 5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&self) -> DDFT_MUX_CFG1_R {
        DDFT_MUX_CFG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\] 5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&self) -> DDFT_MUX_CFG2_R {
        DDFT_MUX_CFG2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&mut self) -> DDFT_ENABLE_W {
        DDFT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&mut self) -> BLERD_DDFT_EN_W {
        BLERD_DDFT_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\] 5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&mut self) -> DDFT_MUX_CFG1_W {
        DDFT_MUX_CFG1_W { w: self }
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\] 5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&mut self) -> DDFT_MUX_CFG2_W {
        DDFT_MUX_CFG2_W { w: self }
    }
}
