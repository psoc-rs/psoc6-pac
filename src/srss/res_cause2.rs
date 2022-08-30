#[doc = "Register `RES_CAUSE2` reader"]
pub struct R(crate::R<RES_CAUSE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE2` writer"]
pub struct W(crate::W<RES_CAUSE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE2_SPEC>;
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
impl From<crate::W<RES_CAUSE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CSV_HF_LOSS` reader - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_LOSS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESET_CSV_HF_LOSS` writer - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_LOSS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RES_CAUSE2_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESET_CSV_HF_FREQ` reader - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_FREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESET_CSV_HF_FREQ` writer - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RES_CAUSE2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&self) -> RESET_CSV_HF_LOSS_R {
        RESET_CSV_HF_LOSS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&self) -> RESET_CSV_HF_FREQ_R {
        RESET_CSV_HF_FREQ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&mut self) -> RESET_CSV_HF_LOSS_W<0> {
        RESET_CSV_HF_LOSS_W::new(self)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&mut self) -> RESET_CSV_HF_FREQ_W<16> {
        RESET_CSV_HF_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause2](index.html) module"]
pub struct RES_CAUSE2_SPEC;
impl crate::RegisterSpec for RES_CAUSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause2::R](R) reader structure"]
impl crate::Readable for RES_CAUSE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause2::W](W) writer structure"]
impl crate::Writable for RES_CAUSE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES_CAUSE2 to value 0"]
impl crate::Resettable for RES_CAUSE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
