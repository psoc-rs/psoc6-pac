#[doc = "Reader of register HOST_EP1_STATUS"]
pub type R = crate::R<u32, super::HOST_EP1_STATUS>;
#[doc = "Reader of field `SIZE1`"]
pub type SIZE1_R = crate::R<u16, u16>;
#[doc = "Reader of field `VAL_DATA`"]
pub type VAL_DATA_R = crate::R<bool, bool>;
#[doc = "Reader of field `INI_ST`"]
pub type INI_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD_18`"]
pub type RSVD_18_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP1 has finished. The indication range is from 0x000 to 0x100. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn size1(&self) -> SIZE1_R {
        SIZE1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This bit shows that there is valid data in the EP1 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn val_data(&self) -> VAL_DATA_R {
        VAL_DATA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit shows that EP1 is initialized. If the init bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' and EP1 is initialized, this bit is to '1'. '0' : Release of the initialization '1' : Initialization Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '0' or '1'."]
    #[inline(always)]
    pub fn ini_st(&self) -> INI_ST_R {
        INI_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> RSVD_18_R {
        RSVD_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
