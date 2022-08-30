#[doc = "Register `UART_TX_CTRL` reader"]
pub struct R(crate::R<UART_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_TX_CTRL` writer"]
pub struct W(crate::W<UART_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_TX_CTRL_SPEC>;
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
impl From<crate::W<UART_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type STOP_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BITS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type STOP_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_TX_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PARITY` reader - N/A"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - N/A"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CTRL_SPEC, bool, O>;
#[doc = "Field `PARITY_ENABLED` reader - N/A"]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - N/A"]
pub type PARITY_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CTRL_SPEC, bool, O>;
#[doc = "Field `RETRY_ON_NACK` reader - N/A"]
pub type RETRY_ON_NACK_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_ON_NACK` writer - N/A"]
pub type RETRY_ON_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_TX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn retry_on_nack(&self) -> RETRY_ON_NACK_R {
        RETRY_ON_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<0> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<4> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W<5> {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn retry_on_nack(&mut self) -> RETRY_ON_NACK_W<8> {
        RETRY_ON_NACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tx_ctrl](index.html) module"]
pub struct UART_TX_CTRL_SPEC;
impl crate::RegisterSpec for UART_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_tx_ctrl::R](R) reader structure"]
impl crate::Readable for UART_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_tx_ctrl::W](W) writer structure"]
impl crate::Writable for UART_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_TX_CTRL to value 0x02"]
impl crate::Resettable for UART_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
