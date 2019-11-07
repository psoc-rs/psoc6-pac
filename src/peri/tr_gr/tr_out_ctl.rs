#[doc = "Reader of register TR_OUT_CTL[%s]"]
pub type R = crate::R<u32, super::TR_OUT_CTL>;
#[doc = "Writer for register TR_OUT_CTL[%s]"]
pub type W = crate::W<u32, super::TR_OUT_CTL>;
#[doc = "Register TR_OUT_CTL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_OUT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR_SEL`"]
pub type TR_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TR_SEL`"]
pub struct TR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TR_INV`"]
pub type TR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_INV`"]
pub struct TR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TR_EDGE`"]
pub type TR_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_EDGE`"]
pub struct TR_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&self) -> TR_INV_R {
        TR_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) ouput trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) ouput trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn tr_sel(&mut self) -> TR_SEL_W {
        TR_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&mut self) -> TR_INV_W {
        TR_INV_W { w: self }
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) ouput trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) ouput trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&mut self) -> TR_EDGE_W {
        TR_EDGE_W { w: self }
    }
}
