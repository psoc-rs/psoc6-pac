#[doc = "Reader of register DDFT_CTL"]
pub type R = crate::R<u32, super::DDFT_CTL>;
#[doc = "Writer for register DDFT_CTL"]
pub type W = crate::W<u32, super::DDFT_CTL>;
#[doc = "Register DDFT_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DDFT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDFT_OUT0_SEL`"]
pub type DDFT_OUT0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_OUT0_SEL`"]
pub struct DDFT_OUT0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_OUT0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DDFT_OUT1_SEL`"]
pub type DDFT_OUT1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_OUT1_SEL`"]
pub struct DDFT_OUT1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_OUT1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out0_sel(&self) -> DDFT_OUT0_SEL_R {
        DDFT_OUT0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out1_sel(&self) -> DDFT_OUT1_SEL_R {
        DDFT_OUT1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out0_sel(&mut self) -> DDFT_OUT0_SEL_W {
        DDFT_OUT0_SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Select signal for CPUSS DDFT\\[0\\] 0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\] (Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\] (Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\] (Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out1_sel(&mut self) -> DDFT_OUT1_SEL_W {
        DDFT_OUT1_SEL_W { w: self }
    }
}
