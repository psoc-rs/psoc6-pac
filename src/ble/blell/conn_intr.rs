#[doc = "Register `CONN_INTR` reader"]
pub struct R(crate::R<CONN_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_INTR` writer"]
pub struct W(crate::W<CONN_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_INTR_SPEC>;
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
impl From<crate::W<CONN_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_CLOSED` reader - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
pub type CONN_CLOSED_R = crate::BitReader<bool>;
#[doc = "Field `CONN_CLOSED` writer - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
pub type CONN_CLOSED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `CONN_ESTB` reader - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
pub type CONN_ESTB_R = crate::BitReader<bool>;
#[doc = "Field `CONN_ESTB` writer - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
pub type CONN_ESTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `MAP_UPDT_DONE` reader - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
pub type MAP_UPDT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `MAP_UPDT_DONE` writer - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
pub type MAP_UPDT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `START_CE` reader - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
pub type START_CE_R = crate::BitReader<bool>;
#[doc = "Field `START_CE` writer - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
pub type START_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `CLOSE_CE` reader - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
pub type CLOSE_CE_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE_CE` writer - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
pub type CLOSE_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `CE_TX_ACK` reader - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
pub type CE_TX_ACK_R = crate::BitReader<bool>;
#[doc = "Field `CE_TX_ACK` writer - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
pub type CE_TX_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `CE_RX` reader - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
pub type CE_RX_R = crate::BitReader<bool>;
#[doc = "Field `CE_RX` writer - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
pub type CE_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `CON_UPDT_DONE` reader - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
pub type CON_UPDT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `CON_UPDT_DONE` writer - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
pub type CON_UPDT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `DISCON_STATUS` reader - Reason for disconnect - indicates the reason the link is disconnected by hardware. 001 - connection failed to be established 010 - supervision timeout 011 - kill connection by host 100 - kill connection after ACK transmitted 101 - PDU response timer expired"]
pub type DISCON_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PDU_STATUS` reader - Status of PDU received. This information is valid along with receive interrupt. xx1 - Bad Packet (packet with CRC error) 000 - empty PDU 010 - new data (non-empty) PDU 110 - Duplicate Packet"]
pub type RX_PDU_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PING_TIMER_EXPIRD_INTR` reader - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
pub type PING_TIMER_EXPIRD_INTR_R = crate::BitReader<bool>;
#[doc = "Field `PING_TIMER_EXPIRD_INTR` writer - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
pub type PING_TIMER_EXPIRD_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` reader - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
pub type PING_NEARLY_EXPIRD_INTR_R = crate::BitReader<bool>;
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` writer - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
pub type PING_NEARLY_EXPIRD_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn conn_closed(&self) -> CONN_CLOSED_R {
        CONN_CLOSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
    #[inline(always)]
    pub fn conn_estb(&self) -> CONN_ESTB_R {
        CONN_ESTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
    #[inline(always)]
    pub fn map_updt_done(&self) -> MAP_UPDT_DONE_R {
        MAP_UPDT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
    #[inline(always)]
    pub fn start_ce(&self) -> START_CE_R {
        START_CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
    #[inline(always)]
    pub fn close_ce(&self) -> CLOSE_CE_R {
        CLOSE_CE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
    #[inline(always)]
    pub fn ce_tx_ack(&self) -> CE_TX_ACK_R {
        CE_TX_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
    #[inline(always)]
    pub fn ce_rx(&self) -> CE_RX_R {
        CE_RX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn con_updt_done(&self) -> CON_UPDT_DONE_R {
        CON_UPDT_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reason for disconnect - indicates the reason the link is disconnected by hardware. 001 - connection failed to be established 010 - supervision timeout 011 - kill connection by host 100 - kill connection after ACK transmitted 101 - PDU response timer expired"]
    #[inline(always)]
    pub fn discon_status(&self) -> DISCON_STATUS_R {
        DISCON_STATUS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Status of PDU received. This information is valid along with receive interrupt. xx1 - Bad Packet (packet with CRC error) 000 - empty PDU 010 - new data (non-empty) PDU 110 - Duplicate Packet"]
    #[inline(always)]
    pub fn rx_pdu_status(&self) -> RX_PDU_STATUS_R {
        RX_PDU_STATUS_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&self) -> PING_TIMER_EXPIRD_INTR_R {
        PING_TIMER_EXPIRD_INTR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&self) -> PING_NEARLY_EXPIRD_INTR_R {
        PING_NEARLY_EXPIRD_INTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates that the link is disconnected. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn conn_closed(&mut self) -> CONN_CLOSED_W<0> {
        CONN_CLOSED_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set it indicates that the connection has been established. The bit is also set when a connection update procedure is complet-ed, at the start of the first anchor point with the updated parameters. If this bit is written with 1, it clears the connection established interrupt."]
    #[inline(always)]
    pub fn conn_estb(&mut self) -> CONN_ESTB_W<1> {
        CONN_ESTB_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the channel map update is completed at the instant specified by the firmware. If this bit is written with 1, it clears the map update done interrupt."]
    #[inline(always)]
    pub fn map_updt_done(&mut self) -> MAP_UPDT_DONE_W<2> {
        MAP_UPDT_DONE_W::new(self)
    }
    #[doc = "Bit 3 - If this bit is set it indicates that the connection event started interrupt has happened. If this bit is written with 1, it clears the connection event started interrupt."]
    #[inline(always)]
    pub fn start_ce(&mut self) -> START_CE_W<3> {
        START_CE_W::new(self)
    }
    #[doc = "Bit 4 - If this bit is set it indicates that the connection event closed interrupt has happened. If this bit is written with 1, it clears the connection event closed interrupt."]
    #[inline(always)]
    pub fn close_ce(&mut self) -> CLOSE_CE_W<4> {
        CLOSE_CE_W::new(self)
    }
    #[doc = "Bit 5 - If this bit is set it indicates that the connection event transmission acknowledgement is received for the previous non-empty packet transmitted. If this bit is written with 1, it clears the ce transmission acknowledgement interrupt."]
    #[inline(always)]
    pub fn ce_tx_ack(&mut self) -> CE_TX_ACK_W<5> {
        CE_TX_ACK_W::new(self)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that a packet is received in the connection event. If this bit is written with 1, it clears the connection event received interrupt."]
    #[inline(always)]
    pub fn ce_rx(&mut self) -> CE_RX_W<6> {
        CE_RX_W::new(self)
    }
    #[doc = "Bit 7 - This bit is set when the last connection event with previous connec-tion parameters is reached. The bit is set immediately after the re-ceive operation at the anchor point of the last connection event. If this bit is written with 1, it clears the connection updated interrupt."]
    #[inline(always)]
    pub fn con_updt_done(&mut self) -> CON_UPDT_DONE_W<7> {
        CON_UPDT_DONE_W::new(self)
    }
    #[doc = "Bit 14 - If this is set, it indicates that ping timer has expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&mut self) -> PING_TIMER_EXPIRD_INTR_W<14> {
        PING_TIMER_EXPIRD_INTR_W::new(self)
    }
    #[doc = "Bit 15 - If this is set, it indicates that ping timer has nearly expired. If this bit is written with 1, it clears the interrupt."]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&mut self) -> PING_NEARLY_EXPIRD_INTR_W<15> {
        PING_NEARLY_EXPIRD_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_intr](index.html) module"]
pub struct CONN_INTR_SPEC;
impl crate::RegisterSpec for CONN_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_intr::R](R) reader structure"]
impl crate::Readable for CONN_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_intr::W](W) writer structure"]
impl crate::Writable for CONN_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_INTR to value 0"]
impl crate::Resettable for CONN_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
