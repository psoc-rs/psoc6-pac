#[doc = "Register `CM4_STATUS` reader"]
pub struct R(crate::R<CM4_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLEEPING` reader - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
pub type SLEEPING_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEP` reader - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
pub type SLEEPDEEP_R = crate::BitReader<bool>;
#[doc = "Field `PWR_DONE` reader - After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
pub type PWR_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub fn sleeping(&self) -> SLEEPING_R {
        SLEEPING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
    #[inline(always)]
    pub fn pwr_done(&self) -> PWR_DONE_R {
        PWR_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CM4 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_status](index.html) module"]
pub struct CM4_STATUS_SPEC;
impl crate::RegisterSpec for CM4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_status::R](R) reader structure"]
impl crate::Readable for CM4_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_STATUS to value 0x13"]
impl crate::Resettable for CM4_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13
    }
}
