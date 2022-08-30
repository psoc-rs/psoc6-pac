#[doc = "Register `PEER_SEC_ADDR_ADV_H` reader"]
pub struct R(crate::R<PEER_SEC_ADDR_ADV_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEER_SEC_ADDR_ADV_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEER_SEC_ADDR_ADV_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEER_SEC_ADDR_ADV_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEER_SEC_ADDR_ADV_H` writer"]
pub struct W(crate::W<PEER_SEC_ADDR_ADV_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEER_SEC_ADDR_ADV_H_SPEC>;
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
impl From<crate::W<PEER_SEC_ADDR_ADV_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEER_SEC_ADDR_ADV_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEER_SEC_ADDR_H` reader - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
pub type PEER_SEC_ADDR_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PEER_SEC_ADDR_H` writer - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
pub type PEER_SEC_ADDR_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PEER_SEC_ADDR_ADV_H_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
    #[inline(always)]
    pub fn peer_sec_addr_h(&self) -> PEER_SEC_ADDR_H_R {
        PEER_SEC_ADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit secondary address of the peer device for ADV_DIR. While doing directed Advertising in device privacy mode, if the peer device has shared its IRK, then the peer device RPA is written into the PEER_ADDR registers and the peer device identity address is written into this register. If the peer device has not shared its IRK, then the peer identity address is written into the PEER_ADDR registers and this register must be cleared."]
    #[inline(always)]
    pub fn peer_sec_addr_h(&mut self) -> PEER_SEC_ADDR_H_W<0> {
        PEER_SEC_ADDR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Higher 16 bits of the secondary address of the peer device for ADV_DIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_sec_addr_adv_h](index.html) module"]
pub struct PEER_SEC_ADDR_ADV_H_SPEC;
impl crate::RegisterSpec for PEER_SEC_ADDR_ADV_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peer_sec_addr_adv_h::R](R) reader structure"]
impl crate::Readable for PEER_SEC_ADDR_ADV_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peer_sec_addr_adv_h::W](W) writer structure"]
impl crate::Writable for PEER_SEC_ADDR_ADV_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEER_SEC_ADDR_ADV_H to value 0"]
impl crate::Resettable for PEER_SEC_ADDR_ADV_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
