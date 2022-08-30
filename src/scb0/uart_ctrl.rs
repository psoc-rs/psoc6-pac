#[doc = "Register `UART_CTRL` reader"]
pub struct R(crate::R<UART_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CTRL` writer"]
pub struct W(crate::W<UART_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CTRL_SPEC>;
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
impl From<crate::W<UART_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CTRL_SPEC, bool, O>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    UART_STD = 0,
    #[doc = "1: N/A"]
    UART_SMARTCARD = 1,
    #[doc = "2: N/A"]
    UART_IRDA = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::UART_STD),
            1 => Some(MODE_A::UART_SMARTCARD),
            2 => Some(MODE_A::UART_IRDA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UART_STD`"]
    #[inline(always)]
    pub fn is_uart_std(&self) -> bool {
        *self == MODE_A::UART_STD
    }
    #[doc = "Checks if the value of the field is `UART_SMARTCARD`"]
    #[inline(always)]
    pub fn is_uart_smartcard(&self) -> bool {
        *self == MODE_A::UART_SMARTCARD
    }
    #[doc = "Checks if the value of the field is `UART_IRDA`"]
    #[inline(always)]
    pub fn is_uart_irda(&self) -> bool {
        *self == MODE_A::UART_IRDA
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_CTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_std(self) -> &'a mut W {
        self.variant(MODE_A::UART_STD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_smartcard(self) -> &'a mut W {
        self.variant(MODE_A::UART_SMARTCARD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart_irda(self) -> &'a mut W {
        self.variant(MODE_A::UART_IRDA)
    }
}
impl R {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). 0: Loopback is not enabled 1: UART_TX is connected to UART_RX. UART_RTS is connected to UART_CTS. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<16> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ctrl](index.html) module"]
pub struct UART_CTRL_SPEC;
impl crate::RegisterSpec for UART_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ctrl::R](R) reader structure"]
impl crate::Readable for UART_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ctrl::W](W) writer structure"]
impl crate::Writable for UART_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CTRL to value 0x0300_0000"]
impl crate::Resettable for UART_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
