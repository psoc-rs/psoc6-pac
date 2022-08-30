#[doc = "Register `WL_CONNECTION_STATUS` reader"]
pub struct R(crate::R<WL_CONNECTION_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WL_CONNECTION_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WL_CONNECTION_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WL_CONNECTION_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WL_CONNECTION_STATUS` writer"]
pub struct W(crate::W<WL_CONNECTION_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WL_CONNECTION_STATUS_SPEC>;
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
impl From<crate::W<WL_CONNECTION_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WL_CONNECTION_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WL_ENTRY_CONNECTED` reader - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
pub type WL_ENTRY_CONNECTED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WL_ENTRY_CONNECTED` writer - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
pub type WL_ENTRY_CONNECTED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WL_CONNECTION_STATUS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
    #[inline(always)]
    pub fn wl_entry_connected(&self) -> WL_ENTRY_CONNECTED_R {
        WL_ENTRY_CONNECTED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
    #[inline(always)]
    pub fn wl_entry_connected(&mut self) -> WL_ENTRY_CONNECTED_W<0> {
        WL_ENTRY_CONNECTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "whitelist valid entry bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_connection_status](index.html) module"]
pub struct WL_CONNECTION_STATUS_SPEC;
impl crate::RegisterSpec for WL_CONNECTION_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wl_connection_status::R](R) reader structure"]
impl crate::Readable for WL_CONNECTION_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wl_connection_status::W](W) writer structure"]
impl crate::Writable for WL_CONNECTION_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WL_CONNECTION_STATUS to value 0"]
impl crate::Resettable for WL_CONNECTION_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
