#[doc = "Register `CE_CNFG_STS_REGISTER_EXT` reader"]
pub struct R(crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_CNFG_STS_REGISTER_EXT` writer"]
pub struct W(crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>;
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
impl From<crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_2M` reader - transmittion on 2M"]
pub type TX_2M_R = crate::BitReader<bool>;
#[doc = "Field `TX_2M` writer - transmittion on 2M"]
pub type TX_2M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_EXT_SPEC, bool, O>;
#[doc = "Field `RX_2M` reader - receiving on 2M"]
pub type RX_2M_R = crate::BitReader<bool>;
#[doc = "Field `RX_2M` writer - receiving on 2M"]
pub type RX_2M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_EXT_SPEC, bool, O>;
#[doc = "Field `SN` reader - Sequence number for next scheduled connection index"]
pub type SN_R = crate::BitReader<bool>;
#[doc = "Field `SN` writer - Sequence number for next scheduled connection index"]
pub type SN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_EXT_SPEC, bool, O>;
#[doc = "Field `NESN` reader - Next Sequence number for next scheduled connection index"]
pub type NESN_R = crate::BitReader<bool>;
#[doc = "Field `NESN` writer - Next Sequence number for next scheduled connection index"]
pub type NESN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_EXT_SPEC, bool, O>;
#[doc = "Field `LAST_UNMAPPED_CHANNEL` reader - Last unmapped channel for next scheduled connection index"]
pub type LAST_UNMAPPED_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LAST_UNMAPPED_CHANNEL` writer - Last unmapped channel for next scheduled connection index"]
pub type LAST_UNMAPPED_CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CE_CNFG_STS_REGISTER_EXT_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&self) -> TX_2M_R {
        TX_2M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&self) -> RX_2M_R {
        RX_2M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&self) -> SN_R {
        SN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&self) -> NESN_R {
        NESN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&self) -> LAST_UNMAPPED_CHANNEL_R {
        LAST_UNMAPPED_CHANNEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&mut self) -> TX_2M_W<0> {
        TX_2M_W::new(self)
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&mut self) -> RX_2M_W<1> {
        RX_2M_W::new(self)
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&mut self) -> SN_W<2> {
        SN_W::new(self)
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&mut self) -> NESN_W<3> {
        NESN_W::new(self)
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&mut self) -> LAST_UNMAPPED_CHANNEL_W<8> {
        LAST_UNMAPPED_CHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "connection configuration & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cnfg_sts_register_ext](index.html) module"]
pub struct CE_CNFG_STS_REGISTER_EXT_SPEC;
impl crate::RegisterSpec for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_cnfg_sts_register_ext::R](R) reader structure"]
impl crate::Readable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_cnfg_sts_register_ext::W](W) writer structure"]
impl crate::Writable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_CNFG_STS_REGISTER_EXT to value 0"]
impl crate::Resettable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
