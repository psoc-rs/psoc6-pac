#[doc = "Reader of register FM_MEM_DATA[%s]"]
pub type R = crate::R<u32, super::FM_MEM_DATA>;
#[doc = "Reader of field `DATA32`"]
pub type DATA32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sense amplifier and cloumn multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is '0': data as specified by the R interface address - IF_SEL is '1': data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
