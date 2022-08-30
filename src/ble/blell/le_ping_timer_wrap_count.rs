#[doc = "Register `LE_PING_TIMER_WRAP_COUNT` reader"]
pub struct R(crate::R<LE_PING_TIMER_WRAP_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_PING_TIMER_WRAP_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_PING_TIMER_WRAP_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_PING_TIMER_WRAP_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONN_SEC_CURRENT_WRAP` reader - This register holds the current position of the Ping timer."]
pub type CONN_SEC_CURRENT_WRAP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register holds the current position of the Ping timer."]
    #[inline(always)]
    pub fn conn_sec_current_wrap(&self) -> CONN_SEC_CURRENT_WRAP_R {
        CONN_SEC_CURRENT_WRAP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LE Ping Timer wrap count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_wrap_count](index.html) module"]
pub struct LE_PING_TIMER_WRAP_COUNT_SPEC;
impl crate::RegisterSpec for LE_PING_TIMER_WRAP_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le_ping_timer_wrap_count::R](R) reader structure"]
impl crate::Readable for LE_PING_TIMER_WRAP_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LE_PING_TIMER_WRAP_COUNT to value 0"]
impl crate::Resettable for LE_PING_TIMER_WRAP_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
