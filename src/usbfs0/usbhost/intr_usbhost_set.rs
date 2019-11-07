#[doc = "Reader of register INTR_USBHOST_SET"]
pub type R = crate::R<u32, super::INTR_USBHOST_SET>;
#[doc = "Writer for register INTR_USBHOST_SET"]
pub type W = crate::W<u32, super::INTR_USBHOST_SET>;
#[doc = "Register INTR_USBHOST_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_USBHOST_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFIRQS`"]
pub type SOFIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIRQS`"]
pub struct SOFIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQS_W<'a> {
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
#[doc = "Reader of field `DIRQS`"]
pub type DIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRQS`"]
pub struct DIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQS_W<'a> {
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
#[doc = "Reader of field `CNNIRQS`"]
pub type CNNIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNNIRQS`"]
pub struct CNNIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQS_W<'a> {
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
#[doc = "Reader of field `CMPIRQS`"]
pub type CMPIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPIRQS`"]
pub struct CMPIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQS_W<'a> {
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
#[doc = "Reader of field `URIRQS`"]
pub type URIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URIRQS`"]
pub struct URIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQS_W<'a> {
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
#[doc = "Reader of field `RWKIRQS`"]
pub type RWKIRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKIRQS`"]
pub struct RWKIRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQS_W<'a> {
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
#[doc = "Reader of field `TCANS`"]
pub type TCANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCANS`"]
pub struct TCANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCANS_W<'a> {
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
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&self) -> SOFIRQS_R {
        SOFIRQS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&self) -> DIRQS_R {
        DIRQS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&self) -> CNNIRQS_R {
        CNNIRQS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&self) -> CMPIRQS_R {
        CMPIRQS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&self) -> URIRQS_R {
        URIRQS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&self) -> RWKIRQS_R {
        RWKIRQS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&self) -> TCANS_R {
        TCANS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&mut self) -> SOFIRQS_W {
        SOFIRQS_W { w: self }
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&mut self) -> DIRQS_W {
        DIRQS_W { w: self }
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&mut self) -> CNNIRQS_W {
        CNNIRQS_W { w: self }
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&mut self) -> CMPIRQS_W {
        CMPIRQS_W { w: self }
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&mut self) -> URIRQS_W {
        URIRQS_W { w: self }
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&mut self) -> RWKIRQS_W {
        RWKIRQS_W { w: self }
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&mut self) -> TCANS_W {
        TCANS_W { w: self }
    }
}
