#[doc = "Reader of register MMMS_CONN_STATUS"]
pub type R = crate::R<u32, super::MMMS_CONN_STATUS>;
#[doc = "Reader of field `CURR_CONN_INDEX`"]
pub type CURR_CONN_INDEX_R = crate::R<u8, u8>;
#[doc = "Reader of field `CURR_CONN_TYPE`"]
pub type CURR_CONN_TYPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SN_CURR`"]
pub type SN_CURR_R = crate::R<bool, bool>;
#[doc = "Reader of field `NESN_CURR`"]
pub type NESN_CURR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LAST_UNMAPPED_CHANNEL`"]
pub type LAST_UNMAPPED_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKT_MISS`"]
pub type PKT_MISS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANCHOR_PT_STATE`"]
pub type ANCHOR_PT_STATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Connection Index that was serviced. Legal values are 0,1,2,3."]
    #[inline(always)]
    pub fn curr_conn_index(&self) -> CURR_CONN_INDEX_R {
        CURR_CONN_INDEX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn curr_conn_type(&self) -> CURR_CONN_TYPE_R {
        CURR_CONN_TYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequence Number of Packets exchanged"]
    #[inline(always)]
    pub fn sn_curr(&self) -> SN_CURR_R {
        SN_CURR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Next Sequence Number"]
    #[inline(always)]
    pub fn nesn_curr(&self) -> NESN_CURR_R {
        NESN_CURR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Last Unmapped Channel"]
    #[inline(always)]
    pub fn last_unmapped_channel(&self) -> LAST_UNMAPPED_CHANNEL_R {
        LAST_UNMAPPED_CHANNEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 1 - Packet Missed 0 - Connection exchanged packets"]
    #[inline(always)]
    pub fn pkt_miss(&self) -> PKT_MISS_R {
        PKT_MISS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Anchor Point State 0 - Anchor point missed 1 - Anchor point established"]
    #[inline(always)]
    pub fn anchor_pt_state(&self) -> ANCHOR_PT_STATE_R {
        ANCHOR_PT_STATE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
