#[doc = "Register `BIST_CTL` reader"]
pub struct R(crate::R<BIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CTL` writer"]
pub struct W(crate::W<BIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CTL_SPEC>;
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
impl From<crate::W<BIST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
pub type OPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE` writer - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
pub type OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIST_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `UP` reader - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
pub type UP_R = crate::BitReader<bool>;
#[doc = "Field `UP` writer - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
#[doc = "Field `ROW_FIRST` reader - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
pub type ROW_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `ROW_FIRST` writer - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
pub type ROW_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
#[doc = "Field `ADDR_START_ENABLED` reader - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
pub type ADDR_START_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_START_ENABLED` writer - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
pub type ADDR_START_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
#[doc = "Field `ADDR_COMPLIMENT_ENABLED` reader - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
pub type ADDR_COMPLIMENT_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_COMPLIMENT_ENABLED` writer - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
pub type ADDR_COMPLIMENT_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
#[doc = "Field `INCR_DECR_BOTH` reader - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
pub type INCR_DECR_BOTH_R = crate::BitReader<bool>;
#[doc = "Field `INCR_DECR_BOTH` writer - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
pub type INCR_DECR_BOTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
#[doc = "Field `STOP_ON_ERROR` reader - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
pub type STOP_ON_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `STOP_ON_ERROR` writer - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
pub type STOP_ON_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&self) -> ROW_FIRST_R {
        ROW_FIRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&self) -> ADDR_START_ENABLED_R {
        ADDR_START_ENABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&self) -> ADDR_COMPLIMENT_ENABLED_R {
        ADDR_COMPLIMENT_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&self) -> INCR_DECR_BOTH_R {
        INCR_DECR_BOTH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub fn stop_on_error(&self) -> STOP_ON_ERROR_R {
        STOP_ON_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field specifies how the data check should be performed after reading the data from Flash memory. 0: Read the Flash and compare the output to BIST_DATA (R0). 1: Read the Flash and compare the output to the binary complement of BIST_DATA (R1). 3: Read the Flash and compare with BIST_DATA\\[\\]
and compliment of BIST_DATA alternately (R01). The expected data of the first read is BIST_DATA, expected data of the second read is binary compliment of BIST_DATA, third read expected data is BIST_DATA, fourth read expected data is binary compliment of BIST_DATA and so on."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W<0> {
        OPCODE_W::new(self)
    }
    #[doc = "Bit 2 - Specifies direction in which Flash BIST steps through addresses: 0: BIST steps through the Flash from the maximum row and column addresses (as specified by a design time configuration parameter when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1') to the minimum row and column addresses. 1: BIST steps through the Flash from the minimum row and column addresses ('0' when ADDR_START_ENABLED is '0' and as specified by BIST_ADDR_START when ADDR_START_ENABLED is '1'' to the maximum row and column addresses."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<2> {
        UP_W::new(self)
    }
    #[doc = "Bit 3 - Specifies how the Flash BIST addresses are generated: '0': Column address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the row address incremented/decremented. '1': Row address is incremented/decremented till it reaches its maximum/minimum value. Once it reach its maximum/minimum value, it is set to its minimum/maximum value and only then is the column address incremented/decremented."]
    #[inline(always)]
    pub fn row_first(&mut self) -> ROW_FIRST_W<3> {
        ROW_FIRST_W::new(self)
    }
    #[doc = "Bit 4 - Specifies Flash BIST start addresses: '0': Row and column addresses start with their maximum/minimum values. '1': Row and column addresses start with their values as specified by BIST_ADDR_START. This feature is supported only for simple increment/decrement patterns. It is not supported with address compliment pattern (BIST_CTL.ADDR_COMPLIMENT_ENABLED) or address pattern which increments/decrements both row address and column address (BIST_CTL.INCR_DECR_BOTH) for every read."]
    #[inline(always)]
    pub fn addr_start_enabled(&mut self) -> ADDR_START_ENABLED_W<4> {
        ADDR_START_ENABLED_W::new(self)
    }
    #[doc = "Bit 5 - Specifies to generate address compliment patterns. 0: Generate normal increment/decrement patterns. 1: Generate address patterns which interleaves compliment of previous address in between. Example: The following is an example pattern, With UP=1 and ROW_FIRST =0 00_00 11_11 00_01 11_10 00_10 11_01 ..."]
    #[inline(always)]
    pub fn addr_compliment_enabled(&mut self) -> ADDR_COMPLIMENT_ENABLED_W<5> {
        ADDR_COMPLIMENT_ENABLED_W::new(self)
    }
    #[doc = "Bit 6 - Specifies to generate patterns where both column address and row address are incremented/decremented simultaneously. 0: Generate normal increment/decrement patterns. 1: Generate address patterns with both row and column address changing. Example: With UP = 1 and ROW_FIRST = 0 00_00 01_01 10_10 11_11 00_01 01_10 10_11 11_00 00_10 ..."]
    #[inline(always)]
    pub fn incr_decr_both(&mut self) -> INCR_DECR_BOTH_W<6> {
        INCR_DECR_BOTH_W::new(self)
    }
    #[doc = "Bit 7 - Specifies the BIST to continue indefinitely, regardless of occurrence of errors or not. 0: BIST controller doesn't stop on the data failures, it continues regardless of the errors. 1: BIST controller stops on when the first data failure is encountered."]
    #[inline(always)]
    pub fn stop_on_error(&mut self) -> STOP_ON_ERROR_W<7> {
        STOP_ON_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_ctl](index.html) module"]
pub struct BIST_CTL_SPEC;
impl crate::RegisterSpec for BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_ctl::R](R) reader structure"]
impl crate::Readable for BIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_ctl::W](W) writer structure"]
impl crate::Writable for BIST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CTL to value 0"]
impl crate::Resettable for BIST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
