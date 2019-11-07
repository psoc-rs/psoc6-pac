#[doc = "Reader of register DU_CTL"]
pub type R = crate::R<u32, super::DU_CTL>;
#[doc = "Writer for register DU_CTL"]
pub type W = crate::W<u32, super::DU_CTL>;
#[doc = "Register DU_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DU_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DU_SIZE`"]
pub type DU_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_SIZE`"]
pub struct DU_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DU_OPC`"]
pub type DU_OPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_OPC`"]
pub struct DU_OPC_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_OPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&self) -> DU_SIZE_R {
        DU_SIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&self) -> DU_OPC_R {
        DU_OPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&mut self) -> DU_SIZE_W {
        DU_SIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&mut self) -> DU_OPC_W {
        DU_OPC_W { w: self }
    }
}
