#[doc = "Reader of register BIST_ADDR_START"]
pub type R = crate::R<u32, super::BIST_ADDR_START>;
#[doc = "Writer for register BIST_ADDR_START"]
pub type W = crate::W<u32, super::BIST_ADDR_START>;
#[doc = "Register BIST_ADDR_START `reset()`'s with value 0"]
impl crate::ResetValue for super::BIST_ADDR_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COL_ADDR_START`"]
pub type COL_ADDR_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COL_ADDR_START`"]
pub struct COL_ADDR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_ADDR_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ROW_ADDR_START`"]
pub type ROW_ADDR_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ROW_ADDR_START`"]
pub struct ROW_ADDR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_ADDR_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub fn col_addr_start(&self) -> COL_ADDR_START_R {
        COL_ADDR_START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub fn row_addr_start(&self) -> ROW_ADDR_START_R {
        ROW_ADDR_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub fn col_addr_start(&mut self) -> COL_ADDR_START_W {
        COL_ADDR_START_W { w: self }
    }
    #[doc = "Bits 16:31 - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub fn row_addr_start(&mut self) -> ROW_ADDR_START_W {
        ROW_ADDR_START_W { w: self }
    }
}
