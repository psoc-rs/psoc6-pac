#[doc = "Register `FM_HV_DATA[%s]` reader"]
pub struct R(crate::R<FM_HV_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_HV_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_HV_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_HV_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_HV_DATA[%s]` writer"]
pub struct W(crate::W<FM_HV_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_HV_DATA_SPEC>;
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
impl From<crate::W<FM_HV_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_HV_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA32` reader - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
pub type DATA32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA32` writer - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
pub type DATA32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FM_HV_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W<0> {
        DATA32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash macro high Voltage page latches data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_hv_data](index.html) module"]
pub struct FM_HV_DATA_SPEC;
impl crate::RegisterSpec for FM_HV_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_hv_data::R](R) reader structure"]
impl crate::Readable for FM_HV_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_hv_data::W](W) writer structure"]
impl crate::Writable for FM_HV_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM_HV_DATA[%s]
to value 0"]
impl crate::Resettable for FM_HV_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
