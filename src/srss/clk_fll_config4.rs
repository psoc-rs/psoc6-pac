#[doc = "Register `CLK_FLL_CONFIG4` reader"]
pub struct R(crate::R<CLK_FLL_CONFIG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_FLL_CONFIG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_FLL_CONFIG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_FLL_CONFIG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_FLL_CONFIG4` writer"]
pub struct W(crate::W<CLK_FLL_CONFIG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_FLL_CONFIG4_SPEC>;
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
impl From<crate::W<CLK_FLL_CONFIG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_FLL_CONFIG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCO_LIMIT` reader - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CCO_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCO_LIMIT` writer - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
pub type CCO_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG4_SPEC, u8, u8, 8, O>;
#[doc = "Frequency range of CCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCO_RANGE_A {
    #[doc = "0: Target frequency is in range \\[48, 64) MHz"]
    RANGE0 = 0,
    #[doc = "1: Target frequency is in range \\[64, 85) MHz"]
    RANGE1 = 1,
    #[doc = "2: Target frequency is in range \\[85, 113) MHz"]
    RANGE2 = 2,
    #[doc = "3: Target frequency is in range \\[113, 150) MHz"]
    RANGE3 = 3,
    #[doc = "4: Target frequency is in range \\[150, 200\\]
MHz"]
    RANGE4 = 4,
}
impl From<CCO_RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCO_RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CCO_RANGE` reader - Frequency range of CCO"]
pub type CCO_RANGE_R = crate::FieldReader<u8, CCO_RANGE_A>;
impl CCO_RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCO_RANGE_A> {
        match self.bits {
            0 => Some(CCO_RANGE_A::RANGE0),
            1 => Some(CCO_RANGE_A::RANGE1),
            2 => Some(CCO_RANGE_A::RANGE2),
            3 => Some(CCO_RANGE_A::RANGE3),
            4 => Some(CCO_RANGE_A::RANGE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == CCO_RANGE_A::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == CCO_RANGE_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == CCO_RANGE_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == CCO_RANGE_A::RANGE3
    }
    #[doc = "Checks if the value of the field is `RANGE4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == CCO_RANGE_A::RANGE4
    }
}
#[doc = "Field `CCO_RANGE` writer - Frequency range of CCO"]
pub type CCO_RANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG4_SPEC, u8, CCO_RANGE_A, 3, O>;
impl<'a, const O: u8> CCO_RANGE_W<'a, O> {
    #[doc = "Target frequency is in range \\[48, 64) MHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE0)
    }
    #[doc = "Target frequency is in range \\[64, 85) MHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE1)
    }
    #[doc = "Target frequency is in range \\[85, 113) MHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE2)
    }
    #[doc = "Target frequency is in range \\[113, 150) MHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE3)
    }
    #[doc = "Target frequency is in range \\[150, 200\\]
MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE4)
    }
}
#[doc = "Field `CCO_FREQ` reader - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CCO_FREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCO_FREQ` writer - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
pub type CCO_FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG4_SPEC, u16, u16, 9, O>;
#[doc = "Field `CCO_HW_UPDATE_DIS` reader - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CCO_HW_UPDATE_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CCO_HW_UPDATE_DIS` writer - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
pub type CCO_HW_UPDATE_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_FLL_CONFIG4_SPEC, bool, O>;
#[doc = "Field `CCO_ENABLE` reader - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CCO_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CCO_ENABLE` writer - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
pub type CCO_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_FLL_CONFIG4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&self) -> CCO_LIMIT_R {
        CCO_LIMIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&self) -> CCO_RANGE_R {
        CCO_RANGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&self) -> CCO_FREQ_R {
        CCO_FREQ_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&self) -> CCO_HW_UPDATE_DIS_R {
        CCO_HW_UPDATE_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&self) -> CCO_ENABLE_R {
        CCO_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&mut self) -> CCO_LIMIT_W<0> {
        CCO_LIMIT_W::new(self)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&mut self) -> CCO_RANGE_W<8> {
        CCO_RANGE_W::new(self)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&mut self) -> CCO_FREQ_W<16> {
        CCO_FREQ_W::new(self)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&mut self) -> CCO_HW_UPDATE_DIS_W<30> {
        CCO_HW_UPDATE_DIS_W::new(self)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&mut self) -> CCO_ENABLE_W<31> {
        CCO_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLL Configuration Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config4](index.html) module"]
pub struct CLK_FLL_CONFIG4_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_fll_config4::R](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_fll_config4::W](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG4 to value 0xff"]
impl crate::Resettable for CLK_FLL_CONFIG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
