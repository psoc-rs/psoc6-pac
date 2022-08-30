#[doc = "Register `ENC_INTR` reader"]
pub struct R(crate::R<ENC_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_INTR` writer"]
pub struct W(crate::W<ENC_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_INTR_SPEC>;
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
impl From<crate::W<ENC_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTH_PASS_INTR` reader - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
pub type AUTH_PASS_INTR_R = crate::BitReader<bool>;
#[doc = "Field `AUTH_PASS_INTR` writer - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
pub type AUTH_PASS_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_SPEC, bool, O>;
#[doc = "Field `ECB_PROC_INTR` reader - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
pub type ECB_PROC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `ECB_PROC_INTR` writer - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
pub type ECB_PROC_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_SPEC, bool, O>;
#[doc = "Field `CCM_PROC_INTR` reader - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
pub type CCM_PROC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CCM_PROC_INTR` writer - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
pub type CCM_PROC_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_SPEC, bool, O>;
#[doc = "Field `IN_DATA_CLEAR` reader - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
pub type IN_DATA_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `IN_DATA_CLEAR` writer - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
pub type IN_DATA_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&self) -> AUTH_PASS_INTR_R {
        AUTH_PASS_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&self) -> ECB_PROC_INTR_R {
        ECB_PROC_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&self) -> CCM_PROC_INTR_R {
        CCM_PROC_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&self) -> IN_DATA_CLEAR_R {
        IN_DATA_CLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&mut self) -> AUTH_PASS_INTR_W<0> {
        AUTH_PASS_INTR_W::new(self)
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&mut self) -> ECB_PROC_INTR_W<1> {
        ECB_PROC_INTR_W::new(self)
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&mut self) -> CCM_PROC_INTR_W<2> {
        CCM_PROC_INTR_W::new(self)
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&mut self) -> IN_DATA_CLEAR_W<3> {
        IN_DATA_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Interrupt status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_intr](index.html) module"]
pub struct ENC_INTR_SPEC;
impl crate::RegisterSpec for ENC_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_intr::R](R) reader structure"]
impl crate::Readable for ENC_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_intr::W](W) writer structure"]
impl crate::Writable for ENC_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_INTR to value 0"]
impl crate::Resettable for ENC_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
