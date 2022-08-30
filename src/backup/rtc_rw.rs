#[doc = "Register `RTC_RW` reader"]
pub struct R(crate::R<RTC_RW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_RW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_RW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_RW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_RW` writer"]
pub struct W(crate::W<RTC_RW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_RW_SPEC>;
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
impl From<crate::W<RTC_RW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_RW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
pub type READ_R = crate::BitReader<bool>;
#[doc = "Field `READ` writer - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_RW_SPEC, bool, O>;
#[doc = "Field `WRITE` reader - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_RW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<0> {
        READ_W::new(self)
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Read Write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_rw](index.html) module"]
pub struct RTC_RW_SPEC;
impl crate::RegisterSpec for RTC_RW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_rw::R](R) reader structure"]
impl crate::Readable for RTC_RW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_rw::W](W) writer structure"]
impl crate::Writable for RTC_RW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_RW to value 0"]
impl crate::Resettable for RTC_RW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
