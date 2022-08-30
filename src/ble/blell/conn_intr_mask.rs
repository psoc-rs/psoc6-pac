#[doc = "Register `CONN_INTR_MASK` reader"]
pub struct R(crate::R<CONN_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_INTR_MASK` writer"]
pub struct W(crate::W<CONN_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_INTR_MASK_SPEC>;
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
impl From<crate::W<CONN_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_CL_INT_EN` reader - If this bit is set connection closed interrupt is enabled."]
pub type CONN_CL_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONN_CL_INT_EN` writer - If this bit is set connection closed interrupt is enabled."]
pub type CONN_CL_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CONN_ESTB_INT_EN` reader - If this bit is set connection establishment interrupt is enabled."]
pub type CONN_ESTB_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONN_ESTB_INT_EN` writer - If this bit is set connection establishment interrupt is enabled."]
pub type CONN_ESTB_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `MAP_UPDT_INT_EN` reader - If this bit is set, channel map update interrupt is enabled."]
pub type MAP_UPDT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAP_UPDT_INT_EN` writer - If this bit is set, channel map update interrupt is enabled."]
pub type MAP_UPDT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `START_CE_INT_EN` reader - If this bit is set connection event start interrupt is enabled"]
pub type START_CE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `START_CE_INT_EN` writer - If this bit is set connection event start interrupt is enabled"]
pub type START_CE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CLOSE_CE_INT_EN` reader - If this bit is set connection event closed interrupt is enabled."]
pub type CLOSE_CE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE_CE_INT_EN` writer - If this bit is set connection event closed interrupt is enabled."]
pub type CLOSE_CE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CE_TX_ACK_INT_EN` reader - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
pub type CE_TX_ACK_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CE_TX_ACK_INT_EN` writer - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
pub type CE_TX_ACK_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CE_RX_INT_EN` reader - If this bit is set interrupt is enabled for reception of packet in a connection event."]
pub type CE_RX_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CE_RX_INT_EN` writer - If this bit is set interrupt is enabled for reception of packet in a connection event."]
pub type CE_RX_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CONN_UPDATE_INTR_EN` reader - If this bit is set connection update interrupt is enabled."]
pub type CONN_UPDATE_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONN_UPDATE_INTR_EN` writer - If this bit is set connection update interrupt is enabled."]
pub type CONN_UPDATE_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_GOOD_PDU_INT_EN` reader - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
pub type RX_GOOD_PDU_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_GOOD_PDU_INT_EN` writer - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
pub type RX_GOOD_PDU_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RX_BAD_PDU_INT_EN` reader - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
pub type RX_BAD_PDU_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_BAD_PDU_INT_EN` writer - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
pub type RX_BAD_PDU_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `CE_CLOSE_NULL_RX_INT_EN` reader - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
pub type CE_CLOSE_NULL_RX_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CE_CLOSE_NULL_RX_INT_EN` writer - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
pub type CE_CLOSE_NULL_RX_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PING_TIMER_EXPIRD_INTR` reader - If this bit is set ping timer expired interrupt is enabled."]
pub type PING_TIMER_EXPIRD_INTR_R = crate::BitReader<bool>;
#[doc = "Field `PING_TIMER_EXPIRD_INTR` writer - If this bit is set ping timer expired interrupt is enabled."]
pub type PING_TIMER_EXPIRD_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` reader - If this bit is set ping timer nearly expired interrupt is enabled"]
pub type PING_NEARLY_EXPIRD_INTR_R = crate::BitReader<bool>;
#[doc = "Field `PING_NEARLY_EXPIRD_INTR` writer - If this bit is set ping timer nearly expired interrupt is enabled"]
pub type PING_NEARLY_EXPIRD_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set connection closed interrupt is enabled."]
    #[inline(always)]
    pub fn conn_cl_int_en(&self) -> CONN_CL_INT_EN_R {
        CONN_CL_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set connection establishment interrupt is enabled."]
    #[inline(always)]
    pub fn conn_estb_int_en(&self) -> CONN_ESTB_INT_EN_R {
        CONN_ESTB_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set, channel map update interrupt is enabled."]
    #[inline(always)]
    pub fn map_updt_int_en(&self) -> MAP_UPDT_INT_EN_R {
        MAP_UPDT_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set connection event start interrupt is enabled"]
    #[inline(always)]
    pub fn start_ce_int_en(&self) -> START_CE_INT_EN_R {
        START_CE_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set connection event closed interrupt is enabled."]
    #[inline(always)]
    pub fn close_ce_int_en(&self) -> CLOSE_CE_INT_EN_R {
        CLOSE_CE_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
    #[inline(always)]
    pub fn ce_tx_ack_int_en(&self) -> CE_TX_ACK_INT_EN_R {
        CE_TX_ACK_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set interrupt is enabled for reception of packet in a connection event."]
    #[inline(always)]
    pub fn ce_rx_int_en(&self) -> CE_RX_INT_EN_R {
        CE_RX_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set connection update interrupt is enabled."]
    #[inline(always)]
    pub fn conn_update_intr_en(&self) -> CONN_UPDATE_INTR_EN_R {
        CONN_UPDATE_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_good_pdu_int_en(&self) -> RX_GOOD_PDU_INT_EN_R {
        RX_GOOD_PDU_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_bad_pdu_int_en(&self) -> RX_BAD_PDU_INT_EN_R {
        RX_BAD_PDU_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
    #[inline(always)]
    pub fn ce_close_null_rx_int_en(&self) -> CE_CLOSE_NULL_RX_INT_EN_R {
        CE_CLOSE_NULL_RX_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is set ping timer expired interrupt is enabled."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&self) -> PING_TIMER_EXPIRD_INTR_R {
        PING_TIMER_EXPIRD_INTR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If this bit is set ping timer nearly expired interrupt is enabled"]
    #[inline(always)]
    pub fn ping_nearly_expird_intr(&self) -> PING_NEARLY_EXPIRD_INTR_R {
        PING_NEARLY_EXPIRD_INTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set connection closed interrupt is enabled."]
    #[inline(always)]
    pub fn conn_cl_int_en(&mut self) -> CONN_CL_INT_EN_W<0> {
        CONN_CL_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set connection establishment interrupt is enabled."]
    #[inline(always)]
    pub fn conn_estb_int_en(&mut self) -> CONN_ESTB_INT_EN_W<1> {
        CONN_ESTB_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set, channel map update interrupt is enabled."]
    #[inline(always)]
    pub fn map_updt_int_en(&mut self) -> MAP_UPDT_INT_EN_W<2> {
        MAP_UPDT_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - If this bit is set connection event start interrupt is enabled"]
    #[inline(always)]
    pub fn start_ce_int_en(&mut self) -> START_CE_INT_EN_W<3> {
        START_CE_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - If this bit is set connection event closed interrupt is enabled."]
    #[inline(always)]
    pub fn close_ce_int_en(&mut self) -> CLOSE_CE_INT_EN_W<4> {
        CLOSE_CE_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - If this bit is set transmission acknowledgement interrupt is enabled: This interrupt is generated to indicate to the firmware that a non-empty packet transmitted is successfully acknowledged by the remote device. For negative acknowledgements from remote device, this interrupt indication is not generated."]
    #[inline(always)]
    pub fn ce_tx_ack_int_en(&mut self) -> CE_TX_ACK_INT_EN_W<5> {
        CE_TX_ACK_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - If this bit is set interrupt is enabled for reception of packet in a connection event."]
    #[inline(always)]
    pub fn ce_rx_int_en(&mut self) -> CE_RX_INT_EN_W<6> {
        CE_RX_INT_EN_W::new(self)
    }
    #[doc = "Bit 7 - If this bit is set connection update interrupt is enabled."]
    #[inline(always)]
    pub fn conn_update_intr_en(&mut self) -> CONN_UPDATE_INTR_EN_W<7> {
        CONN_UPDATE_INTR_EN_W::new(self)
    }
    #[doc = "Bit 8 - If this bit is set packet receive good pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_good_pdu_int_en(&mut self) -> RX_GOOD_PDU_INT_EN_W<8> {
        RX_GOOD_PDU_INT_EN_W::new(self)
    }
    #[doc = "Bit 9 - If this bit is set packet receive bad pdu interrupt is enabled. Effective only when bit 6 is set."]
    #[inline(always)]
    pub fn rx_bad_pdu_int_en(&mut self) -> RX_BAD_PDU_INT_EN_W<9> {
        RX_BAD_PDU_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - If this but us set, the RX interrupt is triggerred for an end of connection event with a null packet"]
    #[inline(always)]
    pub fn ce_close_null_rx_int_en(&mut self) -> CE_CLOSE_NULL_RX_INT_EN_W<13> {
        CE_CLOSE_NULL_RX_INT_EN_W::new(self)
    }
    #[doc = "Bit 14 - If this bit is set ping timer expired interrupt is enabled."]
    #[inline(always)]
    pub fn ping_timer_expird_intr(&mut self) -> PING_TIMER_EXPIRD_INTR_W<14> {
        PING_TIMER_EXPIRD_INTR_W::new(self)
    }
    #[doc = "Bit 15 - If this bit is set ping timer nearly expired interrupt is enabled"]
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
#[doc = "Connection Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_intr_mask](index.html) module"]
pub struct CONN_INTR_MASK_SPEC;
impl crate::RegisterSpec for CONN_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_intr_mask::R](R) reader structure"]
impl crate::Readable for CONN_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_intr_mask::W](W) writer structure"]
impl crate::Writable for CONN_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_INTR_MASK to value 0x2000"]
impl crate::Resettable for CONN_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
