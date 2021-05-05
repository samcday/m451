#[doc = "Register `DAC_DAT` reader"]
pub struct R(crate::R<DAC_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAC_DAT_SPEC>> for R {
    fn from(reader: crate::R<DAC_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DAT` writer"]
pub struct W(crate::W<DAC_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DAT_SPEC>;
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
impl core::convert::From<crate::W<DAC_DAT_SPEC>> for W {
    fn from(writer: crate::W<DAC_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_DAT` reader - DAC 12-bit Holding Data\\nThese bits are written by user software which specifies 12-bit conversion data for DAC output. The unused bits (DAC_DAT\\[3:0\\]
in left-alignment mode and DAC_DAT\\[15:12\\]
in right alignment mode) are ignored by DAC controller hardware.\\n12 bit left alignment: user has to load data into DAC_DAT\\[15:4\\]
bits.\\n12 bit right alignment: user has to load data into DAC_DAT\\[11:0\\]
bits."]
pub struct DAC_DAT_R(crate::FieldReader<u16, u16>);
impl DAC_DAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DAC_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DAT` writer - DAC 12-bit Holding Data\\nThese bits are written by user software which specifies 12-bit conversion data for DAC output. The unused bits (DAC_DAT\\[3:0\\]
in left-alignment mode and DAC_DAT\\[15:12\\]
in right alignment mode) are ignored by DAC controller hardware.\\n12 bit left alignment: user has to load data into DAC_DAT\\[15:4\\]
bits.\\n12 bit right alignment: user has to load data into DAC_DAT\\[11:0\\]
bits."]
pub struct DAC_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DAC 12-bit Holding Data\\nThese bits are written by user software which specifies 12-bit conversion data for DAC output. The unused bits (DAC_DAT\\[3:0\\]
in left-alignment mode and DAC_DAT\\[15:12\\]
in right alignment mode) are ignored by DAC controller hardware.\\n12 bit left alignment: user has to load data into DAC_DAT\\[15:4\\]
bits.\\n12 bit right alignment: user has to load data into DAC_DAT\\[11:0\\]
bits."]
    #[inline(always)]
    pub fn dac_dat(&self) -> DAC_DAT_R {
        DAC_DAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC 12-bit Holding Data\\nThese bits are written by user software which specifies 12-bit conversion data for DAC output. The unused bits (DAC_DAT\\[3:0\\]
in left-alignment mode and DAC_DAT\\[15:12\\]
in right alignment mode) are ignored by DAC controller hardware.\\n12 bit left alignment: user has to load data into DAC_DAT\\[15:4\\]
bits.\\n12 bit right alignment: user has to load data into DAC_DAT\\[11:0\\]
bits."]
    #[inline(always)]
    pub fn dac_dat(&mut self) -> DAC_DAT_W {
        DAC_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Data Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dat](index.html) module"]
pub struct DAC_DAT_SPEC;
impl crate::RegisterSpec for DAC_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dat::R](R) reader structure"]
impl crate::Readable for DAC_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dat::W](W) writer structure"]
impl crate::Writable for DAC_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DAT to value 0"]
impl crate::Resettable for DAC_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
