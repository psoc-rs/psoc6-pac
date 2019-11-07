#[doc = "Reader of register TIMER_CTL"]
pub type R = crate::R<u32, super::TIMER_CTL>;
#[doc = "Writer for register TIMER_CTL"]
pub type W = crate::W<u32, super::TIMER_CTL>;
#[doc = "Register TIMER_CTL `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::TIMER_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SCALE`"]
pub type SCALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCALE`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
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
#[doc = "Reader of field `PUMP_CLOCK_SEL`"]
pub type PUMP_CLOCK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUMP_CLOCK_SEL`"]
pub struct PUMP_CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_CLOCK_SEL_W<'a> {
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
#[doc = "Reader of field `PRE_PROG`"]
pub type PRE_PROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRE_PROG`"]
pub struct PRE_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PROG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PRE_PROG_CSL`"]
pub type PRE_PROG_CSL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRE_PROG_CSL`"]
pub struct PRE_PROG_CSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PROG_CSL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PUMP_EN`"]
pub type PUMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUMP_EN`"]
pub struct PUMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ACLK_EN`"]
pub type ACLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_EN`"]
pub struct ACLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TIMER_EN`"]
pub type TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_EN`"]
pub struct TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&self) -> PUMP_CLOCK_SEL_R {
        PUMP_CLOCK_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&self) -> PRE_PROG_R {
        PRE_PROG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&self) -> PRE_PROG_CSL_R {
        PRE_PROG_CSL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is reuired to prevent non intential clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&self) -> ACLK_EN_R {
        ACLK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&mut self) -> PUMP_CLOCK_SEL_W {
        PUMP_CLOCK_SEL_W { w: self }
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&mut self) -> PRE_PROG_W {
        PRE_PROG_W { w: self }
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&mut self) -> PRE_PROG_CSL_W {
        PRE_PROG_CSL_W { w: self }
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is reuired to prevent non intential clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&mut self) -> PUMP_EN_W {
        PUMP_EN_W { w: self }
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&mut self) -> ACLK_EN_W {
        ACLK_EN_W { w: self }
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W {
        TIMER_EN_W { w: self }
    }
}
