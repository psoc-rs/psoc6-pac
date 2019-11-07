#[doc = "Reader of register PORT_SEL1"]
pub type R = crate::R<u32, super::PORT_SEL1>;
#[doc = "Writer for register PORT_SEL1"]
pub type W = crate::W<u32, super::PORT_SEL1>;
#[doc = "Register PORT_SEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PORT_SEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO4_SEL`"]
pub type IO4_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO4_SEL`"]
pub struct IO4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `IO5_SEL`"]
pub type IO5_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO5_SEL`"]
pub struct IO5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `IO6_SEL`"]
pub type IO6_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO6_SEL`"]
pub struct IO6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IO7_SEL`"]
pub type IO7_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO7_SEL`"]
pub struct IO7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
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
    pub fn io4_sel(&mut self) -> IO4_SEL_W {
        IO4_SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn io5_sel(&mut self) -> IO5_SEL_W {
        IO5_SEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn io6_sel(&mut self) -> IO6_SEL_W {
        IO6_SEL_W { w: self }
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn io7_sel(&mut self) -> IO7_SEL_W {
        IO7_SEL_W { w: self }
    }
}
