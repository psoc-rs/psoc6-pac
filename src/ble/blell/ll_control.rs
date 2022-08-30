#[doc = "Register `LL_CONTROL` reader"]
pub struct R(crate::R<LL_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LL_CONTROL` writer"]
pub struct W(crate::W<LL_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LL_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LL_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV_1_2` reader - Enables Privacy 1.2 Feature."]
pub type PRIV_1_2_R = crate::BitReader<bool>;
#[doc = "Field `PRIV_1_2` writer - Enables Privacy 1.2 Feature."]
pub type PRIV_1_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `DLE` reader - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
pub type DLE_R = crate::BitReader<bool>;
#[doc = "Field `DLE` writer - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
pub type DLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `WL_READ_AS_MEM` reader - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
pub type WL_READ_AS_MEM_R = crate::BitReader<bool>;
#[doc = "Field `WL_READ_AS_MEM` writer - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
pub type WL_READ_AS_MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL` reader - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
pub type ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL` writer - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
pub type ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `HW_RSLV_LIST_FULL` reader - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
pub type HW_RSLV_LIST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `HW_RSLV_LIST_FULL` writer - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
pub type HW_RSLV_LIST_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV` reader - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R = crate::BitReader<bool>;
#[doc = "Field `RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV` writer - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV` reader - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R = crate::BitReader<bool>;
#[doc = "Field `RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV` writer - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN` reader - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R = crate::BitReader<bool>;
#[doc = "Field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN` writer - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI` reader - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R = crate::BitReader<bool>;
#[doc = "Field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI` writer - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI` reader - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R = crate::BitReader<bool>;
#[doc = "Field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI` writer - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PRIV_1_2_ADV` reader - Enables Privacy 1.2 for ADV engine"]
pub type PRIV_1_2_ADV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV_1_2_ADV` writer - Enables Privacy 1.2 for ADV engine"]
pub type PRIV_1_2_ADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PRIV_1_2_SCAN` reader - Enables Privacy 1.2 for SCAN engine"]
pub type PRIV_1_2_SCAN_R = crate::BitReader<bool>;
#[doc = "Field `PRIV_1_2_SCAN` writer - Enables Privacy 1.2 for SCAN engine"]
pub type PRIV_1_2_SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PRIV_1_2_INIT` reader - Enables Privacy 1.2 for INIT engine"]
pub type PRIV_1_2_INIT_R = crate::BitReader<bool>;
#[doc = "Field `PRIV_1_2_INIT` writer - Enables Privacy 1.2 for INIT engine"]
pub type PRIV_1_2_INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `EN_CONN_RX_EN_MOD` reader - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
pub type EN_CONN_RX_EN_MOD_R = crate::BitReader<bool>;
#[doc = "Field `EN_CONN_RX_EN_MOD` writer - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
pub type EN_CONN_RX_EN_MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `SLV_CONN_PEER_RPA_NOT_RSLVD` reader - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
pub type SLV_CONN_PEER_RPA_NOT_RSLVD_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CONN_PEER_RPA_NOT_RSLVD` writer - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
pub type SLV_CONN_PEER_RPA_NOT_RSLVD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
#[doc = "Field `ADVCH_FIFO_FLUSH` writer - When set, flushes the ADVCH FIFO. The bit is auto cleared. Note that this should be used only when the FIFO is not read by the firmware. If firmware has started reading the FIFO, then the FIFO must be emptied exclusively by firmware reads"]
pub type ADVCH_FIFO_FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables Privacy 1.2 Feature."]
    #[inline(always)]
    pub fn priv_1_2(&self) -> PRIV_1_2_R {
        PRIV_1_2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
    #[inline(always)]
    pub fn dle(&self) -> DLE_R {
        DLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
    #[inline(always)]
    pub fn wl_read_as_mem(&self) -> WL_READ_AS_MEM_R {
        WL_READ_AS_MEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
    #[inline(always)]
    pub fn advch_fifo_priv_1_2_flush_ctrl(&self) -> ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R {
        ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
    #[inline(always)]
    pub fn hw_rslv_list_full(&self) -> HW_RSLV_LIST_FULL_R {
        HW_RSLV_LIST_FULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_init_addr_match_priv_mismatch_adv(&self) -> RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R {
        RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_scan_addr_match_priv_mismatch_adv(&self) -> RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R {
        RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_scn(&self) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_ini(&self) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_ini(&self) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables Privacy 1.2 for ADV engine"]
    #[inline(always)]
    pub fn priv_1_2_adv(&self) -> PRIV_1_2_ADV_R {
        PRIV_1_2_ADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables Privacy 1.2 for SCAN engine"]
    #[inline(always)]
    pub fn priv_1_2_scan(&self) -> PRIV_1_2_SCAN_R {
        PRIV_1_2_SCAN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables Privacy 1.2 for INIT engine"]
    #[inline(always)]
    pub fn priv_1_2_init(&self) -> PRIV_1_2_INIT_R {
        PRIV_1_2_INIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
    #[inline(always)]
    pub fn en_conn_rx_en_mod(&self) -> EN_CONN_RX_EN_MOD_R {
        EN_CONN_RX_EN_MOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_not_rslvd(&self) -> SLV_CONN_PEER_RPA_NOT_RSLVD_R {
        SLV_CONN_PEER_RPA_NOT_RSLVD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables Privacy 1.2 Feature."]
    #[inline(always)]
    pub fn priv_1_2(&mut self) -> PRIV_1_2_W<0> {
        PRIV_1_2_W::new(self)
    }
    #[doc = "Bit 1 - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
    #[inline(always)]
    pub fn dle(&mut self) -> DLE_W<1> {
        DLE_W::new(self)
    }
    #[doc = "Bit 2 - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
    #[inline(always)]
    pub fn wl_read_as_mem(&mut self) -> WL_READ_AS_MEM_W<2> {
        WL_READ_AS_MEM_W::new(self)
    }
    #[doc = "Bit 3 - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
    #[inline(always)]
    pub fn advch_fifo_priv_1_2_flush_ctrl(&mut self) -> ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W<3> {
        ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W::new(self)
    }
    #[doc = "Bit 4 - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
    #[inline(always)]
    pub fn hw_rslv_list_full(&mut self) -> HW_RSLV_LIST_FULL_W<4> {
        HW_RSLV_LIST_FULL_W::new(self)
    }
    #[doc = "Bit 5 - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_init_addr_match_priv_mismatch_adv(
        &mut self,
    ) -> RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W<5> {
        RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W::new(self)
    }
    #[doc = "Bit 6 - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_scan_addr_match_priv_mismatch_adv(
        &mut self,
    ) -> RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W<6> {
        RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W::new(self)
    }
    #[doc = "Bit 7 - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_scn(
        &mut self,
    ) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W<7> {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W::new(self)
    }
    #[doc = "Bit 8 - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_ini(
        &mut self,
    ) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W<8> {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W::new(self)
    }
    #[doc = "Bit 9 - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_ini(
        &mut self,
    ) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W<9> {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W::new(self)
    }
    #[doc = "Bit 10 - Enables Privacy 1.2 for ADV engine"]
    #[inline(always)]
    pub fn priv_1_2_adv(&mut self) -> PRIV_1_2_ADV_W<10> {
        PRIV_1_2_ADV_W::new(self)
    }
    #[doc = "Bit 11 - Enables Privacy 1.2 for SCAN engine"]
    #[inline(always)]
    pub fn priv_1_2_scan(&mut self) -> PRIV_1_2_SCAN_W<11> {
        PRIV_1_2_SCAN_W::new(self)
    }
    #[doc = "Bit 12 - Enables Privacy 1.2 for INIT engine"]
    #[inline(always)]
    pub fn priv_1_2_init(&mut self) -> PRIV_1_2_INIT_W<12> {
        PRIV_1_2_INIT_W::new(self)
    }
    #[doc = "Bit 13 - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
    #[inline(always)]
    pub fn en_conn_rx_en_mod(&mut self) -> EN_CONN_RX_EN_MOD_W<13> {
        EN_CONN_RX_EN_MOD_W::new(self)
    }
    #[doc = "Bit 14 - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_not_rslvd(&mut self) -> SLV_CONN_PEER_RPA_NOT_RSLVD_W<14> {
        SLV_CONN_PEER_RPA_NOT_RSLVD_W::new(self)
    }
    #[doc = "Bit 15 - When set, flushes the ADVCH FIFO. The bit is auto cleared. Note that this should be used only when the FIFO is not read by the firmware. If firmware has started reading the FIFO, then the FIFO must be emptied exclusively by firmware reads"]
    #[inline(always)]
    pub fn advch_fifo_flush(&mut self) -> ADVCH_FIFO_FLUSH_W<15> {
        ADVCH_FIFO_FLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LL Backward compatibility\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_control](index.html) module"]
pub struct LL_CONTROL_SPEC;
impl crate::RegisterSpec for LL_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_control::R](R) reader structure"]
impl crate::Readable for LL_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ll_control::W](W) writer structure"]
impl crate::Writable for LL_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LL_CONTROL to value 0x02"]
impl crate::Resettable for LL_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
