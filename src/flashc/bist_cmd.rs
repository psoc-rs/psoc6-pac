#[doc = "Register `BIST_CMD` reader"]
pub struct R(crate::R<BIST_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_CMD` writer"]
pub struct W(crate::W<BIST_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_CMD_SPEC>;
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
impl From<crate::W<BIST_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - 1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - 1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, BIST_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Start FLASH BIST. Hardware set this field to '0' when BIST is completed."]
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
#[doc = "BIST command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_cmd](index.html) module"]
pub struct BIST_CMD_SPEC;
impl crate::RegisterSpec for BIST_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_cmd::R](R) reader structure"]
impl crate::Readable for BIST_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_cmd::W](W) writer structure"]
impl crate::Writable for BIST_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_CMD to value 0"]
impl crate::Resettable for BIST_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
