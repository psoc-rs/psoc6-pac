#[doc = "Reader of register BUF_SIZE"]
pub type R = crate::R<u32, super::BUF_SIZE>;
#[doc = "Writer for register BUF_SIZE"]
pub type W = crate::W<u32, super::BUF_SIZE>;
#[doc = "Register BUF_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_BUF`"]
pub type IN_BUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN_BUF`"]
pub struct IN_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `OUT_BUF`"]
pub type OUT_BUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_BUF`"]
pub struct OUT_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&self) -> IN_BUF_R {
        IN_BUF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&self) -> OUT_BUF_R {
        OUT_BUF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&mut self) -> IN_BUF_W {
        IN_BUF_W { w: self }
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&mut self) -> OUT_BUF_W {
        OUT_BUF_W { w: self }
    }
}
