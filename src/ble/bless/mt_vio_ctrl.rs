#[doc = "Reader of register MT_VIO_CTRL"]
pub type R = crate::R<u32, super::MT_VIO_CTRL>;
#[doc = "Writer for register MT_VIO_CTRL"]
pub type W = crate::W<u32, super::MT_VIO_CTRL>;
#[doc = "Register MT_VIO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MT_VIO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRSS_SWITCH_EN`"]
pub type SRSS_SWITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRSS_SWITCH_EN`"]
pub struct SRSS_SWITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_SWITCH_EN_W<'a> {
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
#[doc = "Reader of field `SRSS_SWITCH_EN_DLY`"]
pub type SRSS_SWITCH_EN_DLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRSS_SWITCH_EN_DLY`"]
pub struct SRSS_SWITCH_EN_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSS_SWITCH_EN_DLY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en(&self) -> SRSS_SWITCH_EN_R {
        SRSS_SWITCH_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en_dly(&self) -> SRSS_SWITCH_EN_DLY_R {
        SRSS_SWITCH_EN_DLY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en(&mut self) -> SRSS_SWITCH_EN_W {
        SRSS_SWITCH_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en_dly(&mut self) -> SRSS_SWITCH_EN_DLY_W {
        SRSS_SWITCH_EN_DLY_W { w: self }
    }
}
