#[doc = "Register `FAST_CA_CMD` reader"]
pub struct R(crate::R<FAST_CA_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAST_CA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAST_CA_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAST_CA_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAST_CA_CMD` writer"]
pub struct W(crate::W<FAST_CA_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAST_CA_CMD_SPEC>;
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
impl From<crate::W<FAST_CA_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAST_CA_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - See SLOW_CA_CMD.INV."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - See SLOW_CA_CMD.INV."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FAST_CA_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See SLOW_CA_CMD.INV."]
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
#[doc = "Fast cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fast_ca_cmd](index.html) module"]
pub struct FAST_CA_CMD_SPEC;
impl crate::RegisterSpec for FAST_CA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fast_ca_cmd::R](R) reader structure"]
impl crate::Readable for FAST_CA_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fast_ca_cmd::W](W) writer structure"]
impl crate::Writable for FAST_CA_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAST_CA_CMD to value 0"]
impl crate::Resettable for FAST_CA_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
