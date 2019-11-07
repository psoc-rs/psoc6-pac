#[doc = "Reader of register EP_ACTIVE"]
pub type R = crate::R<u32, super::EP_ACTIVE>;
#[doc = "Writer for register EP_ACTIVE"]
pub type W = crate::W<u32, super::EP_ACTIVE>;
#[doc = "Register EP_ACTIVE `reset()`'s with value 0"]
impl crate::ResetValue for super::EP_ACTIVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1_ACT`"]
pub type EP1_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_ACT`"]
pub struct EP1_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_ACT_W<'a> {
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
#[doc = "Reader of field `EP2_ACT`"]
pub type EP2_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_ACT`"]
pub struct EP2_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_ACT_W<'a> {
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
#[doc = "Reader of field `EP3_ACT`"]
pub type EP3_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_ACT`"]
pub struct EP3_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_ACT_W<'a> {
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
#[doc = "Reader of field `EP4_ACT`"]
pub type EP4_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_ACT`"]
pub struct EP4_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_ACT_W<'a> {
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
#[doc = "Reader of field `EP5_ACT`"]
pub type EP5_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_ACT`"]
pub struct EP5_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_ACT_W<'a> {
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
#[doc = "Reader of field `EP6_ACT`"]
pub type EP6_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_ACT`"]
pub struct EP6_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_ACT_W<'a> {
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
#[doc = "Reader of field `EP7_ACT`"]
pub type EP7_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_ACT`"]
pub struct EP7_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_ACT_W<'a> {
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
#[doc = "Reader of field `EP8_ACT`"]
pub type EP8_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_ACT`"]
pub struct EP8_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_ACT_W<'a> {
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
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&self) -> EP1_ACT_R {
        EP1_ACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&self) -> EP2_ACT_R {
        EP2_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&self) -> EP3_ACT_R {
        EP3_ACT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&self) -> EP4_ACT_R {
        EP4_ACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&self) -> EP5_ACT_R {
        EP5_ACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&self) -> EP6_ACT_R {
        EP6_ACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&self) -> EP7_ACT_R {
        EP7_ACT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&self) -> EP8_ACT_R {
        EP8_ACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&mut self) -> EP1_ACT_W {
        EP1_ACT_W { w: self }
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&mut self) -> EP2_ACT_W {
        EP2_ACT_W { w: self }
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&mut self) -> EP3_ACT_W {
        EP3_ACT_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&mut self) -> EP4_ACT_W {
        EP4_ACT_W { w: self }
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&mut self) -> EP5_ACT_W {
        EP5_ACT_W { w: self }
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&mut self) -> EP6_ACT_W {
        EP6_ACT_W { w: self }
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&mut self) -> EP7_ACT_W {
        EP7_ACT_W { w: self }
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&mut self) -> EP8_ACT_W {
        EP8_ACT_W { w: self }
    }
}
