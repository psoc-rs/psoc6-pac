#[doc = "Reader of register INTR_HOST_EP_MASK"]
pub type R = crate::R<u32, super::INTR_HOST_EP_MASK>;
#[doc = "Writer for register INTR_HOST_EP_MASK"]
pub type W = crate::W<u32, super::INTR_HOST_EP_MASK>;
#[doc = "Register INTR_HOST_EP_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_HOST_EP_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1DRQM`"]
pub type EP1DRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1DRQM`"]
pub struct EP1DRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1DRQM_W<'a> {
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
#[doc = "Reader of field `EP1SPKM`"]
pub type EP1SPKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1SPKM`"]
pub struct EP1SPKM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1SPKM_W<'a> {
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
#[doc = "Reader of field `EP2DRQM`"]
pub type EP2DRQM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2DRQM`"]
pub struct EP2DRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2DRQM_W<'a> {
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
#[doc = "Reader of field `EP2SPKM`"]
pub type EP2SPKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2SPKM`"]
pub struct EP2SPKM_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2SPKM_W<'a> {
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
impl R {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&self) -> EP1DRQM_R {
        EP1DRQM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&self) -> EP1SPKM_R {
        EP1SPKM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&self) -> EP2DRQM_R {
        EP2DRQM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&self) -> EP2SPKM_R {
        EP2SPKM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&mut self) -> EP1DRQM_W {
        EP1DRQM_W { w: self }
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&mut self) -> EP1SPKM_W {
        EP1SPKM_W { w: self }
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&mut self) -> EP2DRQM_W {
        EP2DRQM_W { w: self }
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&mut self) -> EP2SPKM_W {
        EP2SPKM_W { w: self }
    }
}
