#[doc = "Reader of register FM_ADDR"]
pub type R = crate::R<u32, super::FM_ADDR>;
#[doc = "Writer for register FM_ADDR"]
pub type W = crate::W<u32, super::FM_ADDR>;
#[doc = "Register FM_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BA`"]
pub type BA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BA`"]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AXA`"]
pub type AXA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXA`"]
pub struct AXA_W<'a> {
    w: &'a mut W,
}
impl<'a> AXA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auxiliairy address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&self) -> AXA_R {
        AXA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
    #[doc = "Bit 24 - Auxiliairy address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&mut self) -> AXA_W {
        AXA_W { w: self }
    }
}
