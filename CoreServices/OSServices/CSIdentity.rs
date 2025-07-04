//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-security")]
use objc2_security::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentity?language=objc)
#[repr(C)]
pub struct CSIdentity {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CSIdentity {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CSIdentity"> for CSIdentity {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentityquery?language=objc)
#[repr(C)]
pub struct CSIdentityQuery {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CSIdentityQuery {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CSIdentityQuery"> for CSIdentityQuery {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentitygenerateposixname?language=objc)
    pub static kCSIdentityGeneratePosixName: Option<&'static CFString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentityclassuser?language=objc)
pub const kCSIdentityClassUser: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentityclassgroup?language=objc)
pub const kCSIdentityClassGroup: c_uint = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentityclass?language=objc)
pub type CSIdentityClass = CFIndex;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentityflagnone?language=objc)
pub const kCSIdentityFlagNone: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentityflaghidden?language=objc)
pub const kCSIdentityFlagHidden: c_uint = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentityflags?language=objc)
pub type CSIdentityFlags = CFOptionFlags;

unsafe impl ConcreteType for CSIdentity {
    #[doc(alias = "CSIdentityGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CSIdentityGetTypeID() -> CFTypeID;
        }
        unsafe { CSIdentityGetTypeID() }
    }
}

impl CSIdentity {
    #[doc(alias = "CSIdentityCreate")]
    #[cfg(feature = "CSIdentityAuthority")]
    #[inline]
    pub unsafe fn new(
        allocator: Option<&CFAllocator>,
        identity_class: CSIdentityClass,
        full_name: Option<&CFString>,
        posix_name: Option<&CFString>,
        flags: CSIdentityFlags,
        authority: Option<&CSIdentityAuthority>,
    ) -> Option<CFRetained<CSIdentity>> {
        extern "C-unwind" {
            fn CSIdentityCreate(
                allocator: Option<&CFAllocator>,
                identity_class: CSIdentityClass,
                full_name: Option<&CFString>,
                posix_name: Option<&CFString>,
                flags: CSIdentityFlags,
                authority: Option<&CSIdentityAuthority>,
            ) -> Option<NonNull<CSIdentity>>;
        }
        let ret = unsafe {
            CSIdentityCreate(
                allocator,
                identity_class,
                full_name,
                posix_name,
                flags,
                authority,
            )
        };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CSIdentityCreateCopy")]
    #[inline]
    pub unsafe fn new_copy(
        allocator: Option<&CFAllocator>,
        identity: Option<&CSIdentity>,
    ) -> Option<CFRetained<CSIdentity>> {
        extern "C-unwind" {
            fn CSIdentityCreateCopy(
                allocator: Option<&CFAllocator>,
                identity: Option<&CSIdentity>,
            ) -> Option<NonNull<CSIdentity>>;
        }
        let ret = unsafe { CSIdentityCreateCopy(allocator, identity) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CSIdentityGetClass")]
    #[inline]
    pub unsafe fn class(self: &CSIdentity) -> CSIdentityClass {
        extern "C-unwind" {
            fn CSIdentityGetClass(identity: &CSIdentity) -> CSIdentityClass;
        }
        unsafe { CSIdentityGetClass(self) }
    }

    #[doc(alias = "CSIdentityGetAuthority")]
    #[cfg(feature = "CSIdentityAuthority")]
    #[inline]
    pub unsafe fn authority(self: &CSIdentity) -> Option<CFRetained<CSIdentityAuthority>> {
        extern "C-unwind" {
            fn CSIdentityGetAuthority(
                identity: &CSIdentity,
            ) -> Option<NonNull<CSIdentityAuthority>>;
        }
        let ret = unsafe { CSIdentityGetAuthority(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetUUID")]
    #[inline]
    pub unsafe fn uuid(self: &CSIdentity) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CSIdentityGetUUID(identity: &CSIdentity) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe { CSIdentityGetUUID(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetFullName")]
    #[inline]
    pub unsafe fn full_name(self: &CSIdentity) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CSIdentityGetFullName(identity: &CSIdentity) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CSIdentityGetFullName(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetPosixID")]
    #[cfg(feature = "libc")]
    #[inline]
    pub unsafe fn posix_id(self: &CSIdentity) -> libc::id_t {
        extern "C-unwind" {
            fn CSIdentityGetPosixID(identity: &CSIdentity) -> libc::id_t;
        }
        unsafe { CSIdentityGetPosixID(self) }
    }

    #[doc(alias = "CSIdentityGetPosixName")]
    #[inline]
    pub unsafe fn posix_name(self: &CSIdentity) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CSIdentityGetPosixName(identity: &CSIdentity) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CSIdentityGetPosixName(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetEmailAddress")]
    #[inline]
    pub unsafe fn email_address(self: &CSIdentity) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CSIdentityGetEmailAddress(identity: &CSIdentity) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CSIdentityGetEmailAddress(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetImageURL")]
    #[inline]
    pub unsafe fn image_url(self: &CSIdentity) -> Option<CFRetained<CFURL>> {
        extern "C-unwind" {
            fn CSIdentityGetImageURL(identity: &CSIdentity) -> Option<NonNull<CFURL>>;
        }
        let ret = unsafe { CSIdentityGetImageURL(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetImageData")]
    #[inline]
    pub unsafe fn image_data(self: &CSIdentity) -> Option<CFRetained<CFData>> {
        extern "C-unwind" {
            fn CSIdentityGetImageData(identity: &CSIdentity) -> Option<NonNull<CFData>>;
        }
        let ret = unsafe { CSIdentityGetImageData(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetImageDataType")]
    #[inline]
    pub unsafe fn image_data_type(self: &CSIdentity) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CSIdentityGetImageDataType(identity: &CSIdentity) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CSIdentityGetImageDataType(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityGetAliases")]
    #[inline]
    pub unsafe fn aliases(self: &CSIdentity) -> Option<CFRetained<CFArray>> {
        extern "C-unwind" {
            fn CSIdentityGetAliases(identity: &CSIdentity) -> Option<NonNull<CFArray>>;
        }
        let ret = unsafe { CSIdentityGetAliases(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityIsMemberOfGroup")]
    #[inline]
    pub unsafe fn is_member_of_group(self: &CSIdentity, group: Option<&CSIdentity>) -> bool {
        extern "C-unwind" {
            fn CSIdentityIsMemberOfGroup(
                identity: &CSIdentity,
                group: Option<&CSIdentity>,
            ) -> Boolean;
        }
        let ret = unsafe { CSIdentityIsMemberOfGroup(self, group) };
        ret != 0
    }

    #[doc(alias = "CSIdentityIsHidden")]
    #[inline]
    pub unsafe fn is_hidden(self: &CSIdentity) -> bool {
        extern "C-unwind" {
            fn CSIdentityIsHidden(identity: &CSIdentity) -> Boolean;
        }
        let ret = unsafe { CSIdentityIsHidden(self) };
        ret != 0
    }

    #[doc(alias = "CSIdentityCreatePersistentReference")]
    #[inline]
    pub unsafe fn new_persistent_reference(
        allocator: Option<&CFAllocator>,
        identity: Option<&CSIdentity>,
    ) -> Option<CFRetained<CFData>> {
        extern "C-unwind" {
            fn CSIdentityCreatePersistentReference(
                allocator: Option<&CFAllocator>,
                identity: Option<&CSIdentity>,
            ) -> Option<NonNull<CFData>>;
        }
        let ret = unsafe { CSIdentityCreatePersistentReference(allocator, identity) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CSIdentityIsEnabled")]
    #[inline]
    pub unsafe fn is_enabled(self: &CSIdentity) -> bool {
        extern "C-unwind" {
            fn CSIdentityIsEnabled(user: &CSIdentity) -> Boolean;
        }
        let ret = unsafe { CSIdentityIsEnabled(self) };
        ret != 0
    }

    #[doc(alias = "CSIdentityAuthenticateUsingPassword")]
    #[inline]
    pub unsafe fn authenticate_using_password(
        self: &CSIdentity,
        password: Option<&CFString>,
    ) -> bool {
        extern "C-unwind" {
            fn CSIdentityAuthenticateUsingPassword(
                user: &CSIdentity,
                password: Option<&CFString>,
            ) -> Boolean;
        }
        let ret = unsafe { CSIdentityAuthenticateUsingPassword(self, password) };
        ret != 0
    }

    #[doc(alias = "CSIdentityGetCertificate")]
    #[cfg(feature = "objc2-security")]
    #[inline]
    pub unsafe fn certificate(self: &CSIdentity) -> Option<CFRetained<SecCertificate>> {
        extern "C-unwind" {
            fn CSIdentityGetCertificate(user: &CSIdentity) -> Option<NonNull<SecCertificate>>;
        }
        let ret = unsafe { CSIdentityGetCertificate(self) };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CSIdentityCreateGroupMembershipQuery")]
    #[inline]
    pub unsafe fn new_group_membership_query(
        allocator: Option<&CFAllocator>,
        group: Option<&CSIdentity>,
    ) -> Option<CFRetained<CSIdentityQuery>> {
        extern "C-unwind" {
            fn CSIdentityCreateGroupMembershipQuery(
                allocator: Option<&CFAllocator>,
                group: Option<&CSIdentity>,
            ) -> Option<NonNull<CSIdentityQuery>>;
        }
        let ret = unsafe { CSIdentityCreateGroupMembershipQuery(allocator, group) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CSIdentitySetFullName")]
    #[inline]
    pub unsafe fn set_full_name(self: &CSIdentity, full_name: Option<&CFString>) {
        extern "C-unwind" {
            fn CSIdentitySetFullName(identity: &CSIdentity, full_name: Option<&CFString>);
        }
        unsafe { CSIdentitySetFullName(self, full_name) }
    }

    #[doc(alias = "CSIdentitySetEmailAddress")]
    #[inline]
    pub unsafe fn set_email_address(self: &CSIdentity, email_address: Option<&CFString>) {
        extern "C-unwind" {
            fn CSIdentitySetEmailAddress(identity: &CSIdentity, email_address: Option<&CFString>);
        }
        unsafe { CSIdentitySetEmailAddress(self, email_address) }
    }

    #[doc(alias = "CSIdentitySetImageURL")]
    #[inline]
    pub unsafe fn set_image_url(self: &CSIdentity, url: Option<&CFURL>) {
        extern "C-unwind" {
            fn CSIdentitySetImageURL(identity: &CSIdentity, url: Option<&CFURL>);
        }
        unsafe { CSIdentitySetImageURL(self, url) }
    }

    #[doc(alias = "CSIdentitySetImageData")]
    #[inline]
    pub unsafe fn set_image_data(
        self: &CSIdentity,
        image_data: Option<&CFData>,
        image_data_type: Option<&CFString>,
    ) {
        extern "C-unwind" {
            fn CSIdentitySetImageData(
                identity: &CSIdentity,
                image_data: Option<&CFData>,
                image_data_type: Option<&CFString>,
            );
        }
        unsafe { CSIdentitySetImageData(self, image_data, image_data_type) }
    }

    #[doc(alias = "CSIdentityAddAlias")]
    #[inline]
    pub unsafe fn add_alias(self: &CSIdentity, alias: Option<&CFString>) {
        extern "C-unwind" {
            fn CSIdentityAddAlias(identity: &CSIdentity, alias: Option<&CFString>);
        }
        unsafe { CSIdentityAddAlias(self, alias) }
    }

    #[doc(alias = "CSIdentityRemoveAlias")]
    #[inline]
    pub unsafe fn remove_alias(self: &CSIdentity, alias: Option<&CFString>) {
        extern "C-unwind" {
            fn CSIdentityRemoveAlias(identity: &CSIdentity, alias: Option<&CFString>);
        }
        unsafe { CSIdentityRemoveAlias(self, alias) }
    }

    #[doc(alias = "CSIdentityAddMember")]
    #[inline]
    pub unsafe fn add_member(self: &CSIdentity, member: Option<&CSIdentity>) {
        extern "C-unwind" {
            fn CSIdentityAddMember(group: &CSIdentity, member: Option<&CSIdentity>);
        }
        unsafe { CSIdentityAddMember(self, member) }
    }

    #[doc(alias = "CSIdentityRemoveMember")]
    #[inline]
    pub unsafe fn remove_member(self: &CSIdentity, member: Option<&CSIdentity>) {
        extern "C-unwind" {
            fn CSIdentityRemoveMember(group: &CSIdentity, member: Option<&CSIdentity>);
        }
        unsafe { CSIdentityRemoveMember(self, member) }
    }

    #[doc(alias = "CSIdentitySetIsEnabled")]
    #[inline]
    pub unsafe fn set_is_enabled(self: &CSIdentity, is_enabled: bool) {
        extern "C-unwind" {
            fn CSIdentitySetIsEnabled(user: &CSIdentity, is_enabled: Boolean);
        }
        unsafe { CSIdentitySetIsEnabled(self, is_enabled as _) }
    }

    #[doc(alias = "CSIdentitySetPassword")]
    #[inline]
    pub unsafe fn set_password(self: &CSIdentity, password: Option<&CFString>) {
        extern "C-unwind" {
            fn CSIdentitySetPassword(user: &CSIdentity, password: Option<&CFString>);
        }
        unsafe { CSIdentitySetPassword(self, password) }
    }

    #[doc(alias = "CSIdentitySetCertificate")]
    #[cfg(feature = "objc2-security")]
    #[inline]
    pub unsafe fn set_certificate(self: &CSIdentity, certificate: Option<&SecCertificate>) {
        extern "C-unwind" {
            fn CSIdentitySetCertificate(user: &CSIdentity, certificate: Option<&SecCertificate>);
        }
        unsafe { CSIdentitySetCertificate(self, certificate) }
    }

    #[doc(alias = "CSIdentityDelete")]
    #[inline]
    pub unsafe fn delete(self: &CSIdentity) {
        extern "C-unwind" {
            fn CSIdentityDelete(identity: &CSIdentity);
        }
        unsafe { CSIdentityDelete(self) }
    }

    #[doc(alias = "CSIdentityCommit")]
    #[cfg(feature = "objc2-security")]
    #[inline]
    pub unsafe fn commit(
        self: &CSIdentity,
        authorization: AuthorizationRef,
        error: *mut *mut CFError,
    ) -> bool {
        extern "C-unwind" {
            fn CSIdentityCommit(
                identity: &CSIdentity,
                authorization: AuthorizationRef,
                error: *mut *mut CFError,
            ) -> Boolean;
        }
        let ret = unsafe { CSIdentityCommit(self, authorization, error) };
        ret != 0
    }
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/kcsidentitycommitcompleted?language=objc)
pub const kCSIdentityCommitCompleted: c_uint = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentitystatusupdatedcallback?language=objc)
pub type CSIdentityStatusUpdatedCallback =
    Option<unsafe extern "C-unwind" fn(*mut CSIdentity, CFIndex, *mut CFError, *mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coreservices/csidentityclientcontext?language=objc)
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSIdentityClientContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
    pub statusUpdated: CSIdentityStatusUpdatedCallback,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CSIdentityClientContext {
    const ENCODING: Encoding = Encoding::Struct(
        "CSIdentityClientContext",
        &[
            <CFIndex>::ENCODING,
            <*mut c_void>::ENCODING,
            <CFAllocatorRetainCallBack>::ENCODING,
            <CFAllocatorReleaseCallBack>::ENCODING,
            <CFAllocatorCopyDescriptionCallBack>::ENCODING,
            <CSIdentityStatusUpdatedCallback>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CSIdentityClientContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CSIdentity {
    #[doc(alias = "CSIdentityCommitAsynchronously")]
    #[cfg(feature = "objc2-security")]
    #[inline]
    pub unsafe fn commit_asynchronously(
        self: &CSIdentity,
        client_context: *const CSIdentityClientContext,
        run_loop: Option<&CFRunLoop>,
        run_loop_mode: Option<&CFString>,
        authorization: AuthorizationRef,
    ) -> bool {
        extern "C-unwind" {
            fn CSIdentityCommitAsynchronously(
                identity: &CSIdentity,
                client_context: *const CSIdentityClientContext,
                run_loop: Option<&CFRunLoop>,
                run_loop_mode: Option<&CFString>,
                authorization: AuthorizationRef,
            ) -> Boolean;
        }
        let ret = unsafe {
            CSIdentityCommitAsynchronously(
                self,
                client_context,
                run_loop,
                run_loop_mode,
                authorization,
            )
        };
        ret != 0
    }

    #[doc(alias = "CSIdentityIsCommitting")]
    #[inline]
    pub unsafe fn is_committing(self: &CSIdentity) -> bool {
        extern "C-unwind" {
            fn CSIdentityIsCommitting(identity: &CSIdentity) -> Boolean;
        }
        let ret = unsafe { CSIdentityIsCommitting(self) };
        ret != 0
    }

    #[doc(alias = "CSIdentityRemoveClient")]
    #[inline]
    pub unsafe fn remove_client(self: &CSIdentity) {
        extern "C-unwind" {
            fn CSIdentityRemoveClient(identity: &CSIdentity);
        }
        unsafe { CSIdentityRemoveClient(self) }
    }
}

#[cfg(feature = "CSIdentityAuthority")]
#[deprecated = "renamed to `CSIdentity::new`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCreate(
    allocator: Option<&CFAllocator>,
    identity_class: CSIdentityClass,
    full_name: Option<&CFString>,
    posix_name: Option<&CFString>,
    flags: CSIdentityFlags,
    authority: Option<&CSIdentityAuthority>,
) -> Option<CFRetained<CSIdentity>> {
    extern "C-unwind" {
        fn CSIdentityCreate(
            allocator: Option<&CFAllocator>,
            identity_class: CSIdentityClass,
            full_name: Option<&CFString>,
            posix_name: Option<&CFString>,
            flags: CSIdentityFlags,
            authority: Option<&CSIdentityAuthority>,
        ) -> Option<NonNull<CSIdentity>>;
    }
    let ret = unsafe {
        CSIdentityCreate(
            allocator,
            identity_class,
            full_name,
            posix_name,
            flags,
            authority,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CSIdentity::new_copy`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCreateCopy(
    allocator: Option<&CFAllocator>,
    identity: Option<&CSIdentity>,
) -> Option<CFRetained<CSIdentity>> {
    extern "C-unwind" {
        fn CSIdentityCreateCopy(
            allocator: Option<&CFAllocator>,
            identity: Option<&CSIdentity>,
        ) -> Option<NonNull<CSIdentity>>;
    }
    let ret = unsafe { CSIdentityCreateCopy(allocator, identity) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::class`"]
    pub fn CSIdentityGetClass(identity: &CSIdentity) -> CSIdentityClass;
}

#[cfg(feature = "CSIdentityAuthority")]
#[deprecated = "renamed to `CSIdentity::authority`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetAuthority(
    identity: &CSIdentity,
) -> Option<CFRetained<CSIdentityAuthority>> {
    extern "C-unwind" {
        fn CSIdentityGetAuthority(identity: &CSIdentity) -> Option<NonNull<CSIdentityAuthority>>;
    }
    let ret = unsafe { CSIdentityGetAuthority(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::uuid`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetUUID(
    identity: &CSIdentity,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CSIdentityGetUUID(identity: &CSIdentity) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe { CSIdentityGetUUID(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::full_name`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetFullName(
    identity: &CSIdentity,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CSIdentityGetFullName(identity: &CSIdentity) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CSIdentityGetFullName(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    #[deprecated = "renamed to `CSIdentity::posix_id`"]
    pub fn CSIdentityGetPosixID(identity: &CSIdentity) -> libc::id_t;
}

#[deprecated = "renamed to `CSIdentity::posix_name`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetPosixName(
    identity: &CSIdentity,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CSIdentityGetPosixName(identity: &CSIdentity) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CSIdentityGetPosixName(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::email_address`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetEmailAddress(
    identity: &CSIdentity,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CSIdentityGetEmailAddress(identity: &CSIdentity) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CSIdentityGetEmailAddress(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::image_url`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetImageURL(
    identity: &CSIdentity,
) -> Option<CFRetained<CFURL>> {
    extern "C-unwind" {
        fn CSIdentityGetImageURL(identity: &CSIdentity) -> Option<NonNull<CFURL>>;
    }
    let ret = unsafe { CSIdentityGetImageURL(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::image_data`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetImageData(
    identity: &CSIdentity,
) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CSIdentityGetImageData(identity: &CSIdentity) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CSIdentityGetImageData(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::image_data_type`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetImageDataType(
    identity: &CSIdentity,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CSIdentityGetImageDataType(identity: &CSIdentity) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CSIdentityGetImageDataType(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::aliases`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetAliases(
    identity: &CSIdentity,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn CSIdentityGetAliases(identity: &CSIdentity) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { CSIdentityGetAliases(identity) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::is_member_of_group`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityIsMemberOfGroup(
    identity: &CSIdentity,
    group: Option<&CSIdentity>,
) -> bool {
    extern "C-unwind" {
        fn CSIdentityIsMemberOfGroup(identity: &CSIdentity, group: Option<&CSIdentity>) -> Boolean;
    }
    let ret = unsafe { CSIdentityIsMemberOfGroup(identity, group) };
    ret != 0
}

#[deprecated = "renamed to `CSIdentity::is_hidden`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityIsHidden(identity: &CSIdentity) -> bool {
    extern "C-unwind" {
        fn CSIdentityIsHidden(identity: &CSIdentity) -> Boolean;
    }
    let ret = unsafe { CSIdentityIsHidden(identity) };
    ret != 0
}

#[deprecated = "renamed to `CSIdentity::new_persistent_reference`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCreatePersistentReference(
    allocator: Option<&CFAllocator>,
    identity: Option<&CSIdentity>,
) -> Option<CFRetained<CFData>> {
    extern "C-unwind" {
        fn CSIdentityCreatePersistentReference(
            allocator: Option<&CFAllocator>,
            identity: Option<&CSIdentity>,
        ) -> Option<NonNull<CFData>>;
    }
    let ret = unsafe { CSIdentityCreatePersistentReference(allocator, identity) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CSIdentity::is_enabled`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityIsEnabled(user: &CSIdentity) -> bool {
    extern "C-unwind" {
        fn CSIdentityIsEnabled(user: &CSIdentity) -> Boolean;
    }
    let ret = unsafe { CSIdentityIsEnabled(user) };
    ret != 0
}

#[deprecated = "renamed to `CSIdentity::authenticate_using_password`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityAuthenticateUsingPassword(
    user: &CSIdentity,
    password: Option<&CFString>,
) -> bool {
    extern "C-unwind" {
        fn CSIdentityAuthenticateUsingPassword(
            user: &CSIdentity,
            password: Option<&CFString>,
        ) -> Boolean;
    }
    let ret = unsafe { CSIdentityAuthenticateUsingPassword(user, password) };
    ret != 0
}

#[cfg(feature = "objc2-security")]
#[deprecated = "renamed to `CSIdentity::certificate`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityGetCertificate(
    user: &CSIdentity,
) -> Option<CFRetained<SecCertificate>> {
    extern "C-unwind" {
        fn CSIdentityGetCertificate(user: &CSIdentity) -> Option<NonNull<SecCertificate>>;
    }
    let ret = unsafe { CSIdentityGetCertificate(user) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CSIdentity::new_group_membership_query`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCreateGroupMembershipQuery(
    allocator: Option<&CFAllocator>,
    group: Option<&CSIdentity>,
) -> Option<CFRetained<CSIdentityQuery>> {
    extern "C-unwind" {
        fn CSIdentityCreateGroupMembershipQuery(
            allocator: Option<&CFAllocator>,
            group: Option<&CSIdentity>,
        ) -> Option<NonNull<CSIdentityQuery>>;
    }
    let ret = unsafe { CSIdentityCreateGroupMembershipQuery(allocator, group) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::set_full_name`"]
    pub fn CSIdentitySetFullName(identity: &CSIdentity, full_name: Option<&CFString>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::set_email_address`"]
    pub fn CSIdentitySetEmailAddress(identity: &CSIdentity, email_address: Option<&CFString>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::set_image_url`"]
    pub fn CSIdentitySetImageURL(identity: &CSIdentity, url: Option<&CFURL>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::set_image_data`"]
    pub fn CSIdentitySetImageData(
        identity: &CSIdentity,
        image_data: Option<&CFData>,
        image_data_type: Option<&CFString>,
    );
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::add_alias`"]
    pub fn CSIdentityAddAlias(identity: &CSIdentity, alias: Option<&CFString>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::remove_alias`"]
    pub fn CSIdentityRemoveAlias(identity: &CSIdentity, alias: Option<&CFString>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::add_member`"]
    pub fn CSIdentityAddMember(group: &CSIdentity, member: Option<&CSIdentity>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::remove_member`"]
    pub fn CSIdentityRemoveMember(group: &CSIdentity, member: Option<&CSIdentity>);
}

#[deprecated = "renamed to `CSIdentity::set_is_enabled`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentitySetIsEnabled(user: &CSIdentity, is_enabled: bool) {
    extern "C-unwind" {
        fn CSIdentitySetIsEnabled(user: &CSIdentity, is_enabled: Boolean);
    }
    unsafe { CSIdentitySetIsEnabled(user, is_enabled as _) }
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::set_password`"]
    pub fn CSIdentitySetPassword(user: &CSIdentity, password: Option<&CFString>);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-security")]
    #[deprecated = "renamed to `CSIdentity::set_certificate`"]
    pub fn CSIdentitySetCertificate(user: &CSIdentity, certificate: Option<&SecCertificate>);
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::delete`"]
    pub fn CSIdentityDelete(identity: &CSIdentity);
}

#[cfg(feature = "objc2-security")]
#[deprecated = "renamed to `CSIdentity::commit`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCommit(
    identity: &CSIdentity,
    authorization: AuthorizationRef,
    error: *mut *mut CFError,
) -> bool {
    extern "C-unwind" {
        fn CSIdentityCommit(
            identity: &CSIdentity,
            authorization: AuthorizationRef,
            error: *mut *mut CFError,
        ) -> Boolean;
    }
    let ret = unsafe { CSIdentityCommit(identity, authorization, error) };
    ret != 0
}

#[cfg(feature = "objc2-security")]
#[deprecated = "renamed to `CSIdentity::commit_asynchronously`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityCommitAsynchronously(
    identity: &CSIdentity,
    client_context: *const CSIdentityClientContext,
    run_loop: Option<&CFRunLoop>,
    run_loop_mode: Option<&CFString>,
    authorization: AuthorizationRef,
) -> bool {
    extern "C-unwind" {
        fn CSIdentityCommitAsynchronously(
            identity: &CSIdentity,
            client_context: *const CSIdentityClientContext,
            run_loop: Option<&CFRunLoop>,
            run_loop_mode: Option<&CFString>,
            authorization: AuthorizationRef,
        ) -> Boolean;
    }
    let ret = unsafe {
        CSIdentityCommitAsynchronously(
            identity,
            client_context,
            run_loop,
            run_loop_mode,
            authorization,
        )
    };
    ret != 0
}

#[deprecated = "renamed to `CSIdentity::is_committing`"]
#[inline]
pub unsafe extern "C-unwind" fn CSIdentityIsCommitting(identity: &CSIdentity) -> bool {
    extern "C-unwind" {
        fn CSIdentityIsCommitting(identity: &CSIdentity) -> Boolean;
    }
    let ret = unsafe { CSIdentityIsCommitting(identity) };
    ret != 0
}

extern "C-unwind" {
    #[deprecated = "renamed to `CSIdentity::remove_client`"]
    pub fn CSIdentityRemoveClient(identity: &CSIdentity);
}
