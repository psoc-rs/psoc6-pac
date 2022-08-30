#[doc = "Register `SEQ_DEFAULT` reader"]
pub struct R(crate::R<SEQ_DEFAULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_DEFAULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_DEFAULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_DEFAULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_DEFAULT` writer"]
pub struct W(crate::W<SEQ_DEFAULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_DEFAULT_SPEC>;
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
impl From<crate::W<SEQ_DEFAULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_DEFAULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STROBE_A` reader - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_A_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_A` writer - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_B` reader - Specifies value of eFUSEcontrol signal strobe_b"]
pub type STROBE_B_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_B` writer - Specifies value of eFUSEcontrol signal strobe_b"]
pub type STROBE_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_C` reader - Specifies value of eFUSE control signal strobe_c"]
pub type STROBE_C_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_C` writer - Specifies value of eFUSE control signal strobe_c"]
pub type STROBE_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_D` reader - Specifies value of eFUSE control signal strobe_d"]
pub type STROBE_D_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_D` writer - Specifies value of eFUSE control signal strobe_d"]
pub type STROBE_D_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_E` reader - Specifies value of eFUSE control signal strobe_e"]
pub type STROBE_E_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_E` writer - Specifies value of eFUSE control signal strobe_e"]
pub type STROBE_E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_F` reader - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_F_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_F` writer - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
#[doc = "Field `STROBE_G` reader - Specifies value of eFUSE control signal strobe_g"]
pub type STROBE_G_R = crate::BitReader<bool>;
#[doc = "Field `STROBE_G` writer - Specifies value of eFUSE control signal strobe_g"]
pub type STROBE_G_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DEFAULT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&self) -> STROBE_A_R {
        STROBE_A_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> STROBE_B_R {
        STROBE_B_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> STROBE_C_R {
        STROBE_C_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> STROBE_D_R {
        STROBE_D_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> STROBE_E_R {
        STROBE_E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> STROBE_F_R {
        STROBE_F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> STROBE_G_R {
        STROBE_G_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&mut self) -> STROBE_A_W<16> {
        STROBE_A_W::new(self)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&mut self) -> STROBE_B_W<17> {
        STROBE_B_W::new(self)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&mut self) -> STROBE_C_W<18> {
        STROBE_C_W::new(self)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&mut self) -> STROBE_D_W<19> {
        STROBE_D_W::new(self)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&mut self) -> STROBE_E_W<20> {
        STROBE_E_W::new(self)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&mut self) -> STROBE_F_W<21> {
        STROBE_F_W::new(self)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&mut self) -> STROBE_G_W<22> {
        STROBE_G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer Default value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_default](index.html) module"]
pub struct SEQ_DEFAULT_SPEC;
impl crate::RegisterSpec for SEQ_DEFAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_default::R](R) reader structure"]
impl crate::Readable for SEQ_DEFAULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_default::W](W) writer structure"]
impl crate::Writable for SEQ_DEFAULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_DEFAULT to value 0x001d_0000"]
impl crate::Resettable for SEQ_DEFAULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001d_0000
    }
}
