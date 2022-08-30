#[doc = "Register `SLV_WIN_ADJ` reader"]
pub struct R(crate::R<SLV_WIN_ADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_WIN_ADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_WIN_ADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_WIN_ADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_WIN_ADJ` writer"]
pub struct W(crate::W<SLV_WIN_ADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_WIN_ADJ_SPEC>;
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
impl From<crate::W<SLV_WIN_ADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_WIN_ADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_WIN_ADJ` reader - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
pub type SLV_WIN_ADJ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLV_WIN_ADJ` writer - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
pub type SLV_WIN_ADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLV_WIN_ADJ_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
    #[inline(always)]
    pub fn slv_win_adj(&self) -> SLV_WIN_ADJ_R {
        SLV_WIN_ADJ_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
    #[inline(always)]
    pub fn slv_win_adj(&mut self) -> SLV_WIN_ADJ_W<0> {
        SLV_WIN_ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave window adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_win_adj](index.html) module"]
pub struct SLV_WIN_ADJ_SPEC;
impl crate::RegisterSpec for SLV_WIN_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_win_adj::R](R) reader structure"]
impl crate::Readable for SLV_WIN_ADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_win_adj::W](W) writer structure"]
impl crate::Writable for SLV_WIN_ADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_WIN_ADJ to value 0x10"]
impl crate::Resettable for SLV_WIN_ADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
