#[doc = "Reader of register AMUX_SPLIT_CTL[%s]"]
pub type R = crate::R<u32, super::AMUX_SPLIT_CTL>;
#[doc = "Writer for register AMUX_SPLIT_CTL[%s]"]
pub type W = crate::W<u32, super::AMUX_SPLIT_CTL>;
#[doc = "Register AMUX_SPLIT_CTL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::AMUX_SPLIT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWITCH_AA_SL`"]
pub type SWITCH_AA_SL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_AA_SL`"]
pub struct SWITCH_AA_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_SL_W<'a> {
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
#[doc = "Reader of field `SWITCH_AA_SR`"]
pub type SWITCH_AA_SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_AA_SR`"]
pub struct SWITCH_AA_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_SR_W<'a> {
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
#[doc = "Reader of field `SWITCH_AA_S0`"]
pub type SWITCH_AA_S0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_AA_S0`"]
pub struct SWITCH_AA_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_S0_W<'a> {
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
#[doc = "Reader of field `SWITCH_BB_SL`"]
pub type SWITCH_BB_SL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_BB_SL`"]
pub struct SWITCH_BB_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_SL_W<'a> {
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
#[doc = "Reader of field `SWITCH_BB_SR`"]
pub type SWITCH_BB_SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_BB_SR`"]
pub struct SWITCH_BB_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_SR_W<'a> {
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
#[doc = "Reader of field `SWITCH_BB_S0`"]
pub type SWITCH_BB_S0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITCH_BB_S0`"]
pub struct SWITCH_BB_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_S0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&self) -> SWITCH_AA_SL_R {
        SWITCH_AA_SL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&self) -> SWITCH_AA_SR_R {
        SWITCH_AA_SR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&self) -> SWITCH_AA_S0_R {
        SWITCH_AA_S0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&self) -> SWITCH_BB_SL_R {
        SWITCH_BB_SL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&self) -> SWITCH_BB_SR_R {
        SWITCH_BB_SR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&self) -> SWITCH_BB_S0_R {
        SWITCH_BB_S0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&mut self) -> SWITCH_AA_SL_W {
        SWITCH_AA_SL_W { w: self }
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&mut self) -> SWITCH_AA_SR_W {
        SWITCH_AA_SR_W { w: self }
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&mut self) -> SWITCH_AA_S0_W {
        SWITCH_AA_S0_W { w: self }
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&mut self) -> SWITCH_BB_SL_W {
        SWITCH_BB_SL_W { w: self }
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&mut self) -> SWITCH_BB_SR_W {
        SWITCH_BB_SR_W { w: self }
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&mut self) -> SWITCH_BB_S0_W {
        SWITCH_BB_S0_W { w: self }
    }
}
