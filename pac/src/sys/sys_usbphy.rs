#[doc = "Register `SYS_USBPHY` reader"]
pub struct R(crate::R<SYS_USBPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_USBPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYS_USBPHY_SPEC>> for R {
    fn from(reader: crate::R<SYS_USBPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_USBPHY` writer"]
pub struct W(crate::W<SYS_USBPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_USBPHY_SPEC>;
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
impl core::convert::From<crate::W<SYS_USBPHY_SPEC>> for W {
    fn from(writer: crate::W<SYS_USBPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB Role Option (Write Protect) (M45xG/M45xE Only)\\nThese two bits are used to select the role of USB.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBROLE_A {
    #[doc = "0: Standard USB Device mode"]
    _0 = 0,
    #[doc = "1: Standard USB Host mode"]
    _1 = 1,
    #[doc = "2: ID dependent mode"]
    _2 = 2,
    #[doc = "3: On-The-Go device mode"]
    _3 = 3,
}
impl From<USBROLE_A> for u8 {
    #[inline(always)]
    fn from(variant: USBROLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBROLE` reader - USB Role Option (Write Protect) (M45xG/M45xE Only)\\nThese two bits are used to select the role of USB.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBROLE_R(crate::FieldReader<u8, USBROLE_A>);
impl USBROLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBROLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBROLE_A {
        match self.bits {
            0 => USBROLE_A::_0,
            1 => USBROLE_A::_1,
            2 => USBROLE_A::_2,
            3 => USBROLE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == USBROLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == USBROLE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == USBROLE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == USBROLE_A::_3
    }
}
impl core::ops::Deref for USBROLE_R {
    type Target = crate::FieldReader<u8, USBROLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBROLE` writer - USB Role Option (Write Protect) (M45xG/M45xE Only)\\nThese two bits are used to select the role of USB.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct USBROLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBROLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBROLE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Standard USB Device mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBROLE_A::_0)
    }
    #[doc = "Standard USB Host mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBROLE_A::_1)
    }
    #[doc = "ID dependent mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(USBROLE_A::_2)
    }
    #[doc = "On-The-Go device mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(USBROLE_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "USB LDO33 Enable Bit (Write Protect) (M45xG/M45xE Only)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDO33EN_A {
    #[doc = "0: USB LDO33 Disabled"]
    _0 = 0,
    #[doc = "1: USB LDO33 Enabled"]
    _1 = 1,
}
impl From<LDO33EN_A> for bool {
    #[inline(always)]
    fn from(variant: LDO33EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDO33EN` reader - USB LDO33 Enable Bit (Write Protect) (M45xG/M45xE Only)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LDO33EN_R(crate::FieldReader<bool, LDO33EN_A>);
impl LDO33EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO33EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDO33EN_A {
        match self.bits {
            false => LDO33EN_A::_0,
            true => LDO33EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LDO33EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LDO33EN_A::_1
    }
}
impl core::ops::Deref for LDO33EN_R {
    type Target = crate::FieldReader<bool, LDO33EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO33EN` writer - USB LDO33 Enable Bit (Write Protect) (M45xG/M45xE Only)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct LDO33EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO33EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDO33EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB LDO33 Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDO33EN_A::_0)
    }
    #[doc = "USB LDO33 Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDO33EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB Role Option (Write Protect) (M45xG/M45xE Only)\\nThese two bits are used to select the role of USB.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbrole(&self) -> USBROLE_R {
        USBROLE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - USB LDO33 Enable Bit (Write Protect) (M45xG/M45xE Only)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ldo33en(&self) -> LDO33EN_R {
        LDO33EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB Role Option (Write Protect) (M45xG/M45xE Only)\\nThese two bits are used to select the role of USB.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn usbrole(&mut self) -> USBROLE_W {
        USBROLE_W { w: self }
    }
    #[doc = "Bit 8 - USB LDO33 Enable Bit (Write Protect) (M45xG/M45xE Only)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ldo33en(&mut self) -> LDO33EN_W {
        LDO33EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Control Register (M45xG/M45xE Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_usbphy](index.html) module"]
pub struct SYS_USBPHY_SPEC;
impl crate::RegisterSpec for SYS_USBPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_usbphy::R](R) reader structure"]
impl crate::Readable for SYS_USBPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_usbphy::W](W) writer structure"]
impl crate::Writable for SYS_USBPHY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_USBPHY to value 0x03"]
impl crate::Resettable for SYS_USBPHY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
