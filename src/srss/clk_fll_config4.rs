#[doc = "Reader of register CLK_FLL_CONFIG4"]
pub type R = crate::R<u32, super::CLK_FLL_CONFIG4>;
#[doc = "Writer for register CLK_FLL_CONFIG4"]
pub type W = crate::W<u32, super::CLK_FLL_CONFIG4>;
#[doc = "Register CLK_FLL_CONFIG4 `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CLK_FLL_CONFIG4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CCO_LIMIT`"]
pub type CCO_LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_LIMIT`"]
pub struct CCO_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Frequency range of CCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCO_RANGE_A {
    #[doc = "0: Target frequency is in range \\[48, 64) MHz"]
    RANGE0,
    #[doc = "1: Target frequency is in range \\[64, 85) MHz"]
    RANGE1,
    #[doc = "2: Target frequency is in range \\[85, 113) MHz"]
    RANGE2,
    #[doc = "3: Target frequency is in range \\[113, 150) MHz"]
    RANGE3,
    #[doc = "4: Target frequency is in range \\[150, 200\\] MHz"]
    RANGE4,
}
impl From<CCO_RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCO_RANGE_A) -> Self {
        match variant {
            CCO_RANGE_A::RANGE0 => 0,
            CCO_RANGE_A::RANGE1 => 1,
            CCO_RANGE_A::RANGE2 => 2,
            CCO_RANGE_A::RANGE3 => 3,
            CCO_RANGE_A::RANGE4 => 4,
        }
    }
}
#[doc = "Reader of field `CCO_RANGE`"]
pub type CCO_RANGE_R = crate::R<u8, CCO_RANGE_A>;
impl CCO_RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CCO_RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CCO_RANGE_A::RANGE0),
            1 => Val(CCO_RANGE_A::RANGE1),
            2 => Val(CCO_RANGE_A::RANGE2),
            3 => Val(CCO_RANGE_A::RANGE3),
            4 => Val(CCO_RANGE_A::RANGE4),
            i => Res(i),
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
#[doc = "Write proxy for field `CCO_RANGE`"]
pub struct CCO_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCO_RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = "Target frequency is in range \\[150, 200\\] MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(CCO_RANGE_A::RANGE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CCO_FREQ`"]
pub type CCO_FREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCO_FREQ`"]
pub struct CCO_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCO_HW_UPDATE_DIS`"]
pub type CCO_HW_UPDATE_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCO_HW_UPDATE_DIS`"]
pub struct CCO_HW_UPDATE_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_HW_UPDATE_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CCO_ENABLE`"]
pub type CCO_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCO_ENABLE`"]
pub struct CCO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&self) -> CCO_LIMIT_R {
        CCO_LIMIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&self) -> CCO_RANGE_R {
        CCO_RANGE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&self) -> CCO_FREQ_R {
        CCO_FREQ_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&self) -> CCO_HW_UPDATE_DIS_R {
        CCO_HW_UPDATE_DIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&self) -> CCO_ENABLE_R {
        CCO_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum CCO offset allowed (used to prevent FLL dynamics from selecting an CCO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn cco_limit(&mut self) -> CCO_LIMIT_W {
        CCO_LIMIT_W { w: self }
    }
    #[doc = "Bits 8:10 - Frequency range of CCO"]
    #[inline(always)]
    pub fn cco_range(&mut self) -> CCO_RANGE_W {
        CCO_RANGE_W { w: self }
    }
    #[doc = "Bits 16:24 - CCO frequency code. This is updated by HW when the FLL is enabled. It can be manually updated to use the CCO in an open loop configuration. The meaning of each frequency code depends on the range."]
    #[inline(always)]
    pub fn cco_freq(&mut self) -> CCO_FREQ_W {
        CCO_FREQ_W { w: self }
    }
    #[doc = "Bit 30 - Disable CCO frequency update by FLL hardware 0: Hardware update of CCO settings is allowed. Use this setting for normal FLL operation. 1: Hardware update of CCO settings is disabled. Use this setting for open-loop FLL operation."]
    #[inline(always)]
    pub fn cco_hw_update_dis(&mut self) -> CCO_HW_UPDATE_DIS_W {
        CCO_HW_UPDATE_DIS_W { w: self }
    }
    #[doc = "Bit 31 - Enable the CCO. It is required to enable the CCO before using the FLL. 0: Block is powered off 1: Block is powered on"]
    #[inline(always)]
    pub fn cco_enable(&mut self) -> CCO_ENABLE_W {
        CCO_ENABLE_W { w: self }
    }
}
