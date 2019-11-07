#[doc = "Reader of register VDD_INTR_MASKED"]
pub type R = crate::R<u32, super::VDD_INTR_MASKED>;
#[doc = "Reader of field `VDDIO_ACTIVE`"]
pub type VDDIO_ACTIVE_R = crate::R<u16, u16>;
#[doc = "Reader of field `VDDA_ACTIVE`"]
pub type VDDA_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDD_ACTIVE`"]
pub type VDDD_ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Supply transistion detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
