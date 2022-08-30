#[doc = "Register `LPM_CTL` reader"]
pub struct R(crate::R<LPM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPM_CTL` writer"]
pub struct W(crate::W<LPM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPM_CTL_SPEC>;
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
impl From<crate::W<LPM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPM_EN` reader - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LPM_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPM_EN` writer - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LPM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPM_CTL_SPEC, bool, O>;
#[doc = "Field `LPM_ACK_RESP` reader - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LPM_ACK_RESP_R = crate::BitReader<bool>;
#[doc = "Field `LPM_ACK_RESP` writer - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LPM_ACK_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPM_CTL_SPEC, bool, O>;
#[doc = "Field `NYET_EN` reader - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NYET_EN_R = crate::BitReader<bool>;
#[doc = "Field `NYET_EN` writer - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NYET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPM_CTL_SPEC, bool, O>;
#[doc = "Field `SUB_RESP` reader - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SUB_RESP_R = crate::BitReader<bool>;
#[doc = "Field `SUB_RESP` writer - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SUB_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPM_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&self) -> LPM_ACK_RESP_R {
        LPM_ACK_RESP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&self) -> NYET_EN_R {
        NYET_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&self) -> SUB_RESP_R {
        SUB_RESP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LPM_EN_W<0> {
        LPM_EN_W::new(self)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&mut self) -> LPM_ACK_RESP_W<1> {
        LPM_ACK_RESP_W::new(self)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&mut self) -> NYET_EN_W<2> {
        NYET_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&mut self) -> SUB_RESP_W<4> {
        SUB_RESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm_ctl](index.html) module"]
pub struct LPM_CTL_SPEC;
impl crate::RegisterSpec for LPM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpm_ctl::R](R) reader structure"]
impl crate::Readable for LPM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpm_ctl::W](W) writer structure"]
impl crate::Writable for LPM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPM_CTL to value 0"]
impl crate::Resettable for LPM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
