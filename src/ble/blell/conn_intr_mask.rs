#[doc = "Reader of register CONN_INTR_MASK"]
pub type R = crate::R<u32, super::CONN_INTR_MASK>;
#[doc = "Writer for register CONN_INTR_MASK"]
pub type W = crate::W<u32, super::CONN_INTR_MASK>;
#[doc = "Register CONN_INTR_MASK `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CONN_INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `CONN_CL_INT_EN`"]
pub type CONN_CL_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_CL_INT_EN`"]
pub struct CONN_CL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_CL_INT_EN_W<'a> {
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
#[doc = "Reader of field `CONN_ESTB_INT_EN`"]
pub type CONN_ESTB_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_ESTB_INT_EN`"]
pub struct CONN_ESTB_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_ESTB_INT_EN_W<'a> {
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
#[doc = "Reader of field `MAP_UPDT_INT_EN`"]
pub type MAP_UPDT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAP_UPDT_INT_EN`"]
pub struct MAP_UPDT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_UPDT_INT_EN_W<'a> {
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
#[doc = "Reader of field `START_CE_INT_EN`"]
pub type START_CE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_CE_INT_EN`"]
pub struct START_CE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CLOSE_CE_INT_EN`"]
pub type CLOSE_CE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLOSE_CE_INT_EN`"]
pub struct CLOSE_CE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOSE_CE_INT_EN_W<'a> {
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
#[doc = "Reader of field `CE_TX_ACK_INT_EN`"]
pub type CE_TX_ACK_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE_TX_ACK_INT_EN`"]
pub struct CE_TX_ACK_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_TX_ACK_INT_EN_W<'a> {
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
#[doc = "Reader of field `CE_RX_INT_EN`"]
pub type CE_RX_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE_RX_INT_EN`"]
pub struct CE_RX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_RX_INT_EN_W<'a> {
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
#[doc = "Reader of field `CONN_UPDATE_INTR_EN`"]
pub type CONN_UPDATE_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_UPDATE_INTR_EN`"]
pub struct CONN_UPDATE_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_UPDATE_INTR_EN_W<'a> {
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
#[doc = "Reader of field `RX_GOOD_PDU_INT_EN`"]
pub type RX_GOOD_PDU_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_GOOD_PDU_INT_EN`"]
pub struct RX_GOOD_PDU_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_GOOD_PDU_INT_EN_W<'a> {
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
#[doc = "Reader of field `RX_BAD_PDU_INT_EN`"]
pub type RX_BAD_PDU_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_BAD_PDU_INT_EN`"]
pub struct RX_BAD_PDU_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BAD_PDU_INT_EN_W<'a> {
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
#[doc = "Reader of field `CE_CLOSE_NULL_RX_INT_EN`"]
pub type CE_CLOSE_NULL_RX_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE_CLOSE_NULL_RX_INT_EN`"]
pub struct CE_CLOSE_NULL_RX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_CLOSE_NULL_RX_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PING_TIMER_EXPIRD_INTR`"]
pub type PING_TIMER_EXPIRD_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PING_TIMER_EXPIRD_INTR`"]
pub struct PING_TIMER_EXPIRD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PING_TIMER_EXPIRD_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PING_NEARLY_EXPIRD_INTR`"]
pub type PING_NEARLY_EXPIRD_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PING_NEARLY_EXPIRD_INTR`"]
pub struct PING_NEARLY_EXPIRD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PING_NEARLY_EXPIRD_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set connection closed interrupt is enabled."]
    #[inline(always)]
    pub fn conn_cl_int_en(&self) -> CONN_CL_INT_EN_R {
        CONN_CL_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set connection establishment interrupt is enabled."]
    #[inline(always)]
    pub fn conn_estb_int_en(&self) -> CONN_ESTB_INT_EN_R {
        CONN_ESTB_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set, channel map update interrupt is enabled."]
    #[inline(always)]
    pub fn map_updt_int_en(&self) -> MAP_UPDT_INT_EN_R {
        MAP_UPDT_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set connection event start interrupt is enabled"]
    #[inline(always)]
    pub fn start_ce_int_en(&self) -> START_CE_INT_EN_R {
        START_CE_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set connection event closed interrupt is enabled."]
    #[inline(always)]
    pub fn close_ce_int_en(&self) -> CLOSE_CE_INT_EN_R {
        CLOSE_CE_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
    #[inline(always)]
    pub fn ce_tx_ack_int_en(&self) -> CE_TX_ACK_INT_EN_R {
        CE_TX_ACK_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set interrupt is enabled for reception of packet in a connection event."]
    #[inline(always)]
    pub fn ce_rx_int_en(&self) -> CE_RX_INT_EN_R {
        CE_RX_INT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set connection update interrupt is enabled."]
    #[inline(always)]
    pub fn conn_update_intr_en(&self) -> CONN_UPDATE_INTR_EN_R {
        CONN_UPDATE_INTR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_good_pdu_int_en(&self) -> RX_GOOD_PDU_INT_EN_R {
        RX_GOOD_PDU_INT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_bad_pdu_int_en(&self) -> RX_BAD_PDU_INT_EN_R {
        RX_BAD_PDU_INT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
    #[inline(always)]
    pub fn ce_close_null_rx_int_en(&self) -> CE_CLOSE_NULL_RX_INT_EN_R {
        CE_CLOSE_NULL_RX_INT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If this bit is set ping timer expired interrupt is enabled."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&self) -> PING_TIMER_EXPIRD_INTR_R {
        PING_TIMER_EXPIRD_INTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If this bit is set ping timer nearly expired interrupt is enabled"]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&self) -> PING_NEARLY_EXPIRD_INTR_R {
        PING_NEARLY_EXPIRD_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set connection closed interrupt is enabled."]
    #[inline(always)]
    pub fn conn_cl_int_en(&mut self) -> CONN_CL_INT_EN_W {
        CONN_CL_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set connection establishment interrupt is enabled."]
    #[inline(always)]
    pub fn conn_estb_int_en(&mut self) -> CONN_ESTB_INT_EN_W {
        CONN_ESTB_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set, channel map update interrupt is enabled."]
    #[inline(always)]
    pub fn map_updt_int_en(&mut self) -> MAP_UPDT_INT_EN_W {
        MAP_UPDT_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set connection event start interrupt is enabled"]
    #[inline(always)]
    pub fn start_ce_int_en(&mut self) -> START_CE_INT_EN_W {
        START_CE_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set connection event closed interrupt is enabled."]
    #[inline(always)]
    pub fn close_ce_int_en(&mut self) -> CLOSE_CE_INT_EN_W {
        CLOSE_CE_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
    #[inline(always)]
    pub fn ce_tx_ack_int_en(&mut self) -> CE_TX_ACK_INT_EN_W {
        CE_TX_ACK_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set interrupt is enabled for reception of packet in a connection event."]
    #[inline(always)]
    pub fn ce_rx_int_en(&mut self) -> CE_RX_INT_EN_W {
        CE_RX_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set connection update interrupt is enabled."]
    #[inline(always)]
    pub fn conn_update_intr_en(&mut self) -> CONN_UPDATE_INTR_EN_W {
        CONN_UPDATE_INTR_EN_W { w: self }
    }
    #[doc = "Bit 8 - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_good_pdu_int_en(&mut self) -> RX_GOOD_PDU_INT_EN_W {
        RX_GOOD_PDU_INT_EN_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_bad_pdu_int_en(&mut self) -> RX_BAD_PDU_INT_EN_W {
        RX_BAD_PDU_INT_EN_W { w: self }
    }
    #[doc = "Bit 13 - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
    #[inline(always)]
    pub fn ce_close_null_rx_int_en(&mut self) -> CE_CLOSE_NULL_RX_INT_EN_W {
        CE_CLOSE_NULL_RX_INT_EN_W { w: self }
    }
    #[doc = "Bit 14 - If this bit is set ping timer expired interrupt is enabled."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&mut self) -> PING_TIMER_EXPIRD_INTR_W {
        PING_TIMER_EXPIRD_INTR_W { w: self }
    }
    #[doc = "Bit 15 - If this bit is set ping timer nearly expired interrupt is enabled"]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&mut self) -> PING_NEARLY_EXPIRD_INTR_W {
        PING_NEARLY_EXPIRD_INTR_W { w: self }
    }
}
