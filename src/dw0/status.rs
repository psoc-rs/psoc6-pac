#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P` reader - Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `NS` reader - Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `B` reader - Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `PC` reader - Active channel protection context."]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_IDX` reader - Active channel index."]
pub type CH_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` reader - Active channel priority."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PREEMPTABLE` reader - Active channel preemptable."]
pub type PREEMPTABLE_R = crate::BitReader<bool>;
#[doc = "Field `STATE` reader - State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': Update of active control information (e.g. source and destination addresses). '5': Wait for trigger de-activation."]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTIVE` reader - Active channel present: '0': No. '1': Yes."]
pub type ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Active channel protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active channel index."]
    #[inline(always)]
    pub fn ch_idx(&self) -> CH_IDX_R {
        CH_IDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Active channel priority."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Active channel preemptable."]
    #[inline(always)]
    pub fn preemptable(&self) -> PREEMPTABLE_R {
        PREEMPTABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': Update of active control information (e.g. source and destination addresses). '5': Wait for trigger de-activation."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 31 - Active channel present: '0': No. '1': Yes."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
