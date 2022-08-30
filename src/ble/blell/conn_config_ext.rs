#[doc = "Register `CONN_CONFIG_EXT` reader"]
pub struct R(crate::R<CONN_CONFIG_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CONFIG_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CONFIG_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CONFIG_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CONFIG_EXT` writer"]
pub struct W(crate::W<CONN_CONFIG_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CONFIG_EXT_SPEC>;
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
impl From<crate::W<CONN_CONFIG_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CONFIG_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_REQ_2SLOT_EARLY` reader - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
pub type CONN_REQ_2SLOT_EARLY_R = crate::BitReader<bool>;
#[doc = "Field `CONN_REQ_2SLOT_EARLY` writer - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
pub type CONN_REQ_2SLOT_EARLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_EXT_SPEC, bool, O>;
#[doc = "Field `CONN_REQ_3SLOT_EARLY` reader - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
pub type CONN_REQ_3SLOT_EARLY_R = crate::BitReader<bool>;
#[doc = "Field `CONN_REQ_3SLOT_EARLY` writer - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
pub type CONN_REQ_3SLOT_EARLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_EXT_SPEC, bool, O>;
#[doc = "Field `FW_PKT_RCV_CONN_INDEX` reader - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
pub type FW_PKT_RCV_CONN_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FW_PKT_RCV_CONN_INDEX` writer - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
pub type FW_PKT_RCV_CONN_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CONFIG_EXT_SPEC, u8, u8, 5, O>;
#[doc = "Field `MMMS_RX_PKT_LIMIT` reader - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
pub type MMMS_RX_PKT_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMMS_RX_PKT_LIMIT` writer - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
pub type MMMS_RX_PKT_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CONFIG_EXT_SPEC, u8, u8, 6, O>;
#[doc = "Field `DEBUG_CE_EXPIRE` reader - MMMS CE expire control bit"]
pub type DEBUG_CE_EXPIRE_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_CE_EXPIRE` writer - MMMS CE expire control bit"]
pub type DEBUG_CE_EXPIRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_EXT_SPEC, bool, O>;
#[doc = "Field `MT_PDU_CE_EXPIRE` reader - MMMS empty PDU CE expire handling control bit"]
pub type MT_PDU_CE_EXPIRE_R = crate::BitReader<bool>;
#[doc = "Field `MT_PDU_CE_EXPIRE` writer - MMMS empty PDU CE expire handling control bit"]
pub type MT_PDU_CE_EXPIRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_EXT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_2slot_early(&self) -> CONN_REQ_2SLOT_EARLY_R {
        CONN_REQ_2SLOT_EARLY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_3slot_early(&self) -> CONN_REQ_3SLOT_EARLY_R {
        CONN_REQ_3SLOT_EARLY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
    #[inline(always)]
    pub fn fw_pkt_rcv_conn_index(&self) -> FW_PKT_RCV_CONN_INDEX_R {
        FW_PKT_RCV_CONN_INDEX_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
    #[inline(always)]
    pub fn mmms_rx_pkt_limit(&self) -> MMMS_RX_PKT_LIMIT_R {
        MMMS_RX_PKT_LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - MMMS CE expire control bit"]
    #[inline(always)]
    pub fn debug_ce_expire(&self) -> DEBUG_CE_EXPIRE_R {
        DEBUG_CE_EXPIRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMMS empty PDU CE expire handling control bit"]
    #[inline(always)]
    pub fn mt_pdu_ce_expire(&self) -> MT_PDU_CE_EXPIRE_R {
        MT_PDU_CE_EXPIRE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_2slot_early(&mut self) -> CONN_REQ_2SLOT_EARLY_W<0> {
        CONN_REQ_2SLOT_EARLY_W::new(self)
    }
    #[doc = "Bit 1 - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_3slot_early(&mut self) -> CONN_REQ_3SLOT_EARLY_W<1> {
        CONN_REQ_3SLOT_EARLY_W::new(self)
    }
    #[doc = "Bits 2:6 - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
    #[inline(always)]
    pub fn fw_pkt_rcv_conn_index(&mut self) -> FW_PKT_RCV_CONN_INDEX_W<2> {
        FW_PKT_RCV_CONN_INDEX_W::new(self)
    }
    #[doc = "Bits 8:13 - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
    #[inline(always)]
    pub fn mmms_rx_pkt_limit(&mut self) -> MMMS_RX_PKT_LIMIT_W<8> {
        MMMS_RX_PKT_LIMIT_W::new(self)
    }
    #[doc = "Bit 14 - MMMS CE expire control bit"]
    #[inline(always)]
    pub fn debug_ce_expire(&mut self) -> DEBUG_CE_EXPIRE_W<14> {
        DEBUG_CE_EXPIRE_W::new(self)
    }
    #[doc = "Bit 15 - MMMS empty PDU CE expire handling control bit"]
    #[inline(always)]
    pub fn mt_pdu_ce_expire(&mut self) -> MT_PDU_CE_EXPIRE_W<15> {
        MT_PDU_CE_EXPIRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection extended configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_config_ext](index.html) module"]
pub struct CONN_CONFIG_EXT_SPEC;
impl crate::RegisterSpec for CONN_CONFIG_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_config_ext::R](R) reader structure"]
impl crate::Readable for CONN_CONFIG_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_config_ext::W](W) writer structure"]
impl crate::Writable for CONN_CONFIG_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CONFIG_EXT to value 0xa000"]
impl crate::Resettable for CONN_CONFIG_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000
    }
}
