#[doc = "Reader of register MT_DELAY_CFG3"]
pub type R = crate::R<u32, super::MT_DELAY_CFG3>;
#[doc = "Writer for register MT_DELAY_CFG3"]
pub type W = crate::W<u32, super::MT_DELAY_CFG3>;
#[doc = "Register MT_DELAY_CFG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MT_DELAY_CFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL_DISABLE_DELAY`"]
pub type XTAL_DISABLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL_DISABLE_DELAY`"]
pub struct XTAL_DISABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_DISABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DIG_LDO_DISABLE_DELAY`"]
pub type DIG_LDO_DISABLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_LDO_DISABLE_DELAY`"]
pub struct DIG_LDO_DISABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_LDO_DISABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VDDR_STABLE_DELAY`"]
pub type VDDR_STABLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_STABLE_DELAY`"]
pub struct VDDR_STABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_STABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
    #[inline(always)]
    pub fn xtal_disable_delay(&self) -> XTAL_DISABLE_DELAY_R {
        XTAL_DISABLE_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
    #[inline(always)]
    pub fn dig_ldo_disable_delay(&self) -> DIG_LDO_DISABLE_DELAY_R {
        DIG_LDO_DISABLE_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
    #[inline(always)]
    pub fn vddr_stable_delay(&self) -> VDDR_STABLE_DELAY_R {
        VDDR_STABLE_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
    #[inline(always)]
    pub fn xtal_disable_delay(&mut self) -> XTAL_DISABLE_DELAY_W {
        XTAL_DISABLE_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
    #[inline(always)]
    pub fn dig_ldo_disable_delay(&mut self) -> DIG_LDO_DISABLE_DELAY_W {
        DIG_LDO_DISABLE_DELAY_W { w: self }
    }
    #[doc = "Bits 16:23 - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
    #[inline(always)]
    pub fn vddr_stable_delay(&mut self) -> VDDR_STABLE_DELAY_W {
        VDDR_STABLE_DELAY_W { w: self }
    }
}
