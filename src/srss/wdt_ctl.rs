#[doc = "Register `WDT_CTL` reader"]
pub struct R(crate::R<WDT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CTL` writer"]
pub struct W(crate::W<WDT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CTL_SPEC>;
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
impl From<crate::W<WDT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_EN` reader - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WDT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EN` writer - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDT_CTL_SPEC, bool, O>;
#[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<WDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDT_LOCK` reader - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WDT_LOCK_R = crate::FieldReader<u8, WDT_LOCK_A>;
impl WDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_LOCK_A {
        match self.bits {
            0 => WDT_LOCK_A::NO_CHG,
            1 => WDT_LOCK_A::CLR0,
            2 => WDT_LOCK_A::CLR1,
            3 => WDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHG`"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == WDT_LOCK_A::NO_CHG
    }
    #[doc = "Checks if the value of the field is `CLR0`"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == WDT_LOCK_A::CLR0
    }
    #[doc = "Checks if the value of the field is `CLR1`"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == WDT_LOCK_A::CLR1
    }
    #[doc = "Checks if the value of the field is `SET01`"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == WDT_LOCK_A::SET01
    }
}
#[doc = "Field `WDT_LOCK` writer - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WDT_LOCK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WDT_CTL_SPEC, u8, WDT_LOCK_A, 2, O>;
impl<'a, const O: u8> WDT_LOCK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::SET01)
    }
}
impl R {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W<0> {
        WDT_EN_W::new(self)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W<30> {
        WDT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctl](index.html) module"]
pub struct WDT_CTL_SPEC;
impl crate::RegisterSpec for WDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_ctl::R](R) reader structure"]
impl crate::Readable for WDT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_ctl::W](W) writer structure"]
impl crate::Writable for WDT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CTL to value 0xc000_0001"]
impl crate::Resettable for WDT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0001
    }
}
