#[doc = "Register `CAN_BTIME` reader"]
pub struct R(crate::R<CAN_BTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_BTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_BTIME_SPEC>> for R {
    fn from(reader: crate::R<CAN_BTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_BTIME` writer"]
pub struct W(crate::W<CAN_BTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_BTIME_SPEC>;
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
impl core::convert::From<crate::W<CAN_BTIME_SPEC>> for W {
    fn from(writer: crate::W<CAN_BTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler \\n0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct BRP_R(crate::FieldReader<u8, u8>);
impl BRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - Baud Rate Prescaler \\n0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SJW` reader - (Re)Synchronization Jump Width\\n0x0-0x3: Valid programmed values are \\[0...3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - (Re)Synchronization Jump Width\\n0x0-0x3: Valid programmed values are \\[0...3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TSeg1` reader - Time Segment Before the Sample Point Minus Sync_Seg\\n0x01-0x0F: valid values for TSeg1 are \\[1...15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
pub struct TSEG1_R(crate::FieldReader<u8, u8>);
impl TSEG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSeg1` writer - Time Segment Before the Sample Point Minus Sync_Seg\\n0x01-0x0F: valid values for TSeg1 are \\[1...15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TSeg2` reader - Time Segment After Sample Point \\n0x0-0x7: Valid values for TSeg2 are \\[0...7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct TSEG2_R(crate::FieldReader<u8, u8>);
impl TSEG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSeg2` writer - Time Segment After Sample Point \\n0x0-0x7: Valid values for TSeg2 are \\[0...7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler \\n0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width\\n0x0-0x3: Valid programmed values are \\[0...3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point Minus Sync_Seg\\n0x01-0x0F: valid values for TSeg1 are \\[1...15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point \\n0x0-0x7: Valid values for TSeg2 are \\[0...7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler \\n0x01-0x3F: The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are \\[0...63\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width\\n0x0-0x3: Valid programmed values are \\[0...3\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point Minus Sync_Seg\\n0x01-0x0F: valid values for TSeg1 are \\[1...15\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed is used."]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 12:14 - Time Segment After Sample Point \\n0x0-0x7: Valid values for TSeg2 are \\[0...7\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_btime](index.html) module"]
pub struct CAN_BTIME_SPEC;
impl crate::RegisterSpec for CAN_BTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_btime::R](R) reader structure"]
impl crate::Readable for CAN_BTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_btime::W](W) writer structure"]
impl crate::Writable for CAN_BTIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_BTIME to value 0x2301"]
impl crate::Resettable for CAN_BTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2301
    }
}
