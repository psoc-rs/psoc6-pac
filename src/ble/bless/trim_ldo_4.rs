#[doc = "Reader of register TRIM_LDO_4"]
pub type R = crate::R<u32, super::TRIM_LDO_4>;
#[doc = "Writer for register TRIM_LDO_4"]
pub type W = crate::W<u32, super::TRIM_LDO_4>;
#[doc = "Register TRIM_LDO_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM_LDO_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T_LDO`"]
pub type T_LDO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_LDO`"]
pub struct T_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> T_LDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - To debug post layout or post silicon"]
    #[inline(always)]
    pub fn t_ldo(&self) -> T_LDO_R {
        T_LDO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - To debug post layout or post silicon"]
    #[inline(always)]
    pub fn t_ldo(&mut self) -> T_LDO_W {
        T_LDO_W { w: self }
    }
}
