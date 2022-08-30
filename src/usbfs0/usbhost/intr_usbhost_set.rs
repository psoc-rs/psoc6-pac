#[doc = "Register `INTR_USBHOST_SET` reader"]
pub struct R(crate::R<INTR_USBHOST_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_USBHOST_SET` writer"]
pub struct W(crate::W<INTR_USBHOST_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_USBHOST_SET_SPEC>;
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
impl From<crate::W<INTR_USBHOST_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_USBHOST_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIRQS` reader - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SOFIRQS_R = crate::BitReader<bool>;
#[doc = "Field `SOFIRQS` writer - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SOFIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `DIRQS` reader - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DIRQS_R = crate::BitReader<bool>;
#[doc = "Field `DIRQS` writer - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `CNNIRQS` reader - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CNNIRQS_R = crate::BitReader<bool>;
#[doc = "Field `CNNIRQS` writer - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CNNIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `CMPIRQS` reader - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CMPIRQS_R = crate::BitReader<bool>;
#[doc = "Field `CMPIRQS` writer - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CMPIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `URIRQS` reader - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type URIRQS_R = crate::BitReader<bool>;
#[doc = "Field `URIRQS` writer - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type URIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `RWKIRQS` reader - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RWKIRQS_R = crate::BitReader<bool>;
#[doc = "Field `RWKIRQS` writer - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RWKIRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `RSVD_6` reader - BCNFTEST interrupt. This bit is test bit"]
pub type RSVD_6_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_6` writer - BCNFTEST interrupt. This bit is test bit"]
pub type RSVD_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
#[doc = "Field `TCANS` reader - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TCANS_R = crate::BitReader<bool>;
#[doc = "Field `TCANS` writer - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TCANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&self) -> SOFIRQS_R {
        SOFIRQS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&self) -> DIRQS_R {
        DIRQS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&self) -> CNNIRQS_R {
        CNNIRQS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&self) -> CMPIRQS_R {
        CMPIRQS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&self) -> URIRQS_R {
        URIRQS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&self) -> RWKIRQS_R {
        RWKIRQS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&self) -> TCANS_R {
        TCANS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&mut self) -> SOFIRQS_W<0> {
        SOFIRQS_W::new(self)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&mut self) -> DIRQS_W<1> {
        DIRQS_W::new(self)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&mut self) -> CNNIRQS_W<2> {
        CNNIRQS_W::new(self)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&mut self) -> CMPIRQS_W<3> {
        CMPIRQS_W::new(self)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&mut self) -> URIRQS_W<4> {
        URIRQS_W::new(self)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&mut self) -> RWKIRQS_W<5> {
        RWKIRQS_W::new(self)
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<6> {
        RSVD_6_W::new(self)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&mut self) -> TCANS_W<7> {
        TCANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_set](index.html) module"]
pub struct INTR_USBHOST_SET_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_set::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_set::W](W) writer structure"]
impl crate::Writable for INTR_USBHOST_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_USBHOST_SET to value 0"]
impl crate::Resettable for INTR_USBHOST_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
