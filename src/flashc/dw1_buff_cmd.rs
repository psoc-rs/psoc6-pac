#[doc = "Register `DW1_BUFF_CMD` reader"]
pub struct R(crate::R<DW1_BUFF_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DW1_BUFF_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DW1_BUFF_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DW1_BUFF_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DW1_BUFF_CMD` writer"]
pub struct W(crate::W<DW1_BUFF_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DW1_BUFF_CMD_SPEC>;
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
impl From<crate::W<DW1_BUFF_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DW1_BUFF_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - See CRYPTO_BUFF_CMD."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - See CRYPTO_BUFF_CMD."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DW1_BUFF_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See CRYPTO_BUFF_CMD."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CRYPTO_BUFF_CMD."]
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
#[doc = "Datawire 1 buffer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dw1_buff_cmd](index.html) module"]
pub struct DW1_BUFF_CMD_SPEC;
impl crate::RegisterSpec for DW1_BUFF_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dw1_buff_cmd::R](R) reader structure"]
impl crate::Readable for DW1_BUFF_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dw1_buff_cmd::W](W) writer structure"]
impl crate::Writable for DW1_BUFF_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DW1_BUFF_CMD to value 0"]
impl crate::Resettable for DW1_BUFF_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
