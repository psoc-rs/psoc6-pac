#[doc = "Reader of register MASK"]
pub type R = crate::R<u32, super::MASK>;
#[doc = "Writer for register MASK"]
pub type W = crate::W<u32, super::MASK>;
#[doc = "Register MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\] with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\] & MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\] = 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\] with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\] & MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\] = 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
