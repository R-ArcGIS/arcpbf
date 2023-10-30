#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureCollectionPBuffer {
    /// Any compliant implementation must first read the version
    /// number encoded in this message and choose the correct
    /// implementation for this version number before proceeding to
    /// decode other parts of this message.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub query_result: ::core::option::Option<feature_collection_p_buffer::QueryResult>,
}
/// Nested message and enum types in `FeatureCollectionPBuffer`.
pub mod feature_collection_p_buffer {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpatialReference {
        #[prost(uint32, tag = "1")]
        pub wkid: u32,
        #[prost(uint32, tag = "2")]
        pub latest_wkid: u32,
        #[prost(uint32, tag = "3")]
        pub vcs_wkid: u32,
        #[prost(uint32, tag = "4")]
        pub latest_vcs_wkid: u32,
        #[prost(string, tag = "5")]
        pub wkt: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(enumeration = "FieldType", tag = "2")]
        pub field_type: i32,
        #[prost(string, tag = "3")]
        pub alias: ::prost::alloc::string::String,
        #[prost(enumeration = "SqlType", tag = "4")]
        pub sql_type: i32,
        #[prost(string, tag = "5")]
        pub domain: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub default_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Value {
        #[prost(oneof = "value::ValueType", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
        pub value_type: ::core::option::Option<value::ValueType>,
    }
    /// Nested message and enum types in `Value`.
    pub mod value {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueType {
            /// Exactly one of these values must be present in a valid message
            #[prost(string, tag = "1")]
            StringValue(::prost::alloc::string::String),
            #[prost(float, tag = "2")]
            FloatValue(f32),
            #[prost(double, tag = "3")]
            DoubleValue(f64),
            #[prost(sint32, tag = "4")]
            SintValue(i32),
            #[prost(uint32, tag = "5")]
            UintValue(u32),
            #[prost(int64, tag = "6")]
            Int64Value(i64),
            #[prost(uint64, tag = "7")]
            Uint64Value(u64),
            #[prost(sint64, tag = "8")]
            Sint64Value(i64),
            #[prost(bool, tag = "9")]
            BoolValue(bool),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Geometry {
        /// coordinate structure in lengths
        #[prost(uint32, repeated, tag = "2")]
        pub lengths: ::prost::alloc::vec::Vec<u32>,
        /// delta-encoded integer values
        #[prost(sint64, repeated, tag = "3")]
        pub coords: ::prost::alloc::vec::Vec<i64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EsriShapeBuffer {
        #[prost(bytes = "vec", tag = "1")]
        pub bytes: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Feature {
        #[prost(message, repeated, tag = "1")]
        pub attributes: ::prost::alloc::vec::Vec<Value>,
        #[prost(message, optional, tag = "4")]
        pub centroid: ::core::option::Option<Geometry>,
        #[prost(oneof = "feature::CompressedGeometry", tags = "2, 3")]
        pub compressed_geometry: ::core::option::Option<feature::CompressedGeometry>,
    }
    /// Nested message and enum types in `Feature`.
    pub mod feature {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CompressedGeometry {
            #[prost(message, tag = "2")]
            Geometry(super::Geometry),
            #[prost(message, tag = "3")]
            ShapeBuffer(super::EsriShapeBuffer),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UniqueIdField {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub is_system_maintained: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeometryProperties {
        #[prost(string, tag = "1")]
        pub shape_area_field_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub shape_length_field_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub units: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGens {
        #[prost(uint64, tag = "1")]
        pub min_server_gen: u64,
        #[prost(uint64, tag = "2")]
        pub server_gen: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scale {
        #[prost(double, tag = "1")]
        pub x_scale: f64,
        #[prost(double, tag = "2")]
        pub y_scale: f64,
        #[prost(double, tag = "3")]
        pub m_scale: f64,
        #[prost(double, tag = "4")]
        pub z_scale: f64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Translate {
        #[prost(double, tag = "1")]
        pub x_translate: f64,
        #[prost(double, tag = "2")]
        pub y_translate: f64,
        #[prost(double, tag = "3")]
        pub m_translate: f64,
        #[prost(double, tag = "4")]
        pub z_translate: f64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transform {
        #[prost(enumeration = "QuantizeOriginPostion", tag = "1")]
        pub quantize_origin_postion: i32,
        #[prost(message, optional, tag = "2")]
        pub scale: ::core::option::Option<Scale>,
        #[prost(message, optional, tag = "3")]
        pub translate: ::core::option::Option<Translate>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeatureResult {
        #[prost(string, tag = "1")]
        pub object_id_field_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub unique_id_field: ::core::option::Option<UniqueIdField>,
        #[prost(string, tag = "3")]
        pub global_id_field_name: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub geohash_field_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "5")]
        pub geometry_properties: ::core::option::Option<GeometryProperties>,
        #[prost(message, optional, tag = "6")]
        pub server_gens: ::core::option::Option<ServerGens>,
        #[prost(enumeration = "GeometryType", tag = "7")]
        pub geometry_type: i32,
        #[prost(message, optional, tag = "8")]
        pub spatial_reference: ::core::option::Option<SpatialReference>,
        #[prost(bool, tag = "9")]
        pub exceeded_transfer_limit: bool,
        #[prost(bool, tag = "10")]
        pub has_z: bool,
        #[prost(bool, tag = "11")]
        pub has_m: bool,
        #[prost(message, optional, tag = "12")]
        pub transform: ::core::option::Option<Transform>,
        #[prost(message, repeated, tag = "13")]
        pub fields: ::prost::alloc::vec::Vec<Field>,
        #[prost(message, repeated, tag = "14")]
        pub values: ::prost::alloc::vec::Vec<Value>,
        #[prost(message, repeated, tag = "15")]
        pub features: ::prost::alloc::vec::Vec<Feature>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CountResult {
        #[prost(uint64, tag = "1")]
        pub count: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectIdsResult {
        #[prost(string, tag = "1")]
        pub object_id_field_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub server_gens: ::core::option::Option<ServerGens>,
        #[prost(uint64, repeated, tag = "3")]
        pub object_ids: ::prost::alloc::vec::Vec<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryResult {
        #[prost(oneof = "query_result::Results", tags = "1, 2, 3")]
        pub results: ::core::option::Option<query_result::Results>,
    }
    /// Nested message and enum types in `QueryResult`.
    pub mod query_result {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Results {
            #[prost(message, tag = "1")]
            FeatureResult(super::FeatureResult),
            #[prost(message, tag = "2")]
            CountResult(super::CountResult),
            #[prost(message, tag = "3")]
            IdsResult(super::ObjectIdsResult),
        }
    }
    /// GeometryType
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
    pub enum GeometryType {
        EsriGeometryTypePoint = 0,
        EsriGeometryTypeMultipoint = 1,
        EsriGeometryTypePolyline = 2,
        EsriGeometryTypePolygon = 3,
        EsriGeometryTypeMultipatch = 4,
        EsriGeometryTypeNone = 127,
    }
    impl GeometryType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GeometryType::EsriGeometryTypePoint => "esriGeometryTypePoint",
                GeometryType::EsriGeometryTypeMultipoint => "esriGeometryTypeMultipoint",
                GeometryType::EsriGeometryTypePolyline => "esriGeometryTypePolyline",
                GeometryType::EsriGeometryTypePolygon => "esriGeometryTypePolygon",
                GeometryType::EsriGeometryTypeMultipatch => "esriGeometryTypeMultipatch",
                GeometryType::EsriGeometryTypeNone => "esriGeometryTypeNone",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "esriGeometryTypePoint" => Some(Self::EsriGeometryTypePoint),
                "esriGeometryTypeMultipoint" => Some(Self::EsriGeometryTypeMultipoint),
                "esriGeometryTypePolyline" => Some(Self::EsriGeometryTypePolyline),
                "esriGeometryTypePolygon" => Some(Self::EsriGeometryTypePolygon),
                "esriGeometryTypeMultipatch" => Some(Self::EsriGeometryTypeMultipatch),
                "esriGeometryTypeNone" => Some(Self::EsriGeometryTypeNone),
                _ => None,
            }
        }
    }
    /// FieldType
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
    pub enum FieldType {
        EsriFieldTypeSmallInteger = 0,
        EsriFieldTypeInteger = 1,
        EsriFieldTypeSingle = 2,
        EsriFieldTypeDouble = 3,
        EsriFieldTypeString = 4,
        EsriFieldTypeDate = 5,
        EsriFieldTypeOid = 6,
        EsriFieldTypeGeometry = 7,
        EsriFieldTypeBlob = 8,
        EsriFieldTypeRaster = 9,
        EsriFieldTypeGuid = 10,
        EsriFieldTypeGlobalId = 11,
        EsriFieldTypeXml = 12,
    }
    impl FieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FieldType::EsriFieldTypeSmallInteger => "esriFieldTypeSmallInteger",
                FieldType::EsriFieldTypeInteger => "esriFieldTypeInteger",
                FieldType::EsriFieldTypeSingle => "esriFieldTypeSingle",
                FieldType::EsriFieldTypeDouble => "esriFieldTypeDouble",
                FieldType::EsriFieldTypeString => "esriFieldTypeString",
                FieldType::EsriFieldTypeDate => "esriFieldTypeDate",
                FieldType::EsriFieldTypeOid => "esriFieldTypeOID",
                FieldType::EsriFieldTypeGeometry => "esriFieldTypeGeometry",
                FieldType::EsriFieldTypeBlob => "esriFieldTypeBlob",
                FieldType::EsriFieldTypeRaster => "esriFieldTypeRaster",
                FieldType::EsriFieldTypeGuid => "esriFieldTypeGUID",
                FieldType::EsriFieldTypeGlobalId => "esriFieldTypeGlobalID",
                FieldType::EsriFieldTypeXml => "esriFieldTypeXML",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "esriFieldTypeSmallInteger" => Some(Self::EsriFieldTypeSmallInteger),
                "esriFieldTypeInteger" => Some(Self::EsriFieldTypeInteger),
                "esriFieldTypeSingle" => Some(Self::EsriFieldTypeSingle),
                "esriFieldTypeDouble" => Some(Self::EsriFieldTypeDouble),
                "esriFieldTypeString" => Some(Self::EsriFieldTypeString),
                "esriFieldTypeDate" => Some(Self::EsriFieldTypeDate),
                "esriFieldTypeOID" => Some(Self::EsriFieldTypeOid),
                "esriFieldTypeGeometry" => Some(Self::EsriFieldTypeGeometry),
                "esriFieldTypeBlob" => Some(Self::EsriFieldTypeBlob),
                "esriFieldTypeRaster" => Some(Self::EsriFieldTypeRaster),
                "esriFieldTypeGUID" => Some(Self::EsriFieldTypeGuid),
                "esriFieldTypeGlobalID" => Some(Self::EsriFieldTypeGlobalId),
                "esriFieldTypeXML" => Some(Self::EsriFieldTypeXml),
                _ => None,
            }
        }
    }
    /// FieldType
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
    pub enum SqlType {
        BigInt = 0,
        Binary = 1,
        Bit = 2,
        Char = 3,
        Date = 4,
        Decimal = 5,
        Double = 6,
        Float = 7,
        Geometry = 8,
        Guid = 9,
        Integer = 10,
        LongNVarchar = 11,
        LongVarbinary = 12,
        LongVarchar = 13,
        NChar = 14,
        NVarchar = 15,
        Other = 16,
        Real = 17,
        SmallInt = 18,
        SqlXml = 19,
        Time = 20,
        Timestamp = 21,
        Timestamp2 = 22,
        TinyInt = 23,
        Varbinary = 24,
        Varchar = 25,
    }
    impl SqlType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SqlType::BigInt => "sqlTypeBigInt",
                SqlType::Binary => "sqlTypeBinary",
                SqlType::Bit => "sqlTypeBit",
                SqlType::Char => "sqlTypeChar",
                SqlType::Date => "sqlTypeDate",
                SqlType::Decimal => "sqlTypeDecimal",
                SqlType::Double => "sqlTypeDouble",
                SqlType::Float => "sqlTypeFloat",
                SqlType::Geometry => "sqlTypeGeometry",
                SqlType::Guid => "sqlTypeGUID",
                SqlType::Integer => "sqlTypeInteger",
                SqlType::LongNVarchar => "sqlTypeLongNVarchar",
                SqlType::LongVarbinary => "sqlTypeLongVarbinary",
                SqlType::LongVarchar => "sqlTypeLongVarchar",
                SqlType::NChar => "sqlTypeNChar",
                SqlType::NVarchar => "sqlTypeNVarchar",
                SqlType::Other => "sqlTypeOther",
                SqlType::Real => "sqlTypeReal",
                SqlType::SmallInt => "sqlTypeSmallInt",
                SqlType::SqlXml => "sqlTypeSqlXml",
                SqlType::Time => "sqlTypeTime",
                SqlType::Timestamp => "sqlTypeTimestamp",
                SqlType::Timestamp2 => "sqlTypeTimestamp2",
                SqlType::TinyInt => "sqlTypeTinyInt",
                SqlType::Varbinary => "sqlTypeVarbinary",
                SqlType::Varchar => "sqlTypeVarchar",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "sqlTypeBigInt" => Some(Self::BigInt),
                "sqlTypeBinary" => Some(Self::Binary),
                "sqlTypeBit" => Some(Self::Bit),
                "sqlTypeChar" => Some(Self::Char),
                "sqlTypeDate" => Some(Self::Date),
                "sqlTypeDecimal" => Some(Self::Decimal),
                "sqlTypeDouble" => Some(Self::Double),
                "sqlTypeFloat" => Some(Self::Float),
                "sqlTypeGeometry" => Some(Self::Geometry),
                "sqlTypeGUID" => Some(Self::Guid),
                "sqlTypeInteger" => Some(Self::Integer),
                "sqlTypeLongNVarchar" => Some(Self::LongNVarchar),
                "sqlTypeLongVarbinary" => Some(Self::LongVarbinary),
                "sqlTypeLongVarchar" => Some(Self::LongVarchar),
                "sqlTypeNChar" => Some(Self::NChar),
                "sqlTypeNVarchar" => Some(Self::NVarchar),
                "sqlTypeOther" => Some(Self::Other),
                "sqlTypeReal" => Some(Self::Real),
                "sqlTypeSmallInt" => Some(Self::SmallInt),
                "sqlTypeSqlXml" => Some(Self::SqlXml),
                "sqlTypeTime" => Some(Self::Time),
                "sqlTypeTimestamp" => Some(Self::Timestamp),
                "sqlTypeTimestamp2" => Some(Self::Timestamp2),
                "sqlTypeTinyInt" => Some(Self::TinyInt),
                "sqlTypeVarbinary" => Some(Self::Varbinary),
                "sqlTypeVarchar" => Some(Self::Varchar),
                _ => None,
            }
        }
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
    pub enum QuantizeOriginPostion {
        UpperLeft = 0,
        LowerLeft = 1,
    }
    impl QuantizeOriginPostion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                QuantizeOriginPostion::UpperLeft => "upperLeft",
                QuantizeOriginPostion::LowerLeft => "lowerLeft",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "upperLeft" => Some(Self::UpperLeft),
                "lowerLeft" => Some(Self::LowerLeft),
                _ => None,
            }
        }
    }
}
