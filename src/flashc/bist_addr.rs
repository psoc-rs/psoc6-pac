#[doc = "Reader of register BIST_ADDR"]
pub type R = crate::R<u32, super::BIST_ADDR>;
#[doc = "Reader of field `COL_ADDR`"]
pub type COL_ADDR_R = crate::R<u16, u16>;
#[doc = "Reader of field `ROW_ADDR`"]
pub type ROW_ADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current column address."]
    #[inline(always)]
    pub fn col_addr(&self) -> COL_ADDR_R {
        COL_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current row address."]
    #[inline(always)]
    pub fn row_addr(&self) -> ROW_ADDR_R {
        ROW_ADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
