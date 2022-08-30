#[doc = "Register `ACQUIRE` reader"]
pub struct R(crate::R<ACQUIRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACQUIRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACQUIRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACQUIRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `NS` reader - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `PC` reader - This field specifies the protection context that successfully acquired the lock."]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MS` reader - This field specifies the bus master identifier that successfully acquired the lock."]
pub type MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUCCESS` reader - Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
pub type SUCCESS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "IPC acquire\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acquire](index.html) module"]
pub struct ACQUIRE_SPEC;
impl crate::RegisterSpec for ACQUIRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acquire::R](R) reader structure"]
impl crate::Readable for ACQUIRE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACQUIRE to value 0"]
impl crate::Resettable for ACQUIRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
