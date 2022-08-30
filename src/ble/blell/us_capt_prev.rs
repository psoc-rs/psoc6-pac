#[doc = "Register `US_CAPT_PREV` reader"]
pub struct R(crate::R<US_CAPT_PREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_CAPT_PREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_CAPT_PREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_CAPT_PREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_CAPT_PREV` writer"]
pub struct W(crate::W<US_CAPT_PREV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_CAPT_PREV_SPEC>;
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
impl From<crate::W<US_CAPT_PREV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_CAPT_PREV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `US_CAPT_LOAD` reader - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
pub type US_CAPT_LOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `US_CAPT_LOAD` writer - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
pub type US_CAPT_LOAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, US_CAPT_PREV_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
    #[inline(always)]
    pub fn us_capt_load(&self) -> US_CAPT_LOAD_R {
        US_CAPT_LOAD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
    #[inline(always)]
    pub fn us_capt_load(&mut self) -> US_CAPT_LOAD_W<0> {
        US_CAPT_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Previous captured US of the BT Slot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_capt_prev](index.html) module"]
pub struct US_CAPT_PREV_SPEC;
impl crate::RegisterSpec for US_CAPT_PREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_capt_prev::R](R) reader structure"]
impl crate::Readable for US_CAPT_PREV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_capt_prev::W](W) writer structure"]
impl crate::Writable for US_CAPT_PREV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_CAPT_PREV to value 0"]
impl crate::Resettable for US_CAPT_PREV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
