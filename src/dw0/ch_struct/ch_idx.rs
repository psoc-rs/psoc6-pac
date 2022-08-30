#[doc = "Register `CH_IDX` reader"]
pub struct R(crate::R<CH_IDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_IDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_IDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_IDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_IDX` writer"]
pub struct W(crate::W<CH_IDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_IDX_SPEC>;
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
impl From<crate::W<CH_IDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_IDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X_IDX` reader - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type X_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_IDX` writer - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type X_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_IDX_SPEC, u8, u8, 8, O>;
#[doc = "Field `Y_IDX` reader - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type Y_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y_IDX` writer - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type Y_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_IDX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&self) -> X_IDX_R {
        X_IDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&self) -> Y_IDX_R {
        Y_IDX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&mut self) -> X_IDX_W<0> {
        X_IDX_W::new(self)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&mut self) -> Y_IDX_W<8> {
        Y_IDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current indices\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_idx](index.html) module"]
pub struct CH_IDX_SPEC;
impl crate::RegisterSpec for CH_IDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_idx::R](R) reader structure"]
impl crate::Readable for CH_IDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_idx::W](W) writer structure"]
impl crate::Writable for CH_IDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_IDX to value 0"]
impl crate::Resettable for CH_IDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
