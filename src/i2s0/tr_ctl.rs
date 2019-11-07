#[doc = "Reader of register TR_CTL"]
pub type R = crate::R<u32, super::TR_CTL>;
#[doc = "Writer for register TR_CTL"]
pub type W = crate::W<u32, super::TR_CTL>;
#[doc = "Register TR_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_REQ_EN`"]
pub type TX_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_REQ_EN`"]
pub struct TX_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_EN_W<'a> {
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
#[doc = "Reader of field `RX_REQ_EN`"]
pub type RX_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_REQ_EN`"]
pub struct RX_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_REQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_req_en(&self) -> TX_REQ_EN_R {
        TX_REQ_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&self) -> RX_REQ_EN_R {
        RX_REQ_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output ('tr_i2s_tx_req') enable for requests of DMA transfer in transmission '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_req_en(&mut self) -> TX_REQ_EN_W {
        TX_REQ_EN_W { w: self }
    }
    #[doc = "Bit 16 - Trigger output ('tr_i2s_rx_req') enable for requests of DMA transfer in reception '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&mut self) -> RX_REQ_EN_W {
        RX_REQ_EN_W { w: self }
    }
}
