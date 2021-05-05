#[doc = "Register `CLK_CLKDIV1` reader"]
pub struct R(crate::R<CLK_CLKDIV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKDIV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKDIV1_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKDIV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKDIV1` writer"]
pub struct W(crate::W<CLK_CLKDIV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKDIV1_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKDIV1_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKDIV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SC0DIV` reader - SC0 Clock Divide Number From SC0 Clock Source"]
pub struct SC0DIV_R(crate::FieldReader<u8, u8>);
impl SC0DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SC0DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC0DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC0DIV` writer - SC0 Clock Divide Number From SC0 Clock Source"]
pub struct SC0DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SC0 Clock Divide Number From SC0 Clock Source"]
    #[inline(always)]
    pub fn sc0div(&self) -> SC0DIV_R {
        SC0DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SC0 Clock Divide Number From SC0 Clock Source"]
    #[inline(always)]
    pub fn sc0div(&mut self) -> SC0DIV_W {
        SC0DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider Number Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clkdiv1](index.html) module"]
pub struct CLK_CLKDIV1_SPEC;
impl crate::RegisterSpec for CLK_CLKDIV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clkdiv1::R](R) reader structure"]
impl crate::Readable for CLK_CLKDIV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clkdiv1::W](W) writer structure"]
impl crate::Writable for CLK_CLKDIV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKDIV1 to value 0"]
impl crate::Resettable for CLK_CLKDIV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
