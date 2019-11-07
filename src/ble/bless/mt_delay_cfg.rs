#[doc = "Reader of register MT_DELAY_CFG"]
pub type R = crate::R<u32, super::MT_DELAY_CFG>;
#[doc = "Writer for register MT_DELAY_CFG"]
pub type W = crate::W<u32, super::MT_DELAY_CFG>;
#[doc = "Register MT_DELAY_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MT_DELAY_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HVLDO_STARTUP_DELAY`"]
pub type HVLDO_STARTUP_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVLDO_STARTUP_DELAY`"]
pub struct HVLDO_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ISOLATE_DEASSERT_DELAY`"]
pub type ISOLATE_DEASSERT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISOLATE_DEASSERT_DELAY`"]
pub struct ISOLATE_DEASSERT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOLATE_DEASSERT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ACT_TO_SWITCH_DELAY`"]
pub type ACT_TO_SWITCH_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_TO_SWITCH_DELAY`"]
pub struct ACT_TO_SWITCH_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_TO_SWITCH_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HVLDO_DISABLE_DELAY`"]
pub type HVLDO_DISABLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVLDO_DISABLE_DELAY`"]
pub struct HVLDO_DISABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_DISABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
    #[inline(always)]
    pub fn hvldo_startup_delay(&self) -> HVLDO_STARTUP_DELAY_R {
        HVLDO_STARTUP_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
    #[inline(always)]
    pub fn isolate_deassert_delay(&self) -> ISOLATE_DEASSERT_DELAY_R {
        ISOLATE_DEASSERT_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
    #[inline(always)]
    pub fn act_to_switch_delay(&self) -> ACT_TO_SWITCH_DELAY_R {
        ACT_TO_SWITCH_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register specifies the time from disabling XTAL to switching of the HVLDO."]
    #[inline(always)]
    pub fn hvldo_disable_delay(&self) -> HVLDO_DISABLE_DELAY_R {
        HVLDO_DISABLE_DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
    #[inline(always)]
    pub fn hvldo_startup_delay(&mut self) -> HVLDO_STARTUP_DELAY_W {
        HVLDO_STARTUP_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
    #[inline(always)]
    pub fn isolate_deassert_delay(&mut self) -> ISOLATE_DEASSERT_DELAY_W {
        ISOLATE_DEASSERT_DELAY_W { w: self }
    }
    #[doc = "Bits 16:23 - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
    #[inline(always)]
    pub fn act_to_switch_delay(&mut self) -> ACT_TO_SWITCH_DELAY_W {
        ACT_TO_SWITCH_DELAY_W { w: self }
    }
    #[doc = "Bits 24:31 - This register specifies the time from disabling XTAL to switching of the HVLDO."]
    #[inline(always)]
    pub fn hvldo_disable_delay(&mut self) -> HVLDO_DISABLE_DELAY_W {
        HVLDO_DISABLE_DELAY_W { w: self }
    }
}
