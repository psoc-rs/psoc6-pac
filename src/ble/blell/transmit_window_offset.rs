#[doc = "Register `TRANSMIT_WINDOW_OFFSET` reader"]
pub struct R(crate::R<TRANSMIT_WINDOW_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_WINDOW_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_WINDOW_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_WINDOW_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_WINDOW_OFFSET` writer"]
pub struct W(crate::W<TRANSMIT_WINDOW_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_WINDOW_OFFSET_SPEC>;
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
impl From<crate::W<TRANSMIT_WINDOW_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_WINDOW_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WINDOW_OFFSET` reader - This is used to determine the first anchor point for the master transmission, from the time of connection creation. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
pub type TX_WINDOW_OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_WINDOW_OFFSET` writer - This is used to determine the first anchor point for the master transmission, from the time of connection creation. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
pub type TX_WINDOW_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSMIT_WINDOW_OFFSET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is used to determine the first anchor point for the master transmission, from the time of connection creation. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
    #[inline(always)]
    pub fn tx_window_offset(&self) -> TX_WINDOW_OFFSET_R {
        TX_WINDOW_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is used to determine the first anchor point for the master transmission, from the time of connection creation. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
    #[inline(always)]
    pub fn tx_window_offset(&mut self) -> TX_WINDOW_OFFSET_W<0> {
        TX_WINDOW_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit window offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_window_offset](index.html) module"]
pub struct TRANSMIT_WINDOW_OFFSET_SPEC;
impl crate::RegisterSpec for TRANSMIT_WINDOW_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_window_offset::R](R) reader structure"]
impl crate::Readable for TRANSMIT_WINDOW_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_window_offset::W](W) writer structure"]
impl crate::Writable for TRANSMIT_WINDOW_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRANSMIT_WINDOW_OFFSET to value 0"]
impl crate::Resettable for TRANSMIT_WINDOW_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
