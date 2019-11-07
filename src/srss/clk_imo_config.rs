#[doc = "Reader of register CLK_IMO_CONFIG"]
pub type R = crate::R<u32, super::CLK_IMO_CONFIG>;
#[doc = "Writer for register CLK_IMO_CONFIG"]
pub type W = crate::W<u32, super::CLK_IMO_CONFIG>;
#[doc = "Register CLK_IMO_CONFIG `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CLK_IMO_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during DEEPSLEEP, HIBERNATE, and XRES."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during DEEPSLEEP, HIBERNATE, and XRES."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
