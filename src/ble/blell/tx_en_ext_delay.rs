#[doc = "Register `TX_EN_EXT_DELAY` reader"]
pub struct R(crate::R<TX_EN_EXT_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_EN_EXT_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_EN_EXT_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_EN_EXT_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_EN_EXT_DELAY` writer"]
pub struct W(crate::W<TX_EN_EXT_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_EN_EXT_DELAY_SPEC>;
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
impl From<crate::W<TX_EN_EXT_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_EN_EXT_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEN_EXT_DELAY` reader - Transmit enable extension delay. This is to extend the active state (high) of rif_tx_en signal after the last bit is sent out from LLH. The unit is in microsecond and the supported range is 00 - 31 us."]
pub type TXEN_EXT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXEN_EXT_DELAY` writer - Transmit enable extension delay. This is to extend the active state (high) of rif_tx_en signal after the last bit is sent out from LLH. The unit is in microsecond and the supported range is 00 - 31 us."]
pub type TXEN_EXT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_EN_EXT_DELAY_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXEN_EXT_DELAY` reader - receiver enable extension delay. This is to extend the active state (high) of dbus_rx_en signal after the last bit is received from demod. The unit is in microsecond and the supported range is 00 - 31 us."]
pub type RXEN_EXT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXEN_EXT_DELAY` writer - receiver enable extension delay. This is to extend the active state (high) of dbus_rx_en signal after the last bit is received from demod. The unit is in microsecond and the supported range is 00 - 31 us."]
pub type RXEN_EXT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_EN_EXT_DELAY_SPEC, u8, u8, 4, O>;
#[doc = "Field `DEMOD_2M_COMP_DLY` reader - 2Mbps demod delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
pub type DEMOD_2M_COMP_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEMOD_2M_COMP_DLY` writer - 2Mbps demod delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
pub type DEMOD_2M_COMP_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_EN_EXT_DELAY_SPEC, u8, u8, 4, O>;
#[doc = "Field `MOD_2M_COMP_DLY` reader - 2Mbps modulation delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
pub type MOD_2M_COMP_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOD_2M_COMP_DLY` writer - 2Mbps modulation delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
pub type MOD_2M_COMP_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_EN_EXT_DELAY_SPEC, u8, u8, 4, O>;
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
    pub fn txen_ext_delay(&mut self) -> TXEN_EXT_DELAY_W<0> {
        TXEN_EXT_DELAY_W::new(self)
    }
    #[doc = "Bits 4:7 - receiver enable extension delay. This is to extend the active state (high) of dbus_rx_en signal after the last bit is received from demod. The unit is in microsecond and the supported range is 00 - 31 us."]
    #[inline(always)]
    pub fn rxen_ext_delay(&mut self) -> RXEN_EXT_DELAY_W<4> {
        RXEN_EXT_DELAY_W::new(self)
    }
    #[doc = "Bits 8:11 - 2Mbps demod delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn demod_2m_comp_dly(&mut self) -> DEMOD_2M_COMP_DLY_W<8> {
        DEMOD_2M_COMP_DLY_W::new(self)
    }
    #[doc = "Bits 12:15 - 2Mbps modulation delay delta compare to 1Mbps demod delay. This data is 2's comp data."]
    #[inline(always)]
    pub fn mod_2m_comp_dly(&mut self) -> MOD_2M_COMP_DLY_W<12> {
        MOD_2M_COMP_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit enable extension delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_en_ext_delay](index.html) module"]
pub struct TX_EN_EXT_DELAY_SPEC;
impl crate::RegisterSpec for TX_EN_EXT_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_en_ext_delay::R](R) reader structure"]
impl crate::Readable for TX_EN_EXT_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_en_ext_delay::W](W) writer structure"]
impl crate::Writable for TX_EN_EXT_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_EN_EXT_DELAY to value 0x1345"]
impl crate::Resettable for TX_EN_EXT_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1345
    }
}
