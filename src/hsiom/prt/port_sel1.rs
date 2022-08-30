#[doc = "Register `PORT_SEL1` reader"]
pub struct R(crate::R<PORT_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL1` writer"]
pub struct W(crate::W<PORT_SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL1_SPEC>;
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
impl From<crate::W<PORT_SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO4_SEL` reader - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type IO4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO4_SEL` writer - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type IO4_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `IO5_SEL` reader - Selects connection for IO pin 5 route."]
pub type IO5_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO5_SEL` writer - Selects connection for IO pin 5 route."]
pub type IO5_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `IO6_SEL` reader - Selects connection for IO pin 6 route."]
pub type IO6_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO6_SEL` writer - Selects connection for IO pin 6 route."]
pub type IO6_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `IO7_SEL` reader - Selects connection for IO pin 7 route."]
pub type IO7_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO7_SEL` writer - Selects connection for IO pin 7 route."]
pub type IO7_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_SEL1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub fn io4_sel(&self) -> IO4_SEL_R {
        IO4_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn io5_sel(&self) -> IO5_SEL_R {
        IO5_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn io6_sel(&self) -> IO6_SEL_R {
        IO6_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn io7_sel(&self) -> IO7_SEL_R {
        IO7_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub fn io4_sel(&mut self) -> IO4_SEL_W<0> {
        IO4_SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn io5_sel(&mut self) -> IO5_SEL_W<8> {
        IO5_SEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn io6_sel(&mut self) -> IO6_SEL_W<16> {
        IO6_SEL_W::new(self)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn io7_sel(&mut self) -> IO7_SEL_W<24> {
        IO7_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel1](index.html) module"]
pub struct PORT_SEL1_SPEC;
impl crate::RegisterSpec for PORT_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel1::R](R) reader structure"]
impl crate::Readable for PORT_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel1::W](W) writer structure"]
impl crate::Writable for PORT_SEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_SEL1 to value 0"]
impl crate::Resettable for PORT_SEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
