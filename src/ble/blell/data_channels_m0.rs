#[doc = "Register `DATA_CHANNELS_M0` reader"]
pub struct R(crate::R<DATA_CHANNELS_M0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CHANNELS_M0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CHANNELS_M0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CHANNELS_M0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_CHANNELS_M0` writer"]
pub struct W(crate::W<DATA_CHANNELS_M0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_CHANNELS_M0_SPEC>;
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
impl From<crate::W<DATA_CHANNELS_M0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_CHANNELS_M0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CHANNELS_M0` reader - This register field indicates which of the data channels are in use. This stores the information for the middle 16 (32:16) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_M0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA_CHANNELS_M0` writer - This register field indicates which of the data channels are in use. This stores the information for the middle 16 (32:16) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_M0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_CHANNELS_M0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the middle 16 (32:16) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_m0(&self) -> DATA_CHANNELS_M0_R {
        DATA_CHANNELS_M0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the middle 16 (32:16) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_m0(&mut self) -> DATA_CHANNELS_M0_W<0> {
        DATA_CHANNELS_M0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data channel map 0 (middle word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_m0](index.html) module"]
pub struct DATA_CHANNELS_M0_SPEC;
impl crate::RegisterSpec for DATA_CHANNELS_M0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_channels_m0::R](R) reader structure"]
impl crate::Readable for DATA_CHANNELS_M0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_channels_m0::W](W) writer structure"]
impl crate::Writable for DATA_CHANNELS_M0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_CHANNELS_M0 to value 0"]
impl crate::Resettable for DATA_CHANNELS_M0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
