#[doc = "Register `FM_HV_DATA_ALL` writer"]
pub struct W(crate::W<FM_HV_DATA_ALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_HV_DATA_ALL_SPEC>;
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
impl From<crate::W<FM_HV_DATA_ALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_HV_DATA_ALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA32` writer - Write all high Voltage page latches with the same 32-bit data in a single write cycle"]
pub type DATA32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FM_HV_DATA_ALL_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle"]
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
#[doc = "Flash macro high Voltage page latches data (for all page latches)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_hv_data_all](index.html) module"]
pub struct FM_HV_DATA_ALL_SPEC;
impl crate::RegisterSpec for FM_HV_DATA_ALL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fm_hv_data_all::W](W) writer structure"]
impl crate::Writable for FM_HV_DATA_ALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM_HV_DATA_ALL to value 0"]
impl crate::Resettable for FM_HV_DATA_ALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
