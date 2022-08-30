#[doc = "Register `CM0_CA_CTL2` reader"]
pub struct R(crate::R<CM0_CA_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CA_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CA_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CA_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_CA_CTL2` writer"]
pub struct W(crate::W<CM0_CA_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_CA_CTL2_SPEC>;
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
impl From<crate::W<CM0_CA_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_CA_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRUP_DELAY` reader - Number clock cycles delay needed after power domain power up"]
pub type PWRUP_DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PWRUP_DELAY` writer - Number clock cycles delay needed after power domain power up"]
pub type PWRUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM0_CA_CTL2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn pwrup_delay(&self) -> PWRUP_DELAY_R {
        PWRUP_DELAY_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn pwrup_delay(&mut self) -> PWRUP_DELAY_W<0> {
        PWRUP_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl2](index.html) module"]
pub struct CM0_CA_CTL2_SPEC;
impl crate::RegisterSpec for CM0_CA_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_ca_ctl2::R](R) reader structure"]
impl crate::Readable for CM0_CA_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl2::W](W) writer structure"]
impl crate::Writable for CM0_CA_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_CA_CTL2 to value 0x012c"]
impl crate::Resettable for CM0_CA_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x012c
    }
}
