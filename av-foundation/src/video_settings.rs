use objc2_foundation::NSString;

pub type AVVideoCodecKey = NSString;

extern "C" {
    pub static AVVideoCodecTypeHEVC: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeH264: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeJPEG: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeAppleProRes4444: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeAppleProRes422: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeAppleProRes422HQ: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeAppleProRes422LT: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeAppleProRes422Proxy: &'static AVVideoCodecKey;
    pub static AVVideoCodecTypeHEVCWithAlpha: &'static AVVideoCodecKey;

    pub static AVVideoCodecHEVC: &'static NSString;
    pub static AVVideoCodecH264: &'static NSString;
    pub static AVVideoCodecJPEG: &'static NSString;
    #[cfg(target_os = "macos")]
    pub static AVVideoCodecAppleProRes4444: &'static NSString;
    #[cfg(target_os = "macos")]
    pub static AVVideoCodecAppleProRes422: &'static NSString;

    pub static AVVideoWidthKey: &'static NSString;
    pub static AVVideoHeightKey: &'static NSString;

    pub static AVVideoPixelAspectRatioKey: &'static NSString;
    pub static AVVideoPixelAspectRatioHorizontalSpacingKey: &'static NSString;
    pub static AVVideoPixelAspectRatioVerticalSpacingKey: &'static NSString;

    pub static AVVideoCleanApertureKey: &'static NSString;
    pub static AVVideoCleanApertureWidthKey: &'static NSString;
    pub static AVVideoCleanApertureHeightKey: &'static NSString;
    pub static AVVideoCleanApertureHorizontalOffsetKey: &'static NSString;
    pub static AVVideoCleanApertureVerticalOffsetKey: &'static NSString;

    pub static AVVideoScalingModeKey: &'static NSString;
    pub static AVVideoScalingModeFit: &'static NSString;
    pub static AVVideoScalingModeResize: &'static NSString;
    pub static AVVideoScalingModeResizeAspect: &'static NSString;
    pub static AVVideoScalingModeResizeAspectFill: &'static NSString;

    pub static AVVideoColorPropertiesKey: &'static NSString;
    pub static AVVideoColorPrimariesKey: &'static NSString;
    pub static AVVideoColorPrimaries_ITU_R_709_2: &'static NSString;
    pub static AVVideoColorPrimaries_EBU_3213: &'static NSString;
    pub static AVVideoColorPrimaries_SMPTE_C: &'static NSString;
    pub static AVVideoColorPrimaries_P3_D65: &'static NSString;
    pub static AVVideoColorPrimaries_ITU_R_2020: &'static NSString;
    pub static AVVideoTransferFunctionKey: &'static NSString;
    pub static AVVideoTransferFunction_ITU_R_709_2: &'static NSString;
    pub static AVVideoTransferFunction_SMPTE_240M_1995: &'static NSString;
    pub static AVVideoTransferFunction_SMPTE_ST_2084_PQ: &'static NSString;
    pub static AVVideoTransferFunction_ITU_R_2100_HLG: &'static NSString;
    pub static AVVideoTransferFunction_Linear: &'static NSString;
    pub static AVVideoYCbCrMatrixKey: &'static NSString;
    pub static AVVideoYCbCrMatrix_ITU_R_709_2: &'static NSString;
    pub static AVVideoYCbCrMatrix_ITU_R_601_4: &'static NSString;
    pub static AVVideoYCbCrMatrix_SMPTE_240M_1995: &'static NSString;
    pub static AVVideoYCbCrMatrix_ITU_R_2020: &'static NSString;

    pub static AVVideoAllowWideColorKey: &'static NSString;

    pub static AVVideoCompressionPropertiesKey: &'static NSString;
    pub static AVVideoAverageBitRateKey: &'static NSString;
    pub static AVVideoMaxKeyFrameIntervalKey: &'static NSString;
    pub static AVVideoMaxKeyFrameIntervalDurationKey: &'static NSString;
    pub static AVVideoAppleProRAWBitDepthKey: &'static NSString;

    pub static AVVideoAllowFrameReorderingKey: &'static NSString;
    pub static AVVideoProfileLevelKey: &'static NSString;
    pub static AVVideoProfileLevelH264Baseline30: &'static NSString;
    pub static AVVideoProfileLevelH264Baseline31: &'static NSString;
    pub static AVVideoProfileLevelH264Baseline41: &'static NSString;
    pub static AVVideoProfileLevelH264BaselineAutoLevel: &'static NSString;
    pub static AVVideoProfileLevelH264Main30: &'static NSString;
    pub static AVVideoProfileLevelH264Main31: &'static NSString;
    pub static AVVideoProfileLevelH264Main32: &'static NSString;
    pub static AVVideoProfileLevelH264Main41: &'static NSString;
    pub static AVVideoProfileLevelH264MainAutoLevel: &'static NSString;
    pub static AVVideoProfileLevelH264High40: &'static NSString;
    pub static AVVideoProfileLevelH264High41: &'static NSString;
    pub static AVVideoProfileLevelH264HighAutoLevel: &'static NSString;

    pub static AVVideoH264EntropyModeKey: &'static NSString;
    pub static AVVideoH264EntropyModeCAVLC: &'static NSString;
    pub static AVVideoH264EntropyModeCABAC: &'static NSString;

    pub static AVVideoExpectedSourceFrameRateKey: &'static NSString;
    pub static AVVideoAverageNonDroppableFrameRateKey: &'static NSString;
    pub static AVVideoDecompressionPropertiesKey: &'static NSString;
    pub static AVVideoEncoderSpecificationKey: &'static NSString;
}

pub type AVVideoApertureMode = NSString;

extern "C" {
    pub static AVVideoApertureModeCleanAperture: &'static AVVideoApertureMode;
    pub static AVVideoApertureModeProductionAperture: &'static AVVideoApertureMode;
    pub static AVVideoApertureModeEncodedPixels: &'static AVVideoApertureMode;
}
