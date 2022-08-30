#[doc = "Register `IPTAT_TRIM0` reader"]
pub struct R(crate::R<IPTAT_TRIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTAT_TRIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTAT_TRIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTAT_TRIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTAT_TRIM0` writer"]
pub struct W(crate::W<IPTAT_TRIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTAT_TRIM0_SPEC>;
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
impl From<crate::W<IPTAT_TRIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPTAT_TRIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPTAT_CORE_TRIM` reader - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IPTAT_CORE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPTAT_CORE_TRIM` writer - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IPTAT_CORE_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPTAT_TRIM0_SPEC, u8, u8, 4, O>;
#[doc = "Field `IPTAT_CTBM_TRIM` reader - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IPTAT_CTBM_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPTAT_CTBM_TRIM` writer - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IPTAT_CTBM_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPTAT_TRIM0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&self) -> IPTAT_CORE_TRIM_R {
        IPTAT_CORE_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&self) -> IPTAT_CTBM_TRIM_R {
        IPTAT_CTBM_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&mut self) -> IPTAT_CORE_TRIM_W<0> {
        IPTAT_CORE_TRIM_W::new(self)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&mut self) -> IPTAT_CTBM_TRIM_W<4> {
        IPTAT_CTBM_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptat_trim0](index.html) module"]
pub struct IPTAT_TRIM0_SPEC;
impl crate::RegisterSpec for IPTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptat_trim0::R](R) reader structure"]
impl crate::Readable for IPTAT_TRIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptat_trim0::W](W) writer structure"]
impl crate::Writable for IPTAT_TRIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPTAT_TRIM0 to value 0"]
impl crate::Resettable for IPTAT_TRIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
