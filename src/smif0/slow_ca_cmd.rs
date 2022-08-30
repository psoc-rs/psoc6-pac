#[doc = "Register `SLOW_CA_CMD` reader"]
pub struct R(crate::R<SLOW_CA_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_CA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_CA_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_CA_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_CA_CMD` writer"]
pub struct W(crate::W<SLOW_CA_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_CA_CMD_SPEC>;
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
impl From<crate::W<SLOW_CA_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_CA_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLOW_CA_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<0> {
        INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_ca_cmd](index.html) module"]
pub struct SLOW_CA_CMD_SPEC;
impl crate::RegisterSpec for SLOW_CA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_ca_cmd::R](R) reader structure"]
impl crate::Readable for SLOW_CA_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_ca_cmd::W](W) writer structure"]
impl crate::Writable for SLOW_CA_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLOW_CA_CMD to value 0"]
impl crate::Resettable for SLOW_CA_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
