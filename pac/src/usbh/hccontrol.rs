#[doc = "Register `HCCONTROL` reader"]
pub struct R(crate::R<HCCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCCONTROL_SPEC>> for R {
    fn from(reader: crate::R<HCCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCONTROL` writer"]
pub struct W(crate::W<HCCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCONTROL_SPEC>;
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
impl core::convert::From<crate::W<HCCONTROL_SPEC>> for W {
    fn from(writer: crate::W<HCCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control Bulk Service Ratio\\nThis specifies the service ratio between Control and Bulk EDs. Before processing any of the non-periodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this\\nValue.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBSR_A {
    #[doc = "0: Number of Control EDs over Bulk EDs served is 1:1"]
    _0 = 0,
    #[doc = "1: Number of Control EDs over Bulk EDs served is 2:1"]
    _1 = 1,
    #[doc = "2: Number of Control EDs over Bulk EDs served is 3:1"]
    _2 = 2,
    #[doc = "3: Number of Control EDs over Bulk EDs served is 4:1"]
    _3 = 3,
}
impl From<CBSR_A> for u8 {
    #[inline(always)]
    fn from(variant: CBSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBSR` reader - Control Bulk Service Ratio\\nThis specifies the service ratio between Control and Bulk EDs. Before processing any of the non-periodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this\\nValue."]
pub struct CBSR_R(crate::FieldReader<u8, CBSR_A>);
impl CBSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBSR_A {
        match self.bits {
            0 => CBSR_A::_0,
            1 => CBSR_A::_1,
            2 => CBSR_A::_2,
            3 => CBSR_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CBSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CBSR_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CBSR_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CBSR_A::_3
    }
}
impl core::ops::Deref for CBSR_R {
    type Target = crate::FieldReader<u8, CBSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBSR` writer - Control Bulk Service Ratio\\nThis specifies the service ratio between Control and Bulk EDs. Before processing any of the non-periodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this\\nValue."]
pub struct CBSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Number of Control EDs over Bulk EDs served is 1:1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CBSR_A::_0)
    }
    #[doc = "Number of Control EDs over Bulk EDs served is 2:1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CBSR_A::_1)
    }
    #[doc = "Number of Control EDs over Bulk EDs served is 3:1"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CBSR_A::_2)
    }
    #[doc = "Number of Control EDs over Bulk EDs served is 4:1"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(CBSR_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Periodic List Enable Bit\\nWhen set, this bit enables processing of the Periodic (interrupt and isochronous) list. The Host Controller checks this bit prior to attempting any periodic transfers in a frame.\\nNote: To enable the processing of the Isochronous list, user has to set both PLE and IE (HcControl\\[3\\]) high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLE_A {
    #[doc = "0: Processing of the Periodic (Interrupt and Isochronous) list after next SOF (Start-Of-Frame) Disabled"]
    _0 = 0,
    #[doc = "1: Processing of the Periodic (Interrupt and Isochronous) list in the next frame Enabled"]
    _1 = 1,
}
impl From<PLE_A> for bool {
    #[inline(always)]
    fn from(variant: PLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLE` reader - Periodic List Enable Bit\\nWhen set, this bit enables processing of the Periodic (interrupt and isochronous) list. The Host Controller checks this bit prior to attempting any periodic transfers in a frame.\\nNote: To enable the processing of the Isochronous list, user has to set both PLE and IE (HcControl\\[3\\]) high."]
pub struct PLE_R(crate::FieldReader<bool, PLE_A>);
impl PLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLE_A {
        match self.bits {
            false => PLE_A::_0,
            true => PLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PLE_A::_1
    }
}
impl core::ops::Deref for PLE_R {
    type Target = crate::FieldReader<bool, PLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLE` writer - Periodic List Enable Bit\\nWhen set, this bit enables processing of the Periodic (interrupt and isochronous) list. The Host Controller checks this bit prior to attempting any periodic transfers in a frame.\\nNote: To enable the processing of the Isochronous list, user has to set both PLE and IE (HcControl\\[3\\]) high."]
pub struct PLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processing of the Periodic (Interrupt and Isochronous) list after next SOF (Start-Of-Frame) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLE_A::_0)
    }
    #[doc = "Processing of the Periodic (Interrupt and Isochronous) list in the next frame Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLE_A::_1)
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
#[doc = "Isochronous List Enable Bit\\nBoth ISOEn and PLE (HcControl\\[2\\]) high enables Host Controller to process the Isochronous list. Either ISOEn or PLE (HcControl\\[2\\]) is low disables Host Controller to process the Isochronous list.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Processing of the Isochronous list after next SOF (Start-Of-Frame) Disabled"]
    _0 = 0,
    #[doc = "1: Processing of the Isochronous list in the next frame Enabled, if the PLE (HcControl\\[2\\]) is high, too"]
    _1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Isochronous List Enable Bit\\nBoth ISOEn and PLE (HcControl\\[2\\]) high enables Host Controller to process the Isochronous list. Either ISOEn or PLE (HcControl\\[2\\]) is low disables Host Controller to process the Isochronous list."]
pub struct IE_R(crate::FieldReader<bool, IE_A>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::_0,
            true => IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IE_A::_1
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Isochronous List Enable Bit\\nBoth ISOEn and PLE (HcControl\\[2\\]) high enables Host Controller to process the Isochronous list. Either ISOEn or PLE (HcControl\\[2\\]) is low disables Host Controller to process the Isochronous list."]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processing of the Isochronous list after next SOF (Start-Of-Frame) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_A::_0)
    }
    #[doc = "Processing of the Isochronous list in the next frame Enabled, if the PLE (HcControl\\[2\\]) is high, too"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_A::_1)
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
#[doc = "Control List Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLE_A {
    #[doc = "0: Processing of the Control list after next SOF (Start-Of-Frame) Disabled"]
    _0 = 0,
    #[doc = "1: Processing of the Control list in the next frame Enabled"]
    _1 = 1,
}
impl From<CLE_A> for bool {
    #[inline(always)]
    fn from(variant: CLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLE` reader - Control List Enable Bit"]
pub struct CLE_R(crate::FieldReader<bool, CLE_A>);
impl CLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLE_A {
        match self.bits {
            false => CLE_A::_0,
            true => CLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLE_A::_1
    }
}
impl core::ops::Deref for CLE_R {
    type Target = crate::FieldReader<bool, CLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLE` writer - Control List Enable Bit"]
pub struct CLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processing of the Control list after next SOF (Start-Of-Frame) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLE_A::_0)
    }
    #[doc = "Processing of the Control list in the next frame Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLE_A::_1)
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
#[doc = "Bulk List Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_A {
    #[doc = "0: Processing of the Bulk list after next SOF (Start-Of-Frame) Disabled"]
    _0 = 0,
    #[doc = "1: Processing of the Bulk list in the next frame Enabled"]
    _1 = 1,
}
impl From<BLE_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLE` reader - Bulk List Enable Bit"]
pub struct BLE_R(crate::FieldReader<bool, BLE_A>);
impl BLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_A {
        match self.bits {
            false => BLE_A::_0,
            true => BLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BLE_A::_1
    }
}
impl core::ops::Deref for BLE_R {
    type Target = crate::FieldReader<bool, BLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE` writer - Bulk List Enable Bit"]
pub struct BLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processing of the Bulk list after next SOF (Start-Of-Frame) Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLE_A::_0)
    }
    #[doc = "Processing of the Bulk list in the next frame Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLE_A::_1)
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
#[doc = "Host Controller Functional State\\nThis field sets the Host Controller state. The Controller may force a state change from USBSUSPEND to USBRESUME after detecting resume signaling from a downstream port. States are:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCFS_A {
    #[doc = "0: USBRESET"]
    _0 = 0,
    #[doc = "1: USBRESUME"]
    _1 = 1,
    #[doc = "2: USBOPERATIONAL"]
    _2 = 2,
    #[doc = "3: USBSUSPEND"]
    _3 = 3,
}
impl From<HCFS_A> for u8 {
    #[inline(always)]
    fn from(variant: HCFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HCFS` reader - Host Controller Functional State\\nThis field sets the Host Controller state. The Controller may force a state change from USBSUSPEND to USBRESUME after detecting resume signaling from a downstream port. States are:"]
pub struct HCFS_R(crate::FieldReader<u8, HCFS_A>);
impl HCFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCFS_A {
        match self.bits {
            0 => HCFS_A::_0,
            1 => HCFS_A::_1,
            2 => HCFS_A::_2,
            3 => HCFS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HCFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HCFS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HCFS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HCFS_A::_3
    }
}
impl core::ops::Deref for HCFS_R {
    type Target = crate::FieldReader<u8, HCFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCFS` writer - Host Controller Functional State\\nThis field sets the Host Controller state. The Controller may force a state change from USBSUSPEND to USBRESUME after detecting resume signaling from a downstream port. States are:"]
pub struct HCFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCFS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "USBRESET"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCFS_A::_0)
    }
    #[doc = "USBRESUME"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCFS_A::_1)
    }
    #[doc = "USBOPERATIONAL"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HCFS_A::_2)
    }
    #[doc = "USBSUSPEND"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HCFS_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Control Bulk Service Ratio\\nThis specifies the service ratio between Control and Bulk EDs. Before processing any of the non-periodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this\\nValue."]
    #[inline(always)]
    pub fn cbsr(&self) -> CBSR_R {
        CBSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Periodic List Enable Bit\\nWhen set, this bit enables processing of the Periodic (interrupt and isochronous) list. The Host Controller checks this bit prior to attempting any periodic transfers in a frame.\\nNote: To enable the processing of the Isochronous list, user has to set both PLE and IE (HcControl\\[3\\]) high."]
    #[inline(always)]
    pub fn ple(&self) -> PLE_R {
        PLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Isochronous List Enable Bit\\nBoth ISOEn and PLE (HcControl\\[2\\]) high enables Host Controller to process the Isochronous list. Either ISOEn or PLE (HcControl\\[2\\]) is low disables Host Controller to process the Isochronous list."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control List Enable Bit"]
    #[inline(always)]
    pub fn cle(&self) -> CLE_R {
        CLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bulk List Enable Bit"]
    #[inline(always)]
    pub fn ble(&self) -> BLE_R {
        BLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Host Controller Functional State\\nThis field sets the Host Controller state. The Controller may force a state change from USBSUSPEND to USBRESUME after detecting resume signaling from a downstream port. States are:"]
    #[inline(always)]
    pub fn hcfs(&self) -> HCFS_R {
        HCFS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control Bulk Service Ratio\\nThis specifies the service ratio between Control and Bulk EDs. Before processing any of the non-periodic lists, HC must compare the ratio specified with its internal count on how many nonempty Control EDs have been processed, in determining whether to continue serving another Control ED or switching to Bulk EDs. The internal count will be retained when crossing the frame boundary. In case of reset, HCD is responsible for restoring this\\nValue."]
    #[inline(always)]
    pub fn cbsr(&mut self) -> CBSR_W {
        CBSR_W { w: self }
    }
    #[doc = "Bit 2 - Periodic List Enable Bit\\nWhen set, this bit enables processing of the Periodic (interrupt and isochronous) list. The Host Controller checks this bit prior to attempting any periodic transfers in a frame.\\nNote: To enable the processing of the Isochronous list, user has to set both PLE and IE (HcControl\\[3\\]) high."]
    #[inline(always)]
    pub fn ple(&mut self) -> PLE_W {
        PLE_W { w: self }
    }
    #[doc = "Bit 3 - Isochronous List Enable Bit\\nBoth ISOEn and PLE (HcControl\\[2\\]) high enables Host Controller to process the Isochronous list. Either ISOEn or PLE (HcControl\\[2\\]) is low disables Host Controller to process the Isochronous list."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 4 - Control List Enable Bit"]
    #[inline(always)]
    pub fn cle(&mut self) -> CLE_W {
        CLE_W { w: self }
    }
    #[doc = "Bit 5 - Bulk List Enable Bit"]
    #[inline(always)]
    pub fn ble(&mut self) -> BLE_W {
        BLE_W { w: self }
    }
    #[doc = "Bits 6:7 - Host Controller Functional State\\nThis field sets the Host Controller state. The Controller may force a state change from USBSUSPEND to USBRESUME after detecting resume signaling from a downstream port. States are:"]
    #[inline(always)]
    pub fn hcfs(&mut self) -> HCFS_W {
        HCFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrol](index.html) module"]
pub struct HCCONTROL_SPEC;
impl crate::RegisterSpec for HCCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccontrol::R](R) reader structure"]
impl crate::Readable for HCCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccontrol::W](W) writer structure"]
impl crate::Writable for HCCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCONTROL to value 0"]
impl crate::Resettable for HCCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
