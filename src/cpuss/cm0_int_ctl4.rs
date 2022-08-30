#[doc = "Register `CM0_INT_CTL4` reader"]
pub struct R(crate::R<CM0_INT_CTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_INT_CTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_INT_CTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_INT_CTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_INT_CTL4` writer"]
pub struct W(crate::W<CM0_INT_CTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_INT_CTL4_SPEC>;
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
impl From<crate::W<CM0_INT_CTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_INT_CTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX0_SEL` reader - System interrupt select for CPU interrupt source 16."]
pub type MUX0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX0_SEL` writer - System interrupt select for CPU interrupt source 16."]
pub type MUX0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_INT_CTL4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MUX1_SEL` reader - System interrupt select for CPU interrupt source 17."]
pub type MUX1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX1_SEL` writer - System interrupt select for CPU interrupt source 17."]
pub type MUX1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_INT_CTL4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MUX2_SEL` reader - System interrupt select for CPU interrupt source 18."]
pub type MUX2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX2_SEL` writer - System interrupt select for CPU interrupt source 18."]
pub type MUX2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_INT_CTL4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MUX3_SEL` reader - System interrupt select for CPU interrupt source 19."]
pub type MUX3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX3_SEL` writer - System interrupt select for CPU interrupt source 19."]
pub type MUX3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_INT_CTL4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 16."]
    #[inline(always)]
    pub fn mux0_sel(&self) -> MUX0_SEL_R {
        MUX0_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 17."]
    #[inline(always)]
    pub fn mux1_sel(&self) -> MUX1_SEL_R {
        MUX1_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 18."]
    #[inline(always)]
    pub fn mux2_sel(&self) -> MUX2_SEL_R {
        MUX2_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 19."]
    #[inline(always)]
    pub fn mux3_sel(&self) -> MUX3_SEL_R {
        MUX3_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System interrupt select for CPU interrupt source 16."]
    #[inline(always)]
    pub fn mux0_sel(&mut self) -> MUX0_SEL_W<0> {
        MUX0_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - System interrupt select for CPU interrupt source 17."]
    #[inline(always)]
    pub fn mux1_sel(&mut self) -> MUX1_SEL_W<8> {
        MUX1_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - System interrupt select for CPU interrupt source 18."]
    #[inline(always)]
    pub fn mux2_sel(&mut self) -> MUX2_SEL_W<16> {
        MUX2_SEL_W::new(self)
    }
    #[doc = "Bits 24:31 - System interrupt select for CPU interrupt source 19."]
    #[inline(always)]
    pub fn mux3_sel(&mut self) -> MUX3_SEL_W<24> {
        MUX3_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ interrupt control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl4](index.html) module"]
pub struct CM0_INT_CTL4_SPEC;
impl crate::RegisterSpec for CM0_INT_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_int_ctl4::R](R) reader structure"]
impl crate::Readable for CM0_INT_CTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl4::W](W) writer structure"]
impl crate::Writable for CM0_INT_CTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_INT_CTL4 to value 0xf0f0_f0f0"]
impl crate::Resettable for CM0_INT_CTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0f0_f0f0
    }
}
