#[doc = "Reader of register INIT_INTR"]
pub type R = crate::R<u32, super::INIT_INTR>;
#[doc = "Writer for register INIT_INTR"]
pub type W = crate::W<u32, super::INIT_INTR>;
#[doc = "Register INIT_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_INTERVAL_EXPIRE_INTR`"]
pub type INIT_INTERVAL_EXPIRE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_INTERVAL_EXPIRE_INTR`"]
pub struct INIT_INTERVAL_EXPIRE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_INTERVAL_EXPIRE_INTR_W<'a> {
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
#[doc = "Reader of field `INIT_CLOSE_WINDOW_INR`"]
pub type INIT_CLOSE_WINDOW_INR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_CLOSE_WINDOW_INR`"]
pub struct INIT_CLOSE_WINDOW_INR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_CLOSE_WINDOW_INR_W<'a> {
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
#[doc = "Reader of field `INIT_TX_START_INTR`"]
pub type INIT_TX_START_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_TX_START_INTR`"]
pub struct INIT_TX_START_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_TX_START_INTR_W<'a> {
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
#[doc = "Reader of field `MASTER_CONN_CREATED`"]
pub type MASTER_CONN_CREATED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_CONN_CREATED`"]
pub struct MASTER_CONN_CREATED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CONN_CREATED_W<'a> {
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
#[doc = "Reader of field `ADV_RX_SELF_ADDR_UNMCH_INTR`"]
pub type ADV_RX_SELF_ADDR_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_SELF_ADDR_UNMCH_INTR`"]
pub struct ADV_RX_SELF_ADDR_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_SELF_ADDR_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_RX_PEER_ADDR_UNMCH_INTR`"]
pub type ADV_RX_PEER_ADDR_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_PEER_ADDR_UNMCH_INTR`"]
pub struct ADV_RX_PEER_ADDR_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_PEER_ADDR_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `INITA_TX_ADDR_NOT_SET_INTR`"]
pub type INITA_TX_ADDR_NOT_SET_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITA_TX_ADDR_NOT_SET_INTR`"]
pub struct INITA_TX_ADDR_NOT_SET_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_TX_ADDR_NOT_SET_INTR_W<'a> {
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
#[doc = "Reader of field `INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_interval_expire_intr(&self) -> INIT_INTERVAL_EXPIRE_INTR_R {
        INIT_INTERVAL_EXPIRE_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_close_window_inr(&self) -> INIT_CLOSE_WINDOW_INR_R {
        INIT_CLOSE_WINDOW_INR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_tx_start_intr(&self) -> INIT_TX_START_INTR_R {
        INIT_TX_START_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn master_conn_created(&self) -> MASTER_CONN_CREATED_R {
        MASTER_CONN_CREATED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_self_addr_unmch_intr(&self) -> ADV_RX_SELF_ADDR_UNMCH_INTR_R {
        ADV_RX_SELF_ADDR_UNMCH_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_addr_unmch_intr(&self) -> ADV_RX_PEER_ADDR_UNMCH_INTR_R {
        ADV_RX_PEER_ADDR_UNMCH_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr(&self) -> INITA_TX_ADDR_NOT_SET_INTR_R {
        INITA_TX_ADDR_NOT_SET_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_peer_addr_match_priv_mismatch_intr(
        &self,
    ) -> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_self_addr_match_priv_mismatch_intr(
        &self,
    ) -> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates initiator scan window has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_interval_expire_intr(&mut self) -> INIT_INTERVAL_EXPIRE_INTR_W {
        INIT_INTERVAL_EXPIRE_INTR_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set it indicates initiator scan window has finished. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_close_window_inr(&mut self) -> INIT_CLOSE_WINDOW_INR_W {
        INIT_CLOSE_WINDOW_INR_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates initiator packet (CONREQ) transmission has started. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn init_tx_start_intr(&mut self) -> INIT_TX_START_INTR_W {
        INIT_TX_START_INTR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set it indicates connection is created as master. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn master_conn_created(&mut self) -> MASTER_CONN_CREATED_W {
        MASTER_CONN_CREATED_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_self_addr_unmch_intr(&mut self) -> ADV_RX_SELF_ADDR_UNMCH_INTR_W {
        ADV_RX_SELF_ADDR_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV packet received but the peer device Address is not matched yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_addr_unmch_intr(&mut self) -> ADV_RX_PEER_ADDR_UNMCH_INTR_W {
        ADV_RX_PEER_ADDR_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid INITA RPA to be transmitted in CONN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr(&mut self) -> INITA_TX_ADDR_NOT_SET_INTR_W {
        INITA_TX_ADDR_NOT_SET_INTR_W { w: self }
    }
    #[doc = "Bit 8 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_peer_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        INI_PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set it indicates that - an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator - or an RPA is received from an initiator and matches an entry in the resolving list, but peer IRK is not set and hence a corresponding Identity address is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn ini_self_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        INI_SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
}
