#[doc = "Reader of register SW_REFGEN_SEL"]
pub type R = crate::R<u32, super::SW_REFGEN_SEL>;
#[doc = "Writer for register SW_REFGEN_SEL"]
pub type W = crate::W<u32, super::SW_REFGEN_SEL>;
#[doc = "Register SW_REFGEN_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_REFGEN_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_IAIB`"]
pub type SW_IAIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IAIB`"]
pub struct SW_IAIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IAIB_W<'a> {
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
#[doc = "Reader of field `SW_IBCB`"]
pub type SW_IBCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_IBCB`"]
pub struct SW_IBCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_IBCB_W<'a> {
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
#[doc = "Reader of field `SW_SGMB`"]
pub type SW_SGMB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SGMB`"]
pub struct SW_SGMB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGMB_W<'a> {
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
#[doc = "Reader of field `SW_SGRP`"]
pub type SW_SGRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SGRP`"]
pub struct SW_SGRP_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGRP_W<'a> {
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
#[doc = "Reader of field `SW_SGRE`"]
pub type SW_SGRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SGRE`"]
pub struct SW_SGRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGRE_W<'a> {
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
#[doc = "Reader of field `SW_SGR`"]
pub type SW_SGR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SGR`"]
pub struct SW_SGR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SGR_W<'a> {
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
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&self) -> SW_IAIB_R {
        SW_IAIB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&self) -> SW_IBCB_R {
        SW_IBCB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&self) -> SW_SGMB_R {
        SW_SGMB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&self) -> SW_SGRP_R {
        SW_SGRP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&self) -> SW_SGRE_R {
        SW_SGRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&self) -> SW_SGR_R {
        SW_SGR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&mut self) -> SW_IAIB_W {
        SW_IAIB_W { w: self }
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&mut self) -> SW_IBCB_W {
        SW_IBCB_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&mut self) -> SW_SGMB_W {
        SW_SGMB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&mut self) -> SW_SGRP_W {
        SW_SGRP_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&mut self) -> SW_SGRE_W {
        SW_SGRE_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&mut self) -> SW_SGR_W {
        SW_SGR_W { w: self }
    }
}
