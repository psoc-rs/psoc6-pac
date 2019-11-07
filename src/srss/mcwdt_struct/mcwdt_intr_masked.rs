#[doc = "Reader of register MCWDT_INTR_MASKED"]
pub type R = crate::R<u32, super::MCWDT_INTR_MASKED>;
#[doc = "Reader of field `MCWDT_INT0`"]
pub type MCWDT_INT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCWDT_INT1`"]
pub type MCWDT_INT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCWDT_INT2`"]
pub type MCWDT_INT2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
