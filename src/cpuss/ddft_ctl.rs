#[doc = "Register `DDFT_CTL` reader"]
pub struct R(crate::R<DDFT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDFT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDFT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDFT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDFT_CTL` writer"]
pub struct W(crate::W<DDFT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDFT_CTL_SPEC>;
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
impl From<crate::W<DDFT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDFT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDFT_OUT0_SEL` reader - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
pub type DDFT_OUT0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDFT_OUT0_SEL` writer - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
pub type DDFT_OUT0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDFT_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DDFT_OUT1_SEL` reader - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
pub type DDFT_OUT1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDFT_OUT1_SEL` writer - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
pub type DDFT_OUT1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDFT_CTL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out0_sel(&self) -> DDFT_OUT0_SEL_R {
        DDFT_OUT0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out1_sel(&self) -> DDFT_OUT1_SEL_R {
        DDFT_OUT1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out0_sel(&mut self) -> DDFT_OUT0_SEL_W<0> {
        DDFT_OUT0_SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Select signal for CPUSS DDFT\\[0\\]
0: clk_r of the Main flash (which is clk_hf for SONOS Flash) 1: Flash data output bit '0' (r_q\\[0\\]) 2: Flash data output bit '32' (r_q\\[32\\]) 3: Flash data output bit '64' (r_q\\[64\\]) 4: Flash data output bit '127' (r_q\\[127\\]) 5: bist_fm_enabled 6: bist_fail 7: cm0_sleeping 8: cm0_sleepdeep 9: cm0_sleep_hold_ack_n 10: cm4_sleeping 11: cm4_sleepdeep 12: cm4_sleep_hold_ack_n 13: cm4_power 14: cm4_act_retain_n 15: cm4_act_isolate_n 16: cm4_enabled 17: cm4_reset_n 18: cm4_pwr_done 19: mmio_ram0_ctl1_power\\[0\\]
(Power control for SRAM0 macro0) 20: mmio_ram0_ctl1_retain_n\\[0\\]
(Retention control for SRAM0 macro0) 21: mmio_ram0_ctl1_isolate_n\\[0\\]
(Isolation control for SRAM0 macro0)"]
    #[inline(always)]
    pub fn ddft_out1_sel(&mut self) -> DDFT_OUT1_SEL_W<8> {
        DDFT_OUT1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDFT control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_ctl](index.html) module"]
pub struct DDFT_CTL_SPEC;
impl crate::RegisterSpec for DDFT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddft_ctl::R](R) reader structure"]
impl crate::Readable for DDFT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddft_ctl::W](W) writer structure"]
impl crate::Writable for DDFT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDFT_CTL to value 0"]
impl crate::Resettable for DDFT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
