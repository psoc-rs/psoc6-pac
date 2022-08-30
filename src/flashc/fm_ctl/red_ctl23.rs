#[doc = "Register `RED_CTL23` reader"]
pub struct R(crate::R<RED_CTL23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL23` writer"]
pub struct W(crate::W<RED_CTL23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL23_SPEC>;
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
impl From<crate::W<RED_CTL23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED_ADDR_2` reader - Bad Row Pair Address for Sector 2"]
pub type RED_ADDR_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_ADDR_2` writer - Bad Row Pair Address for Sector 2"]
pub type RED_ADDR_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED_CTL23_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED_EN_2` reader - 1': Redundancy Enable for Sector 2"]
pub type RED_EN_2_R = crate::BitReader<bool>;
#[doc = "Field `RED_EN_2` writer - 1': Redundancy Enable for Sector 2"]
pub type RED_EN_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL23_SPEC, bool, O>;
#[doc = "Field `RED_ADDR_3` reader - Bad Row Pair Address for Sector 3"]
pub type RED_ADDR_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_ADDR_3` writer - Bad Row Pair Address for Sector 3"]
pub type RED_ADDR_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RED_CTL23_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED_EN_3` reader - 1': Redundancy Enable for Sector 3"]
pub type RED_EN_3_R = crate::BitReader<bool>;
#[doc = "Field `RED_EN_3` writer - 1': Redundancy Enable for Sector 3"]
pub type RED_EN_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL23_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn red_addr_2(&self) -> RED_ADDR_2_R {
        RED_ADDR_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1': Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn red_en_2(&self) -> RED_EN_2_R {
        RED_EN_2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn red_addr_3(&self) -> RED_ADDR_3_R {
        RED_ADDR_3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1': Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn red_en_3(&self) -> RED_EN_3_R {
        RED_EN_3_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn red_addr_2(&mut self) -> RED_ADDR_2_W<0> {
        RED_ADDR_2_W::new(self)
    }
    #[doc = "Bit 8 - 1': Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn red_en_2(&mut self) -> RED_EN_2_W<8> {
        RED_EN_2_W::new(self)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn red_addr_3(&mut self) -> RED_ADDR_3_W<16> {
        RED_ADDR_3_W::new(self)
    }
    #[doc = "Bit 24 - 1': Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn red_en_3(&mut self) -> RED_EN_3_W<24> {
        RED_EN_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Controll normal sectors 2,3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl23](index.html) module"]
pub struct RED_CTL23_SPEC;
impl crate::RegisterSpec for RED_CTL23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl23::R](R) reader structure"]
impl crate::Readable for RED_CTL23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl23::W](W) writer structure"]
impl crate::Writable for RED_CTL23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL23 to value 0"]
impl crate::Resettable for RED_CTL23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
