#[doc = "Reader of register IO_SEL"]
pub type R = crate::R<u32, super::IO_SEL>;
#[doc = "Writer for register IO_SEL"]
pub type W = crate::W<u32, super::IO_SEL>;
#[doc = "Register IO_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::IO_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSD_TX_OUT`"]
pub type CSD_TX_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_OUT`"]
pub struct CSD_TX_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CSD_TX_OUT_EN`"]
pub type CSD_TX_OUT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_OUT_EN`"]
pub struct CSD_TX_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_OUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CSD_TX_AMUXB_EN`"]
pub type CSD_TX_AMUXB_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_AMUXB_EN`"]
pub struct CSD_TX_AMUXB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_AMUXB_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CSD_TX_N_OUT`"]
pub type CSD_TX_N_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_N_OUT`"]
pub struct CSD_TX_N_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_N_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CSD_TX_N_OUT_EN`"]
pub type CSD_TX_N_OUT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_N_OUT_EN`"]
pub struct CSD_TX_N_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_N_OUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CSD_TX_N_AMUXA_EN`"]
pub type CSD_TX_N_AMUXA_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSD_TX_N_AMUXA_EN`"]
pub struct CSD_TX_N_AMUXA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSD_TX_N_AMUXA_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
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
    pub fn csd_tx_out(&mut self) -> CSD_TX_OUT_W {
        CSD_TX_OUT_W { w: self }
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_out_en(&mut self) -> CSD_TX_OUT_EN_W {
        CSD_TX_OUT_EN_W { w: self }
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn csd_tx_amuxb_en(&mut self) -> CSD_TX_AMUXB_EN_W {
        CSD_TX_AMUXB_EN_W { w: self }
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out(&mut self) -> CSD_TX_N_OUT_W {
        CSD_TX_N_OUT_W { w: self }
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out_en(&mut self) -> CSD_TX_N_OUT_EN_W {
        CSD_TX_N_OUT_EN_W { w: self }
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_amuxa_en(&mut self) -> CSD_TX_N_AMUXA_EN_W {
        CSD_TX_N_AMUXA_EN_W { w: self }
    }
}
