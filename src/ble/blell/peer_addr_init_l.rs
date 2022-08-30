#[doc = "Register `PEER_ADDR_INIT_L` reader"]
pub struct R(crate::R<PEER_ADDR_INIT_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEER_ADDR_INIT_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEER_ADDR_INIT_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEER_ADDR_INIT_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEER_ADDR_INIT_L` writer"]
pub struct W(crate::W<PEER_ADDR_INIT_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEER_ADDR_INIT_L_SPEC>;
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
impl From<crate::W<PEER_ADDR_INIT_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEER_ADDR_INIT_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEER_ADDR_L` reader - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
pub type PEER_ADDR_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PEER_ADDR_L` writer - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
pub type PEER_ADDR_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PEER_ADDR_INIT_L_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
    #[inline(always)]
    pub fn peer_addr_l(&self) -> PEER_ADDR_L_R {
        PEER_ADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit address of the peer device. This is used only in MMMS mode The peer address registers are used for multiple purposes. The register is written by firmware to provide the peer address to be used for a hardware procedure. When firmware reads the register, it reads back peer address values updated by hardware. While device is configured as an initiator without white list filtering, the peer address specified in the peer_address field of the create connection command is programmed into this register, which is used by hard-ware procedures. While device is configured as an initiator and white list is enabled, firmware can read this register to get the address of the peer device from which connectable ADV packet was received and to which the connection is created."]
    #[inline(always)]
    pub fn peer_addr_l(&mut self) -> PEER_ADDR_L_W<0> {
        PEER_ADDR_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower 16 bit address of the peer device for INIT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_init_l](index.html) module"]
pub struct PEER_ADDR_INIT_L_SPEC;
impl crate::RegisterSpec for PEER_ADDR_INIT_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peer_addr_init_l::R](R) reader structure"]
impl crate::Readable for PEER_ADDR_INIT_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peer_addr_init_l::W](W) writer structure"]
impl crate::Writable for PEER_ADDR_INIT_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEER_ADDR_INIT_L to value 0"]
impl crate::Resettable for PEER_ADDR_INIT_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
