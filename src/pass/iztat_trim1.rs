#[doc = "Register `IZTAT_TRIM1` reader"]
pub struct R(crate::R<IZTAT_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IZTAT_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IZTAT_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IZTAT_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IZTAT_TRIM1` writer"]
pub struct W(crate::W<IZTAT_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IZTAT_TRIM1_SPEC>;
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
impl From<crate::W<IZTAT_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IZTAT_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IZTAT_TC_TRIM` reader - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IZTAT_TC_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IZTAT_TC_TRIM` writer - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IZTAT_TC_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IZTAT_TRIM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub fn iztat_tc_trim(&self) -> IZTAT_TC_TRIM_R {
        IZTAT_TC_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub fn iztat_tc_trim(&mut self) -> IZTAT_TC_TRIM_W<0> {
        IZTAT_TC_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IZTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim1](index.html) module"]
pub struct IZTAT_TRIM1_SPEC;
impl crate::RegisterSpec for IZTAT_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iztat_trim1::R](R) reader structure"]
impl crate::Readable for IZTAT_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iztat_trim1::W](W) writer structure"]
impl crate::Writable for IZTAT_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IZTAT_TRIM1 to value 0"]
impl crate::Resettable for IZTAT_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
