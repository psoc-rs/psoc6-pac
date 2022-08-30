#[doc = "Register `LE_PING_TIMER_NEXT_EXP` reader"]
pub struct R(crate::R<LE_PING_TIMER_NEXT_EXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_PING_TIMER_NEXT_EXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_PING_TIMER_NEXT_EXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_PING_TIMER_NEXT_EXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONN_PING_TIMER_NEXT_EXP` reader - The value of ping timer next expiry instant in the terms of native clock value (least 16 bit value of the 17 bit ping counter). This together with CONN_PING_TIMER_NEXT_EXP_WRAP will provide the correct status of ping timer duration."]
pub type CONN_PING_TIMER_NEXT_EXP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The value of ping timer next expiry instant in the terms of native clock value (least 16 bit value of the 17 bit ping counter). This together with CONN_PING_TIMER_NEXT_EXP_WRAP will provide the correct status of ping timer duration."]
    #[inline(always)]
    pub fn conn_ping_timer_next_exp(&self) -> CONN_PING_TIMER_NEXT_EXP_R {
        CONN_PING_TIMER_NEXT_EXP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LE Ping timer next expiry instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_next_exp](index.html) module"]
pub struct LE_PING_TIMER_NEXT_EXP_SPEC;
impl crate::RegisterSpec for LE_PING_TIMER_NEXT_EXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le_ping_timer_next_exp::R](R) reader structure"]
impl crate::Readable for LE_PING_TIMER_NEXT_EXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LE_PING_TIMER_NEXT_EXP to value 0"]
impl crate::Resettable for LE_PING_TIMER_NEXT_EXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
