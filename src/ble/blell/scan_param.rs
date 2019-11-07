#[doc = "Reader of register SCAN_PARAM"]
pub type R = crate::R<u32, super::SCAN_PARAM>;
#[doc = "Writer for register SCAN_PARAM"]
pub type W = crate::W<u32, super::SCAN_PARAM>;
#[doc = "Register SCAN_PARAM `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAN_PARAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_ADDR`"]
pub type TX_ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ADDR`"]
pub struct TX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ADDR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SCAN_TYPE`"]
pub type SCAN_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCAN_TYPE`"]
pub struct SCAN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `SCAN_FILT_POLICY`"]
pub type SCAN_FILT_POLICY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCAN_FILT_POLICY`"]
pub struct SCAN_FILT_POLICY_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_FILT_POLICY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DUP_FILT_EN`"]
pub type DUP_FILT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUP_FILT_EN`"]
pub struct DUP_FILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUP_FILT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DUP_FILT_CHK_ADV_DIR`"]
pub type DUP_FILT_CHK_ADV_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUP_FILT_CHK_ADV_DIR`"]
pub struct DUP_FILT_CHK_ADV_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUP_FILT_CHK_ADV_DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SCAN_RSP_ADVA_CHECK`"]
pub type SCAN_RSP_ADVA_CHECK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_RSP_ADVA_CHECK`"]
pub struct SCAN_RSP_ADVA_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RSP_ADVA_CHECK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCAN_RCV_IA_IN_PRIV`"]
pub type SCAN_RCV_IA_IN_PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_RCV_IA_IN_PRIV`"]
pub struct SCAN_RCV_IA_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RCV_IA_IN_PRIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV`"]
pub type SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV`"]
pub struct SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&self) -> SCAN_TYPE_R {
        SCAN_TYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&self) -> SCAN_FILT_POLICY_R {
        SCAN_FILT_POLICY_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&self) -> DUP_FILT_EN_R {
        DUP_FILT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&self) -> DUP_FILT_CHK_ADV_DIR_R {
        DUP_FILT_CHK_ADV_DIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&self) -> SCAN_RSP_ADVA_CHECK_R {
        SCAN_RSP_ADVA_CHECK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&self) -> SCAN_RCV_IA_IN_PRIV_R {
        SCAN_RCV_IA_IN_PRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W {
        TX_ADDR_W { w: self }
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&mut self) -> SCAN_TYPE_W {
        SCAN_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&mut self) -> SCAN_FILT_POLICY_W {
        SCAN_FILT_POLICY_W { w: self }
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&mut self) -> DUP_FILT_EN_W {
        DUP_FILT_EN_W { w: self }
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&mut self) -> DUP_FILT_CHK_ADV_DIR_W {
        DUP_FILT_CHK_ADV_DIR_W { w: self }
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&mut self) -> SCAN_RSP_ADVA_CHECK_W {
        SCAN_RSP_ADVA_CHECK_W { w: self }
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&mut self) -> SCAN_RCV_IA_IN_PRIV_W {
        SCAN_RCV_IA_IN_PRIV_W { w: self }
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&mut self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W { w: self }
    }
}
