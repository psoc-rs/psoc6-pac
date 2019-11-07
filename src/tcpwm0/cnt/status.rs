#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, bool>;
#[doc = "Reader of field `GENERIC`"]
pub type GENERIC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
