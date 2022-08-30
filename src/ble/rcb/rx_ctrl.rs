#[doc = "Register `RX_CTRL` reader"]
pub struct R(crate::R<RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL` writer"]
pub struct W(crate::W<RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL_SPEC>;
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
impl From<crate::W<RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL_SPEC>) -> Self {
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
pub type MSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Least significant bit first ('0') or most significant bit first ('1'). This field also affects the Address field When MSB_FIRST = 1, then \\[15:0\\]
is data and \\[(ADDR_WIDTH+15):16\\]
is used for address When MSB_FIRST = 0, then \\[15:0\\]
is for data. No address field"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new((self.bits & 1) != 0)
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl](index.html) module"]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ctrl::R](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x01"]
impl crate::Resettable for RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
