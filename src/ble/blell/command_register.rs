#[doc = "Register `COMMAND_REGISTER` writer"]
pub struct W(crate::W<COMMAND_REGISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND_REGISTER_SPEC>;
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
impl From<crate::W<COMMAND_REGISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND_REGISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND` writer - N/A"]
pub type COMMAND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMAND_REGISTER_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W<0> {
        COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command_register](index.html) module"]
pub struct COMMAND_REGISTER_SPEC;
impl crate::RegisterSpec for COMMAND_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [command_register::W](W) writer structure"]
impl crate::Writable for COMMAND_REGISTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMMAND_REGISTER to value 0"]
impl crate::Resettable for COMMAND_REGISTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
