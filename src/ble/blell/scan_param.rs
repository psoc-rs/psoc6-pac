#[doc = "Register `SCAN_PARAM` reader"]
pub struct R(crate::R<SCAN_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_PARAM` writer"]
pub struct W(crate::W<SCAN_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_PARAM_SPEC>;
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
impl From<crate::W<SCAN_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ADDR` reader - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
pub type TX_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `TX_ADDR` writer - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
pub type TX_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
#[doc = "Field `SCAN_TYPE` reader - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
pub type SCAN_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCAN_TYPE` writer - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
pub type SCAN_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCAN_PARAM_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCAN_FILT_POLICY` reader - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
pub type SCAN_FILT_POLICY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCAN_FILT_POLICY` writer - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
pub type SCAN_FILT_POLICY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCAN_PARAM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DUP_FILT_EN` reader - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
pub type DUP_FILT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DUP_FILT_EN` writer - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
pub type DUP_FILT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
#[doc = "Field `DUP_FILT_CHK_ADV_DIR` reader - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
pub type DUP_FILT_CHK_ADV_DIR_R = crate::BitReader<bool>;
#[doc = "Field `DUP_FILT_CHK_ADV_DIR` writer - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
pub type DUP_FILT_CHK_ADV_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
#[doc = "Field `SCAN_RSP_ADVA_CHECK` reader - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
pub type SCAN_RSP_ADVA_CHECK_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_RSP_ADVA_CHECK` writer - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
pub type SCAN_RSP_ADVA_CHECK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
#[doc = "Field `SCAN_RCV_IA_IN_PRIV` reader - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub type SCAN_RCV_IA_IN_PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_RCV_IA_IN_PRIV` writer - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
pub type SCAN_RCV_IA_IN_PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
#[doc = "Field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV` reader - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
pub type SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV` writer - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
pub type SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_PARAM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&self) -> SCAN_TYPE_R {
        SCAN_TYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&self) -> SCAN_FILT_POLICY_R {
        SCAN_FILT_POLICY_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&self) -> DUP_FILT_EN_R {
        DUP_FILT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&self) -> DUP_FILT_CHK_ADV_DIR_R {
        DUP_FILT_CHK_ADV_DIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&self) -> SCAN_RSP_ADVA_CHECK_R {
        SCAN_RSP_ADVA_CHECK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&self) -> SCAN_RCV_IA_IN_PRIV_R {
        SCAN_RCV_IA_IN_PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device's own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W<0> {
        TX_ADDR_W::new(self)
    }
    #[doc = "Bits 1:2 - 0x00 - passive scanning.(default) 0x01 - active scanning. 0x10 - RFU 0x11 - RFU"]
    #[inline(always)]
    pub fn scan_type(&mut self) -> SCAN_TYPE_W<1> {
        SCAN_TYPE_W::new(self)
    }
    #[doc = "Bits 3:4 - The scanner filter policy determines how the scanner processes advertising packets. 0x00 - Accept advertising packets from any device. 0x01 - Accept advertising packets from only devices in the whitelist. In the above 2 policies, the directed advertising packets which are not addressed to this device are ignored. 0x10 - Accept all undirected advertising packets and directed advertising packet addressed to this device. 0x11 - Accept undirected advertising packets from devices in the whitelist and directed advertising packet addressed to this device In the above 2 policies, the directed advertising packets where the initiator address is a resolvable private address are accepted. The above 2 policies are extended scanner filter policies."]
    #[inline(always)]
    pub fn scan_filt_policy(&mut self) -> SCAN_FILT_POLICY_W<3> {
        SCAN_FILT_POLICY_W::new(self)
    }
    #[doc = "Bit 5 - Filter duplicate packets. 1- Duplicate filtering enabled. 0- Duplicate filtering not enabled. This field is derived from the LE_set_scan_enable command."]
    #[inline(always)]
    pub fn dup_filt_en(&mut self) -> DUP_FILT_EN_W<5> {
        DUP_FILT_EN_W::new(self)
    }
    #[doc = "Bit 6 - This bit field is used to specify the Scanner duplicate filter behavior for ADV_DIRECT_IND packet when duplicate DUP_FILT_EN is set. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - Do not filter ADV_DIRECT_IND duplicate packets. 1 - Filter ADV_DIRECT_IND duplicate packets"]
    #[inline(always)]
    pub fn dup_filt_chk_adv_dir(&mut self) -> DUP_FILT_CHK_ADV_DIR_W<6> {
        DUP_FILT_CHK_ADV_DIR_W::new(self)
    }
    #[doc = "Bit 7 - This bit field is used to specify the Scanner behavior with respect to ADVA while receiving a SCAN_RSP packet. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 0 - The ADVA in SCAN_RSP packets are not verified 1 - The ADVA in SCAN_RSP packets are verified against ADVA received in ADV packet . If it fails, then abort the packet."]
    #[inline(always)]
    pub fn scan_rsp_adva_check(&mut self) -> SCAN_RSP_ADVA_CHECK_W<7> {
        SCAN_RSP_ADVA_CHECK_W::new(self)
    }
    #[doc = "Bit 8 - Scanner behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode"]
    #[inline(always)]
    pub fn scan_rcv_ia_in_priv(&mut self) -> SCAN_RCV_IA_IN_PRIV_W<8> {
        SCAN_RCV_IA_IN_PRIV_W::new(self)
    }
    #[doc = "Bit 9 - Scanner behavior when a peer Non Resolvable Private Address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. This is applicable when whitelist is disabled. 1 - Only report packets with peer NRPA address in privacy mode 0 - Respond packets with peer NRPA address in privacy mode"]
    #[inline(always)]
    pub fn scan_rpt_peer_nrpa_addr_in_priv(&mut self) -> SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W<9> {
        SCAN_RPT_PEER_NRPA_ADDR_IN_PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scanning parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_param](index.html) module"]
pub struct SCAN_PARAM_SPEC;
impl crate::RegisterSpec for SCAN_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_param::R](R) reader structure"]
impl crate::Readable for SCAN_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_param::W](W) writer structure"]
impl crate::Writable for SCAN_PARAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_PARAM to value 0"]
impl crate::Resettable for SCAN_PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
