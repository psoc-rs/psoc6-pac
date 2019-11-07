#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the version number is 0x0001"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
