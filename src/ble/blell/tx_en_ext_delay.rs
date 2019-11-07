#[doc = "Reader of register TX_EN_EXT_DELAY"]
pub type R = crate::R<u32, super::TX_EN_EXT_DELAY>;
#[doc = "Writer for register TX_EN_EXT_DELAY"]
pub type W = crate::W<u32, super::TX_EN_EXT_DELAY>;
#[doc = "Register TX_EN_EXT_DELAY `reset()`'s with value 0x1345"]
impl crate::ResetValue for super::TX_EN_EXT_DELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1345
    }
}
#[doc = "Reader of field `TXEN_EXT_DELAY`"]
pub type TXEN_EXT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXEN_EXT_DELAY`"]
pub struct TXEN_EXT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_EXT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RXEN_EXT_DELAY`"]
pub type RXEN_EXT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXEN_EXT_DELAY`"]
pub struct RXEN_EXT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_EXT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DEMOD_2M_COMP_DLY`"]
pub type DEMOD_2M_COMP_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEMOD_2M_COMP_DLY`"]
pub struct DEMOD_2M_COMP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_2M_COMP_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MOD_2M_COMP_DLY`"]
pub type MOD_2M_COMP_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOD_2M_COMP_DLY`"]
pub struct MOD_2M_COMP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_2M_COMP_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmit enable extension delay. This is to extend the active state (high) of rif_tx_en signal after the last bit is sent out from LLH. The unit is in microsecond and the supported range is 00 - 31 us."]
    #[inline(always)]
    pub fn txen_ext_delay(&self) -> TXEN_EXT_DELAY_R {
        TXEN_EXT_DELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - receiver enable extension delay. This is to extend the active state (high) of dbus_rx_en signal after the last bit is received from demod. The unit is in microsecond and the supported range is 00 - 31 us."]
    #[inline(always)]
    pub fn rxen_ext_delay(&self) -> RXEN_EXT_DELAY_R {
        RXEN_EXT_DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 2Mbps demod delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn demod_2m_comp_dly(&self) -> DEMOD_2M_COMP_DLY_R {
        DEMOD_2M_COMP_DLY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 2Mbps modulation delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn mod_2m_comp_dly(&self) -> MOD_2M_COMP_DLY_R {
        MOD_2M_COMP_DLY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit enable extension delay. This is to extend the active state (high) of rif_tx_en signal after the last bit is sent out from LLH. The unit is in microsecond and the supported range is 00 - 31 us."]
    #[inline(always)]
    pub fn txen_ext_delay(&mut self) -> TXEN_EXT_DELAY_W {
        TXEN_EXT_DELAY_W { w: self }
    }
    #[doc = "Bits 4:7 - receiver enable extension delay. This is to extend the active state (high) of dbus_rx_en signal after the last bit is received from demod. The unit is in microsecond and the supported range is 00 - 31 us."]
    #[inline(always)]
    pub fn rxen_ext_delay(&mut self) -> RXEN_EXT_DELAY_W {
        RXEN_EXT_DELAY_W { w: self }
    }
    #[doc = "Bits 8:11 - 2Mbps demod delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn demod_2m_comp_dly(&mut self) -> DEMOD_2M_COMP_DLY_W {
        DEMOD_2M_COMP_DLY_W { w: self }
    }
    #[doc = "Bits 12:15 - 2Mbps modulation delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn mod_2m_comp_dly(&mut self) -> MOD_2M_COMP_DLY_W {
        MOD_2M_COMP_DLY_W { w: self }
    }
}
