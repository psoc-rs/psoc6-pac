#[doc = "Register `SW_FW_MOD_SEL` reader"]
pub struct R(crate::R<SW_FW_MOD_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_FW_MOD_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_FW_MOD_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_FW_MOD_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_FW_MOD_SEL` writer"]
pub struct W(crate::W<SW_FW_MOD_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_FW_MOD_SEL_SPEC>;
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
impl From<crate::W<SW_FW_MOD_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_FW_MOD_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_F1PM` reader - Set corresponding switch"]
pub type SW_F1PM_R = crate::BitReader<bool>;
#[doc = "Field `SW_F1PM` writer - Set corresponding switch"]
pub type SW_F1PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_MOD_SEL_SPEC, bool, O>;
#[doc = "Field `SW_F1MA` reader - Select waveform for corresponding switch"]
pub type SW_F1MA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_F1MA` writer - Select waveform for corresponding switch"]
pub type SW_F1MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_FW_MOD_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_F1CA` reader - Select waveform for corresponding switch"]
pub type SW_F1CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_F1CA` writer - Select waveform for corresponding switch"]
pub type SW_F1CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_FW_MOD_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_C1CC` reader - Set corresponding switch"]
pub type SW_C1CC_R = crate::BitReader<bool>;
#[doc = "Field `SW_C1CC` writer - Set corresponding switch"]
pub type SW_C1CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_MOD_SEL_SPEC, bool, O>;
#[doc = "Field `SW_C1CD` reader - Set corresponding switch"]
pub type SW_C1CD_R = crate::BitReader<bool>;
#[doc = "Field `SW_C1CD` writer - Set corresponding switch"]
pub type SW_C1CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_MOD_SEL_SPEC, bool, O>;
#[doc = "Field `SW_C1F1` reader - Set corresponding switch"]
pub type SW_C1F1_R = crate::BitReader<bool>;
#[doc = "Field `SW_C1F1` writer - Set corresponding switch"]
pub type SW_C1F1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_MOD_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&self) -> SW_F1PM_R {
        SW_F1PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&self) -> SW_F1MA_R {
        SW_F1MA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&self) -> SW_F1CA_R {
        SW_F1CA_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&self) -> SW_C1CC_R {
        SW_C1CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&self) -> SW_C1CD_R {
        SW_C1CD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&self) -> SW_C1F1_R {
        SW_C1F1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&mut self) -> SW_F1PM_W<0> {
        SW_F1PM_W::new(self)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&mut self) -> SW_F1MA_W<8> {
        SW_F1MA_W::new(self)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&mut self) -> SW_F1CA_W<16> {
        SW_F1CA_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&mut self) -> SW_C1CC_W<20> {
        SW_C1CC_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&mut self) -> SW_C1CD_W<24> {
        SW_C1CD_W::new(self)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&mut self) -> SW_C1F1_W<28> {
        SW_C1F1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Full Wave Cmod Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_mod_sel](index.html) module"]
pub struct SW_FW_MOD_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_MOD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_fw_mod_sel::R](R) reader structure"]
impl crate::Readable for SW_FW_MOD_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_fw_mod_sel::W](W) writer structure"]
impl crate::Writable for SW_FW_MOD_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_FW_MOD_SEL to value 0"]
impl crate::Resettable for SW_FW_MOD_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
