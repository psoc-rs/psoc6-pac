#[doc = "Reader of register FLASH_PWR_CTL"]
pub type R = crate::R<u32, super::FLASH_PWR_CTL>;
#[doc = "Writer for register FLASH_PWR_CTL"]
pub type W = crate::W<u32, super::FLASH_PWR_CTL>;
#[doc = "Register FLASH_PWR_CTL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::FLASH_PWR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `ENABLE_HV`"]
pub type ENABLE_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_HV`"]
pub struct ENABLE_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_HV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&self) -> ENABLE_HV_R {
        ENABLE_HV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&mut self) -> ENABLE_HV_W {
        ENABLE_HV_W { w: self }
    }
}
