#[doc = "Reader of register SW_AMUXBUF_SEL"]
pub type R = crate::R<u32, super::SW_AMUXBUF_SEL>;
#[doc = "Writer for register SW_AMUXBUF_SEL"]
pub type W = crate::W<u32, super::SW_AMUXBUF_SEL>;
#[doc = "Register SW_AMUXBUF_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_AMUXBUF_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_IRBY`"]
pub type SW_IRBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IRBY`"]
pub struct SW_IRBY_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRBY_W<'a> {
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
#[doc = "Reader of field `SW_IRLB`"]
pub type SW_IRLB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IRLB`"]
pub struct SW_IRLB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRLB_W<'a> {
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
#[doc = "Reader of field `SW_ICA`"]
pub type SW_ICA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_ICA`"]
pub struct SW_ICA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_ICA_W<'a> {
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
#[doc = "Reader of field `SW_ICB`"]
pub type SW_ICB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_ICB`"]
pub struct SW_ICB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_ICB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_IRLI`"]
pub type SW_IRLI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IRLI`"]
pub struct SW_IRLI_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRLI_W<'a> {
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
#[doc = "Reader of field `SW_IRH`"]
pub type SW_IRH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IRH`"]
pub struct SW_IRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRH_W<'a> {
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
#[doc = "Reader of field `SW_IRL`"]
pub type SW_IRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IRL`"]
pub struct SW_IRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&self) -> SW_IRBY_R {
        SW_IRBY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&self) -> SW_IRLB_R {
        SW_IRLB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&self) -> SW_ICA_R {
        SW_ICA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&self) -> SW_ICB_R {
        SW_ICB_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&self) -> SW_IRLI_R {
        SW_IRLI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&self) -> SW_IRH_R {
        SW_IRH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&self) -> SW_IRL_R {
        SW_IRL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&mut self) -> SW_IRBY_W {
        SW_IRBY_W { w: self }
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&mut self) -> SW_IRLB_W {
        SW_IRLB_W { w: self }
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&mut self) -> SW_ICA_W {
        SW_ICA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&mut self) -> SW_ICB_W {
        SW_ICB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&mut self) -> SW_IRLI_W {
        SW_IRLI_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&mut self) -> SW_IRH_W {
        SW_IRH_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&mut self) -> SW_IRL_W {
        SW_IRL_W { w: self }
    }
}
