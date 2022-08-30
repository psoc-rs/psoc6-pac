#[doc = "Register `INIT_PARAM` reader"]
pub struct R(crate::R<INIT_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_PARAM` writer"]
pub struct W(crate::W<INIT_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_PARAM_SPEC>;
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
impl From<crate::W<INIT_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ADDR` reader - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
pub type TX_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `TX_ADDR` writer - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
pub type TX_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_PARAM_SPEC, bool, O>;
#[doc = "Field `RX_ADDR__RX_TX_ADDR` reader - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
pub type RX_ADDR__RX_TX_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `RX_ADDR__RX_TX_ADDR` writer - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
pub type RX_ADDR__RX_TX_ADDR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INIT_PARAM_SPEC, bool, O>;
#[doc = "Field `INIT_FILT_POLICY` reader - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
pub type INIT_FILT_POLICY_R = crate::BitReader<bool>;
#[doc = "Field `INIT_FILT_POLICY` writer - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
pub type INIT_FILT_POLICY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_PARAM_SPEC, bool, O>;
#[doc = "Field `INIT_RCV_IA_IN_PRIV` reader - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
pub type INIT_RCV_IA_IN_PRIV_R = crate::BitReader<bool>;
#[doc = "Field `INIT_RCV_IA_IN_PRIV` writer - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
pub type INIT_RCV_IA_IN_PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INIT_PARAM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn rx_addr__rx_tx_addr(&self) -> RX_ADDR__RX_TX_ADDR_R {
        RX_ADDR__RX_TX_ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
    #[inline(always)]
    pub fn init_filt_policy(&self) -> INIT_FILT_POLICY_R {
        INIT_FILT_POLICY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
    #[inline(always)]
    pub fn init_rcv_ia_in_priv(&self) -> INIT_RCV_IA_IN_PRIV_R {
        INIT_RCV_IA_IN_PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W<0> {
        TX_ADDR_W::new(self)
    }
    #[doc = "Bit 1 - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn rx_addr__rx_tx_addr(&mut self) -> RX_ADDR__RX_TX_ADDR_W<1> {
        RX_ADDR__RX_TX_ADDR_W::new(self)
    }
    #[doc = "Bit 3 - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
    #[inline(always)]
    pub fn init_filt_policy(&mut self) -> INIT_FILT_POLICY_W<3> {
        INIT_FILT_POLICY_W::new(self)
    }
    #[doc = "Bit 4 - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
    #[inline(always)]
    pub fn init_rcv_ia_in_priv(&mut self) -> INIT_RCV_IA_IN_PRIV_W<4> {
        INIT_RCV_IA_IN_PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initiator parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_param](index.html) module"]
pub struct INIT_PARAM_SPEC;
impl crate::RegisterSpec for INIT_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_param::R](R) reader structure"]
impl crate::Readable for INIT_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_param::W](W) writer structure"]
impl crate::Writable for INIT_PARAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_PARAM to value 0"]
impl crate::Resettable for INIT_PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
