#[doc = "Register `SW_FW_TANK_SEL` reader"]
pub struct R(crate::R<SW_FW_TANK_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_FW_TANK_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_FW_TANK_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_FW_TANK_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_FW_TANK_SEL` writer"]
pub struct W(crate::W<SW_FW_TANK_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_FW_TANK_SEL_SPEC>;
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
impl From<crate::W<SW_FW_TANK_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_FW_TANK_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_F2PT` reader - Set corresponding switch"]
pub type SW_F2PT_R = crate::BitReader<bool>;
#[doc = "Field `SW_F2PT` writer - Set corresponding switch"]
pub type SW_F2PT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_TANK_SEL_SPEC, bool, O>;
#[doc = "Field `SW_F2MA` reader - Select waveform for corresponding switch"]
pub type SW_F2MA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_F2MA` writer - Select waveform for corresponding switch"]
pub type SW_F2MA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_FW_TANK_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_F2CA` reader - Select waveform for corresponding switch"]
pub type SW_F2CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_F2CA` writer - Select waveform for corresponding switch"]
pub type SW_F2CA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_FW_TANK_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_F2CB` reader - Select waveform for corresponding switch"]
pub type SW_F2CB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_F2CB` writer - Select waveform for corresponding switch"]
pub type SW_F2CB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_FW_TANK_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SW_C2CC` reader - Set corresponding switch"]
pub type SW_C2CC_R = crate::BitReader<bool>;
#[doc = "Field `SW_C2CC` writer - Set corresponding switch"]
pub type SW_C2CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_TANK_SEL_SPEC, bool, O>;
#[doc = "Field `SW_C2CD` reader - Set corresponding switch"]
pub type SW_C2CD_R = crate::BitReader<bool>;
#[doc = "Field `SW_C2CD` writer - Set corresponding switch"]
pub type SW_C2CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_TANK_SEL_SPEC, bool, O>;
#[doc = "Field `SW_C2F2` reader - Set corresponding switch"]
pub type SW_C2F2_R = crate::BitReader<bool>;
#[doc = "Field `SW_C2F2` writer - Set corresponding switch"]
pub type SW_C2F2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SW_FW_TANK_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&self) -> SW_F2PT_R {
        SW_F2PT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&self) -> SW_F2MA_R {
        SW_F2MA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&self) -> SW_F2CA_R {
        SW_F2CA_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&self) -> SW_F2CB_R {
        SW_F2CB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&self) -> SW_C2CC_R {
        SW_C2CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&self) -> SW_C2CD_R {
        SW_C2CD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&self) -> SW_C2F2_R {
        SW_C2F2_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&mut self) -> SW_F2PT_W<4> {
        SW_F2PT_W::new(self)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&mut self) -> SW_F2MA_W<8> {
        SW_F2MA_W::new(self)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&mut self) -> SW_F2CA_W<12> {
        SW_F2CA_W::new(self)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&mut self) -> SW_F2CB_W<16> {
        SW_F2CB_W::new(self)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&mut self) -> SW_C2CC_W<20> {
        SW_C2CC_W::new(self)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&mut self) -> SW_C2CD_W<24> {
        SW_C2CD_W::new(self)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&mut self) -> SW_C2F2_W<28> {
        SW_C2F2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Full Wave Csh_tank Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_tank_sel](index.html) module"]
pub struct SW_FW_TANK_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_TANK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_fw_tank_sel::R](R) reader structure"]
impl crate::Readable for SW_FW_TANK_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_fw_tank_sel::W](W) writer structure"]
impl crate::Writable for SW_FW_TANK_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_FW_TANK_SEL to value 0"]
impl crate::Resettable for SW_FW_TANK_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
