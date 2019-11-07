#[doc = "Reader of register SW_CMP_N_SEL"]
pub type R = crate::R<u32, super::SW_CMP_N_SEL>;
#[doc = "Writer for register SW_CMP_N_SEL"]
pub type W = crate::W<u32, super::SW_CMP_N_SEL>;
#[doc = "Register SW_CMP_N_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_CMP_N_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_SCRH`"]
pub type SW_SCRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_SCRH`"]
pub struct SW_SCRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SCRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `SW_SCRL`"]
pub type SW_SCRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_SCRL`"]
pub struct SW_SCRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SCRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&self) -> SW_SCRH_R {
        SW_SCRH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&self) -> SW_SCRL_R {
        SW_SCRL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&mut self) -> SW_SCRH_W {
        SW_SCRH_W { w: self }
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&mut self) -> SW_SCRL_W {
        SW_SCRL_W { w: self }
    }
}
