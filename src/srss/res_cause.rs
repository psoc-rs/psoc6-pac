#[doc = "Register `RES_CAUSE` reader"]
pub struct R(crate::R<RES_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE` writer"]
pub struct W(crate::W<RES_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE_SPEC>;
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
impl From<crate::W<RES_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_WDT` reader - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type RESET_WDT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_WDT` writer - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type RESET_WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_ACT_FAULT` reader - Fault logging system requested a reset from its Active logic."]
pub type RESET_ACT_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_ACT_FAULT` writer - Fault logging system requested a reset from its Active logic."]
pub type RESET_ACT_FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_DPSLP_FAULT` reader - Fault logging system requested a reset from its DeepSleep logic."]
pub type RESET_DPSLP_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_DPSLP_FAULT` writer - Fault logging system requested a reset from its DeepSleep logic."]
pub type RESET_DPSLP_FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_CSV_WCO_LOSS` reader - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type RESET_CSV_WCO_LOSS_R = crate::BitReader<bool>;
#[doc = "Field `RESET_CSV_WCO_LOSS` writer - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type RESET_CSV_WCO_LOSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_SOFT` reader - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_SOFT` writer - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT0` reader - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type RESET_MCWDT0_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT0` writer - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type RESET_MCWDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT1` reader - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type RESET_MCWDT1_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT1` writer - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type RESET_MCWDT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT2` reader - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type RESET_MCWDT2_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT2` writer - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type RESET_MCWDT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT3` reader - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type RESET_MCWDT3_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT3` writer - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type RESET_MCWDT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&self) -> RESET_ACT_FAULT_R {
        RESET_ACT_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&self) -> RESET_DPSLP_FAULT_R {
        RESET_DPSLP_FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&self) -> RESET_CSV_WCO_LOSS_R {
        RESET_CSV_WCO_LOSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&self) -> RESET_MCWDT0_R {
        RESET_MCWDT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&self) -> RESET_MCWDT1_R {
        RESET_MCWDT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&self) -> RESET_MCWDT2_R {
        RESET_MCWDT2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&self) -> RESET_MCWDT3_R {
        RESET_MCWDT3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W<0> {
        RESET_WDT_W::new(self)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&mut self) -> RESET_ACT_FAULT_W<1> {
        RESET_ACT_FAULT_W::new(self)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&mut self) -> RESET_DPSLP_FAULT_W<2> {
        RESET_DPSLP_FAULT_W::new(self)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&mut self) -> RESET_CSV_WCO_LOSS_W<3> {
        RESET_CSV_WCO_LOSS_W::new(self)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W<4> {
        RESET_SOFT_W::new(self)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&mut self) -> RESET_MCWDT0_W<5> {
        RESET_MCWDT0_W::new(self)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&mut self) -> RESET_MCWDT1_W<6> {
        RESET_MCWDT1_W::new(self)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&mut self) -> RESET_MCWDT2_W<7> {
        RESET_MCWDT2_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&mut self) -> RESET_MCWDT3_W<8> {
        RESET_MCWDT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause](index.html) module"]
pub struct RES_CAUSE_SPEC;
impl crate::RegisterSpec for RES_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause::R](R) reader structure"]
impl crate::Readable for RES_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause::W](W) writer structure"]
impl crate::Writable for RES_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0"]
impl crate::Resettable for RES_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
