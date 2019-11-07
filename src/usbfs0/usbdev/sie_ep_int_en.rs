#[doc = "Reader of register SIE_EP_INT_EN"]
pub type R = crate::R<u32, super::SIE_EP_INT_EN>;
#[doc = "Writer for register SIE_EP_INT_EN"]
pub type W = crate::W<u32, super::SIE_EP_INT_EN>;
#[doc = "Register SIE_EP_INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::SIE_EP_INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1_INTR_EN`"]
pub type EP1_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_INTR_EN`"]
pub struct EP1_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP2_INTR_EN`"]
pub type EP2_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_INTR_EN`"]
pub struct EP2_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP3_INTR_EN`"]
pub type EP3_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_INTR_EN`"]
pub struct EP3_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP4_INTR_EN`"]
pub type EP4_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_INTR_EN`"]
pub struct EP4_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP5_INTR_EN`"]
pub type EP5_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_INTR_EN`"]
pub struct EP5_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP6_INTR_EN`"]
pub type EP6_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_INTR_EN`"]
pub struct EP6_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP7_INTR_EN`"]
pub type EP7_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_INTR_EN`"]
pub struct EP7_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_INTR_EN_W<'a> {
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
#[doc = "Reader of field `EP8_INTR_EN`"]
pub type EP8_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_INTR_EN`"]
pub struct EP8_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_INTR_EN_W<'a> {
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
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&self) -> EP1_INTR_EN_R {
        EP1_INTR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&self) -> EP2_INTR_EN_R {
        EP2_INTR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&self) -> EP3_INTR_EN_R {
        EP3_INTR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&self) -> EP4_INTR_EN_R {
        EP4_INTR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&self) -> EP5_INTR_EN_R {
        EP5_INTR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&self) -> EP6_INTR_EN_R {
        EP6_INTR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&self) -> EP7_INTR_EN_R {
        EP7_INTR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&self) -> EP8_INTR_EN_R {
        EP8_INTR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&mut self) -> EP1_INTR_EN_W {
        EP1_INTR_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&mut self) -> EP2_INTR_EN_W {
        EP2_INTR_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&mut self) -> EP3_INTR_EN_W {
        EP3_INTR_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&mut self) -> EP4_INTR_EN_W {
        EP4_INTR_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&mut self) -> EP5_INTR_EN_W {
        EP5_INTR_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&mut self) -> EP6_INTR_EN_W {
        EP6_INTR_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&mut self) -> EP7_INTR_EN_W {
        EP7_INTR_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&mut self) -> EP8_INTR_EN_W {
        EP8_INTR_EN_W { w: self }
    }
}
