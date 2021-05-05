#[doc = "Register `PWM_CTL0` reader"]
pub struct R(crate::R<PWM_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_CTL0_SPEC>> for R {
    fn from(reader: crate::R<PWM_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL0` writer"]
pub struct W(crate::W<PWM_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL0_SPEC>;
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
impl core::convert::From<crate::W<PWM_CTL0_SPEC>> for W {
    fn from(writer: crate::W<PWM_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRLDn` reader - Center Re-load\\nEach bit n controls the corresponding PWM channel n.\\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLDN_R(crate::FieldReader<u8, u8>);
impl CTRLDN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRLDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLDN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLDn` writer - Center Re-load\\nEach bit n controls the corresponding PWM channel n.\\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
pub struct CTRLDN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Window Load Enable Bit\\nEach bit n controls the corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINLDENN_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point of each period when valid reload window is set. The valid reload window is set by software write 1 to PWM_LOAD register and cleared by hardware after load success"]
    _1 = 1,
}
impl From<WINLDENN_A> for u8 {
    #[inline(always)]
    fn from(variant: WINLDENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINLDENn` reader - Window Load Enable Bit\\nEach bit n controls the corresponding PWM channel n."]
pub struct WINLDENN_R(crate::FieldReader<u8, WINLDENN_A>);
impl WINLDENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WINLDENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINLDENN_A> {
        match self.bits {
            0 => Some(WINLDENN_A::_0),
            1 => Some(WINLDENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WINLDENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WINLDENN_A::_1
    }
}
impl core::ops::Deref for WINLDENN_R {
    type Target = crate::FieldReader<u8, WINLDENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLDENn` writer - Window Load Enable Bit\\nEach bit n controls the corresponding PWM channel n."]
pub struct WINLDENN_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLDENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINLDENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WINLDENN_A::_0)
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point of each period when valid reload window is set. The valid reload window is set by software write 1 to PWM_LOAD register and cleared by hardware after load success"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WINLDENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Immediately Load Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: If IMMLDENn Enabled, WINLDENn and CTRLDn will be invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMMLDENN_A {
    #[doc = "0: PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    _0 = 0,
    #[doc = "1: PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    _1 = 1,
}
impl From<IMMLDENN_A> for u8 {
    #[inline(always)]
    fn from(variant: IMMLDENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IMMLDENn` reader - Immediately Load Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: If IMMLDENn Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDENN_R(crate::FieldReader<u8, IMMLDENN_A>);
impl IMMLDENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMMLDENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMMLDENN_A> {
        match self.bits {
            0 => Some(IMMLDENN_A::_0),
            1 => Some(IMMLDENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IMMLDENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IMMLDENN_A::_1
    }
}
impl core::ops::Deref for IMMLDENN_R {
    type Target = crate::FieldReader<u8, IMMLDENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMLDENn` writer - Immediately Load Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: If IMMLDENn Enabled, WINLDENn and CTRLDn will be invalid."]
pub struct IMMLDENN_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMLDENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMMLDENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the end point or center point of each period by setting CTRLD bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMMLDENN_A::_0)
    }
    #[doc = "PERIOD/CMPDAT will load to PBUF and CMPBUF immediately when software update PERIOD/CMPDAT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMMLDENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Group Function Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUPEN_A {
    #[doc = "0: The output waveform of each PWM channel are independent"]
    _0 = 0,
    #[doc = "1: Unify the PWM_CH2 and PWM_CH4 to output the same waveform as PWM_CH0 and unify the PWM_CH3 and PWM_CH5 to output the same waveform as PWM_CH1"]
    _1 = 1,
}
impl From<GROUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GROUPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GROUPEN` reader - Group Function Enable Bit"]
pub struct GROUPEN_R(crate::FieldReader<bool, GROUPEN_A>);
impl GROUPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GROUPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROUPEN_A {
        match self.bits {
            false => GROUPEN_A::_0,
            true => GROUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GROUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GROUPEN_A::_1
    }
}
impl core::ops::Deref for GROUPEN_R {
    type Target = crate::FieldReader<bool, GROUPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROUPEN` writer - Group Function Enable Bit"]
pub struct GROUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The output waveform of each PWM channel are independent"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GROUPEN_A::_0)
    }
    #[doc = "Unify the PWM_CH2 and PWM_CH4 to output the same waveform as PWM_CH0 and unify the PWM_CH3 and PWM_CH5 to output the same waveform as PWM_CH1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GROUPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "ICE Debug Mode Counter Halt (Write Protect)\\nIf counter halt is enabled, PWM all counters will keep current value until exit ICE debug mode. \\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGHALT_A {
    #[doc = "0: ICE debug mode counter halt Disabled"]
    _0 = 0,
    #[doc = "1: ICE debug mode counter halt Enabled"]
    _1 = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - ICE Debug Mode Counter Halt (Write Protect)\\nIf counter halt is enabled, PWM all counters will keep current value until exit ICE debug mode. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGHALT_R(crate::FieldReader<bool, DBGHALT_A>);
impl DBGHALT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGHALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::_0,
            true => DBGHALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGHALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGHALT_A::_1
    }
}
impl core::ops::Deref for DBGHALT_R {
    type Target = crate::FieldReader<bool, DBGHALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGHALT` writer - ICE Debug Mode Counter Halt (Write Protect)\\nIf counter halt is enabled, PWM all counters will keep current value until exit ICE debug mode. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGHALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGHALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode counter halt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGHALT_A::_0)
    }
    #[doc = "ICE debug mode counter halt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGHALT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "ICE Debug Mode Acknowledge Disable (Write Protect)\\nPWM pin will keep output no matter ICE debug mode acknowledged or not.\\nNote: This register is write protected. Refer to SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGTRIOFF_A {
    #[doc = "0: ICE debug mode acknowledgement effects PWM output"]
    _0 = 0,
    #[doc = "1: ICE debug mode acknowledgement disabled"]
    _1 = 1,
}
impl From<DBGTRIOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DBGTRIOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGTRIOFF` reader - ICE Debug Mode Acknowledge Disable (Write Protect)\\nPWM pin will keep output no matter ICE debug mode acknowledged or not.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGTRIOFF_R(crate::FieldReader<bool, DBGTRIOFF_A>);
impl DBGTRIOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGTRIOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGTRIOFF_A {
        match self.bits {
            false => DBGTRIOFF_A::_0,
            true => DBGTRIOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGTRIOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGTRIOFF_A::_1
    }
}
impl core::ops::Deref for DBGTRIOFF_R {
    type Target = crate::FieldReader<bool, DBGTRIOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGTRIOFF` writer - ICE Debug Mode Acknowledge Disable (Write Protect)\\nPWM pin will keep output no matter ICE debug mode acknowledged or not.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
pub struct DBGTRIOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGTRIOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGTRIOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode acknowledgement effects PWM output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGTRIOFF_A::_0)
    }
    #[doc = "ICE debug mode acknowledgement disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGTRIOFF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Center Re-load\\nEach bit n controls the corresponding PWM channel n.\\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrldn(&self) -> CTRLDN_R {
        CTRLDN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Window Load Enable Bit\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn winldenn(&self) -> WINLDENN_R {
        WINLDENN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Immediately Load Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: If IMMLDENn Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immldenn(&self) -> IMMLDENN_R {
        IMMLDENN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Group Function Enable Bit"]
    #[inline(always)]
    pub fn groupen(&self) -> GROUPEN_R {
        GROUPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ICE Debug Mode Counter Halt (Write Protect)\\nIf counter halt is enabled, PWM all counters will keep current value until exit ICE debug mode. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable (Write Protect)\\nPWM pin will keep output no matter ICE debug mode acknowledged or not.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbgtrioff(&self) -> DBGTRIOFF_R {
        DBGTRIOFF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Center Re-load\\nEach bit n controls the corresponding PWM channel n.\\nIn up-down counter type, PERIOD will load to PBUF at the end point of each period. CMPDAT will load to CMPBUF at the center point of a period."]
    #[inline(always)]
    pub fn ctrldn(&mut self) -> CTRLDN_W {
        CTRLDN_W { w: self }
    }
    #[doc = "Bits 8:13 - Window Load Enable Bit\\nEach bit n controls the corresponding PWM channel n."]
    #[inline(always)]
    pub fn winldenn(&mut self) -> WINLDENN_W {
        WINLDENN_W { w: self }
    }
    #[doc = "Bits 16:21 - Immediately Load Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: If IMMLDENn Enabled, WINLDENn and CTRLDn will be invalid."]
    #[inline(always)]
    pub fn immldenn(&mut self) -> IMMLDENN_W {
        IMMLDENN_W { w: self }
    }
    #[doc = "Bit 24 - Group Function Enable Bit"]
    #[inline(always)]
    pub fn groupen(&mut self) -> GROUPEN_W {
        GROUPEN_W { w: self }
    }
    #[doc = "Bit 30 - ICE Debug Mode Counter Halt (Write Protect)\\nIf counter halt is enabled, PWM all counters will keep current value until exit ICE debug mode. \\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable (Write Protect)\\nPWM pin will keep output no matter ICE debug mode acknowledged or not.\\nNote: This register is write protected. Refer to SYS_REGLCTL register."]
    #[inline(always)]
    pub fn dbgtrioff(&mut self) -> DBGTRIOFF_W {
        DBGTRIOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl0](index.html) module"]
pub struct PWM_CTL0_SPEC;
impl crate::RegisterSpec for PWM_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl0::R](R) reader structure"]
impl crate::Readable for PWM_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl0::W](W) writer structure"]
impl crate::Writable for PWM_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CTL0 to value 0"]
impl crate::Resettable for PWM_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
