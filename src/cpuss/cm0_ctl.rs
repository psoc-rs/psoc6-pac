#[doc = "Register `CM0_CTL` reader"]
pub struct R(crate::R<CM0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_CTL` writer"]
pub struct W(crate::W<CM0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CM0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_STALL` reader - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
pub type SLV_STALL_R = crate::BitReader<bool>;
#[doc = "Field `SLV_STALL` writer - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
pub type SLV_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM0_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM0_CTL_SPEC, bool, O>;
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VECTKEYSTAT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Processor debug access control: '0': Access. '1': Stall access. This field is used to stall/delay debug accesses. This is useful to protect execution of code that needs to be protected from debug accesses."]
    #[inline(always)]
    pub fn slv_stall(&self) -> SLV_STALL_R {
        SLV_STALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 1) & 1) != 0)
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
    pub fn slv_stall(&mut self) -> SLV_STALL_W<0> {
        SLV_STALL_W::new(self)
    }
    #[doc = "Bit 1 - Processor enable: '0': Disabled. Processor clock is turned off and reset is activated. After SW clears this field to '0', HW automatically sets this field to '1'. This effectively results in a CM0+ reset, followed by a CM0+ warm boot. '1': Enabled. Note: The intent is that this bit is modified only through an external probe or by the CM4 while the CM0+ is in Sleep or DeepSleep power mode. If this field is cleared to '0' by the CM0+ itself, it should be done under controlled conditions (such that undesirable side effects can be prevented). Note: The CM0+ CPU has a AIRCR.SYSRESETREQ register field that allows the CM0+ to reset the complete device (ENABLED only disables/enables the CM0+), resulting in a warm boot. This CPU register field has similar 'built-in protection' as this CM0_CTL register to prevent accidental system writes (the upper 16-bits of the register need to be written with a 0x05fa key value; see CPU user manual for more details)."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<1> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ctl](index.html) module"]
pub struct CM0_CTL_SPEC;
impl crate::RegisterSpec for CM0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_ctl::R](R) reader structure"]
impl crate::Readable for CM0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_ctl::W](W) writer structure"]
impl crate::Writable for CM0_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_CTL to value 0xfa05_0002"]
impl crate::Resettable for CM0_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfa05_0002
    }
}
