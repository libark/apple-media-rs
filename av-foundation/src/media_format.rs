use objc2_foundation::NSString;

pub type AVMediaType = NSString;

extern "C" {
    pub static AVMediaTypeVideo: &'static AVMediaType;
    pub static AVMediaTypeAudio: &'static AVMediaType;
    pub static AVMediaTypeText: &'static AVMediaType;
    pub static AVMediaTypeClosedCaption: &'static AVMediaType;
    pub static AVMediaTypeSubtitle: &'static AVMediaType;
    pub static AVMediaTypeTimecode: &'static AVMediaType;
    pub static AVMediaTypeMetadata: &'static AVMediaType;
    pub static AVMediaTypeMuxed: &'static AVMediaType;
    pub static AVMediaTypeHaptic: &'static AVMediaType;
    pub static AVMediaTypeMetadataObject: &'static AVMediaType;
    pub static AVMediaTypeDepthData: &'static AVMediaType;
}

pub type AVVideoRange = NSString;

extern "C" {
    pub static AVVideoRangeSDR: &'static AVVideoRange;
    pub static AVVideoRangeHLG: &'static AVVideoRange;
    pub static AVVideoRangePQ: &'static AVVideoRange;
}

pub type AVMediaCharacteristic = NSString;

extern "C" {
    pub static AVMediaCharacteristicVisual: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicAudible: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicLegible: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicFrameBased: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicUsesWideGamutColorSpace: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicContainsHDRVideo: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicContainsAlphaChannel: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicIsMainProgramContent: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicIsAuxiliaryContent: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicIsOriginalContent: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicContainsOnlyForcedSubtitles: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicTranscribesSpokenDialogForAccessibility: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicDescribesMusicAndSoundForAccessibility: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicEnhancesSpeechIntelligibility: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicEasyToRead: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicDescribesVideoForAccessibility: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicLanguageTranslation: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicDubbedTranslation: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicVoiceOverTranslation: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicTactileMinimal: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicContainsStereoMultiviewVideo: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicCarriesVideoStereoMetadata: &'static AVMediaCharacteristic;
    pub static AVMediaCharacteristicIndicatesHorizontalFieldOfView: &'static AVMediaCharacteristic;
}
