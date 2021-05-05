#[doc = "Register `CAN_IF2_MASK2` reader"]
pub struct R(crate::R<CAN_IF2_MASK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IF2_MASK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IF2_MASK2_SPEC>> for R {
    fn from(reader: crate::R<CAN_IF2_MASK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_IF2_MASK2` writer"]
pub struct W(crate::W<CAN_IF2_MASK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_IF2_MASK2_SPEC>;
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
impl core::convert::From<crate::W<CAN_IF2_MASK2_SPEC>> for W {
    fn from(writer: crate::W<CAN_IF2_MASK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Identifier Mask 28-16\n\nValue on reset: 8191"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum MSK_A {
    #[doc = "0: The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering"]
    _0 = 0,
    #[doc = "1: The corresponding identifier bit is used for acceptance filtering"]
    _1 = 1,
}
impl From<MSK_A> for u16 {
    #[inline(always)]
    fn from(variant: MSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `Msk` reader - Identifier Mask 28-16"]
pub struct MSK_R(crate::FieldReader<u16, MSK_A>);
impl MSK_R {
    pub(crate) fn new(bits: u16) -> Self {
        MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSK_A> {
        match self.bits {
            0 => Some(MSK_A::_0),
            1 => Some(MSK_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSK_A::_1
    }
}
impl core::ops::Deref for MSK_R {
    type Target = crate::FieldReader<u16, MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Msk` writer - Identifier Mask 28-16"]
pub struct MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the identifier of the message object cannot inhibit the match in the acceptance filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSK_A::_0)
    }
    #[doc = "The corresponding identifier bit is used for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSK_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Mask Message Direction\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIR_A {
    #[doc = "0: The message direction bit (Dir (CAN_IFn_ARB2\\[13\\])) has no effect on the acceptance filtering"]
    _0 = 0,
    #[doc = "1: The message direction bit (Dir) is used for acceptance filtering"]
    _1 = 1,
}
impl From<MDIR_A> for bool {
    #[inline(always)]
    fn from(variant: MDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDir` reader - Mask Message Direction"]
pub struct MDIR_R(crate::FieldReader<bool, MDIR_A>);
impl MDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIR_A {
        match self.bits {
            false => MDIR_A::_0,
            true => MDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MDIR_A::_1
    }
}
impl core::ops::Deref for MDIR_R {
    type Target = crate::FieldReader<bool, MDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDir` writer - Mask Message Direction"]
pub struct MDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The message direction bit (Dir (CAN_IFn_ARB2\\[13\\])) has no effect on the acceptance filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIR_A::_0)
    }
    #[doc = "The message direction bit (Dir) is used for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Mask Extended Identifier\\nNote: When 11-bit ('standard') Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18 (CAN_IFn_ARB2\\[12:2\\]). For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 (CAN_IFn_MASK2\\[12:2\\]) are considered.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MXTD_A {
    #[doc = "0: The extended identifier bit (IDE) has no effect on the acceptance filtering"]
    _0 = 0,
    #[doc = "1: The extended identifier bit (IDE) is used for acceptance filtering"]
    _1 = 1,
}
impl From<MXTD_A> for bool {
    #[inline(always)]
    fn from(variant: MXTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MXtd` reader - Mask Extended Identifier\\nNote: When 11-bit ('standard') Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18 (CAN_IFn_ARB2\\[12:2\\]). For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 (CAN_IFn_MASK2\\[12:2\\]) are considered."]
pub struct MXTD_R(crate::FieldReader<bool, MXTD_A>);
impl MXTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MXTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MXTD_A {
        match self.bits {
            false => MXTD_A::_0,
            true => MXTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MXTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MXTD_A::_1
    }
}
impl core::ops::Deref for MXTD_R {
    type Target = crate::FieldReader<bool, MXTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MXtd` writer - Mask Extended Identifier\\nNote: When 11-bit ('standard') Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18 (CAN_IFn_ARB2\\[12:2\\]). For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 (CAN_IFn_MASK2\\[12:2\\]) are considered."]
pub struct MXTD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MXTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The extended identifier bit (IDE) has no effect on the acceptance filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MXTD_A::_0)
    }
    #[doc = "The extended identifier bit (IDE) is used for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MXTD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Identifier Mask 28-16"]
    #[inline(always)]
    pub fn msk(&self) -> MSK_R {
        MSK_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask Extended Identifier\\nNote: When 11-bit ('standard') Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18 (CAN_IFn_ARB2\\[12:2\\]). For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 (CAN_IFn_MASK2\\[12:2\\]) are considered."]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier Mask 28-16"]
    #[inline(always)]
    pub fn msk(&mut self) -> MSK_W {
        MSK_W { w: self }
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&mut self) -> MDIR_W {
        MDIR_W { w: self }
    }
    #[doc = "Bit 15 - Mask Extended Identifier\\nNote: When 11-bit ('standard') Identifiers are used for a Message Object, the identifiers of received Data Frames are written into bits ID28 to ID18 (CAN_IFn_ARB2\\[12:2\\]). For acceptance filtering, only these bits together with mask bits Msk28 to Msk18 (CAN_IFn_MASK2\\[12:2\\]) are considered."]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MXTD_W {
        MXTD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFn Mask 2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_if2_mask2](index.html) module"]
pub struct CAN_IF2_MASK2_SPEC;
impl crate::RegisterSpec for CAN_IF2_MASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_if2_mask2::R](R) reader structure"]
impl crate::Readable for CAN_IF2_MASK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_if2_mask2::W](W) writer structure"]
impl crate::Writable for CAN_IF2_MASK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_IF2_MASK2 to value 0xffff"]
impl crate::Resettable for CAN_IF2_MASK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
