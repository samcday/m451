#[doc = "Register `SYS_BODCTL` reader"]
pub struct R(crate::R<SYS_BODCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_BODCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_BODCTL_SPEC>> for R {
    fn from(reader: crate::R<SYS_BODCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_BODCTL` writer"]
pub struct W(crate::W<SYS_BODCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_BODCTL_SPEC>;
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
impl core::convert::From<crate::W<SYS_BODCTL_SPEC>> for W {
    fn from(writer: crate::W<SYS_BODCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Brown-out Detector Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBODEN (CONFIG0 \\[23\\]).\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: LIRC must be enabled before enable BOD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODEN_A {
    #[doc = "0: Brown-out Detector function Disabled"]
    _0 = 0,
    #[doc = "1: Brown-out Detector function Enabled"]
    _1 = 1,
}
impl From<BODEN_A> for bool {
    #[inline(always)]
    fn from(variant: BODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODEN` reader - Brown-out Detector Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBODEN (CONFIG0 \\[23\\]).\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: LIRC must be enabled before enable BOD."]
pub struct BODEN_R(crate::FieldReader<bool, BODEN_A>);
impl BODEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODEN_A {
        match self.bits {
            false => BODEN_A::_0,
            true => BODEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODEN_A::_1
    }
}
impl core::ops::Deref for BODEN_R {
    type Target = crate::FieldReader<bool, BODEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODEN` writer - Brown-out Detector Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBODEN (CONFIG0 \\[23\\]).\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: LIRC must be enabled before enable BOD."]
pub struct BODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brown-out Detector function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODEN_A::_0)
    }
    #[doc = "Brown-out Detector function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODEN_A::_1)
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
#[doc = "Brown-out Detector Threshold Voltage Selection (Write Protect)\\nThe default value is set by flash controller user configuration register CBOV (CONFIG0 \\[22:21\\]).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODVL_A {
    #[doc = "0: Brown-Out Detector threshold voltage is 2.2V"]
    _0 = 0,
    #[doc = "1: Brown-Out Detector threshold voltage is 2.7V"]
    _1 = 1,
    #[doc = "2: Brown-Out Detector threshold voltage is 3.7V"]
    _2 = 2,
    #[doc = "3: Brown-Out Detector threshold voltage is 4.4V"]
    _3 = 3,
}
impl From<BODVL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODVL` reader - Brown-out Detector Threshold Voltage Selection (Write Protect)\\nThe default value is set by flash controller user configuration register CBOV (CONFIG0 \\[22:21\\]).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODVL_R(crate::FieldReader<u8, BODVL_A>);
impl BODVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BODVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVL_A {
        match self.bits {
            0 => BODVL_A::_0,
            1 => BODVL_A::_1,
            2 => BODVL_A::_2,
            3 => BODVL_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODVL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODVL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BODVL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BODVL_A::_3
    }
}
impl core::ops::Deref for BODVL_R {
    type Target = crate::FieldReader<u8, BODVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVL` writer - Brown-out Detector Threshold Voltage Selection (Write Protect)\\nThe default value is set by flash controller user configuration register CBOV (CONFIG0 \\[22:21\\]).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODVL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODVL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Brown-Out Detector threshold voltage is 2.2V"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODVL_A::_0)
    }
    #[doc = "Brown-Out Detector threshold voltage is 2.7V"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODVL_A::_1)
    }
    #[doc = "Brown-Out Detector threshold voltage is 3.7V"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BODVL_A::_2)
    }
    #[doc = "Brown-Out Detector threshold voltage is 4.4V"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BODVL_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Brown-out Reset Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBORST(CONFIG0\\[20\\]) bit .\\nNote1: \\nWhile the Brown-out Detector function is enabled (BODEN high) and BOD reset function is enabled (BODRSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BODOUT high).\\nWhile the BOD function is enabled (BODEN high) and BOD interrupt function is enabled (BODRSTEN low), BOD will assert an interrupt if BODOUT is high. BOD interrupt will keep till to the BODEN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BODEN low).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTEN_A {
    #[doc = "0: Brown-out 'INTERRUPT' function Enabled"]
    _0 = 0,
    #[doc = "1: Brown-out 'RESET' function Enabled"]
    _1 = 1,
}
impl From<BODRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTEN` reader - Brown-out Reset Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBORST(CONFIG0\\[20\\]) bit .\\nNote1: \\nWhile the Brown-out Detector function is enabled (BODEN high) and BOD reset function is enabled (BODRSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BODOUT high).\\nWhile the BOD function is enabled (BODEN high) and BOD interrupt function is enabled (BODRSTEN low), BOD will assert an interrupt if BODOUT is high. BOD interrupt will keep till to the BODEN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BODEN low).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODRSTEN_R(crate::FieldReader<bool, BODRSTEN_A>);
impl BODRSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTEN_A {
        match self.bits {
            false => BODRSTEN_A::_0,
            true => BODRSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODRSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODRSTEN_A::_1
    }
}
impl core::ops::Deref for BODRSTEN_R {
    type Target = crate::FieldReader<bool, BODRSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRSTEN` writer - Brown-out Reset Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBORST(CONFIG0\\[20\\]) bit .\\nNote1: \\nWhile the Brown-out Detector function is enabled (BODEN high) and BOD reset function is enabled (BODRSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BODOUT high).\\nWhile the BOD function is enabled (BODEN high) and BOD interrupt function is enabled (BODRSTEN low), BOD will assert an interrupt if BODOUT is high. BOD interrupt will keep till to the BODEN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BODEN low).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brown-out 'INTERRUPT' function Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODRSTEN_A::_0)
    }
    #[doc = "Brown-out 'RESET' function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODRSTEN_A::_1)
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
#[doc = "Brown-out Detector Interrupt Flag\\nNote: Write 1 to clear this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODIF_A {
    #[doc = "0: Brown-out Detector does not detect any voltage draft at VDD down through or up through the voltage of BODVL setting"]
    _0 = 0,
    #[doc = "1: When Brown-out Detector detects the VDD is dropped down through the voltage of BODVL setting or the VDD is raised up through the voltage of BODVL setting, this bit is set to 1 and the brown-out interrupt is requested if brown-out interrupt is enabled"]
    _1 = 1,
}
impl From<BODIF_A> for bool {
    #[inline(always)]
    fn from(variant: BODIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODIF` reader - Brown-out Detector Interrupt Flag\\nNote: Write 1 to clear this bit to 0."]
pub struct BODIF_R(crate::FieldReader<bool, BODIF_A>);
impl BODIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODIF_A {
        match self.bits {
            false => BODIF_A::_0,
            true => BODIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODIF_A::_1
    }
}
impl core::ops::Deref for BODIF_R {
    type Target = crate::FieldReader<bool, BODIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODIF` writer - Brown-out Detector Interrupt Flag\\nNote: Write 1 to clear this bit to 0."]
pub struct BODIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BODIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brown-out Detector does not detect any voltage draft at VDD down through or up through the voltage of BODVL setting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODIF_A::_0)
    }
    #[doc = "When Brown-out Detector detects the VDD is dropped down through the voltage of BODVL setting or the VDD is raised up through the voltage of BODVL setting, this bit is set to 1 and the brown-out interrupt is requested if brown-out interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODIF_A::_1)
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
#[doc = "Brown-out Detector Low Power Mode (Write Protect)\\nNote1: The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODLPM_A {
    #[doc = "0: BOD operate in normal mode (default)"]
    _0 = 0,
    #[doc = "1: BOD Low Power mode Enabled"]
    _1 = 1,
}
impl From<BODLPM_A> for bool {
    #[inline(always)]
    fn from(variant: BODLPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODLPM` reader - Brown-out Detector Low Power Mode (Write Protect)\\nNote1: The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODLPM_R(crate::FieldReader<bool, BODLPM_A>);
impl BODLPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODLPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODLPM_A {
        match self.bits {
            false => BODLPM_A::_0,
            true => BODLPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODLPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODLPM_A::_1
    }
}
impl core::ops::Deref for BODLPM_R {
    type Target = crate::FieldReader<bool, BODLPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODLPM` writer - Brown-out Detector Low Power Mode (Write Protect)\\nNote1: The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct BODLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> BODLPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODLPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BOD operate in normal mode (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODLPM_A::_0)
    }
    #[doc = "BOD Low Power mode Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODLPM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Brown-out Detector Output Status\\nIt means the detected voltage is lower than BODVL setting. If the BODEN is 0, BOD function disabled , this bit always responds 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODOUT_A {
    #[doc = "0: Brown-out Detector output status is 0"]
    _0 = 0,
    #[doc = "1: Brown-out Detector output status is 1"]
    _1 = 1,
}
impl From<BODOUT_A> for bool {
    #[inline(always)]
    fn from(variant: BODOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODOUT` reader - Brown-out Detector Output Status\\nIt means the detected voltage is lower than BODVL setting. If the BODEN is 0, BOD function disabled , this bit always responds 0."]
pub struct BODOUT_R(crate::FieldReader<bool, BODOUT_A>);
impl BODOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODOUT_A {
        match self.bits {
            false => BODOUT_A::_0,
            true => BODOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODOUT_A::_1
    }
}
impl core::ops::Deref for BODOUT_R {
    type Target = crate::FieldReader<bool, BODOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODOUT` writer - Brown-out Detector Output Status\\nIt means the detected voltage is lower than BODVL setting. If the BODEN is 0, BOD function disabled , this bit always responds 0."]
pub struct BODOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Brown-out Detector output status is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODOUT_A::_0)
    }
    #[doc = "Brown-out Detector output status is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODOUT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Low Voltage Reset Enable Bit (Write Protect)\\nThe LVR function resets the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled by default.\\nNote1: After enabling the bit, the LVR function will be active with 100us delay for LVR output stable (default).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote3: LIRC must be enabled before enable LVR.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVREN_A {
    #[doc = "0: Low Voltage Reset function Disabled"]
    _0 = 0,
    #[doc = "1: Low Voltage Reset function Enabled"]
    _1 = 1,
}
impl From<LVREN_A> for bool {
    #[inline(always)]
    fn from(variant: LVREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVREN` reader - Low Voltage Reset Enable Bit (Write Protect)\\nThe LVR function resets the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled by default.\\nNote1: After enabling the bit, the LVR function will be active with 100us delay for LVR output stable (default).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote3: LIRC must be enabled before enable LVR."]
pub struct LVREN_R(crate::FieldReader<bool, LVREN_A>);
impl LVREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVREN_A {
        match self.bits {
            false => LVREN_A::_0,
            true => LVREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVREN_A::_1
    }
}
impl core::ops::Deref for LVREN_R {
    type Target = crate::FieldReader<bool, LVREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVREN` writer - Low Voltage Reset Enable Bit (Write Protect)\\nThe LVR function resets the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled by default.\\nNote1: After enabling the bit, the LVR function will be active with 100us delay for LVR output stable (default).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote3: LIRC must be enabled before enable LVR."]
pub struct LVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Voltage Reset function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVREN_A::_0)
    }
    #[doc = "Low Voltage Reset function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Brown-out Detector Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODDGSEL_A {
    #[doc = "0: BOD output is sampled by RC10K clock"]
    _0 = 0,
    #[doc = "1: 4 system clock (HCLK)"]
    _1 = 1,
    #[doc = "2: 8 system clock (HCLK)"]
    _2 = 2,
    #[doc = "3: 16 system clock (HCLK)"]
    _3 = 3,
    #[doc = "4: 32 system clock (HCLK)"]
    _4 = 4,
    #[doc = "5: 64 system clock (HCLK)"]
    _5 = 5,
    #[doc = "6: 128 system clock (HCLK)"]
    _6 = 6,
    #[doc = "7: 256 system clock (HCLK)"]
    _7 = 7,
}
impl From<BODDGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODDGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BODDGSEL` reader - Brown-out Detector Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
pub struct BODDGSEL_R(crate::FieldReader<u8, BODDGSEL_A>);
impl BODDGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BODDGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODDGSEL_A {
        match self.bits {
            0 => BODDGSEL_A::_0,
            1 => BODDGSEL_A::_1,
            2 => BODDGSEL_A::_2,
            3 => BODDGSEL_A::_3,
            4 => BODDGSEL_A::_4,
            5 => BODDGSEL_A::_5,
            6 => BODDGSEL_A::_6,
            7 => BODDGSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BODDGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BODDGSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == BODDGSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == BODDGSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == BODDGSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == BODDGSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == BODDGSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == BODDGSEL_A::_7
    }
}
impl core::ops::Deref for BODDGSEL_R {
    type Target = crate::FieldReader<u8, BODDGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODDGSEL` writer - Brown-out Detector Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
pub struct BODDGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODDGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODDGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "BOD output is sampled by RC10K clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_0)
    }
    #[doc = "4 system clock (HCLK)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_1)
    }
    #[doc = "8 system clock (HCLK)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_2)
    }
    #[doc = "16 system clock (HCLK)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_3)
    }
    #[doc = "32 system clock (HCLK)"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_4)
    }
    #[doc = "64 system clock (HCLK)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_5)
    }
    #[doc = "128 system clock (HCLK)"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_6)
    }
    #[doc = "256 system clock (HCLK)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(BODDGSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "LVR Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVRDGSEL_A {
    #[doc = "0: Without de-glitch function"]
    _0 = 0,
    #[doc = "1: 4 system clock (HCLK)"]
    _1 = 1,
    #[doc = "2: 8 system clock (HCLK)"]
    _2 = 2,
    #[doc = "3: 16 system clock (HCLK)"]
    _3 = 3,
    #[doc = "4: 32 system clock (HCLK)"]
    _4 = 4,
    #[doc = "5: 64 system clock (HCLK)"]
    _5 = 5,
    #[doc = "6: 128 system clock (HCLK)"]
    _6 = 6,
    #[doc = "7: 256 system clock (HCLK)"]
    _7 = 7,
}
impl From<LVRDGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVRDGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVRDGSEL` reader - LVR Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
pub struct LVRDGSEL_R(crate::FieldReader<u8, LVRDGSEL_A>);
impl LVRDGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LVRDGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVRDGSEL_A {
        match self.bits {
            0 => LVRDGSEL_A::_0,
            1 => LVRDGSEL_A::_1,
            2 => LVRDGSEL_A::_2,
            3 => LVRDGSEL_A::_3,
            4 => LVRDGSEL_A::_4,
            5 => LVRDGSEL_A::_5,
            6 => LVRDGSEL_A::_6,
            7 => LVRDGSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVRDGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVRDGSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == LVRDGSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == LVRDGSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == LVRDGSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == LVRDGSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == LVRDGSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == LVRDGSEL_A::_7
    }
}
impl core::ops::Deref for LVRDGSEL_R {
    type Target = crate::FieldReader<u8, LVRDGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVRDGSEL` writer - LVR Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
pub struct LVRDGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRDGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVRDGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Without de-glitch function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_0)
    }
    #[doc = "4 system clock (HCLK)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_1)
    }
    #[doc = "8 system clock (HCLK)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_2)
    }
    #[doc = "16 system clock (HCLK)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_3)
    }
    #[doc = "32 system clock (HCLK)"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_4)
    }
    #[doc = "64 system clock (HCLK)"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_5)
    }
    #[doc = "128 system clock (HCLK)"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_6)
    }
    #[doc = "256 system clock (HCLK)"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(LVRDGSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Brown-out Detector Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBODEN (CONFIG0 \\[23\\]).\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: LIRC must be enabled before enable BOD."]
    #[inline(always)]
    pub fn boden(&self) -> BODEN_R {
        BODEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Brown-out Detector Threshold Voltage Selection (Write Protect)\\nThe default value is set by flash controller user configuration register CBOV (CONFIG0 \\[22:21\\]).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodvl(&self) -> BODVL_R {
        BODVL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Brown-out Reset Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBORST(CONFIG0\\[20\\]) bit .\\nNote1: \\nWhile the Brown-out Detector function is enabled (BODEN high) and BOD reset function is enabled (BODRSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BODOUT high).\\nWhile the BOD function is enabled (BODEN high) and BOD interrupt function is enabled (BODRSTEN low), BOD will assert an interrupt if BODOUT is high. BOD interrupt will keep till to the BODEN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BODEN low).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodrsten(&self) -> BODRSTEN_R {
        BODRSTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Brown-out Detector Interrupt Flag\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn bodif(&self) -> BODIF_R {
        BODIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown-out Detector Low Power Mode (Write Protect)\\nNote1: The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodlpm(&self) -> BODLPM_R {
        BODLPM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Brown-out Detector Output Status\\nIt means the detected voltage is lower than BODVL setting. If the BODEN is 0, BOD function disabled , this bit always responds 0."]
    #[inline(always)]
    pub fn bodout(&self) -> BODOUT_R {
        BODOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low Voltage Reset Enable Bit (Write Protect)\\nThe LVR function resets the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled by default.\\nNote1: After enabling the bit, the LVR function will be active with 100us delay for LVR output stable (default).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote3: LIRC must be enabled before enable LVR."]
    #[inline(always)]
    pub fn lvren(&self) -> LVREN_R {
        LVREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Brown-out Detector Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn boddgsel(&self) -> BODDGSEL_R {
        BODDGSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - LVR Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lvrdgsel(&self) -> LVRDGSEL_R {
        LVRDGSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Brown-out Detector Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBODEN (CONFIG0 \\[23\\]).\\nNote1: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote2: LIRC must be enabled before enable BOD."]
    #[inline(always)]
    pub fn boden(&mut self) -> BODEN_W {
        BODEN_W { w: self }
    }
    #[doc = "Bits 1:2 - Brown-out Detector Threshold Voltage Selection (Write Protect)\\nThe default value is set by flash controller user configuration register CBOV (CONFIG0 \\[22:21\\]).\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodvl(&mut self) -> BODVL_W {
        BODVL_W { w: self }
    }
    #[doc = "Bit 3 - Brown-out Reset Enable Bit (Write Protect)\\nThe default value is set by flash controller user configuration register CBORST(CONFIG0\\[20\\]) bit .\\nNote1: \\nWhile the Brown-out Detector function is enabled (BODEN high) and BOD reset function is enabled (BODRSTEN high), BOD will assert a signal to reset chip when the detected voltage is lower than the threshold (BODOUT high).\\nWhile the BOD function is enabled (BODEN high) and BOD interrupt function is enabled (BODRSTEN low), BOD will assert an interrupt if BODOUT is high. BOD interrupt will keep till to the BODEN set to 0. BOD interrupt can be blocked by disabling the NVIC BOD interrupt or disabling BOD function (set BODEN low).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodrsten(&mut self) -> BODRSTEN_W {
        BODRSTEN_W { w: self }
    }
    #[doc = "Bit 4 - Brown-out Detector Interrupt Flag\\nNote: Write 1 to clear this bit to 0."]
    #[inline(always)]
    pub fn bodif(&mut self) -> BODIF_W {
        BODIF_W { w: self }
    }
    #[doc = "Bit 5 - Brown-out Detector Low Power Mode (Write Protect)\\nNote1: The BOD consumes about 100uA in normal mode, the low power mode can reduce the current to about 1/10 but slow the BOD response.\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn bodlpm(&mut self) -> BODLPM_W {
        BODLPM_W { w: self }
    }
    #[doc = "Bit 6 - Brown-out Detector Output Status\\nIt means the detected voltage is lower than BODVL setting. If the BODEN is 0, BOD function disabled , this bit always responds 0."]
    #[inline(always)]
    pub fn bodout(&mut self) -> BODOUT_W {
        BODOUT_W { w: self }
    }
    #[doc = "Bit 7 - Low Voltage Reset Enable Bit (Write Protect)\\nThe LVR function resets the chip when the input power voltage is lower than LVR circuit setting. LVR function is enabled by default.\\nNote1: After enabling the bit, the LVR function will be active with 100us delay for LVR output stable (default).\\nNote2: This bit is write protected. Refer to the SYS_REGLCTL register.\\nNote3: LIRC must be enabled before enable LVR."]
    #[inline(always)]
    pub fn lvren(&mut self) -> LVREN_W {
        LVREN_W { w: self }
    }
    #[doc = "Bits 8:10 - Brown-out Detector Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn boddgsel(&mut self) -> BODDGSEL_W {
        BODDGSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - LVR Output De-glitch Time Select (Write Protect)\\nNote: These bits are write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn lvrdgsel(&mut self) -> LVRDGSEL_W {
        LVRDGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown-out Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_bodctl](index.html) module"]
pub struct SYS_BODCTL_SPEC;
impl crate::RegisterSpec for SYS_BODCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_bodctl::R](R) reader structure"]
impl crate::Readable for SYS_BODCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_bodctl::W](W) writer structure"]
impl crate::Writable for SYS_BODCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_BODCTL to value 0x0380"]
impl crate::Resettable for SYS_BODCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0380
    }
}
