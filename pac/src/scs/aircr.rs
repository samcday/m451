#[doc = "Register `AIRCR` reader"]
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AIRCR_SPEC>> for R {
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIRCR` writer"]
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
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
impl core::convert::From<crate::W<AIRCR_SPEC>> for W {
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTCLRACTIVE` reader - Exception Active Status Clear Bit\\nSetting This Bit To 1 Will Clears All Active State Information For Fixed And Configurable Exceptions\\nThis bit is write only and can only be written when the core is halted.\\nNote: It is the debugger's responsibility to re-initialize the stack."]
pub struct VECTCLRACTIVE_R(crate::FieldReader<bool, bool>);
impl VECTCLRACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VECTCLRACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTCLRACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTCLRACTIVE` writer - Exception Active Status Clear Bit\\nSetting This Bit To 1 Will Clears All Active State Information For Fixed And Configurable Exceptions\\nThis bit is write only and can only be written when the core is halted.\\nNote: It is the debugger's responsibility to re-initialize the stack."]
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SYSRESETREQ` reader - System Reset Request\\nWriting This Bit to 1 Will Cause A Reset Signal To Be Asserted To The Chip And Indicate A Reset Is Requested\\nThis bit is write only and self-cleared as part of the reset sequence."]
pub struct SYSRESETREQ_R(crate::FieldReader<bool, bool>);
impl SYSRESETREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSRESETREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRESETREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRESETREQ` writer - System Reset Request\\nWriting This Bit to 1 Will Cause A Reset Signal To Be Asserted To The Chip And Indicate A Reset Is Requested\\nThis bit is write only and self-cleared as part of the reset sequence."]
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt Priority Grouping\\nThis field determines the Split Of Group priority from subpriority,"]
pub struct PRIGROUP_R(crate::FieldReader<u8, u8>);
impl PRIGROUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIGROUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIGROUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIGROUP` writer - Interrupt Priority Grouping\\nThis field determines the Split Of Group priority from subpriority,"]
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Data Endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    _0 = 0,
    #[doc = "1: Big-endian"]
    _1 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIANNESS` reader - Data Endianness"]
pub struct ENDIANNESS_R(crate::FieldReader<bool, ENDIANNESS_A>);
impl ENDIANNESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDIANNESS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::_0,
            true => ENDIANNESS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENDIANNESS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENDIANNESS_A::_1
    }
}
impl core::ops::Deref for ENDIANNESS_R {
    type Target = crate::FieldReader<bool, ENDIANNESS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDIANNESS` writer - Data Endianness"]
pub struct ENDIANNESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIANNESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIANNESS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDIANNESS_A::_0)
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDIANNESS_A::_1)
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
#[doc = "Field `VECTORKEY` reader - Register Access Key\\nWhen writing this register, this field should be 0x05FA, otherwise the write action will be unpredictable.\\nThe VECTORKEY filed is used to prevent accidental write to this register from resetting the system or clearing of the exception status."]
pub struct VECTORKEY_R(crate::FieldReader<u16, u16>);
impl VECTORKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        VECTORKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECTORKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VECTORKEY` writer - Register Access Key\\nWhen writing this register, this field should be 0x05FA, otherwise the write action will be unpredictable.\\nThe VECTORKEY filed is used to prevent accidental write to this register from resetting the system or clearing of the exception status."]
pub struct VECTORKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTORKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Exception Active Status Clear Bit\\nSetting This Bit To 1 Will Clears All Active State Information For Fixed And Configurable Exceptions\\nThis bit is write only and can only be written when the core is halted.\\nNote: It is the debugger's responsibility to re-initialize the stack."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Reset Request\\nWriting This Bit to 1 Will Cause A Reset Signal To Be Asserted To The Chip And Indicate A Reset Is Requested\\nThis bit is write only and self-cleared as part of the reset sequence."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt Priority Grouping\\nThis field determines the Split Of Group priority from subpriority,"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Data Endianness"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register Access Key\\nWhen writing this register, this field should be 0x05FA, otherwise the write action will be unpredictable.\\nThe VECTORKEY filed is used to prevent accidental write to this register from resetting the system or clearing of the exception status."]
    #[inline(always)]
    pub fn vectorkey(&self) -> VECTORKEY_R {
        VECTORKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Exception Active Status Clear Bit\\nSetting This Bit To 1 Will Clears All Active State Information For Fixed And Configurable Exceptions\\nThis bit is write only and can only be written when the core is halted.\\nNote: It is the debugger's responsibility to re-initialize the stack."]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    #[doc = "Bit 2 - System Reset Request\\nWriting This Bit to 1 Will Cause A Reset Signal To Be Asserted To The Chip And Indicate A Reset Is Requested\\nThis bit is write only and self-cleared as part of the reset sequence."]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt Priority Grouping\\nThis field determines the Split Of Group priority from subpriority,"]
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    #[doc = "Bit 15 - Data Endianness"]
    #[inline(always)]
    pub fn endianness(&mut self) -> ENDIANNESS_W {
        ENDIANNESS_W { w: self }
    }
    #[doc = "Bits 16:31 - Register Access Key\\nWhen writing this register, this field should be 0x05FA, otherwise the write action will be unpredictable.\\nThe VECTORKEY filed is used to prevent accidental write to this register from resetting the system or clearing of the exception status."]
    #[inline(always)]
    pub fn vectorkey(&mut self) -> VECTORKEY_W {
        VECTORKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](index.html) module"]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aircr::R](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aircr::W](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfa05_0000
    }
}
