#[doc = "Reader of register ARB_EP2_CFG"]
pub type R = crate::R<u32, super::ARB_EP2_CFG>;
#[doc = "Writer for register ARB_EP2_CFG"]
pub type W = crate::W<u32, super::ARB_EP2_CFG>;
#[doc = "Register ARB_EP2_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_EP2_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_DATA_RDY`"]
pub type IN_DATA_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DATA_RDY`"]
pub struct IN_DATA_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DATA_RDY_W<'a> {
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
#[doc = "Reader of field `DMA_REQ`"]
pub type DMA_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_REQ`"]
pub struct DMA_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_W<'a> {
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
#[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_BYPASS_A {
    #[doc = "0: No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "1: CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 1,
}
impl From<CRC_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC_BYPASS`"]
pub type CRC_BYPASS_R = crate::R<bool, CRC_BYPASS_A>;
impl CRC_BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_BYPASS_A {
        match self.bits {
            false => CRC_BYPASS_A::CRC_NORMAL,
            true => CRC_BYPASS_A::CRC_BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_NORMAL`"]
    #[inline(always)]
    pub fn is_crc_normal(&self) -> bool {
        *self == CRC_BYPASS_A::CRC_NORMAL
    }
    #[doc = "Checks if the value of the field is `CRC_BYPASS`"]
    #[inline(always)]
    pub fn is_crc_bypass(&self) -> bool {
        *self == CRC_BYPASS_A::CRC_BYPASS
    }
}
#[doc = "Write proxy for field `CRC_BYPASS`"]
pub struct CRC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn crc_normal(self) -> &'a mut W {
        self.variant(CRC_BYPASS_A::CRC_NORMAL)
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn crc_bypass(self) -> &'a mut W {
        self.variant(CRC_BYPASS_A::CRC_BYPASS)
    }
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
#[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_PTR_A {
    #[doc = "0: Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "1: Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 1,
}
impl From<RESET_PTR_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_PTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESET_PTR`"]
pub type RESET_PTR_R = crate::R<bool, RESET_PTR_A>;
impl RESET_PTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_PTR_A {
        match self.bits {
            false => RESET_PTR_A::RESET_KRYPTON,
            true => RESET_PTR_A::RESET_NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_KRYPTON`"]
    #[inline(always)]
    pub fn is_reset_krypton(&self) -> bool {
        *self == RESET_PTR_A::RESET_KRYPTON
    }
    #[doc = "Checks if the value of the field is `RESET_NORMAL`"]
    #[inline(always)]
    pub fn is_reset_normal(&self) -> bool {
        *self == RESET_PTR_A::RESET_NORMAL
    }
}
#[doc = "Write proxy for field `RESET_PTR`"]
pub struct RESET_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_PTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_PTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn reset_krypton(self) -> &'a mut W {
        self.variant(RESET_PTR_A::RESET_KRYPTON)
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn reset_normal(self) -> &'a mut W {
        self.variant(RESET_PTR_A::RESET_NORMAL)
    }
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
impl R {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&self) -> IN_DATA_RDY_R {
        IN_DATA_RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&self) -> CRC_BYPASS_R {
        CRC_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&self) -> RESET_PTR_R {
        RESET_PTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&mut self) -> IN_DATA_RDY_W {
        IN_DATA_RDY_W { w: self }
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&mut self) -> DMA_REQ_W {
        DMA_REQ_W { w: self }
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&mut self) -> CRC_BYPASS_W {
        CRC_BYPASS_W { w: self }
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&mut self) -> RESET_PTR_W {
        RESET_PTR_W { w: self }
    }
}
