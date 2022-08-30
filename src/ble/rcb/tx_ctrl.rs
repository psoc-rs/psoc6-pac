#[doc = "Register `TX_CTRL` reader"]
pub struct R(crate::R<TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL` writer"]
pub struct W(crate::W<TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL_SPEC>;
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
impl From<crate::W<TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
pub type MSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTRL_SPEC, bool, O>;
#[doc = "Field `FIFO_RECONFIG` reader - Setting this bit, clears the FIFO and resets the pointer"]
pub type FIFO_RECONFIG_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_RECONFIG` writer - Setting this bit, clears the FIFO and resets the pointer"]
pub type FIFO_RECONFIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTRL_SPEC, bool, O>;
#[doc = "Field `TX_ENTRIES` reader - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
pub type TX_ENTRIES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_ENTRIES` writer - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
pub type TX_ENTRIES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&self) -> FIFO_RECONFIG_R {
        FIFO_RECONFIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&self) -> TX_ENTRIES_R {
        TX_ENTRIES_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<0> {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit, clears the FIFO and resets the pointer"]
    #[inline(always)]
    pub fn fifo_reconfig(&mut self) -> FIFO_RECONFIG_W<1> {
        FIFO_RECONFIG_W::new(self)
    }
    #[doc = "Bits 2:6 - This field determines the depth of the TX_FIFO. Allowed legal values are 8 and 16 only"]
    #[inline(always)]
    pub fn tx_entries(&mut self) -> TX_ENTRIES_W<2> {
        TX_ENTRIES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](index.html) module"]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl::R](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x21"]
impl crate::Resettable for TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
