#[doc = "Register `CH_CURR_PTR` reader"]
pub struct R(crate::R<CH_CURR_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CURR_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CURR_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CURR_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_CURR_PTR` writer"]
pub struct W(crate::W<CH_CURR_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CURR_PTR_SPEC>;
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
impl From<crate::W<CH_CURR_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CURR_PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CURR_PTR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_curr_ptr](index.html) module"]
pub struct CH_CURR_PTR_SPEC;
impl crate::RegisterSpec for CH_CURR_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_curr_ptr::R](R) reader structure"]
impl crate::Readable for CH_CURR_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_curr_ptr::W](W) writer structure"]
impl crate::Writable for CH_CURR_PTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_CURR_PTR to value 0"]
impl crate::Resettable for CH_CURR_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
