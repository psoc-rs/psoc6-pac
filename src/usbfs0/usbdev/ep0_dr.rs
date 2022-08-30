#[doc = "Register `EP0_DR[%s]` reader"]
pub struct R(crate::R<EP0_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP0_DR[%s]` writer"]
pub struct W(crate::W<EP0_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0_DR_SPEC>;
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
impl From<crate::W<EP0_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BYTE` reader - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DATA_BYTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BYTE` writer - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DATA_BYTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0_DR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&mut self) -> DATA_BYTE_W<0> {
        DATA_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control End point EP0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_dr](index.html) module"]
pub struct EP0_DR_SPEC;
impl crate::RegisterSpec for EP0_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep0_dr::R](R) reader structure"]
impl crate::Readable for EP0_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep0_dr::W](W) writer structure"]
impl crate::Writable for EP0_DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP0_DR[%s]
to value 0"]
impl crate::Resettable for EP0_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
