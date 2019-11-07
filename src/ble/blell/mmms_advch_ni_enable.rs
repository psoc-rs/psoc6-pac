#[doc = "Reader of register MMMS_ADVCH_NI_ENABLE"]
pub type R = crate::R<u32, super::MMMS_ADVCH_NI_ENABLE>;
#[doc = "Writer for register MMMS_ADVCH_NI_ENABLE"]
pub type W = crate::W<u32, super::MMMS_ADVCH_NI_ENABLE>;
#[doc = "Register MMMS_ADVCH_NI_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::MMMS_ADVCH_NI_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_NI_ENABLE`"]
pub type ADV_NI_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_NI_ENABLE`"]
pub struct ADV_NI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_NI_ENABLE_W<'a> {
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
#[doc = "Reader of field `SCAN_NI_ENABLE`"]
pub type SCAN_NI_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_NI_ENABLE`"]
pub struct SCAN_NI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_NI_ENABLE_W<'a> {
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
#[doc = "Reader of field `INIT_NI_ENABLE`"]
pub type INIT_NI_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_NI_ENABLE`"]
pub struct INIT_NI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_NI_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn adv_ni_enable(&self) -> ADV_NI_ENABLE_R {
        ADV_NI_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn scan_ni_enable(&self) -> SCAN_NI_ENABLE_R {
        SCAN_NI_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn init_ni_enable(&self) -> INIT_NI_ENABLE_R {
        INIT_NI_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn adv_ni_enable(&mut self) -> ADV_NI_ENABLE_W {
        ADV_NI_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn scan_ni_enable(&mut self) -> SCAN_NI_ENABLE_W {
        SCAN_NI_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn init_ni_enable(&mut self) -> INIT_NI_ENABLE_W {
        INIT_NI_ENABLE_W { w: self }
    }
}
