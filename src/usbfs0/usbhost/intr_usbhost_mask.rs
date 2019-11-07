#[doc = "Reader of register INTR_USBHOST_MASK"]
pub type R = crate::R<u32, super::INTR_USBHOST_MASK>;
#[doc = "Writer for register INTR_USBHOST_MASK"]
pub type W = crate::W<u32, super::INTR_USBHOST_MASK>;
#[doc = "Register INTR_USBHOST_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_USBHOST_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFIRQM`"]
pub type SOFIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIRQM`"]
pub struct SOFIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQM_W<'a> {
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
#[doc = "Reader of field `DIRQM`"]
pub type DIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRQM`"]
pub struct DIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQM_W<'a> {
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
#[doc = "Reader of field `CNNIRQM`"]
pub type CNNIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNNIRQM`"]
pub struct CNNIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQM_W<'a> {
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
#[doc = "Reader of field `CMPIRQM`"]
pub type CMPIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPIRQM`"]
pub struct CMPIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQM_W<'a> {
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
#[doc = "Reader of field `URIRQM`"]
pub type URIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URIRQM`"]
pub struct URIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQM_W<'a> {
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
#[doc = "Reader of field `RWKIRQM`"]
pub type RWKIRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKIRQM`"]
pub struct RWKIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQM_W<'a> {
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
#[doc = "Reader of field `RSVD_6`"]
pub type RSVD_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_6`"]
pub struct RSVD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_6_W<'a> {
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
#[doc = "Reader of field `TCANM`"]
pub type TCANM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCANM`"]
pub struct TCANM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCANM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&self) -> SOFIRQM_R {
        SOFIRQM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&self) -> DIRQM_R {
        DIRQM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&self) -> CNNIRQM_R {
        CNNIRQM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&self) -> CMPIRQM_R {
        CMPIRQM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&self) -> URIRQM_R {
        URIRQM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&self) -> RWKIRQM_R {
        RWKIRQM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&self) -> TCANM_R {
        TCANM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&mut self) -> SOFIRQM_W {
        SOFIRQM_W { w: self }
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&mut self) -> DIRQM_W {
        DIRQM_W { w: self }
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&mut self) -> CNNIRQM_W {
        CNNIRQM_W { w: self }
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&mut self) -> CMPIRQM_W {
        CMPIRQM_W { w: self }
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&mut self) -> URIRQM_W {
        URIRQM_W { w: self }
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&mut self) -> RWKIRQM_W {
        RWKIRQM_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&mut self) -> TCANM_W {
        TCANM_W { w: self }
    }
}
