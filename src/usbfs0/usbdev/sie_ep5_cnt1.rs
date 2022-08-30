#[doc = "Register `SIE_EP5_CNT1` reader"]
pub struct R(crate::R<SIE_EP5_CNT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_EP5_CNT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_EP5_CNT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_EP5_CNT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_EP5_CNT1` writer"]
pub struct W(crate::W<SIE_EP5_CNT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_EP5_CNT1_SPEC>;
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
impl From<crate::W<SIE_EP5_CNT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_EP5_CNT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_COUNT` reader - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DATA_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_COUNT` writer - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DATA_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIE_EP5_CNT1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&self) -> DATA_COUNT_R {
        DATA_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&mut self) -> DATA_COUNT_W<0> {
        DATA_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep5_cnt1](index.html) module"]
pub struct SIE_EP5_CNT1_SPEC;
impl crate::RegisterSpec for SIE_EP5_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ep5_cnt1::R](R) reader structure"]
impl crate::Readable for SIE_EP5_CNT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ep5_cnt1::W](W) writer structure"]
impl crate::Writable for SIE_EP5_CNT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_EP5_CNT1 to value 0"]
impl crate::Resettable for SIE_EP5_CNT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
