#[doc = "Reader of register VDD_ACTIVE"]
pub type R = crate::R<u32, super::VDD_ACTIVE>;
#[doc = "Reader of field `VDDIO_ACTIVE`"]
pub type VDDIO_ACTIVE_R = crate::R<u16, u16>;
#[doc = "Reader of field `VDDA_ACTIVE`"]
pub type VDDA_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDD_ACTIVE`"]
pub type VDDD_ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Indicates presence or absence of VDDIO supplies (i.e. other than VDDD, VDDA) on the device (supplies are numbered 0..n-1). Note that VDDIO supplies have basic (crude) supply detectors only. If separate, robust, brown-out detection is desired on IO supplies, on-chip or off-chip analog resources need to provide it. For these bits to work reliable, the supply must be within valid spec range (per datasheet) or held at ground. Any in-between voltage has an undefined result. '0': Supply is not present '1': Supply is present When multiple VDDIO supplies are present, they will be assigned in alphanumeric ascending order to these bits during implementation. For example 'vddusb, vddio_0, vddio_a, vbackup, vddio_r, vddio_1' are present then they will be assigned to these bits as below: 0: vbackup, 1: vddio_0, 2: vddio_1, 3: vddio_a, 4: vddio_r, 5: vddusb'"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit indicates presence of the VDDD supply. This bit will always read-back 1. The VDDD supply has robust brown-out protection monitoring and it is not possible to read back this register without a valid supply. (This bit is used in certain test-modes to observe the brown-out detector status.)"]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
