// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub phones: ::prost::alloc::vec::Vec<person::PhoneNumber>,
}
/// Nested message and enum types in `Person`.
pub mod person {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhoneNumber {
        #[prost(string, tag = "1")]
        pub number: ::prost::alloc::string::String,
        #[prost(enumeration = "phone_number::Type", tag = "2")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `PhoneNumber`.
    pub mod phone_number {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Type {
            Unspecified = 0,
            Mobile = 1,
            Home = 2,
            Work = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "TYPE_UNSPECIFIED",
                    Self::Mobile => "TYPE_MOBILE",
                    Self::Home => "TYPE_HOME",
                    Self::Work => "TYPE_WORK",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "TYPE_MOBILE" => Some(Self::Mobile),
                    "TYPE_HOME" => Some(Self::Home),
                    "TYPE_WORK" => Some(Self::Work),
                    _ => None,
                }
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Company {
    #[prost(message, repeated, tag = "1")]
    pub emails: ::prost::alloc::vec::Vec<company::EmailAddress>,
    #[prost(message, repeated, tag = "2")]
    pub phones: ::prost::alloc::vec::Vec<company::PhoneNumber>,
}
/// Nested message and enum types in `Company`.
pub mod company {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmailAddress {
        #[prost(string, tag = "1")]
        pub email: ::prost::alloc::string::String,
        #[prost(enumeration = "Department", tag = "2")]
        pub department: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhoneNumber {
        #[prost(string, tag = "1")]
        pub number: ::prost::alloc::string::String,
        #[prost(enumeration = "Department", tag = "2")]
        pub department: i32,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Department {
        Unspecified = 0,
        Hr = 1,
        CustomerService = 2,
    }
    impl Department {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DEPARTMENT_UNSPECIFIED",
                Self::Hr => "DEPARTMENT_HR",
                Self::CustomerService => "DEPARTMENT_CUSTOMER_SERVICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPARTMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "DEPARTMENT_HR" => Some(Self::Hr),
                "DEPARTMENT_CUSTOMER_SERVICE" => Some(Self::CustomerService),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(message, optional, tag = "1")]
    pub last_updated: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "contact::Kind", tags = "2, 3")]
    pub kind: ::core::option::Option<contact::Kind>,
}
/// Nested message and enum types in `Contact`.
pub mod contact {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "2")]
        Person(super::Person),
        #[prost(message, tag = "3")]
        Company(super::Company),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBook {
    #[prost(map = "string, message", tag = "1")]
    pub contacts: ::std::collections::HashMap<::prost::alloc::string::String, Contact>,
}
