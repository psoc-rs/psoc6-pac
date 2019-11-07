#[doc = "Reader of register RES_CAUSE"]
pub type R = crate::R<u32, super::RES_CAUSE>;
#[doc = "Writer for register RES_CAUSE"]
pub type W = crate::W<u32, super::RES_CAUSE>;
#[doc = "Register RES_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::RES_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESET_WDT`"]
pub type RESET_WDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_WDT`"]
pub struct RESET_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_WDT_W<'a> {
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
#[doc = "Reader of field `RESET_ACT_FAULT`"]
pub type RESET_ACT_FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_ACT_FAULT`"]
pub struct RESET_ACT_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_ACT_FAULT_W<'a> {
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
#[doc = "Reader of field `RESET_DPSLP_FAULT`"]
pub type RESET_DPSLP_FAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_DPSLP_FAULT`"]
pub struct RESET_DPSLP_FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_DPSLP_FAULT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESET_CSV_WCO_LOSS`"]
pub type RESET_CSV_WCO_LOSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_CSV_WCO_LOSS`"]
pub struct RESET_CSV_WCO_LOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CSV_WCO_LOSS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RESET_SOFT`"]
pub type RESET_SOFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_SOFT`"]
pub struct RESET_SOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_SOFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESET_MCWDT0`"]
pub type RESET_MCWDT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_MCWDT0`"]
pub struct RESET_MCWDT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_MCWDT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESET_MCWDT1`"]
pub type RESET_MCWDT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_MCWDT1`"]
pub struct RESET_MCWDT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_MCWDT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESET_MCWDT2`"]
pub type RESET_MCWDT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_MCWDT2`"]
pub struct RESET_MCWDT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_MCWDT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESET_MCWDT3`"]
pub type RESET_MCWDT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_MCWDT3`"]
pub struct RESET_MCWDT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_MCWDT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&self) -> RESET_ACT_FAULT_R {
        RESET_ACT_FAULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&self) -> RESET_DPSLP_FAULT_R {
        RESET_DPSLP_FAULT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&self) -> RESET_CSV_WCO_LOSS_R {
        RESET_CSV_WCO_LOSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&self) -> RESET_MCWDT0_R {
        RESET_MCWDT0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&self) -> RESET_MCWDT1_R {
        RESET_MCWDT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&self) -> RESET_MCWDT2_R {
        RESET_MCWDT2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&self) -> RESET_MCWDT3_R {
        RESET_MCWDT3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W {
        RESET_WDT_W { w: self }
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&mut self) -> RESET_ACT_FAULT_W {
        RESET_ACT_FAULT_W { w: self }
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&mut self) -> RESET_DPSLP_FAULT_W {
        RESET_DPSLP_FAULT_W { w: self }
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&mut self) -> RESET_CSV_WCO_LOSS_W {
        RESET_CSV_WCO_LOSS_W { w: self }
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W {
        RESET_SOFT_W { w: self }
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&mut self) -> RESET_MCWDT0_W {
        RESET_MCWDT0_W { w: self }
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&mut self) -> RESET_MCWDT1_W {
        RESET_MCWDT1_W { w: self }
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&mut self) -> RESET_MCWDT2_W {
        RESET_MCWDT2_W { w: self }
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&mut self) -> RESET_MCWDT3_W {
        RESET_MCWDT3_W { w: self }
    }
}
