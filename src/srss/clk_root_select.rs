#[doc = "Reader of register CLK_ROOT_SELECT[%s]"]
pub type R = crate::R<u32, super::CLK_ROOT_SELECT>;
#[doc = "Writer for register CLK_ROOT_SELECT[%s]"]
pub type W = crate::W<u32, super::CLK_ROOT_SELECT>;
#[doc = "Register CLK_ROOT_SELECT[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_ROOT_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects a clock path as the root of HFCLK<k> and for SRSS DSI input <k>. Use CLK_SELECT_PATH\\[i\\] to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\] register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROOT_MUX_A {
    #[doc = "0: Select PATH0 (can be configured for FLL)"]
    PATH0,
    #[doc = "1: Select PATH1 (can be configured for PLL0, if available in the product)"]
    PATH1,
    #[doc = "2: Select PATH2 (can be configured for PLL1, if available in the product)"]
    PATH2,
    #[doc = "3: Select PATH3 (can be configured for PLL2, if available in the product)"]
    PATH3,
    #[doc = "4: Select PATH4 (can be configured for PLL3, if available in the product)"]
    PATH4,
    #[doc = "5: Select PATH5 (can be configured for PLL4, if available in the product)"]
    PATH5,
    #[doc = "6: Select PATH6 (can be configured for PLL5, if available in the product)"]
    PATH6,
    #[doc = "7: Select PATH7 (can be configured for PLL6, if available in the product)"]
    PATH7,
    #[doc = "8: Select PATH8 (can be configured for PLL7, if available in the product)"]
    PATH8,
    #[doc = "9: Select PATH9 (can be configured for PLL8, if available in the product)"]
    PATH9,
    #[doc = "10: Select PATH10 (can be configured for PLL9, if available in the product)"]
    PATH10,
    #[doc = "11: Select PATH11 (can be configured for PLL10, if available in the product)"]
    PATH11,
    #[doc = "12: Select PATH12 (can be configured for PLL11, if available in the product)"]
    PATH12,
    #[doc = "13: Select PATH13 (can be configured for PLL12, if available in the product)"]
    PATH13,
    #[doc = "14: Select PATH14 (can be configured for PLL13, if available in the product)"]
    PATH14,
    #[doc = "15: Select PATH15 (can be configured for PLL14, if available in the product)"]
    PATH15,
}
impl From<ROOT_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: ROOT_MUX_A) -> Self {
        match variant {
            ROOT_MUX_A::PATH0 => 0,
            ROOT_MUX_A::PATH1 => 1,
            ROOT_MUX_A::PATH2 => 2,
            ROOT_MUX_A::PATH3 => 3,
            ROOT_MUX_A::PATH4 => 4,
            ROOT_MUX_A::PATH5 => 5,
            ROOT_MUX_A::PATH6 => 6,
            ROOT_MUX_A::PATH7 => 7,
            ROOT_MUX_A::PATH8 => 8,
            ROOT_MUX_A::PATH9 => 9,
            ROOT_MUX_A::PATH10 => 10,
            ROOT_MUX_A::PATH11 => 11,
            ROOT_MUX_A::PATH12 => 12,
            ROOT_MUX_A::PATH13 => 13,
            ROOT_MUX_A::PATH14 => 14,
            ROOT_MUX_A::PATH15 => 15,
        }
    }
}
#[doc = "Reader of field `ROOT_MUX`"]
pub type ROOT_MUX_R = crate::R<u8, ROOT_MUX_A>;
impl ROOT_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROOT_MUX_A {
        match self.bits {
            0 => ROOT_MUX_A::PATH0,
            1 => ROOT_MUX_A::PATH1,
            2 => ROOT_MUX_A::PATH2,
            3 => ROOT_MUX_A::PATH3,
            4 => ROOT_MUX_A::PATH4,
            5 => ROOT_MUX_A::PATH5,
            6 => ROOT_MUX_A::PATH6,
            7 => ROOT_MUX_A::PATH7,
            8 => ROOT_MUX_A::PATH8,
            9 => ROOT_MUX_A::PATH9,
            10 => ROOT_MUX_A::PATH10,
            11 => ROOT_MUX_A::PATH11,
            12 => ROOT_MUX_A::PATH12,
            13 => ROOT_MUX_A::PATH13,
            14 => ROOT_MUX_A::PATH14,
            15 => ROOT_MUX_A::PATH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PATH0`"]
    #[inline(always)]
    pub fn is_path0(&self) -> bool {
        *self == ROOT_MUX_A::PATH0
    }
    #[doc = "Checks if the value of the field is `PATH1`"]
    #[inline(always)]
    pub fn is_path1(&self) -> bool {
        *self == ROOT_MUX_A::PATH1
    }
    #[doc = "Checks if the value of the field is `PATH2`"]
    #[inline(always)]
    pub fn is_path2(&self) -> bool {
        *self == ROOT_MUX_A::PATH2
    }
    #[doc = "Checks if the value of the field is `PATH3`"]
    #[inline(always)]
    pub fn is_path3(&self) -> bool {
        *self == ROOT_MUX_A::PATH3
    }
    #[doc = "Checks if the value of the field is `PATH4`"]
    #[inline(always)]
    pub fn is_path4(&self) -> bool {
        *self == ROOT_MUX_A::PATH4
    }
    #[doc = "Checks if the value of the field is `PATH5`"]
    #[inline(always)]
    pub fn is_path5(&self) -> bool {
        *self == ROOT_MUX_A::PATH5
    }
    #[doc = "Checks if the value of the field is `PATH6`"]
    #[inline(always)]
    pub fn is_path6(&self) -> bool {
        *self == ROOT_MUX_A::PATH6
    }
    #[doc = "Checks if the value of the field is `PATH7`"]
    #[inline(always)]
    pub fn is_path7(&self) -> bool {
        *self == ROOT_MUX_A::PATH7
    }
    #[doc = "Checks if the value of the field is `PATH8`"]
    #[inline(always)]
    pub fn is_path8(&self) -> bool {
        *self == ROOT_MUX_A::PATH8
    }
    #[doc = "Checks if the value of the field is `PATH9`"]
    #[inline(always)]
    pub fn is_path9(&self) -> bool {
        *self == ROOT_MUX_A::PATH9
    }
    #[doc = "Checks if the value of the field is `PATH10`"]
    #[inline(always)]
    pub fn is_path10(&self) -> bool {
        *self == ROOT_MUX_A::PATH10
    }
    #[doc = "Checks if the value of the field is `PATH11`"]
    #[inline(always)]
    pub fn is_path11(&self) -> bool {
        *self == ROOT_MUX_A::PATH11
    }
    #[doc = "Checks if the value of the field is `PATH12`"]
    #[inline(always)]
    pub fn is_path12(&self) -> bool {
        *self == ROOT_MUX_A::PATH12
    }
    #[doc = "Checks if the value of the field is `PATH13`"]
    #[inline(always)]
    pub fn is_path13(&self) -> bool {
        *self == ROOT_MUX_A::PATH13
    }
    #[doc = "Checks if the value of the field is `PATH14`"]
    #[inline(always)]
    pub fn is_path14(&self) -> bool {
        *self == ROOT_MUX_A::PATH14
    }
    #[doc = "Checks if the value of the field is `PATH15`"]
    #[inline(always)]
    pub fn is_path15(&self) -> bool {
        *self == ROOT_MUX_A::PATH15
    }
}
#[doc = "Write proxy for field `ROOT_MUX`"]
pub struct ROOT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> ROOT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROOT_MUX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select PATH0 (can be configured for FLL)"]
    #[inline(always)]
    pub fn path0(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH0)
    }
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    #[inline(always)]
    pub fn path1(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH1)
    }
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    #[inline(always)]
    pub fn path2(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH2)
    }
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    #[inline(always)]
    pub fn path3(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH3)
    }
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    #[inline(always)]
    pub fn path4(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH4)
    }
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    #[inline(always)]
    pub fn path5(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH5)
    }
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    #[inline(always)]
    pub fn path6(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH6)
    }
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    #[inline(always)]
    pub fn path7(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH7)
    }
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    #[inline(always)]
    pub fn path8(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH8)
    }
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    #[inline(always)]
    pub fn path9(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH9)
    }
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    #[inline(always)]
    pub fn path10(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH10)
    }
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    #[inline(always)]
    pub fn path11(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH11)
    }
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    #[inline(always)]
    pub fn path12(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH12)
    }
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    #[inline(always)]
    pub fn path13(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH13)
    }
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    #[inline(always)]
    pub fn path14(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH14)
    }
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    #[inline(always)]
    pub fn path15(self) -> &'a mut W {
        self.variant(ROOT_MUX_A::PATH15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Selects predivider value for this clock root and DSI input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROOT_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8,
}
impl From<ROOT_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ROOT_DIV_A) -> Self {
        match variant {
            ROOT_DIV_A::NO_DIV => 0,
            ROOT_DIV_A::DIV_BY_2 => 1,
            ROOT_DIV_A::DIV_BY_4 => 2,
            ROOT_DIV_A::DIV_BY_8 => 3,
        }
    }
}
#[doc = "Reader of field `ROOT_DIV`"]
pub type ROOT_DIV_R = crate::R<u8, ROOT_DIV_A>;
impl ROOT_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROOT_DIV_A {
        match self.bits {
            0 => ROOT_DIV_A::NO_DIV,
            1 => ROOT_DIV_A::DIV_BY_2,
            2 => ROOT_DIV_A::DIV_BY_4,
            3 => ROOT_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == ROOT_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_8
    }
}
#[doc = "Write proxy for field `ROOT_DIV`"]
pub struct ROOT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ROOT_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROOT_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(ROOT_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(ROOT_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(ROOT_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(ROOT_DIV_A::DIV_BY_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK<k> and for SRSS DSI input <k>. Use CLK_SELECT_PATH\\[i\\] to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\] register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn root_mux(&self) -> ROOT_MUX_R {
        ROOT_MUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub fn root_div(&self) -> ROOT_DIV_R {
        ROOT_DIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK<k> and for SRSS DSI input <k>. Use CLK_SELECT_PATH\\[i\\] to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\] register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn root_mux(&mut self) -> ROOT_MUX_W {
        ROOT_MUX_W { w: self }
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub fn root_div(&mut self) -> ROOT_DIV_W {
        ROOT_DIV_W { w: self }
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
