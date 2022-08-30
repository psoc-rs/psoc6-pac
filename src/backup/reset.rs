#[doc = "Register `RESET` reader"]
pub struct R(crate::R<RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
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
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<31> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset::R](R) reader structure"]
impl crate::Readable for RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
