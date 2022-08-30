#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOS_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EOS_SET_R = crate::BitReader<bool>;
#[doc = "Field `EOS_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EOS_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `OVERFLOW_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_SET_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `FW_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type FW_COLLISION_SET_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type FW_COLLISION_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `DSI_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DSI_COLLISION_SET_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DSI_COLLISION_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `INJ_EOC_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_EOC_SET_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_EOC_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `INJ_SATURATE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_SATURATE_SET_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_SATURATE_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `INJ_RANGE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_RANGE_SET_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_RANGE_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `INJ_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_COLLISION_SET_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type INJ_COLLISION_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&self) -> EOS_SET_R {
        EOS_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&self) -> OVERFLOW_SET_R {
        OVERFLOW_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&self) -> FW_COLLISION_SET_R {
        FW_COLLISION_SET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&self) -> DSI_COLLISION_SET_R {
        DSI_COLLISION_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&self) -> INJ_EOC_SET_R {
        INJ_EOC_SET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&self) -> INJ_SATURATE_SET_R {
        INJ_SATURATE_SET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&self) -> INJ_RANGE_SET_R {
        INJ_RANGE_SET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&self) -> INJ_COLLISION_SET_R {
        INJ_COLLISION_SET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&mut self) -> EOS_SET_W<0> {
        EOS_SET_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&mut self) -> OVERFLOW_SET_W<1> {
        OVERFLOW_SET_W::new(self)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&mut self) -> FW_COLLISION_SET_W<2> {
        FW_COLLISION_SET_W::new(self)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&mut self) -> DSI_COLLISION_SET_W<3> {
        DSI_COLLISION_SET_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&mut self) -> INJ_EOC_SET_W<4> {
        INJ_EOC_SET_W::new(self)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&mut self) -> INJ_SATURATE_SET_W<5> {
        INJ_SATURATE_SET_W::new(self)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&mut self) -> INJ_RANGE_SET_W<6> {
        INJ_RANGE_SET_W::new(self)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&mut self) -> INJ_COLLISION_SET_W<7> {
        INJ_COLLISION_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
