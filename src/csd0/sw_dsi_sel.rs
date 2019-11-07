#[doc = "Reader of register SW_DSI_SEL"]
pub type R = crate::R<u32, super::SW_DSI_SEL>;
#[doc = "Writer for register SW_DSI_SEL"]
pub type W = crate::W<u32, super::SW_DSI_SEL>;
#[doc = "Register SW_DSI_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_DSI_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSI_CSH_TANK`"]
pub type DSI_CSH_TANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSI_CSH_TANK`"]
pub struct DSI_CSH_TANK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_CSH_TANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DSI_CMOD`"]
pub type DSI_CMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSI_CMOD`"]
pub struct DSI_CMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_CMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 & HSCMP 5: phi2 & HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 & HSCMP) 13: !(phi2 & HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    pub fn dsi_csh_tank(&self) -> DSI_CSH_TANK_R {
        DSI_CSH_TANK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    pub fn dsi_cmod(&self) -> DSI_CMOD_R {
        DSI_CMOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 & HSCMP 5: phi2 & HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 & HSCMP) 13: !(phi2 & HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    pub fn dsi_csh_tank(&mut self) -> DSI_CSH_TANK_W {
        DSI_CSH_TANK_W { w: self }
    }
    #[doc = "Bits 4:7 - Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    pub fn dsi_cmod(&mut self) -> DSI_CMOD_W {
        DSI_CMOD_W { w: self }
    }
}
