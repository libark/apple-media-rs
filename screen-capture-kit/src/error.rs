use objc2_foundation::{NSInteger, NSString};

extern "C" {
    pub static SCStreamErrorDomain: &'static NSString;
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SCStreamErrorCode(pub NSInteger);

impl SCStreamErrorCode {
    #[doc(alias = "SCStreamErrorUserDeclined")]
    pub const UserDeclined: Self = Self(-3801);
    #[doc(alias = "SCStreamErrorFailedToStart")]
    pub const FailedToStart: Self = Self(-3802);
    #[doc(alias = "SCStreamErrorMissingEntitlements")]
    pub const MissingEntitlements: Self = Self(-3803);
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInvalid")]
    pub const FailedApplicationConnectionInvalid: Self = Self(-3804);
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInterrupted")]
    pub const FailedApplicationConnectionInterrupted: Self = Self(-3805);
    #[doc(alias = "SCStreamErrorFailedNoMatchingApplicationContext")]
    pub const FailedNoMatchingApplicationContext: Self = Self(-3806);
    #[doc(alias = "SCStreamErrorAttemptToStartStreamState")]
    pub const AttemptToStartStreamState: Self = Self(-3807);
    #[doc(alias = "SCStreamErrorAttemptToStopStreamState")]
    pub const AttemptToStopStreamState: Self = Self(-3808);
    #[doc(alias = "SCStreamErrorAttemptToUpdateFilterState")]
    pub const AttemptToUpdateFilterState: Self = Self(-3809);
    #[doc(alias = "SCStreamErrorAttemptToConfigState")]
    pub const AttemptToConfigState: Self = Self(-3810);
    #[doc(alias = "SCStreamErrorInternalError")]
    pub const InternalError: Self = Self(-3811);
    #[doc(alias = "SCStreamErrorInvalidParameter")]
    pub const InvalidParameter: Self = Self(-3812);
    #[doc(alias = "SCStreamErrorNoWindowList")]
    pub const NoWindowList: Self = Self(-3813);
    #[doc(alias = "SCStreamErrorNoDisplayList")]
    pub const NoDisplayList: Self = Self(-3814);
    #[doc(alias = "SCStreamErrorNoCaptureSource")]
    pub const NoCaptureSource: Self = Self(-3815);
    #[doc(alias = "SCStreamErrorRemovingStream")]
    pub const RemovingStream: Self = Self(-3816);
    #[doc(alias = "SCStreamErrorUserStopped")]
    pub const UserStopped: Self = Self(-3817);
    #[doc(alias = "SCStreamErrorFailedToStartAudioCapture")]
    pub const FailedToStartAudioCapture: Self = Self(-3818);
    #[doc(alias = "SCStreamErrorFailedToStopAudioCapture")]
    pub const FailedToStopAudioCapture: Self = Self(-3819);
}
