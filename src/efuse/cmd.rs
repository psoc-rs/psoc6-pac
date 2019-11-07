#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `BIT_DATA`"]
pub type BIT_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT_DATA`"]
pub struct BIT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_DATA_W<'a> {
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
#[doc = "Reader of field `BIT_ADDR`"]
pub type BIT_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIT_ADDR`"]
pub struct BIT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `BYTE_ADDR`"]
pub type BYTE_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_ADDR`"]
pub struct BYTE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MACRO_ADDR`"]
pub type MACRO_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MACRO_ADDR`"]
pub struct MACRO_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MACRO_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&self) -> BIT_DATA_R {
        BIT_DATA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&self) -> BIT_ADDR_R {
        BIT_ADDR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn byte_addr(&self) -> BYTE_ADDR_R {
        BYTE_ADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn macro_addr(&self) -> MACRO_ADDR_R {
        MACRO_ADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&mut self) -> BIT_DATA_W {
        BIT_DATA_W { w: self }
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&mut self) -> BIT_ADDR_W {
        BIT_ADDR_W { w: self }
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn byte_addr(&mut self) -> BYTE_ADDR_W {
        BYTE_ADDR_W { w: self }
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn macro_addr(&mut self) -> MACRO_ADDR_W {
        MACRO_ADDR_W { w: self }
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
