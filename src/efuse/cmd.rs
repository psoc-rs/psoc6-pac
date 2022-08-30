#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIT_DATA` reader - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BIT_DATA_R = crate::BitReader<bool>;
#[doc = "Field `BIT_DATA` writer - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
pub type BIT_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BIT_ADDR` reader - Bit address. This field specifies a bit within a Byte."]
pub type BIT_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIT_ADDR` writer - Bit address. This field specifies a bit within a Byte."]
pub type BIT_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 3, O>;
#[doc = "Field `BYTE_ADDR` reader - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type BYTE_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE_ADDR` writer - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
pub type BYTE_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `MACRO_ADDR` reader - Macro address. This field specifies an eFUSE macro."]
pub type MACRO_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MACRO_ADDR` writer - Macro address. This field specifies an eFUSE macro."]
pub type MACRO_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 4, O>;
#[doc = "Field `START` reader - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&self) -> BIT_DATA_R {
        BIT_DATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&self) -> BIT_ADDR_R {
        BIT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
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
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit data. This field specifies the bit value that is to be programmed into the eFUSE macro array. The address of the bit is specified by the BIT_ADDR, BYTE_ADDR, and MACRO_ADDR fields. This bit is a don't care for the MXS40 Macro."]
    #[inline(always)]
    pub fn bit_data(&mut self) -> BIT_DATA_W<0> {
        BIT_DATA_W::new(self)
    }
    #[doc = "Bits 4:6 - Bit address. This field specifies a bit within a Byte."]
    #[inline(always)]
    pub fn bit_addr(&mut self) -> BIT_ADDR_W<4> {
        BIT_ADDR_W::new(self)
    }
    #[doc = "Bits 8:12 - Byte address. This field specifies a Byte within a eFUSE macro (each macro has 32 B)."]
    #[inline(always)]
    pub fn byte_addr(&mut self) -> BYTE_ADDR_W<8> {
        BYTE_ADDR_W::new(self)
    }
    #[doc = "Bits 16:19 - Macro address. This field specifies an eFUSE macro."]
    #[inline(always)]
    pub fn macro_addr(&mut self) -> MACRO_ADDR_W<16> {
        MACRO_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - FW sets this field to '1' to start a program operation. HW sets this field to '0' to indicate that the operation has completed. Note: it is good practice to verify the result of a program operation by reading back a programmed eFUSE memory location. Programming can only change an eFUSE memory bit from '0' to '1'; i.e. a programming operation is a 'one-off' operation for each eFUSE memory bit: once a bit is changed to '1', it can NEVER be changed back to '0' as a hardware fuse is blown. Programming a memory bit to '1' requires blowing a fuse and requires an eFUSE macro operation. Therefore, this programmiong operation takes time (as specified by the SEQ_PROGRAM_CTL reguisters). Programming amemory bit to '0' does not require an eFUSE macro operation (it is the default eFUSE macro state). Therefore, this programming operation is almost instantaneous. Note: during a program operation, a read operation can not be performed. An AHB-Lite read transfer to the eFUSE memory during a program operation results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<31> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0x01"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
