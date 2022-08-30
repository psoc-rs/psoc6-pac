#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_OVFLW` reader - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CNT_OVFLW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT_OVFLW` writer - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CNT_OVFLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CNT_OVFLW_R {
        CNT_OVFLW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub fn cnt_ovflw(&mut self) -> CNT_OVFLW_W<0> {
        CNT_OVFLW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Profile interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
