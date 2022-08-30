#[doc = "Register `CTDAC_SW_CLEAR` reader"]
pub struct R(crate::R<CTDAC_SW_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTDAC_SW_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTDAC_SW_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTDAC_SW_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTDAC_SW_CLEAR` writer"]
pub struct W(crate::W<CTDAC_SW_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTDAC_SW_CLEAR_SPEC>;
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
impl From<crate::W<CTDAC_SW_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTDAC_SW_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTDD_CVD` reader - see corresponding bit in CTD_SW"]
pub type CTDD_CVD_R = crate::BitReader<bool>;
#[doc = "Field `CTDD_CVD` writer - see corresponding bit in CTD_SW"]
pub type CTDD_CVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDO_CO6` reader - see corresponding bit in CTD_SW"]
pub type CTDO_CO6_R = crate::BitReader<bool>;
#[doc = "Field `CTDO_CO6` writer - see corresponding bit in CTD_SW"]
pub type CTDO_CO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_SW_CLEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_cvd(&self) -> CTDD_CVD_R {
        CTDD_CVD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_co6(&self) -> CTDO_CO6_R {
        CTDO_CO6_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_cvd(&mut self) -> CTDD_CVD_W<0> {
        CTDD_CVD_W::new(self)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_co6(&mut self) -> CTDO_CO6_W<8> {
        CTDO_CO6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTDAC switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_sw_clear](index.html) module"]
pub struct CTDAC_SW_CLEAR_SPEC;
impl crate::RegisterSpec for CTDAC_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctdac_sw_clear::R](R) reader structure"]
impl crate::Readable for CTDAC_SW_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctdac_sw_clear::W](W) writer structure"]
impl crate::Writable for CTDAC_SW_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTDAC_SW_CLEAR to value 0"]
impl crate::Resettable for CTDAC_SW_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
