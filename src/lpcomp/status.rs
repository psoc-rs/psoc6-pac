#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `OUT0`"]
pub type OUT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUT1`"]
pub type OUT1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Current output value of the comparator 0."]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Current output value of the comparator 1."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
