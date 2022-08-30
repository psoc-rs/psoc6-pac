#[doc = "Register `TRANSMIT_WINDOW_SIZE` reader"]
pub struct R(crate::R<TRANSMIT_WINDOW_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_WINDOW_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_WINDOW_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_WINDOW_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_WINDOW_SIZE` writer"]
pub struct W(crate::W<TRANSMIT_WINDOW_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_WINDOW_SIZE_SPEC>;
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
impl From<crate::W<TRANSMIT_WINDOW_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_WINDOW_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WINDOW_SIZE` reader - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub type TX_WINDOW_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_WINDOW_SIZE` writer - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub type TX_WINDOW_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSMIT_WINDOW_SIZE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&self) -> TX_WINDOW_SIZE_R {
        TX_WINDOW_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&mut self) -> TX_WINDOW_SIZE_W<0> {
        TX_WINDOW_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_window_size](index.html) module"]
pub struct TRANSMIT_WINDOW_SIZE_SPEC;
impl crate::RegisterSpec for TRANSMIT_WINDOW_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_window_size::R](R) reader structure"]
impl crate::Readable for TRANSMIT_WINDOW_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_window_size::W](W) writer structure"]
impl crate::Writable for TRANSMIT_WINDOW_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRANSMIT_WINDOW_SIZE to value 0"]
impl crate::Resettable for TRANSMIT_WINDOW_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
