#[doc = "Reader of register INIT_PARAM"]
pub type R = crate::R<u32, super::INIT_PARAM>;
#[doc = "Writer for register INIT_PARAM"]
pub type W = crate::W<u32, super::INIT_PARAM>;
#[doc = "Register INIT_PARAM `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_PARAM {
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
#[doc = "Reader of field `RX_ADDR__RX_TX_ADDR`"]
pub type RX_ADDR__RX_TX_ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ADDR__RX_TX_ADDR`"]
pub struct RX_ADDR__RX_TX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADDR__RX_TX_ADDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INIT_FILT_POLICY`"]
pub type INIT_FILT_POLICY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_FILT_POLICY`"]
pub struct INIT_FILT_POLICY_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_FILT_POLICY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `INIT_RCV_IA_IN_PRIV`"]
pub type INIT_RCV_IA_IN_PRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_RCV_IA_IN_PRIV`"]
pub struct INIT_RCV_IA_IN_PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_RCV_IA_IN_PRIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&self) -> TX_ADDR_R {
        TX_ADDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn rx_addr__rx_tx_addr(&self) -> RX_ADDR__RX_TX_ADDR_R {
        RX_ADDR__RX_TX_ADDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
    #[inline(always)]
    pub fn init_filt_policy(&self) -> INIT_FILT_POLICY_R {
        INIT_FILT_POLICY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
    #[inline(always)]
    pub fn init_rcv_ia_in_priv(&self) -> INIT_RCV_IA_IN_PRIV_R {
        INIT_RCV_IA_IN_PRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device' own address type. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn tx_addr(&mut self) -> TX_ADDR_W {
        TX_ADDR_W { w: self }
    }
    #[doc = "Bit 1 - Peer address type. The rx_addr field is updated by the receiver with the address type of the received connectable advertising packet. 1 - addr type is random. 0 - addr type is public."]
    #[inline(always)]
    pub fn rx_addr__rx_tx_addr(&mut self) -> RX_ADDR__RX_TX_ADDR_W {
        RX_ADDR__RX_TX_ADDR_W { w: self }
    }
    #[doc = "Bit 3 - The Initiator_Filter_Policy is used to determine whether the White List is used or not. 0 - White list is not used to determine which advertiser to connect to. Instead the Peer_Address_Type and Peer Address fields are used to specify the address type and address of the advertising device to connect to. 1 - White list is used to determine the advertising device to connect to. Peer_Address_Type and Peer_Address fields are ignored when whitelist is used."]
    #[inline(always)]
    pub fn init_filt_policy(&mut self) -> INIT_FILT_POLICY_W {
        INIT_FILT_POLICY_W { w: self }
    }
    #[doc = "Bit 4 - Init behavior when a peer Identity address is received in privacy mode. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set. 1 - Accept packets with peer identity address not in the Resolving list in privacy mode 0 - Reject packets with peer identity address not in the Resolving list in privacy mode & HW_RSLV_LIST_FULL is not set"]
    #[inline(always)]
    pub fn init_rcv_ia_in_priv(&mut self) -> INIT_RCV_IA_IN_PRIV_W {
        INIT_RCV_IA_IN_PRIV_W { w: self }
    }
}
