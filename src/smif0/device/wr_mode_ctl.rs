#[doc = "Register `WR_MODE_CTL` reader"]
pub struct R(crate::R<WR_MODE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_MODE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_MODE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_MODE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_MODE_CTL` writer"]
pub struct W(crate::W<WR_MODE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_MODE_CTL_SPEC>;
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
impl From<crate::W<WR_MODE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_MODE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Mode byte code."]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - Mode byte code."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_MODE_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WR_MODE_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRESENT` reader - Presence of mode field: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `PRESENT` writer - Presence of mode field: '0': not present '1': present"]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WR_MODE_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Mode byte code."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode byte code."]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&mut self) -> PRESENT_W<31> {
        PRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_mode_ctl](index.html) module"]
pub struct WR_MODE_CTL_SPEC;
impl crate::RegisterSpec for WR_MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_mode_ctl::R](R) reader structure"]
impl crate::Readable for WR_MODE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_mode_ctl::W](W) writer structure"]
impl crate::Writable for WR_MODE_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_MODE_CTL to value 0"]
impl crate::Resettable for WR_MODE_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
