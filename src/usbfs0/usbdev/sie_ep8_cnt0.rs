#[doc = "Register `SIE_EP8_CNT0` reader"]
pub struct R(crate::R<SIE_EP8_CNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_EP8_CNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_EP8_CNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_EP8_CNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_EP8_CNT0` writer"]
pub struct W(crate::W<SIE_EP8_CNT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_EP8_CNT0_SPEC>;
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
impl From<crate::W<SIE_EP8_CNT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_EP8_CNT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_COUNT_MSB` reader - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DATA_COUNT_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_COUNT_MSB` writer - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DATA_COUNT_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIE_EP8_CNT0_SPEC, u8, u8, 3, O>;
#[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
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
#[doc = "Field `DATA_VALID` reader - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
#[doc = "Field `DATA_VALID` writer - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DATA_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SIE_EP8_CNT0_SPEC, DATA_VALID_A, O>;
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
pub type DATA_TOGGLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP8_CNT0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn data_count_msb(&self) -> DATA_COUNT_MSB_R {
        DATA_COUNT_MSB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn data_count_msb(&mut self) -> DATA_COUNT_MSB_W<0> {
        DATA_COUNT_MSB_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
#[doc = "Non-control endpoint count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep8_cnt0](index.html) module"]
pub struct SIE_EP8_CNT0_SPEC;
impl crate::RegisterSpec for SIE_EP8_CNT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ep8_cnt0::R](R) reader structure"]
impl crate::Readable for SIE_EP8_CNT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ep8_cnt0::W](W) writer structure"]
impl crate::Writable for SIE_EP8_CNT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_EP8_CNT0 to value 0"]
impl crate::Resettable for SIE_EP8_CNT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
