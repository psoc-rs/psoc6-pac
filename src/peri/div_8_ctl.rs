#[doc = "Register `DIV_8_CTL[%s]` reader"]
pub struct R(crate::R<DIV_8_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_8_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_8_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_8_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_8_CTL[%s]` writer"]
pub struct W(crate::W<DIV_8_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_8_CTL_SPEC>;
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
impl From<crate::W<DIV_8_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_8_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `INT8_DIV` reader - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT8_DIV` writer - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIV_8_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note: this type of divider does NOT allow for a fractional division. For the generation of a divided clock, the integer division range is restricted to \\[2, 256\\]. For the generation of a 50/50 percent duty cycle digital divided clock, the integer division range is resticited to even numbers in the range \\[2, 256\\]. The generation of a 50/50 percent duty cycle analog divided clock has no restrictions. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&mut self) -> INT8_DIV_W<8> {
        INT8_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider control register (for 8.0 divider)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_8_ctl](index.html) module"]
pub struct DIV_8_CTL_SPEC;
impl crate::RegisterSpec for DIV_8_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_8_ctl::R](R) reader structure"]
impl crate::Readable for DIV_8_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_8_ctl::W](W) writer structure"]
impl crate::Writable for DIV_8_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_8_CTL[%s]
to value 0"]
impl crate::Resettable for DIV_8_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
