#[doc = "Reader of register XTAL_CLK_DIV_CONFIG"]
pub type R = crate::R<u32, super::XTAL_CLK_DIV_CONFIG>;
#[doc = "Writer for register XTAL_CLK_DIV_CONFIG"]
pub type W = crate::W<u32, super::XTAL_CLK_DIV_CONFIG>;
#[doc = "Register XTAL_CLK_DIV_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL_CLK_DIV_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCLK_DIV`"]
pub type SYSCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCLK_DIV`"]
pub struct SYSCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `LLCLK_DIV`"]
pub type LLCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LLCLK_DIV`"]
pub struct LLCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LLCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn sysclk_div(&self) -> SYSCLK_DIV_R {
        SYSCLK_DIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn llclk_div(&self) -> LLCLK_DIV_R {
        LLCLK_DIV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn sysclk_div(&mut self) -> SYSCLK_DIV_W {
        SYSCLK_DIV_W { w: self }
    }
    #[doc = "Bits 2:3 - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn llclk_div(&mut self) -> LLCLK_DIV_W {
        LLCLK_DIV_W { w: self }
    }
}
