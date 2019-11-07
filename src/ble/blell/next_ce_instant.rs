#[doc = "Reader of register NEXT_CE_INSTANT"]
pub type R = crate::R<u32, super::NEXT_CE_INSTANT>;
#[doc = "Reader of field `NEXT_CE_INSTANT`"]
pub type NEXT_CE_INSTANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit internal reference clock value at which the next connection event will occur on a connection. The connection index register must be programmed with index of the connection, before reading the register. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
    #[inline(always)]
    pub fn next_ce_instant(&self) -> NEXT_CE_INSTANT_R {
        NEXT_CE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
