#[doc = "Reader of register CE_CNFG_STS_REGISTER_EXT"]
pub type R = crate::R<u32, super::CE_CNFG_STS_REGISTER_EXT>;
#[doc = "Writer for register CE_CNFG_STS_REGISTER_EXT"]
pub type W = crate::W<u32, super::CE_CNFG_STS_REGISTER_EXT>;
#[doc = "Register CE_CNFG_STS_REGISTER_EXT `reset()`'s with value 0"]
impl crate::ResetValue for super::CE_CNFG_STS_REGISTER_EXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_2M`"]
pub type TX_2M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_2M`"]
pub struct TX_2M_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_2M_W<'a> {
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
#[doc = "Reader of field `RX_2M`"]
pub type RX_2M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_2M`"]
pub struct RX_2M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_2M_W<'a> {
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
#[doc = "Reader of field `SN`"]
pub type SN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SN`"]
pub struct SN_W<'a> {
    w: &'a mut W,
}
impl<'a> SN_W<'a> {
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
#[doc = "Reader of field `NESN`"]
pub type NESN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NESN`"]
pub struct NESN_W<'a> {
    w: &'a mut W,
}
impl<'a> NESN_W<'a> {
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
#[doc = "Reader of field `LAST_UNMAPPED_CHANNEL`"]
pub type LAST_UNMAPPED_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LAST_UNMAPPED_CHANNEL`"]
pub struct LAST_UNMAPPED_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_UNMAPPED_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&self) -> TX_2M_R {
        TX_2M_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&self) -> RX_2M_R {
        RX_2M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&self) -> SN_R {
        SN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&self) -> NESN_R {
        NESN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&self) -> LAST_UNMAPPED_CHANNEL_R {
        LAST_UNMAPPED_CHANNEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&mut self) -> TX_2M_W {
        TX_2M_W { w: self }
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&mut self) -> RX_2M_W {
        RX_2M_W { w: self }
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&mut self) -> SN_W {
        SN_W { w: self }
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&mut self) -> NESN_W {
        NESN_W { w: self }
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&mut self) -> LAST_UNMAPPED_CHANNEL_W {
        LAST_UNMAPPED_CHANNEL_W { w: self }
    }
}
