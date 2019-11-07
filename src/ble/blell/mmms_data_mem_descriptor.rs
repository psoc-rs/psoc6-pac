#[doc = "Reader of register MMMS_DATA_MEM_DESCRIPTOR[%s]"]
pub type R = crate::R<u32, super::MMMS_DATA_MEM_DESCRIPTOR>;
#[doc = "Writer for register MMMS_DATA_MEM_DESCRIPTOR[%s]"]
pub type W = crate::W<u32, super::MMMS_DATA_MEM_DESCRIPTOR>;
#[doc = "Register MMMS_DATA_MEM_DESCRIPTOR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MMMS_DATA_MEM_DESCRIPTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LLID_C1`"]
pub type LLID_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LLID_C1`"]
pub struct LLID_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> LLID_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DATA_LENGTH_C1`"]
pub type DATA_LENGTH_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_LENGTH_C1`"]
pub struct DATA_LENGTH_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LENGTH_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&self) -> LLID_C1_R {
        LLID_C1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\] are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&self) -> DATA_LENGTH_C1_R {
        DATA_LENGTH_C1_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&mut self) -> LLID_C1_W {
        LLID_C1_W { w: self }
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\] are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&mut self) -> DATA_LENGTH_C1_W {
        DATA_LENGTH_C1_W { w: self }
    }
}
