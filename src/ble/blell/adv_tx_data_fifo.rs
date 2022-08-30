#[doc = "Register `ADV_TX_DATA_FIFO` reader"]
pub struct R(crate::R<ADV_TX_DATA_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_TX_DATA_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_TX_DATA_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_TX_DATA_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_TX_DATA_FIFO` writer"]
pub struct W(crate::W<ADV_TX_DATA_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_TX_DATA_FIFO_SPEC>;
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
impl From<crate::W<ADV_TX_DATA_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_TX_DATA_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_TX_DATA` reader - IO mapped FIFO of depth 16 (2 byte wide), to store ADV data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same address location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
pub type ADV_TX_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADV_TX_DATA` writer - IO mapped FIFO of depth 16 (2 byte wide), to store ADV data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same address location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
pub type ADV_TX_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADV_TX_DATA_FIFO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 16 (2 byte wide), to store ADV data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same address location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
    #[inline(always)]
    pub fn adv_tx_data(&self) -> ADV_TX_DATA_R {
        ADV_TX_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 16 (2 byte wide), to store ADV data of maximum length 31 bytes for transmitting. Firmware writes consecutive words by writing to the same address location. Note: ADV_TX_DATA_FIFO and ADV_SCN_RSP_TX_FIFO shares same physical FIFO of depth 32. 16 locations for each FIFO are allocated. Reading this location resets the FIFO pointer."]
    #[inline(always)]
    pub fn adv_tx_data(&mut self) -> ADV_TX_DATA_W<0> {
        ADV_TX_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertising data transmit FIFO. Access ADVCH_TX_FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_tx_data_fifo](index.html) module"]
pub struct ADV_TX_DATA_FIFO_SPEC;
impl crate::RegisterSpec for ADV_TX_DATA_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_tx_data_fifo::R](R) reader structure"]
impl crate::Readable for ADV_TX_DATA_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_tx_data_fifo::W](W) writer structure"]
impl crate::Writable for ADV_TX_DATA_FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_TX_DATA_FIFO to value 0"]
impl crate::Resettable for ADV_TX_DATA_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
