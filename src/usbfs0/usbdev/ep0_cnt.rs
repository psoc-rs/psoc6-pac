#[doc = "Register `EP0_CNT` reader"]
pub struct R(crate::R<EP0_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP0_CNT` writer"]
pub struct W(crate::W<EP0_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0_CNT_SPEC>;
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
impl From<crate::W<EP0_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE_COUNT` reader - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
pub type BYTE_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE_COUNT` writer - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
pub type BYTE_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0_CNT_SPEC, u8, u8, 4, O>;
#[doc = "This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_VALID_A {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    DATA_ERROR = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    DATA_VALID = 1,
}
impl From<DATA_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_VALID` reader - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DATA_VALID_R = crate::BitReader<DATA_VALID_A>;
impl DATA_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_VALID_A {
        match self.bits {
            false => DATA_VALID_A::DATA_ERROR,
            true => DATA_VALID_A::DATA_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_ERROR`"]
    #[inline(always)]
    pub fn is_data_error(&self) -> bool {
        *self == DATA_VALID_A::DATA_ERROR
    }
    #[doc = "Checks if the value of the field is `DATA_VALID`"]
    #[inline(always)]
    pub fn is_data_valid(&self) -> bool {
        *self == DATA_VALID_A::DATA_VALID
    }
}
#[doc = "Field `DATA_VALID` writer - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DATA_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CNT_SPEC, DATA_VALID_A, O>;
impl<'a, const O: u8> DATA_VALID_W<'a, O> {
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn data_error(self) -> &'a mut W {
        self.variant(DATA_VALID_A::DATA_ERROR)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn data_valid(self) -> &'a mut W {
        self.variant(DATA_VALID_A::DATA_VALID)
    }
}
#[doc = "Field `DATA_TOGGLE` reader - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DATA_TOGGLE_R = crate::BitReader<bool>;
#[doc = "Field `DATA_TOGGLE` writer - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DATA_TOGGLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CNT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&self) -> DATA_VALID_R {
        DATA_VALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&self) -> DATA_TOGGLE_R {
        DATA_TOGGLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W<0> {
        BYTE_COUNT_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&mut self) -> DATA_VALID_W<6> {
        DATA_VALID_W::new(self)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&mut self) -> DATA_TOGGLE_W<7> {
        DATA_TOGGLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint0 count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_cnt](index.html) module"]
pub struct EP0_CNT_SPEC;
impl crate::RegisterSpec for EP0_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep0_cnt::R](R) reader structure"]
impl crate::Readable for EP0_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep0_cnt::W](W) writer structure"]
impl crate::Writable for EP0_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP0_CNT to value 0"]
impl crate::Resettable for EP0_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
