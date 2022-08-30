#[doc = "Register `BUFF_CTL` reader"]
pub struct R(crate::R<BUFF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFF_CTL` writer"]
pub struct W(crate::W<BUFF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFF_CTL_SPEC>;
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
impl From<crate::W<BUFF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_BUFF` reader - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WRITE_BUFF_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_BUFF` writer - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WRITE_BUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUFF_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&self) -> WRITE_BUFF_R {
        WRITE_BUFF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&mut self) -> WRITE_BUFF_W<0> {
        WRITE_BUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buff_ctl](index.html) module"]
pub struct BUFF_CTL_SPEC;
impl crate::RegisterSpec for BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buff_ctl::R](R) reader structure"]
impl crate::Readable for BUFF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buff_ctl::W](W) writer structure"]
impl crate::Writable for BUFF_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUFF_CTL to value 0x01"]
impl crate::Resettable for BUFF_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
