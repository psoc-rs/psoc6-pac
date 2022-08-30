#[doc = "Register `MS14_CTL` reader"]
pub struct R(crate::R<MS14_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS14_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS14_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS14_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS14_CTL` writer"]
pub struct W(crate::W<MS14_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS14_CTL_SPEC>;
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
impl From<crate::W<MS14_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS14_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - See MS0_CTL.P."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - See MS0_CTL.P."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, MS14_CTL_SPEC, bool, O>;
#[doc = "Field `NS` reader - See MS0_CTL.NS."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - See MS0_CTL.NS."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MS14_CTL_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - See MS0_CTL.PRIO"]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - See MS0_CTL.PRIO"]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MS14_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC_MASK_0` reader - See MS0_CTL.PC_MASK_0."]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MS14_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<1> {
        NS_W::new(self)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<8> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<17> {
        PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master 14 protection context control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms14_ctl](index.html) module"]
pub struct MS14_CTL_SPEC;
impl crate::RegisterSpec for MS14_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms14_ctl::R](R) reader structure"]
impl crate::Readable for MS14_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms14_ctl::W](W) writer structure"]
impl crate::Writable for MS14_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MS14_CTL to value 0x0303"]
impl crate::Resettable for MS14_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0303
    }
}
