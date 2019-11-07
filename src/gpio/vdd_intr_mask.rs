#[doc = "Reader of register VDD_INTR_MASK"]
pub type R = crate::R<u32, super::VDD_INTR_MASK>;
#[doc = "Writer for register VDD_INTR_MASK"]
pub type W = crate::W<u32, super::VDD_INTR_MASK>;
#[doc = "Register VDD_INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::VDD_INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDDIO_ACTIVE`"]
pub type VDDIO_ACTIVE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VDDIO_ACTIVE`"]
pub struct VDDIO_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDIO_ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `VDDA_ACTIVE`"]
pub type VDDA_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDA_ACTIVE`"]
pub struct VDDA_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `VDDD_ACTIVE`"]
pub type VDDD_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDD_ACTIVE`"]
pub struct VDDD_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDD_ACTIVE_W<'a> {
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
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub fn vddio_active(&mut self) -> VDDIO_ACTIVE_W {
        VDDIO_ACTIVE_W { w: self }
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&mut self) -> VDDA_ACTIVE_W {
        VDDA_ACTIVE_W { w: self }
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&mut self) -> VDDD_ACTIVE_W {
        VDDD_ACTIVE_W { w: self }
    }
}
