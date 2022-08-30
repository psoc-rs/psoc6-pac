#[doc = "Register `CFG_IN` reader"]
pub struct R(crate::R<CFG_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_IN` writer"]
pub struct W(crate::W<CFG_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_IN_SPEC>;
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
impl From<crate::W<CFG_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTRIP_SEL0_0_A {
    #[doc = "0: S40E: {CFG_IN_GPIO5V.VTRIP_SEL0_1, CFG_IN.VTRIP_SEL0_0} 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    CMOS = 0,
    #[doc = "1: N/A"]
    TTL = 1,
}
impl From<VTRIP_SEL0_0_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTRIP_SEL0_0` reader - N/A"]
pub type VTRIP_SEL0_0_R = crate::BitReader<VTRIP_SEL0_0_A>;
impl VTRIP_SEL0_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTRIP_SEL0_0_A {
        match self.bits {
            false => VTRIP_SEL0_0_A::CMOS,
            true => VTRIP_SEL0_0_A::TTL,
        }
    }
    #[doc = "Checks if the value of the field is `CMOS`"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        *self == VTRIP_SEL0_0_A::CMOS
    }
    #[doc = "Checks if the value of the field is `TTL`"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        *self == VTRIP_SEL0_0_A::TTL
    }
}
#[doc = "Field `VTRIP_SEL0_0` writer - N/A"]
pub type VTRIP_SEL0_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFG_IN_SPEC, VTRIP_SEL0_0_A, O>;
impl<'a, const O: u8> VTRIP_SEL0_0_W<'a, O> {
    #[doc = "S40E: {CFG_IN_GPIO5V.VTRIP_SEL0_1, CFG_IN.VTRIP_SEL0_0} 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::CMOS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::TTL)
    }
}
#[doc = "Field `VTRIP_SEL1_0` reader - N/A"]
pub type VTRIP_SEL1_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL1_0` writer - N/A"]
pub type VTRIP_SEL1_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL2_0` reader - N/A"]
pub type VTRIP_SEL2_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL2_0` writer - N/A"]
pub type VTRIP_SEL2_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL3_0` reader - N/A"]
pub type VTRIP_SEL3_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL3_0` writer - N/A"]
pub type VTRIP_SEL3_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL4_0` reader - N/A"]
pub type VTRIP_SEL4_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL4_0` writer - N/A"]
pub type VTRIP_SEL4_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL5_0` reader - N/A"]
pub type VTRIP_SEL5_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL5_0` writer - N/A"]
pub type VTRIP_SEL5_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL6_0` reader - N/A"]
pub type VTRIP_SEL6_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL6_0` writer - N/A"]
pub type VTRIP_SEL6_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL7_0` reader - N/A"]
pub type VTRIP_SEL7_0_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL7_0` writer - N/A"]
pub type VTRIP_SEL7_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_IN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&self) -> VTRIP_SEL0_0_R {
        VTRIP_SEL0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&self) -> VTRIP_SEL1_0_R {
        VTRIP_SEL1_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&self) -> VTRIP_SEL2_0_R {
        VTRIP_SEL2_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&self) -> VTRIP_SEL3_0_R {
        VTRIP_SEL3_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&self) -> VTRIP_SEL4_0_R {
        VTRIP_SEL4_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&self) -> VTRIP_SEL5_0_R {
        VTRIP_SEL5_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&self) -> VTRIP_SEL6_0_R {
        VTRIP_SEL6_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&self) -> VTRIP_SEL7_0_R {
        VTRIP_SEL7_0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&mut self) -> VTRIP_SEL0_0_W<0> {
        VTRIP_SEL0_0_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&mut self) -> VTRIP_SEL1_0_W<1> {
        VTRIP_SEL1_0_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&mut self) -> VTRIP_SEL2_0_W<2> {
        VTRIP_SEL2_0_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&mut self) -> VTRIP_SEL3_0_W<3> {
        VTRIP_SEL3_0_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&mut self) -> VTRIP_SEL4_0_W<4> {
        VTRIP_SEL4_0_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&mut self) -> VTRIP_SEL5_0_W<5> {
        VTRIP_SEL5_0_W::new(self)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&mut self) -> VTRIP_SEL6_0_W<6> {
        VTRIP_SEL6_0_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&mut self) -> VTRIP_SEL7_0_W<7> {
        VTRIP_SEL7_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port input buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_in](index.html) module"]
pub struct CFG_IN_SPEC;
impl crate::RegisterSpec for CFG_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_in::R](R) reader structure"]
impl crate::Readable for CFG_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_in::W](W) writer structure"]
impl crate::Writable for CFG_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_IN to value 0"]
impl crate::Resettable for CFG_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
