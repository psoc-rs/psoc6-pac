#[doc = "Reader of register GEOMETRY_GEN"]
pub type R = crate::R<u32, super::GEOMETRY_GEN>;
#[doc = "Reader of field `DNU_0X20_1`"]
pub type DNU_0X20_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DNU_0X20_2`"]
pub type DNU_0X20_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DNU_0X20_3`"]
pub type DNU_0X20_3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_1(&self) -> DNU_0X20_1_R {
        DNU_0X20_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_2(&self) -> DNU_0X20_2_R {
        DNU_0X20_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_3(&self) -> DNU_0X20_3_R {
        DNU_0X20_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
