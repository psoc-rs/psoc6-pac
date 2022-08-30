#[doc = "Register `ATT0` reader"]
pub struct R(crate::R<ATT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATT0` writer"]
pub struct W(crate::W<ATT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATT0_SPEC>;
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
impl From<crate::W<ATT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UR` reader - See corresponding field for PPU structure with programmable address."]
pub type UR_R = crate::BitReader<bool>;
#[doc = "Field `UR` writer - See corresponding field for PPU structure with programmable address."]
pub type UR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `UW` reader - See corresponding field for PPU structure with programmable address."]
pub type UW_R = crate::BitReader<bool>;
#[doc = "Field `UW` writer - See corresponding field for PPU structure with programmable address."]
pub type UW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `UX` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
pub type UX_R = crate::BitReader<bool>;
#[doc = "Field `PR` reader - See corresponding field for PPU structure with programmable address."]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - See corresponding field for PPU structure with programmable address."]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `PW` reader - See corresponding field for PPU structure with programmable address."]
pub type PW_R = crate::BitReader<bool>;
#[doc = "Field `PW` writer - See corresponding field for PPU structure with programmable address."]
pub type PW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `PX` reader - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
pub type PX_R = crate::BitReader<bool>;
#[doc = "Field `NS` reader - See corresponding field for PPU structure with programmable address."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - See corresponding field for PPU structure with programmable address."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `PC_MASK_0` reader - See corresponding field for PPU structure with programmable address."]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - See corresponding field for PPU structure with programmable address."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - See corresponding field for PPU structure with programmable address."]
pub type PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ATT0_SPEC, u16, u16, 15, O>;
#[doc = "Field `REGION_SIZE` reader - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
pub type REGION_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC_MATCH` reader - See corresponding field for PPU structure with programmable address."]
pub type PC_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `PC_MATCH` writer - See corresponding field for PPU structure with programmable address."]
pub type PC_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - See corresponding field for PPU structure with programmable address."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - See corresponding field for PPU structure with programmable address."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&self) -> PC_MATCH_R {
        PC_MATCH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W<0> {
        UR_W::new(self)
    }
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&mut self) -> UW_W<1> {
        UW_W::new(self)
    }
    #[doc = "Bit 3 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<3> {
        PR_W::new(self)
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W<4> {
        PW_W::new(self)
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<6> {
        NS_W::new(self)
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<9> {
        PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&mut self) -> PC_MATCH_W<30> {
        PC_MATCH_W::new(self)
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU region attributes 0 (slave structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att0](index.html) module"]
pub struct ATT0_SPEC;
impl crate::RegisterSpec for ATT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [att0::R](R) reader structure"]
impl crate::Readable for ATT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [att0::W](W) writer structure"]
impl crate::Writable for ATT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATT0 to value 0x0124"]
impl crate::Resettable for ATT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0124
    }
}
