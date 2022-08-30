#[doc = "Register `MCWDT_LOCK` reader"]
pub struct R(crate::R<MCWDT_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_LOCK` writer"]
pub struct W(crate::W<MCWDT_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_LOCK_SPEC>;
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
impl From<crate::W<MCWDT_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCWDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<MCWDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MCWDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCWDT_LOCK` reader - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type MCWDT_LOCK_R = crate::FieldReader<u8, MCWDT_LOCK_A>;
impl MCWDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCWDT_LOCK_A {
        match self.bits {
            0 => MCWDT_LOCK_A::NO_CHG,
            1 => MCWDT_LOCK_A::CLR0,
            2 => MCWDT_LOCK_A::CLR1,
            3 => MCWDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHG`"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == MCWDT_LOCK_A::NO_CHG
    }
    #[doc = "Checks if the value of the field is `CLR0`"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR0
    }
    #[doc = "Checks if the value of the field is `CLR1`"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR1
    }
    #[doc = "Checks if the value of the field is `SET01`"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == MCWDT_LOCK_A::SET01
    }
}
#[doc = "Field `MCWDT_LOCK` writer - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
pub type MCWDT_LOCK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCWDT_LOCK_SPEC, u8, MCWDT_LOCK_A, 2, O>;
impl<'a, const O: u8> MCWDT_LOCK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::SET01)
    }
}
impl R {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&self) -> MCWDT_LOCK_R {
        MCWDT_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&mut self) -> MCWDT_LOCK_W<30> {
        MCWDT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Counter Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_lock](index.html) module"]
pub struct MCWDT_LOCK_SPEC;
impl crate::RegisterSpec for MCWDT_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_lock::R](R) reader structure"]
impl crate::Readable for MCWDT_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_lock::W](W) writer structure"]
impl crate::Writable for MCWDT_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_LOCK to value 0"]
impl crate::Resettable for MCWDT_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
