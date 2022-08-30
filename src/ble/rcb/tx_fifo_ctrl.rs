#[doc = "Register `TX_FIFO_CTRL` reader"]
pub struct R(crate::R<TX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_FIFO_CTRL` writer"]
pub struct W(crate::W<TX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_CTRL_SPEC>;
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
impl From<crate::W<TX_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_TRIGGER_LEVEL` reader - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
pub type TX_TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_TRIGGER_LEVEL` writer - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
pub type TX_TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_FIFO_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CLEAR` reader - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn tx_trigger_level(&self) -> TX_TRIGGER_LEVEL_R {
        TX_TRIGGER_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn tx_trigger_level(&mut self) -> TX_TRIGGER_LEVEL_W<0> {
        TX_TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter FIFO control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctrl](index.html) module"]
pub struct TX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for TX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for TX_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for TX_FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_FIFO_CTRL to value 0"]
impl crate::Resettable for TX_FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
