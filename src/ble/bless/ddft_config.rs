#[doc = "Register `DDFT_CONFIG` reader"]
pub struct R(crate::R<DDFT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDFT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDFT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDFT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDFT_CONFIG` writer"]
pub struct W(crate::W<DDFT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDFT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDFT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDFT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDFT_ENABLE` reader - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
pub type DDFT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DDFT_ENABLE` writer - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
pub type DDFT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDFT_CONFIG_SPEC, bool, O>;
#[doc = "Field `BLERD_DDFT_EN` reader - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
pub type BLERD_DDFT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BLERD_DDFT_EN` writer - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
pub type BLERD_DDFT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDFT_CONFIG_SPEC, bool, O>;
#[doc = "Field `DDFT_MUX_CFG1` reader - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
pub type DDFT_MUX_CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDFT_MUX_CFG1` writer - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
pub type DDFT_MUX_CFG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDFT_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DDFT_MUX_CFG2` reader - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
pub type DDFT_MUX_CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDFT_MUX_CFG2` writer - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
pub type DDFT_MUX_CFG2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDFT_CONFIG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&self) -> DDFT_ENABLE_R {
        DDFT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&self) -> BLERD_DDFT_EN_R {
        BLERD_DDFT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&self) -> DDFT_MUX_CFG1_R {
        DDFT_MUX_CFG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&self) -> DDFT_MUX_CFG2_R {
        DDFT_MUX_CFG2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the DDFT output from BLESS 1: DDFT is enabled 0: DDFT is disabled"]
    #[inline(always)]
    pub fn ddft_enable(&mut self) -> DDFT_ENABLE_W<0> {
        DDFT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enables the DDFT inputs from CYBLERD55 chip 1: DDFT inputs are enabled 0: DDFT inputs are disabled"]
    #[inline(always)]
    pub fn blerd_ddft_en(&mut self) -> BLERD_DDFT_EN_W<1> {
        BLERD_DDFT_EN_W::new(self)
    }
    #[doc = "Bits 8:12 - dbg_mux_pin1 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[0\\]
5'h01 rcb_tx_fifo_empty 5'h02 hv_ldo_lv_detect_raw 5'h03 dbus_rx_en 5'h04 1'b0 5'h05 clk_switch_to_sysclk 5'h06 ll_clk_en_sync 5'h07 dsm_entry_stat 5'h08 proc_tx_en 5'h09 rssi_read_start 5'h0A tx_2mbps 5'h0B rcb_bus_busy 5'h0C hv_ldo_en_mt (act_stdbyb) 5'h0D ll_eco_clk_en 5'h0E blerd_reset_assert 5'h0F hv_ldo_byp_n 5'h10 hv_ldo_lv_detect_mt 5'h11 enable_ldo 5'h12 enable_ldo_dly 5'h13 bless_rcb_le_out 5'h14 bless_rcb_clk_out 5'h15 bless_dig_ldo_on_out 5'h16 bless_act_ldo_en_out 5'h17 bless_clk_en_out 5'h18 bless_buck_en_out 5'h19 bless_ret_switch_hv_out 5'h1A efuse_rw_out 5'h1B efuse_avdd_out 5'h1C efuse_config_efuse_mode 5'h1D bless_dbus_tx_en_pad 5'h1E bless_bpktctl_rd 5'h1F 1'b0"]
    #[inline(always)]
    pub fn ddft_mux_cfg1(&mut self) -> DDFT_MUX_CFG1_W<8> {
        DDFT_MUX_CFG1_W::new(self)
    }
    #[doc = "Bits 16:20 - dbg_mux_pin2 selection, combine with BLERD and BLESS 5'h00 blerd_ddft_out\\[1\\]
5'h01 rcb_rx_fifo_empty 5'h02 ll_decode_rxdata 5'h03 dbus_tx_en 5'h04 fw_clk_en 5'h05 interrupt_ll_n 5'h06 llh_st_sm 5'h07 llh_st_dsm 5'h08 proc_rx_en 5'h09 rssi_rx_done 5'h0A rx_2mbps 5'h0B rcb_ll_ctrl 5'h0C hv_ldo_byp_n 5'h0D reset_deassert 5'h0E rcb_intr 5'h0F rcb_ll_intr 5'h10 hv_ldo_en_mt (act_stdbyb) 5'h11 hv_ldo_lv_detect_raw 5'h12 bless_rcb_data_in 5'h13 bless_xtal_en_out 5'h14 bless_isolate_n_out 5'h15 bless_reset_n_out 5'h16 bless_ret_ldo_ol_hv_out 5'h17 bless_txd_rxd_out 5'h18 tx_rx_ctrl_sel 5'h19 bless_bpktctl_cy 5'h1A efuse_cs_out 5'h1B efuse_pgm_out 5'h1C efuse_sclk_out 5'h1D hv_ldo_lv_detect_mt 5'h1E enable_ldo 5'h1F enable_ldo_dly"]
    #[inline(always)]
    pub fn ddft_mux_cfg2(&mut self) -> DDFT_MUX_CFG2_W<16> {
        DDFT_MUX_CFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLESS DDFT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_config](index.html) module"]
pub struct DDFT_CONFIG_SPEC;
impl crate::RegisterSpec for DDFT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddft_config::R](R) reader structure"]
impl crate::Readable for DDFT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddft_config::W](W) writer structure"]
impl crate::Writable for DDFT_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDFT_CONFIG to value 0"]
impl crate::Resettable for DDFT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
