#[doc = "Register `SYS_SRAM_BISTCTL` reader"]
pub struct R(crate::R<SYS_SRAM_BISTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BISTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_SRAM_BISTCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_SRAM_BISTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BISTCTL` writer"]
pub struct W(crate::W<SYS_SRAM_BISTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BISTCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_SRAM_BISTCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_SRAM_BISTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1st SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_0000 ~0x2000_3FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIST0_A {
    #[doc = "0: system SRAM BIST Disabled"]
    _0 = 0,
    #[doc = "1: system SRAM BIST Enabled"]
    _1 = 1,
}
impl From<SRBIST0_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIST0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBIST0` reader - 1st SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_0000 ~0x2000_3FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST0_R(crate::FieldReader<bool, SRBIST0_A>);
impl SRBIST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBIST0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIST0_A {
        match self.bits {
            false => SRBIST0_A::_0,
            true => SRBIST0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBIST0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBIST0_A::_1
    }
}
impl core::ops::Deref for SRBIST0_R {
    type Target = crate::FieldReader<bool, SRBIST0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBIST0` writer - 1st SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_0000 ~0x2000_3FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBIST0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBIST0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system SRAM BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRBIST0_A::_0)
    }
    #[doc = "system SRAM BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRBIST0_A::_1)
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
#[doc = "2nd SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_4000 ~0x2000_7FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIST1_A {
    #[doc = "0: system SRAM BIST Disabled"]
    _0 = 0,
    #[doc = "1: system SRAM BIST Enabled"]
    _1 = 1,
}
impl From<SRBIST1_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBIST1` reader - 2nd SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_4000 ~0x2000_7FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST1_R(crate::FieldReader<bool, SRBIST1_A>);
impl SRBIST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRBIST1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIST1_A {
        match self.bits {
            false => SRBIST1_A::_0,
            true => SRBIST1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SRBIST1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SRBIST1_A::_1
    }
}
impl core::ops::Deref for SRBIST1_R {
    type Target = crate::FieldReader<bool, SRBIST1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRBIST1` writer - 2nd SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_4000 ~0x2000_7FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct SRBIST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBIST1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBIST1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system SRAM BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRBIST1_A::_0)
    }
    #[doc = "system SRAM BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRBIST1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "CACHE BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CACHE RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRBIST_A {
    #[doc = "0: system CACHE BIST Disabled"]
    _0 = 0,
    #[doc = "1: system CACHE BIST Enabled"]
    _1 = 1,
}
impl From<CRBIST_A> for bool {
    #[inline(always)]
    fn from(variant: CRBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRBIST` reader - CACHE BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CACHE RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CRBIST_R(crate::FieldReader<bool, CRBIST_A>);
impl CRBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRBIST_A {
        match self.bits {
            false => CRBIST_A::_0,
            true => CRBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRBIST_A::_1
    }
}
impl core::ops::Deref for CRBIST_R {
    type Target = crate::FieldReader<bool, CRBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRBIST` writer - CACHE BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CACHE RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CRBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system CACHE BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRBIST_A::_0)
    }
    #[doc = "system CACHE BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRBIST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "CAN BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CAN RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANBIST_A {
    #[doc = "0: system CAN BIST Disabled"]
    _0 = 0,
    #[doc = "1: system CAN BIST Enabled"]
    _1 = 1,
}
impl From<CANBIST_A> for bool {
    #[inline(always)]
    fn from(variant: CANBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CANBIST` reader - CAN BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CAN RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CANBIST_R(crate::FieldReader<bool, CANBIST_A>);
impl CANBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CANBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANBIST_A {
        match self.bits {
            false => CANBIST_A::_0,
            true => CANBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CANBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CANBIST_A::_1
    }
}
impl core::ops::Deref for CANBIST_R {
    type Target = crate::FieldReader<bool, CANBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CANBIST` writer - CAN BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CAN RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct CANBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> CANBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CANBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system CAN BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CANBIST_A::_0)
    }
    #[doc = "system CAN BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CANBIST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "USB BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for USB RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBBIST_A {
    #[doc = "0: system USB BIST Disabled"]
    _0 = 0,
    #[doc = "1: system USB BIST Enabled"]
    _1 = 1,
}
impl From<USBBIST_A> for bool {
    #[inline(always)]
    fn from(variant: USBBIST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBBIST` reader - USB BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for USB RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBBIST_R(crate::FieldReader<bool, USBBIST_A>);
impl USBBIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBIST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBBIST_A {
        match self.bits {
            false => USBBIST_A::_0,
            true => USBBIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBBIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBBIST_A::_1
    }
}
impl core::ops::Deref for USBBIST_R {
    type Target = crate::FieldReader<bool, USBBIST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBBIST` writer - USB BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for USB RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBBIST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "system USB BIST Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBBIST_A::_0)
    }
    #[doc = "system USB BIST Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBBIST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1st SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_0000 ~0x2000_3FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist0(&self) -> SRBIST0_R {
        SRBIST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 2nd SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_4000 ~0x2000_7FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist1(&self) -> SRBIST1_R {
        SRBIST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CACHE BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CACHE RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn crbist(&self) -> CRBIST_R {
        CRBIST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CAN BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CAN RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn canbist(&self) -> CANBIST_R {
        CANBIST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for USB RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbbist(&self) -> USBBIST_R {
        USBBIST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_0000 ~0x2000_3FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist0(&mut self) -> SRBIST0_W {
        SRBIST0_W { w: self }
    }
    #[doc = "Bit 1 - 2nd SRAM BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for SRAM located in address 0x2000_4000 ~0x2000_7FFF\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn srbist1(&mut self) -> SRBIST1_W {
        SRBIST1_W { w: self }
    }
    #[doc = "Bit 2 - CACHE BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CACHE RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn crbist(&mut self) -> CRBIST_W {
        CRBIST_W { w: self }
    }
    #[doc = "Bit 3 - CAN BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for CAN RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn canbist(&mut self) -> CANBIST_W {
        CANBIST_W { w: self }
    }
    #[doc = "Bit 4 - USB BIST Enable Bit (Write Protect)\\nThis bit enables BIST test for USB RAM\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbbist(&mut self) -> USBBIST_W {
        USBBIST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System SRAM BIST Test Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_bistctl](index.html) module"]
pub struct SYS_SRAM_BISTCTL_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BISTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_bistctl::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BISTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_bistctl::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BISTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BISTCTL to value 0"]
impl crate::Resettable for SYS_SRAM_BISTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
