#[doc = "Reader of register ACQUIRE"]
pub type R = crate::R<u32, super::ACQUIRE>;
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUCCESS`"]
pub type SUCCESS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
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
        SUCCESS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
