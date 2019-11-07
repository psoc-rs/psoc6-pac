#[doc = "Reader of register IDENTITY"]
pub type R = crate::R<u32, super::IDENTITY>;
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context of the transfer that reads the register."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier of the transfer that reads the register."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
