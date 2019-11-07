#[doc = "Reader of register CM0_CTL"]
pub type R = crate::R<u32, super::CM0_CTL>;
#[doc = "Writer for register CM0_CTL"]
pub type W = crate::W<u32, super::CM0_CTL>;
#[doc = "Register CM0_CTL `reset()`'s with value 0xfa05_0002"]
impl crate::ResetValue for super::CM0_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0002
    }
}
#[doc = "Reader of field `SLV_STALL`"]
pub type SLV_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_STALL`"]
pub struct SLV_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_STALL_W<'a> {
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
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
#[doc = "Reader of field `VECTKEYSTAT`"]
pub type VECTKEYSTAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub fn slv_stall(&self) -> SLV_STALL_R {
        SLV_STALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VECTKEYSTAT_R {
        VECTKEYSTAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub fn slv_stall(&mut self) -> SLV_STALL_W {
        SLV_STALL_W { w: self }
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
