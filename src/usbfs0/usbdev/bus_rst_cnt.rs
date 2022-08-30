#[doc = "Register `BUS_RST_CNT` reader"]
pub struct R(crate::R<BUS_RST_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_RST_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_RST_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_RST_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_RST_CNT` writer"]
pub struct W(crate::W<BUS_RST_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_RST_CNT_SPEC>;
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
impl From<crate::W<BUS_RST_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_RST_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_RST_CNT` reader - Bus Reset Count Length"]
pub type BUS_RST_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUS_RST_CNT` writer - Bus Reset Count Length"]
pub type BUS_RST_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUS_RST_CNT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&self) -> BUS_RST_CNT_R {
        BUS_RST_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&mut self) -> BUS_RST_CNT_W<0> {
        BUS_RST_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Reset Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_rst_cnt](index.html) module"]
pub struct BUS_RST_CNT_SPEC;
impl crate::RegisterSpec for BUS_RST_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_rst_cnt::R](R) reader structure"]
impl crate::Readable for BUS_RST_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_rst_cnt::W](W) writer structure"]
impl crate::Writable for BUS_RST_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_RST_CNT to value 0x0a"]
impl crate::Resettable for BUS_RST_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
