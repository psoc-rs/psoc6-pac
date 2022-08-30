#[doc = "Register `IO_SEL` reader"]
pub struct R(crate::R<IO_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_SEL` writer"]
pub struct W(crate::W<IO_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_SEL_SPEC>;
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
impl From<crate::W<IO_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSD_TX_OUT` reader - Select waveform for csd_tx_out output signal"]
pub type CSD_TX_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_OUT` writer - Select waveform for csd_tx_out output signal"]
pub type CSD_TX_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSD_TX_OUT_EN` reader - Select waveform for csd_tx_out_en output signal"]
pub type CSD_TX_OUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_OUT_EN` writer - Select waveform for csd_tx_out_en output signal"]
pub type CSD_TX_OUT_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSD_TX_AMUXB_EN` reader - Select waveform for csd_tx_amuxb_en output signal"]
pub type CSD_TX_AMUXB_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_AMUXB_EN` writer - Select waveform for csd_tx_amuxb_en output signal"]
pub type CSD_TX_AMUXB_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSD_TX_N_OUT` reader - Select waveform for csd_tx_n_out output signal"]
pub type CSD_TX_N_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_N_OUT` writer - Select waveform for csd_tx_n_out output signal"]
pub type CSD_TX_N_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSD_TX_N_OUT_EN` reader - Select waveform for csd_tx_n_out_en output signal"]
pub type CSD_TX_N_OUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_N_OUT_EN` writer - Select waveform for csd_tx_n_out_en output signal"]
pub type CSD_TX_N_OUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CSD_TX_N_AMUXA_EN` reader - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CSD_TX_N_AMUXA_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSD_TX_N_AMUXA_EN` writer - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CSD_TX_N_AMUXA_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub fn csd_tx_out(&self) -> CSD_TX_OUT_R {
        CSD_TX_OUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_out_en(&self) -> CSD_TX_OUT_EN_R {
        CSD_TX_OUT_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn csd_tx_amuxb_en(&self) -> CSD_TX_AMUXB_EN_R {
        CSD_TX_AMUXB_EN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out(&self) -> CSD_TX_N_OUT_R {
        CSD_TX_N_OUT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out_en(&self) -> CSD_TX_N_OUT_EN_R {
        CSD_TX_N_OUT_EN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_amuxa_en(&self) -> CSD_TX_N_AMUXA_EN_R {
        CSD_TX_N_AMUXA_EN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub fn csd_tx_out(&mut self) -> CSD_TX_OUT_W<0> {
        CSD_TX_OUT_W::new(self)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_out_en(&mut self) -> CSD_TX_OUT_EN_W<4> {
        CSD_TX_OUT_EN_W::new(self)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn csd_tx_amuxb_en(&mut self) -> CSD_TX_AMUXB_EN_W<12> {
        CSD_TX_AMUXB_EN_W::new(self)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out(&mut self) -> CSD_TX_N_OUT_W<16> {
        CSD_TX_N_OUT_W::new(self)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out_en(&mut self) -> CSD_TX_N_OUT_EN_W<20> {
        CSD_TX_N_OUT_EN_W::new(self)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_amuxa_en(&mut self) -> CSD_TX_N_AMUXA_EN_W<24> {
        CSD_TX_N_AMUXA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO output control Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_sel](index.html) module"]
pub struct IO_SEL_SPEC;
impl crate::RegisterSpec for IO_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_sel::R](R) reader structure"]
impl crate::Readable for IO_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_sel::W](W) writer structure"]
impl crate::Writable for IO_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO_SEL to value 0"]
impl crate::Resettable for IO_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
