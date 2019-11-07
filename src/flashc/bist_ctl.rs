#[doc = "Reader of register BIST_CTL"]
pub type R = crate::R<u32, super::BIST_CTL>;
#[doc = "Writer for register BIST_CTL"]
pub type W = crate::W<u32, super::BIST_CTL>;
#[doc = "Register BIST_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::BIST_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE`"]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UP`"]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
#[doc = "Reader of field `ROW_FIRST`"]
pub type ROW_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROW_FIRST`"]
pub struct ROW_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_FIRST_W<'a> {
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
#[doc = "Reader of field `ADDR_START_ENABLED`"]
pub type ADDR_START_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR_START_ENABLED`"]
pub struct ADDR_START_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_START_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADDR_COMPLIMENT_ENABLED`"]
pub type ADDR_COMPLIMENT_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR_COMPLIMENT_ENABLED`"]
pub struct ADDR_COMPLIMENT_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_COMPLIMENT_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INCR_DECR_BOTH`"]
pub type INCR_DECR_BOTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCR_DECR_BOTH`"]
pub struct INCR_DECR_BOTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INCR_DECR_BOTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `STOP_ON_ERROR`"]
pub type STOP_ON_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_ON_ERROR`"]
pub struct STOP_ON_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_ON_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. '0': Read the Flash and compare the output to BIST_DATA (R0). '1': Read the Flash and compare the output to the binary complement of BIST_DATA (R1). '2': Read the Flash and compare with BIST_DATA\\[\\] and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: ''0': BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configurtion parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. '1': BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&self) -> ROW_FIRST_R {
        ROW_FIRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&self) -> ADDR_START_ENABLED_R {
        ADDR_START_ENABLED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. '0': Generate normal increment/decrement patterns. '1': Generate address patterns which interleaves compliment of previous address in between. Example: The following is an exaple pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&self) -> ADDR_COMPLIMENT_ENABLED_R {
        ADDR_COMPLIMENT_ENABLED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. '0': Generate normal increment/decrement patterns. '1': Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&self) -> INCR_DECR_BOTH_R {
        INCR_DECR_BOTH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. '0': BIST controller doesn't stop on the data failures, it continues regardless of the errors. '1': BIST controller stops on when the first data failure is encounted."]
    #[inline(always)]
    pub fn stop_on_error(&self) -> STOP_ON_ERROR_R {
        STOP_ON_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. '0': Read the Flash and compare the output to BIST_DATA (R0). '1': Read the Flash and compare the output to the binary complement of BIST_DATA (R1). '2': Read the Flash and compare with BIST_DATA\\[\\] and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: ''0': BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configurtion parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. '1': BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its mimimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&mut self) -> ROW_FIRST_W {
        ROW_FIRST_W { w: self }
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&mut self) -> ADDR_START_ENABLED_W {
        ADDR_START_ENABLED_W { w: self }
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. '0': Generate normal increment/decrement patterns. '1': Generate address patterns which interleaves compliment of previous address in between. Example: The following is an exaple pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&mut self) -> ADDR_COMPLIMENT_ENABLED_W {
        ADDR_COMPLIMENT_ENABLED_W { w: self }
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. '0': Generate normal increment/decrement patterns. '1': Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&mut self) -> INCR_DECR_BOTH_W {
        INCR_DECR_BOTH_W { w: self }
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. '0': BIST controller doesn't stop on the data failures, it continues regardless of the errors. '1': BIST controller stops on when the first data failure is encounted."]
    #[inline(always)]
    pub fn stop_on_error(&mut self) -> STOP_ON_ERROR_W {
        STOP_ON_ERROR_W { w: self }
    }
}
