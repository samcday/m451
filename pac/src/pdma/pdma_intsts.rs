#[doc = "Register `PDMA_INTSTS` reader"]
pub struct R(crate::R<PDMA_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_INTSTS_SPEC>> for R {
    fn from(reader: crate::R<PDMA_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_INTSTS` writer"]
pub struct W(crate::W<PDMA_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_INTSTS_SPEC>;
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
impl core::convert::From<crate::W<PDMA_INTSTS_SPEC>> for W {
    fn from(writer: crate::W<PDMA_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDMA Read/Write Target Abort Interrupt Flag (Read-only)\\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTIF_A {
    #[doc = "0: No AHB bus ERROR response received"]
    _0 = 0,
    #[doc = "1: AHB bus ERROR response received"]
    _1 = 1,
}
impl From<ABTIF_A> for bool {
    #[inline(always)]
    fn from(variant: ABTIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTIF` reader - PDMA Read/Write Target Abort Interrupt Flag (Read-only)\\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
pub struct ABTIF_R(crate::FieldReader<bool, ABTIF_A>);
impl ABTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABTIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTIF_A {
        match self.bits {
            false => ABTIF_A::_0,
            true => ABTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABTIF_A::_1
    }
}
impl core::ops::Deref for ABTIF_R {
    type Target = crate::FieldReader<bool, ABTIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABTIF` writer - PDMA Read/Write Target Abort Interrupt Flag (Read-only)\\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
pub struct ABTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AHB bus ERROR response received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABTIF_A::_0)
    }
    #[doc = "AHB bus ERROR response received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABTIF_A::_1)
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
#[doc = "Transfer Done Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIF_A {
    #[doc = "0: Not finished yet"]
    _0 = 0,
    #[doc = "1: PDMA channel has finished transmission"]
    _1 = 1,
}
impl From<TDIF_A> for bool {
    #[inline(always)]
    fn from(variant: TDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIF` reader - Transfer Done Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer."]
pub struct TDIF_R(crate::FieldReader<bool, TDIF_A>);
impl TDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIF_A {
        match self.bits {
            false => TDIF_A::_0,
            true => TDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDIF_A::_1
    }
}
impl core::ops::Deref for TDIF_R {
    type Target = crate::FieldReader<bool, TDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Table Empty Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished each table transmission and the operation is Stop mode. User can read TEIF register to indicate which channel finished transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF_A {
    #[doc = "0: PDMA channel transfer is not finished"]
    _0 = 0,
    #[doc = "1: PDMA channel transfer is finished and the operation is in idle state"]
    _1 = 1,
}
impl From<TEIF_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF` reader - Table Empty Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished each table transmission and the operation is Stop mode. User can read TEIF register to indicate which channel finished transfer."]
pub struct TEIF_R(crate::FieldReader<bool, TEIF_A>);
impl TEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF_A {
        match self.bits {
            false => TEIF_A::_0,
            true => TEIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TEIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TEIF_A::_1
    }
}
impl core::ops::Deref for TEIF_R {
    type Target = crate::FieldReader<bool, TEIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Request Time-out Flag for Each Channel \\[N\\]
(M45xD/M45xC Only)\\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOCn, user can write 1 to clear these bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REQTOFN_A {
    #[doc = "0: No request time-out"]
    _0 = 0,
    #[doc = "1: Peripheral request time-out"]
    _1 = 1,
}
impl From<REQTOFN_A> for u8 {
    #[inline(always)]
    fn from(variant: REQTOFN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REQTOFn` reader - Request Time-out Flag for Each Channel \\[N\\]
(M45xD/M45xC Only)\\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOCn, user can write 1 to clear these bits."]
pub struct REQTOFN_R(crate::FieldReader<u8, REQTOFN_A>);
impl REQTOFN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REQTOFN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQTOFN_A> {
        match self.bits {
            0 => Some(REQTOFN_A::_0),
            1 => Some(REQTOFN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == REQTOFN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == REQTOFN_A::_1
    }
}
impl core::ops::Deref for REQTOFN_R {
    type Target = crate::FieldReader<u8, REQTOFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQTOFn` writer - Request Time-out Flag for Each Channel \\[N\\]
(M45xD/M45xC Only)\\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOCn, user can write 1 to clear these bits."]
pub struct REQTOFN_W<'a> {
    w: &'a mut W,
}
impl<'a> REQTOFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQTOFN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No request time-out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REQTOFN_A::_0)
    }
    #[doc = "Peripheral request time-out"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REQTOFN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Flag (Read-only)\\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
    #[inline(always)]
    pub fn abtif(&self) -> ABTIF_R {
        ABTIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Done Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished transmission; User can read PDMA_TDSTS register to indicate which channel finished transfer."]
    #[inline(always)]
    pub fn tdif(&self) -> TDIF_R {
        TDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Table Empty Interrupt Flag (Read Only)\\nThis bit indicates that PDMA controller has finished each table transmission and the operation is Stop mode. User can read TEIF register to indicate which channel finished transfer."]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Request Time-out Flag for Each Channel \\[N\\]
(M45xD/M45xC Only)\\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOCn, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn reqtofn(&self) -> REQTOFN_R {
        REQTOFN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PDMA Read/Write Target Abort Interrupt Flag (Read-only)\\nThis bit indicates that PDMA has target abort error; Software can read PDMA_ABTSTS register to find which channel has target abort error."]
    #[inline(always)]
    pub fn abtif(&mut self) -> ABTIF_W {
        ABTIF_W { w: self }
    }
    #[doc = "Bits 8:15 - Request Time-out Flag for Each Channel \\[N\\]
(M45xD/M45xC Only)\\nThis flag indicates that PDMA controller has waited peripheral request for a period defined by PDMA_TOCn, user can write 1 to clear these bits."]
    #[inline(always)]
    pub fn reqtofn(&mut self) -> REQTOFN_W {
        REQTOFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_intsts](index.html) module"]
pub struct PDMA_INTSTS_SPEC;
impl crate::RegisterSpec for PDMA_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_intsts::R](R) reader structure"]
impl crate::Readable for PDMA_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_intsts::W](W) writer structure"]
impl crate::Writable for PDMA_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_INTSTS to value 0"]
impl crate::Resettable for PDMA_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
