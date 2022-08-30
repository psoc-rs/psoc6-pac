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
#[doc = "Field `TRIGGER_LEVEL` reader - N/A"]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - N/A"]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_FIFO_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLEAR` reader - N/A"]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - N/A"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTRL_SPEC, bool, O>;
#[doc = "Field `FREEZE` reader - N/A"]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - N/A"]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W<17> {
        FREEZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctrl](index.html) module"]
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
