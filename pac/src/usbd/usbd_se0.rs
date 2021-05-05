#[doc = "Register `USBD_SE0` reader"]
pub struct R(crate::R<USBD_SE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_SE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBD_SE0_SPEC>> for R {
    fn from(reader: crate::R<USBD_SE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBD_SE0` writer"]
pub struct W(crate::W<USBD_SE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBD_SE0_SPEC>;
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
impl core::convert::From<crate::W<USBD_SE0_SPEC>> for W {
    fn from(writer: crate::W<USBD_SE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Drive Single Ended Zero in USB Bus\\nThe Single Ended Zero (SE0) is when both lines (USB_D+ and USB_D-) are being pulled low.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE0_A {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: Force USB PHY transceiver to drive SE0"]
    _1 = 1,
}
impl From<SE0_A> for bool {
    #[inline(always)]
    fn from(variant: SE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE0` reader - Drive Single Ended Zero in USB Bus\\nThe Single Ended Zero (SE0) is when both lines (USB_D+ and USB_D-) are being pulled low."]
pub struct SE0_R(crate::FieldReader<bool, SE0_A>);
impl SE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE0_A {
        match self.bits {
            false => SE0_A::_0,
            true => SE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SE0_A::_1
    }
}
impl core::ops::Deref for SE0_R {
    type Target = crate::FieldReader<bool, SE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE0` writer - Drive Single Ended Zero in USB Bus\\nThe Single Ended Zero (SE0) is when both lines (USB_D+ and USB_D-) are being pulled low."]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SE0_A::_0)
    }
    #[doc = "Force USB PHY transceiver to drive SE0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SE0_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Drive Single Ended Zero in USB Bus\\nThe Single Ended Zero (SE0) is when both lines (USB_D+ and USB_D-) are being pulled low."]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Single Ended Zero in USB Bus\\nThe Single Ended Zero (SE0) is when both lines (USB_D+ and USB_D-) are being pulled low."]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Drive SE0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_se0](index.html) module"]
pub struct USBD_SE0_SPEC;
impl crate::RegisterSpec for USBD_SE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_se0::R](R) reader structure"]
impl crate::Readable for USBD_SE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbd_se0::W](W) writer structure"]
impl crate::Writable for USBD_SE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBD_SE0 to value 0x01"]
impl crate::Resettable for USBD_SE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
