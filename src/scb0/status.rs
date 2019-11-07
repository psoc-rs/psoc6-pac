#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `EC_BUSY`"]
pub type EC_BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ mode). This bit can be used by SW to determine whether it is safe to issue a SW access to the EZ memory (without bus wait states (a blocked SW access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether a SW access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub fn ec_busy(&self) -> EC_BUSY_R {
        EC_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
