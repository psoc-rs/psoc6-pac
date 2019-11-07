#[doc = "Reader of register INIT_CONFIG"]
pub type R = crate::R<u32, super::INIT_CONFIG>;
#[doc = "Writer for register INIT_CONFIG"]
pub type W = crate::W<u32, super::INIT_CONFIG>;
#[doc = "Register INIT_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_STRT_EN`"]
pub type INIT_STRT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_STRT_EN`"]
pub struct INIT_STRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_STRT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `INIT_CLOSE_EN`"]
pub type INIT_CLOSE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_CLOSE_EN`"]
pub struct INIT_CLOSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_CLOSE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CONN_REQ_TX_EN`"]
pub type CONN_REQ_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_REQ_TX_EN`"]
pub struct CONN_REQ_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CONN_CREATED`"]
pub type CONN_CREATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_CREATED`"]
pub struct CONN_CREATED_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_CREATED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN`"]
pub type INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN`"]
pub struct INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN`"]
pub type INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN`"]
pub struct INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `INITA_TX_ADDR_NOT_SET_INTR_EN`"]
pub type INITA_TX_ADDR_NOT_SET_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITA_TX_ADDR_NOT_SET_INTR_EN`"]
pub struct INITA_TX_ADDR_NOT_SET_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_TX_ADDR_NOT_SET_INTR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `INIT_CHANNEL_MAP`"]
pub type INIT_CHANNEL_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INIT_CHANNEL_MAP`"]
pub struct INIT_CHANNEL_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_CHANNEL_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Initiator event start interrupt."]
    #[inline(always)]
    pub fn init_strt_en(&self) -> INIT_STRT_EN_R {
        INIT_STRT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Initiator event close interrupt."]
    #[inline(always)]
    pub fn init_close_en(&self) -> INIT_CLOSE_EN_R {
        INIT_CLOSE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables connection request packet transmission start interrupt."]
    #[inline(always)]
    pub fn conn_req_tx_en(&self) -> CONN_REQ_TX_EN_R {
        CONN_REQ_TX_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable master connection created interrupt"]
    #[inline(always)]
    pub fn conn_created(&self) -> CONN_CREATED_R {
        CONN_CREATED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_self_rpa_unres_en(&self) -> INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R {
        INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_peer_rpa_unres_en(&self) -> INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R {
        INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr_en(&self) -> INITA_TX_ADDR_NOT_SET_INTR_EN_R {
        INITA_TX_ADDR_NOT_SET_INTR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn init_channel_map(&self) -> INIT_CHANNEL_MAP_R {
        INIT_CHANNEL_MAP_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Initiator event start interrupt."]
    #[inline(always)]
    pub fn init_strt_en(&mut self) -> INIT_STRT_EN_W {
        INIT_STRT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Initiator event close interrupt."]
    #[inline(always)]
    pub fn init_close_en(&mut self) -> INIT_CLOSE_EN_W {
        INIT_CLOSE_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enables connection request packet transmission start interrupt."]
    #[inline(always)]
    pub fn conn_req_tx_en(&mut self) -> CONN_REQ_TX_EN_W {
        CONN_REQ_TX_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable master connection created interrupt"]
    #[inline(always)]
    pub fn conn_created(&mut self) -> CONN_CREATED_W {
        CONN_CREATED_W { w: self }
    }
    #[doc = "Bit 5 - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_self_rpa_unres_en(&mut self) -> INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W {
        INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_peer_rpa_unres_en(&mut self) -> INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W {
        INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr_en(&mut self) -> INITA_TX_ADDR_NOT_SET_INTR_EN_W {
        INITA_TX_ADDR_NOT_SET_INTR_EN_W { w: self }
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn init_channel_map(&mut self) -> INIT_CHANNEL_MAP_W {
        INIT_CHANNEL_MAP_W { w: self }
    }
}
