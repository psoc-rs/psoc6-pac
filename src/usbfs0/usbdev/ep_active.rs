#[doc = "Register `EP_ACTIVE` reader"]
pub struct R(crate::R<EP_ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_ACTIVE` writer"]
pub struct W(crate::W<EP_ACTIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_ACTIVE_SPEC>;
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
impl From<crate::W<EP_ACTIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_ACTIVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP1_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP1_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP1_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP2_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP2_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP2_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP2_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP3_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP3_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP3_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP3_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP4_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP4_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP4_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP4_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP5_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP5_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP5_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP5_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP6_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP6_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP6_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP6_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP7_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP7_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP7_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP7_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
#[doc = "Field `EP8_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP8_ACT_R = crate::BitReader<bool>;
#[doc = "Field `EP8_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP8_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP_ACTIVE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&self) -> EP1_ACT_R {
        EP1_ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&self) -> EP2_ACT_R {
        EP2_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&self) -> EP3_ACT_R {
        EP3_ACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&self) -> EP4_ACT_R {
        EP4_ACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&self) -> EP5_ACT_R {
        EP5_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&self) -> EP6_ACT_R {
        EP6_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&self) -> EP7_ACT_R {
        EP7_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&self) -> EP8_ACT_R {
        EP8_ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&mut self) -> EP1_ACT_W<0> {
        EP1_ACT_W::new(self)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&mut self) -> EP2_ACT_W<1> {
        EP2_ACT_W::new(self)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&mut self) -> EP3_ACT_W<2> {
        EP3_ACT_W::new(self)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&mut self) -> EP4_ACT_W<3> {
        EP4_ACT_W::new(self)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&mut self) -> EP5_ACT_W<4> {
        EP5_ACT_W::new(self)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&mut self) -> EP6_ACT_W<5> {
        EP6_ACT_W::new(self)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&mut self) -> EP7_ACT_W<6> {
        EP7_ACT_W::new(self)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&mut self) -> EP8_ACT_W<7> {
        EP8_ACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Active Indication Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_active](index.html) module"]
pub struct EP_ACTIVE_SPEC;
impl crate::RegisterSpec for EP_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_active::R](R) reader structure"]
impl crate::Readable for EP_ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_active::W](W) writer structure"]
impl crate::Writable for EP_ACTIVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP_ACTIVE to value 0"]
impl crate::Resettable for EP_ACTIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
