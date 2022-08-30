#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOS_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EOS_MASK_R = crate::BitReader<bool>;
#[doc = "Field `EOS_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EOS_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `OVERFLOW_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_MASK_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `FW_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FW_COLLISION_MASK_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FW_COLLISION_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `DSI_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type DSI_COLLISION_MASK_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type DSI_COLLISION_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `INJ_EOC_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_EOC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_EOC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `INJ_SATURATE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_SATURATE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_SATURATE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `INJ_RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_RANGE_MASK_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_RANGE_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `INJ_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_COLLISION_MASK_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type INJ_COLLISION_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&self) -> EOS_MASK_R {
        EOS_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&self) -> OVERFLOW_MASK_R {
        OVERFLOW_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&self) -> FW_COLLISION_MASK_R {
        FW_COLLISION_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&self) -> DSI_COLLISION_MASK_R {
        DSI_COLLISION_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&self) -> INJ_EOC_MASK_R {
        INJ_EOC_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&self) -> INJ_SATURATE_MASK_R {
        INJ_SATURATE_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&self) -> INJ_RANGE_MASK_R {
        INJ_RANGE_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&self) -> INJ_COLLISION_MASK_R {
        INJ_COLLISION_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&mut self) -> EOS_MASK_W<0> {
        EOS_MASK_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&mut self) -> OVERFLOW_MASK_W<1> {
        OVERFLOW_MASK_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&mut self) -> FW_COLLISION_MASK_W<2> {
        FW_COLLISION_MASK_W::new(self)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&mut self) -> DSI_COLLISION_MASK_W<3> {
        DSI_COLLISION_MASK_W::new(self)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&mut self) -> INJ_EOC_MASK_W<4> {
        INJ_EOC_MASK_W::new(self)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&mut self) -> INJ_SATURATE_MASK_W<5> {
        INJ_SATURATE_MASK_W::new(self)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&mut self) -> INJ_RANGE_MASK_W<6> {
        INJ_RANGE_MASK_W::new(self)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&mut self) -> INJ_COLLISION_MASK_W<7> {
        INJ_COLLISION_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
