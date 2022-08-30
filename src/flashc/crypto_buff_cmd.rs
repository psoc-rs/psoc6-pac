#[doc = "Register `CRYPTO_BUFF_CMD` reader"]
pub struct R(crate::R<CRYPTO_BUFF_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_BUFF_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_BUFF_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_BUFF_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_BUFF_CMD` writer"]
pub struct W(crate::W<CRYPTO_BUFF_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_BUFF_CMD_SPEC>;
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
impl From<crate::W<CRYPTO_BUFF_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_BUFF_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYPTO_BUFF_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH buffer invalidation. SW writes a '1' to clear the buffer. HW sets this field to '0' when the operation is completed."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<0> {
        INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_buff_cmd](index.html) module"]
pub struct CRYPTO_BUFF_CMD_SPEC;
impl crate::RegisterSpec for CRYPTO_BUFF_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_buff_cmd::R](R) reader structure"]
impl crate::Readable for CRYPTO_BUFF_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_buff_cmd::W](W) writer structure"]
impl crate::Writable for CRYPTO_BUFF_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_BUFF_CMD to value 0"]
impl crate::Resettable for CRYPTO_BUFF_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
