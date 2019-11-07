#[doc = "Reader of register SW_BYP_SEL"]
pub type R = crate::R<u32, super::SW_BYP_SEL>;
#[doc = "Writer for register SW_BYP_SEL"]
pub type W = crate::W<u32, super::SW_BYP_SEL>;
#[doc = "Register SW_BYP_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_BYP_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_BYA`"]
pub type SW_BYA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_BYA`"]
pub struct SW_BYA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_BYA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SW_BYB`"]
pub type SW_BYB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_BYB`"]
pub struct SW_BYB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_BYB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_CBCC`"]
pub type SW_CBCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_CBCC`"]
pub struct SW_CBCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CBCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&self) -> SW_BYA_R {
        SW_BYA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&self) -> SW_BYB_R {
        SW_BYB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&self) -> SW_CBCC_R {
        SW_CBCC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&mut self) -> SW_BYA_W {
        SW_BYA_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&mut self) -> SW_BYB_W {
        SW_BYB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&mut self) -> SW_CBCC_W {
        SW_CBCC_W { w: self }
    }
}
