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
#[doc = "Field `EDGE0` reader - Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
pub type EDGE0_R = crate::BitReader<bool>;
#[doc = "Field `EDGE0` writer - Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
pub type EDGE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE1` reader - Sets edge detect interrupt for IO pin 1"]
pub type EDGE1_R = crate::BitReader<bool>;
#[doc = "Field `EDGE1` writer - Sets edge detect interrupt for IO pin 1"]
pub type EDGE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE2` reader - Sets edge detect interrupt for IO pin 2"]
pub type EDGE2_R = crate::BitReader<bool>;
#[doc = "Field `EDGE2` writer - Sets edge detect interrupt for IO pin 2"]
pub type EDGE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE3` reader - Sets edge detect interrupt for IO pin 3"]
pub type EDGE3_R = crate::BitReader<bool>;
#[doc = "Field `EDGE3` writer - Sets edge detect interrupt for IO pin 3"]
pub type EDGE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE4` reader - Sets edge detect interrupt for IO pin 4"]
pub type EDGE4_R = crate::BitReader<bool>;
#[doc = "Field `EDGE4` writer - Sets edge detect interrupt for IO pin 4"]
pub type EDGE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE5` reader - Sets edge detect interrupt for IO pin 5"]
pub type EDGE5_R = crate::BitReader<bool>;
#[doc = "Field `EDGE5` writer - Sets edge detect interrupt for IO pin 5"]
pub type EDGE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE6` reader - Sets edge detect interrupt for IO pin 6"]
pub type EDGE6_R = crate::BitReader<bool>;
#[doc = "Field `EDGE6` writer - Sets edge detect interrupt for IO pin 6"]
pub type EDGE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `EDGE7` reader - Sets edge detect interrupt for IO pin 7"]
pub type EDGE7_R = crate::BitReader<bool>;
#[doc = "Field `EDGE7` writer - Sets edge detect interrupt for IO pin 7"]
pub type EDGE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `FLT_EDGE` reader - Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `FLT_EDGE` writer - Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sets edge detect interrupt for IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sets edge detect interrupt for IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sets edge detect interrupt for IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sets edge detect interrupt for IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sets edge detect interrupt for IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sets edge detect interrupt for IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sets edge detect interrupt for IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sets edge detect interrupt for IO pin 0 '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn edge0(&mut self) -> EDGE0_W<0> {
        EDGE0_W::new(self)
    }
    #[doc = "Bit 1 - Sets edge detect interrupt for IO pin 1"]
    #[inline(always)]
    pub fn edge1(&mut self) -> EDGE1_W<1> {
        EDGE1_W::new(self)
    }
    #[doc = "Bit 2 - Sets edge detect interrupt for IO pin 2"]
    #[inline(always)]
    pub fn edge2(&mut self) -> EDGE2_W<2> {
        EDGE2_W::new(self)
    }
    #[doc = "Bit 3 - Sets edge detect interrupt for IO pin 3"]
    #[inline(always)]
    pub fn edge3(&mut self) -> EDGE3_W<3> {
        EDGE3_W::new(self)
    }
    #[doc = "Bit 4 - Sets edge detect interrupt for IO pin 4"]
    #[inline(always)]
    pub fn edge4(&mut self) -> EDGE4_W<4> {
        EDGE4_W::new(self)
    }
    #[doc = "Bit 5 - Sets edge detect interrupt for IO pin 5"]
    #[inline(always)]
    pub fn edge5(&mut self) -> EDGE5_W<5> {
        EDGE5_W::new(self)
    }
    #[doc = "Bit 6 - Sets edge detect interrupt for IO pin 6"]
    #[inline(always)]
    pub fn edge6(&mut self) -> EDGE6_W<6> {
        EDGE6_W::new(self)
    }
    #[doc = "Bit 7 - Sets edge detect interrupt for IO pin 7"]
    #[inline(always)]
    pub fn edge7(&mut self) -> EDGE7_W<7> {
        EDGE7_W::new(self)
    }
    #[doc = "Bit 8 - Sets edge detect interrupt for filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&mut self) -> FLT_EDGE_W<8> {
        FLT_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
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
