#[doc = "Register `CLK_PLLCTL` reader"]
pub struct R(crate::R<CLK_PLLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PLLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_PLLCTL_SPEC>> for R {
    fn from(reader: crate::R<CLK_PLLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PLLCTL` writer"]
pub struct W(crate::W<CLK_PLLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PLLCTL_SPEC>;
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
impl core::convert::From<crate::W<CLK_PLLCTL_SPEC>> for W {
    fn from(writer: crate::W<CLK_PLLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBDIV` reader - PLL Feedback Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FBDIV_R(crate::FieldReader<u16, u16>);
impl FBDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        FBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBDIV` writer - PLL Feedback Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct FBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `INDIV` reader - PLL Input Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INDIV_R(crate::FieldReader<u8, u8>);
impl INDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        INDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDIV` writer - PLL Input Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct INDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | ((value as u32 & 0x1f) << 9);
        self.w
    }
}
#[doc = "Field `OUTDIV` reader - PLL Output Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct OUTDIV_R(crate::FieldReader<u8, u8>);
impl OUTDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTDIV` writer - PLL Output Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct OUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Power-down Mode (Write Protect)\\nIf set the PDEN bit to 1 in CLK_PWRCTL register, the PLL will enter Power-down mode, too.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_A {
    #[doc = "0: PLL is in normal mode"]
    _0 = 0,
    #[doc = "1: PLL is in Power-down mode (default)"]
    _1 = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Power-down Mode (Write Protect)\\nIf set the PDEN bit to 1 in CLK_PWRCTL register, the PLL will enter Power-down mode, too.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PD_R(crate::FieldReader<bool, PD_A>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_A {
        match self.bits {
            false => PD_A::_0,
            true => PD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PD_A::_1
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - Power-down Mode (Write Protect)\\nIf set the PDEN bit to 1 in CLK_PWRCTL register, the PLL will enter Power-down mode, too.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL is in normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PD_A::_0)
    }
    #[doc = "PLL is in Power-down mode (default)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "PLL Bypass Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BP_A {
    #[doc = "0: PLL is in normal mode (default)"]
    _0 = 0,
    #[doc = "1: PLL clock output is same as PLL input clock FIN"]
    _1 = 1,
}
impl From<BP_A> for bool {
    #[inline(always)]
    fn from(variant: BP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BP` reader - PLL Bypass Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BP_R(crate::FieldReader<bool, BP_A>);
impl BP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BP_A {
        match self.bits {
            false => BP_A::_0,
            true => BP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BP_A::_1
    }
}
impl core::ops::Deref for BP_R {
    type Target = crate::FieldReader<bool, BP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BP` writer - PLL Bypass Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BP_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL is in normal mode (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BP_A::_0)
    }
    #[doc = "PLL clock output is same as PLL input clock FIN"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "PLL OE (FOUT Enable) Pin Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: PLL FOUT Enabled"]
    _0 = 0,
    #[doc = "1: PLL FOUT is fixed low"]
    _1 = 1,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - PLL OE (FOUT Enable) Pin Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct OE_R(crate::FieldReader<bool, OE_A>);
impl OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::_0,
            true => OE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OE_A::_1
    }
}
impl core::ops::Deref for OE_R {
    type Target = crate::FieldReader<bool, OE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE` writer - PLL OE (FOUT Enable) Pin Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL FOUT Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OE_A::_0)
    }
    #[doc = "PLL FOUT is fixed low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "PLL Source Clock Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRC_A {
    #[doc = "0: PLL source clock from 4~20 MHz external high-speed crystal oscillator (HXT)"]
    _0 = 0,
    #[doc = "1: PLL source clock from 22.1184 MHz internal high-speed oscillator (HIRC)"]
    _1 = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSRC` reader - PLL Source Clock Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PLLSRC_R(crate::FieldReader<bool, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::_0,
            true => PLLSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLLSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLLSRC_A::_1
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<bool, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - PLL Source Clock Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL source clock from 4~20 MHz external high-speed crystal oscillator (HXT)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSRC_A::_0)
    }
    #[doc = "PLL source clock from 22.1184 MHz internal high-speed oscillator (HIRC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "PLL Stable Counter Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBSEL_A {
    #[doc = "0: PLL stable time is 6144 PLL source clock (suitable for source clock is equal to or less than 12 MHz)"]
    _0 = 0,
    #[doc = "1: PLL stable time is 12288 PLL source clock (suitable for source clock is larger than 12 MHz)"]
    _1 = 1,
}
impl From<STBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: STBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBSEL` reader - PLL Stable Counter Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct STBSEL_R(crate::FieldReader<bool, STBSEL_A>);
impl STBSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBSEL_A {
        match self.bits {
            false => STBSEL_A::_0,
            true => STBSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STBSEL_A::_1
    }
}
impl core::ops::Deref for STBSEL_R {
    type Target = crate::FieldReader<bool, STBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBSEL` writer - PLL Stable Counter Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct STBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL stable time is 6144 PLL source clock (suitable for source clock is equal to or less than 12 MHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STBSEL_A::_0)
    }
    #[doc = "PLL stable time is 12288 PLL source clock (suitable for source clock is larger than 12 MHz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STBSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PLL Feedback Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fbdiv(&self) -> FBDIV_R {
        FBDIV_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - PLL Input Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn indiv(&self) -> INDIV_R {
        INDIV_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - PLL Output Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn outdiv(&self) -> OUTDIV_R {
        OUTDIV_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Power-down Mode (Write Protect)\\nIf set the PDEN bit to 1 in CLK_PWRCTL register, the PLL will enter Power-down mode, too.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PLL Bypass Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLL OE (FOUT Enable) Pin Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PLL Source Clock Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PLL Stable Counter Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn stbsel(&self) -> STBSEL_R {
        STBSEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - PLL Feedback Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn fbdiv(&mut self) -> FBDIV_W {
        FBDIV_W { w: self }
    }
    #[doc = "Bits 9:13 - PLL Input Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn indiv(&mut self) -> INDIV_W {
        INDIV_W { w: self }
    }
    #[doc = "Bits 14:15 - PLL Output Divider Control (Write Protect)\\nRefer to the formulas below the table.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn outdiv(&mut self) -> OUTDIV_W {
        OUTDIV_W { w: self }
    }
    #[doc = "Bit 16 - Power-down Mode (Write Protect)\\nIf set the PDEN bit to 1 in CLK_PWRCTL register, the PLL will enter Power-down mode, too.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 17 - PLL Bypass Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W {
        BP_W { w: self }
    }
    #[doc = "Bit 18 - PLL OE (FOUT Enable) Pin Control (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 19 - PLL Source Clock Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bit 23 - PLL Stable Counter Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn stbsel(&mut self) -> STBSEL_W {
        STBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pllctl](index.html) module"]
pub struct CLK_PLLCTL_SPEC;
impl crate::RegisterSpec for CLK_PLLCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pllctl::R](R) reader structure"]
impl crate::Readable for CLK_PLLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pllctl::W](W) writer structure"]
impl crate::Writable for CLK_PLLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PLLCTL to value 0x0005_c02e"]
impl crate::Resettable for CLK_PLLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_c02e
    }
}
