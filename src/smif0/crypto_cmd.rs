#[doc = "Register `CRYPTO_CMD` reader"]
pub struct R(crate::R<CRYPTO_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_CMD` writer"]
pub struct W(crate::W<CRYPTO_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_CMD_SPEC>;
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
impl From<crate::W<CRYPTO_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_cmd](index.html) module"]
pub struct CRYPTO_CMD_SPEC;
impl crate::RegisterSpec for CRYPTO_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_cmd::R](R) reader structure"]
impl crate::Readable for CRYPTO_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_cmd::W](W) writer structure"]
impl crate::Writable for CRYPTO_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_CMD to value 0"]
impl crate::Resettable for CRYPTO_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
