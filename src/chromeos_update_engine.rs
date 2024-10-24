// This file is @generated by prost-build.
/// Data is packed into blocks on disk, always starting from the beginning
/// of the block. If a file's data is too large for one block, it overflows
/// into another block, which may or may not be the following block on the
/// physical partition. An ordered list of extents is another
/// representation of an ordered list of blocks. For example, a file stored
/// in blocks 9, 10, 11, 2, 18, 12 (in that order) would be stored in
/// extents { {9, 3}, {2, 1}, {18, 1}, {12, 1} } (in that order).
/// In general, files are stored sequentially on disk, so it's more efficient
/// to use extents to encode the block lists (this is effectively
/// run-length encoding).
/// A sentinel value (kuint64max) as the start block denotes a sparse-hole
/// in a file whose block-length is specified by num_blocks.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Extent {
    #[prost(uint64, optional, tag = "1")]
    pub start_block: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub num_blocks: ::core::option::Option<u64>,
}
/// Signatures: Updates may be signed by the OS vendor. The client verifies
/// an update's signature by hashing the entire download. The section of the
/// download that contains the signature is at the end of the file, so when
/// signing a file, only the part up to the signature part is signed.
/// Then, the client looks inside the download's Signatures message for a
/// Signature message that it knows how to handle. Generally, a client will
/// only know how to handle one type of signature, but an update may contain
/// many signatures to support many different types of client. Then client
/// selects a Signature message and uses that, along with a known public key,
/// to verify the download. The public key is expected to be part of the
/// client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signatures {
    #[prost(message, repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<signatures::Signature>,
}
/// Nested message and enum types in `Signatures`.
pub mod signatures {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Signature {
        #[deprecated]
        #[prost(uint32, optional, tag = "1")]
        pub version: ::core::option::Option<u32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        /// The DER encoded signature size of EC keys is nondeterministic for
        /// different input of sha256 hash. However, we need the size of the
        /// serialized signatures protobuf string to be fixed before signing;
        /// because this size is part of the content to be signed. Therefore, we
        /// always pad the signature data to the maximum possible signature size of
        /// a given key. And the payload verifier will truncate the signature to
        /// its correct size based on the value of |unpadded_signature_size|.
        #[prost(fixed32, optional, tag = "3")]
        pub unpadded_signature_size: ::core::option::Option<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionInfo {
    #[prost(uint64, optional, tag = "1")]
    pub size: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallOperation {
    #[prost(enumeration = "install_operation::Type", required, tag = "1")]
    pub r#type: i32,
    /// Only minor version 6 or newer support 64 bits |data_offset| and
    /// |data_length|, older client will read them as uint32.
    /// The offset into the delta file (after the protobuf)
    /// where the data (if any) is stored
    #[prost(uint64, optional, tag = "2")]
    pub data_offset: ::core::option::Option<u64>,
    /// The length of the data in the delta file
    #[prost(uint64, optional, tag = "3")]
    pub data_length: ::core::option::Option<u64>,
    /// Ordered list of extents that are read from (if any) and written to.
    #[prost(message, repeated, tag = "4")]
    pub src_extents: ::prost::alloc::vec::Vec<Extent>,
    /// Byte length of src, equal to the number of blocks in src_extents *
    /// block_size. It is used for BSDIFF and SOURCE_BSDIFF, because we need to
    /// pass that external program the number of bytes to read from the blocks we
    /// pass it.  This is not used in any other operation.
    #[prost(uint64, optional, tag = "5")]
    pub src_length: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "6")]
    pub dst_extents: ::prost::alloc::vec::Vec<Extent>,
    /// Byte length of dst, equal to the number of blocks in dst_extents *
    /// block_size. Used for BSDIFF and SOURCE_BSDIFF, but not in any other
    /// operation.
    #[prost(uint64, optional, tag = "7")]
    pub dst_length: ::core::option::Option<u64>,
    /// Optional SHA 256 hash of the blob associated with this operation.
    /// This is used as a primary validation for http-based downloads and
    /// as a defense-in-depth validation for https-based downloads. If
    /// the operation doesn't refer to any blob, this field will have
    /// zero bytes.
    #[prost(bytes = "vec", optional, tag = "8")]
    pub data_sha256_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Indicates the SHA 256 hash of the source data referenced in src_extents at
    /// the time of applying the operation. If present, the update_engine daemon
    /// MUST read and verify the source data before applying the operation.
    #[prost(bytes = "vec", optional, tag = "9")]
    pub src_sha256_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `InstallOperation`.
pub mod install_operation {
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
        /// Replace destination extents w/ attached data.
        Replace = 0,
        /// Replace destination extents w/ attached bzipped data.
        ReplaceBz = 1,
        /// Move source extents to target extents.
        Move = 2,
        /// The data is a bsdiff binary diff.
        Bsdiff = 3,
        /// On minor version 2 or newer, these operations are supported:
        ///
        /// Copy from source to target partition
        SourceCopy = 4,
        /// Like BSDIFF, but read from source partition
        SourceBsdiff = 5,
        /// On minor version 3 or newer and on major version 2 or newer, these
        /// operations are supported:
        ///
        /// Replace destination extents w/ attached xz data.
        ReplaceXz = 8,
        /// On minor version 4 or newer, these operations are supported:
        ///
        /// Write zeros in the destination.
        Zero = 6,
        /// Discard the destination blocks, reading as undefined.
        Discard = 7,
        /// Like SOURCE_BSDIFF, but compressed with brotli.
        BrotliBsdiff = 10,
        /// On minor version 5 or newer, these operations are supported:
        ///
        /// The data is in puffdiff format.
        Puffdiff = 9,
        /// On minor version 8 or newer, these operations are supported:
        Zucchini = 11,
        /// On minor version 9 or newer, these operations are supported:
        Lz4diffBsdiff = 12,
        Lz4diffPuffdiff = 13,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Replace => "REPLACE",
                Self::ReplaceBz => "REPLACE_BZ",
                Self::Move => "MOVE",
                Self::Bsdiff => "BSDIFF",
                Self::SourceCopy => "SOURCE_COPY",
                Self::SourceBsdiff => "SOURCE_BSDIFF",
                Self::ReplaceXz => "REPLACE_XZ",
                Self::Zero => "ZERO",
                Self::Discard => "DISCARD",
                Self::BrotliBsdiff => "BROTLI_BSDIFF",
                Self::Puffdiff => "PUFFDIFF",
                Self::Zucchini => "ZUCCHINI",
                Self::Lz4diffBsdiff => "LZ4DIFF_BSDIFF",
                Self::Lz4diffPuffdiff => "LZ4DIFF_PUFFDIFF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REPLACE" => Some(Self::Replace),
                "REPLACE_BZ" => Some(Self::ReplaceBz),
                "MOVE" => Some(Self::Move),
                "BSDIFF" => Some(Self::Bsdiff),
                "SOURCE_COPY" => Some(Self::SourceCopy),
                "SOURCE_BSDIFF" => Some(Self::SourceBsdiff),
                "REPLACE_XZ" => Some(Self::ReplaceXz),
                "ZERO" => Some(Self::Zero),
                "DISCARD" => Some(Self::Discard),
                "BROTLI_BSDIFF" => Some(Self::BrotliBsdiff),
                "PUFFDIFF" => Some(Self::Puffdiff),
                "ZUCCHINI" => Some(Self::Zucchini),
                "LZ4DIFF_BSDIFF" => Some(Self::Lz4diffBsdiff),
                "LZ4DIFF_PUFFDIFF" => Some(Self::Lz4diffPuffdiff),
                _ => None,
            }
        }
    }
}
/// Hints to VAB snapshot to skip writing some blocks if these blocks are
/// identical to the ones on the source image. The src & dst extents for each
/// CowMergeOperation should be contiguous, and they're a subset of an OTA
/// InstallOperation.
/// During merge time, we need to follow the pre-computed sequence to avoid
/// read after write, similar to the inplace update schema.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CowMergeOperation {
    #[prost(enumeration = "cow_merge_operation::Type", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub src_extent: ::core::option::Option<Extent>,
    #[prost(message, optional, tag = "3")]
    pub dst_extent: ::core::option::Option<Extent>,
    /// For COW_XOR, source location might be unaligned, so this field is in range
    /// [0, block_size), representing how much should the src_extent shift toward
    /// larger block number. If this field is non-zero, then src_extent will
    /// include 1 extra block in the end, as the merge op actually references the
    /// first |src_offset| bytes of that extra block. For example, if |dst_extent|
    /// is \[10, 15\], |src_offset| is 500, then src_extent might look like \[25, 31\].
    /// Note that |src_extent| contains 1 extra block than the |dst_extent|.
    #[prost(uint32, optional, tag = "4")]
    pub src_offset: ::core::option::Option<u32>,
}
/// Nested message and enum types in `CowMergeOperation`.
pub mod cow_merge_operation {
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
        /// identical blocks
        CowCopy = 0,
        /// used when src/dst blocks are highly similar
        CowXor = 1,
        /// Raw replace operation
        CowReplace = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::CowCopy => "COW_COPY",
                Self::CowXor => "COW_XOR",
                Self::CowReplace => "COW_REPLACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COW_COPY" => Some(Self::CowCopy),
                "COW_XOR" => Some(Self::CowXor),
                "COW_REPLACE" => Some(Self::CowReplace),
                _ => None,
            }
        }
    }
}
/// Describes the update to apply to a single partition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionUpdate {
    /// A platform-specific name to identify the partition set being updated. For
    /// example, in Chrome OS this could be "ROOT" or "KERNEL".
    #[prost(string, required, tag = "1")]
    pub partition_name: ::prost::alloc::string::String,
    /// Whether this partition carries a filesystem with post-install program that
    /// must be run to finalize the update process. See also |postinstall_path| and
    /// |filesystem_type|.
    #[prost(bool, optional, tag = "2")]
    pub run_postinstall: ::core::option::Option<bool>,
    /// The path of the executable program to run during the post-install step,
    /// relative to the root of this filesystem. If not set, the default "postinst"
    /// will be used. This setting is only used when |run_postinstall| is set and
    /// true.
    #[prost(string, optional, tag = "3")]
    pub postinstall_path: ::core::option::Option<::prost::alloc::string::String>,
    /// The filesystem type as passed to the mount(2) syscall when mounting the new
    /// filesystem to run the post-install program. If not set, a fixed list of
    /// filesystems will be attempted. This setting is only used if
    /// |run_postinstall| is set and true.
    #[prost(string, optional, tag = "4")]
    pub filesystem_type: ::core::option::Option<::prost::alloc::string::String>,
    /// If present, a list of signatures of the new_partition_info.hash signed with
    /// different keys. If the update_engine daemon requires vendor-signed images
    /// and has its public key installed, one of the signatures should be valid
    /// for /postinstall to run.
    #[prost(message, repeated, tag = "5")]
    pub new_partition_signature: ::prost::alloc::vec::Vec<signatures::Signature>,
    #[prost(message, optional, tag = "6")]
    pub old_partition_info: ::core::option::Option<PartitionInfo>,
    #[prost(message, optional, tag = "7")]
    pub new_partition_info: ::core::option::Option<PartitionInfo>,
    /// The list of operations to be performed to apply this PartitionUpdate. The
    /// associated operation blobs (in operations\[i\].data_offset, data_length)
    /// should be stored contiguously and in the same order.
    #[prost(message, repeated, tag = "8")]
    pub operations: ::prost::alloc::vec::Vec<InstallOperation>,
    /// Whether a failure in the postinstall step for this partition should be
    /// ignored.
    #[prost(bool, optional, tag = "9")]
    pub postinstall_optional: ::core::option::Option<bool>,
    /// On minor version 6 or newer, these fields are supported:
    /// The extent for data covered by verity hash tree.
    #[prost(message, optional, tag = "10")]
    pub hash_tree_data_extent: ::core::option::Option<Extent>,
    /// The extent to store verity hash tree.
    #[prost(message, optional, tag = "11")]
    pub hash_tree_extent: ::core::option::Option<Extent>,
    /// The hash algorithm used in verity hash tree.
    #[prost(string, optional, tag = "12")]
    pub hash_tree_algorithm: ::core::option::Option<::prost::alloc::string::String>,
    /// The salt used for verity hash tree.
    #[prost(bytes = "vec", optional, tag = "13")]
    pub hash_tree_salt: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// The extent for data covered by FEC.
    #[prost(message, optional, tag = "14")]
    pub fec_data_extent: ::core::option::Option<Extent>,
    /// The extent to store FEC.
    #[prost(message, optional, tag = "15")]
    pub fec_extent: ::core::option::Option<Extent>,
    /// The number of FEC roots.
    #[prost(uint32, optional, tag = "16", default = "2")]
    pub fec_roots: ::core::option::Option<u32>,
    /// Per-partition version used for downgrade detection, added
    /// as an effort to support partial updates. For most partitions,
    /// this is the build timestamp.
    #[prost(string, optional, tag = "17")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// A sorted list of CowMergeOperation. When writing cow, we can choose to
    /// skip writing the raw bytes for these extents. During snapshot merge, the
    /// bytes will read from the source partitions instead.
    #[prost(message, repeated, tag = "18")]
    pub merge_operations: ::prost::alloc::vec::Vec<CowMergeOperation>,
    /// Estimated size for COW image. This is used by libsnapshot
    /// as a hint. If set to 0, libsnapshot should use alternative
    /// methods for estimating size.
    #[prost(uint64, optional, tag = "19")]
    pub estimate_cow_size: ::core::option::Option<u64>,
    /// Information about the cow used by Cow Writer to specify
    /// number of cow operations to be written
    #[prost(uint64, optional, tag = "20")]
    pub estimate_op_count_max: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicPartitionGroup {
    /// Name of the group.
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Maximum size of the group. The sum of sizes of all partitions in the group
    /// must not exceed the maximum size of the group.
    #[prost(uint64, optional, tag = "2")]
    pub size: ::core::option::Option<u64>,
    /// A list of partitions that belong to the group.
    #[prost(string, repeated, tag = "3")]
    pub partition_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VabcFeatureSet {
    #[prost(bool, optional, tag = "1")]
    pub threaded: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub batch_writes: ::core::option::Option<bool>,
}
/// Metadata related to all dynamic partitions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicPartitionMetadata {
    /// All updatable groups present in |partitions| of this DeltaArchiveManifest.
    /// - If an updatable group is on the device but not in the manifest, it is
    ///    not updated. Hence, the group will not be resized, and partitions cannot
    ///    be added to or removed from the group.
    /// - If an updatable group is in the manifest but not on the device, the group
    ///    is added to the device.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<DynamicPartitionGroup>,
    /// Whether dynamic partitions have snapshots during the update. If this is
    /// set to true, the update_engine daemon creates snapshots for all dynamic
    /// partitions if possible. If this is unset, the update_engine daemon MUST
    /// NOT create snapshots for dynamic partitions.
    #[prost(bool, optional, tag = "2")]
    pub snapshot_enabled: ::core::option::Option<bool>,
    /// If this is set to false, update_engine should not use VABC regardless. If
    /// this is set to true, update_engine may choose to use VABC if device
    /// supports it, but not guaranteed.
    /// VABC stands for Virtual AB Compression
    #[prost(bool, optional, tag = "3")]
    pub vabc_enabled: ::core::option::Option<bool>,
    /// The compression algorithm used by VABC. Available ones are "gz", "brotli".
    /// See system/core/fs_mgr/libsnapshot/cow_writer.cpp for available options,
    /// as this parameter is ultimated forwarded to libsnapshot's CowWriter
    #[prost(string, optional, tag = "4")]
    pub vabc_compression_param: ::core::option::Option<::prost::alloc::string::String>,
    /// COW version used by VABC. The represents the major version in the COW
    /// header
    #[prost(uint32, optional, tag = "5")]
    pub cow_version: ::core::option::Option<u32>,
    /// A collection of knobs to tune Virtual AB Compression
    #[prost(message, optional, tag = "6")]
    pub vabc_feature_set: ::core::option::Option<VabcFeatureSet>,
    /// Max bytes to be compressed at once during ota. Options: 4k, 8k, 16k, 32k,
    /// 64k, 128k
    #[prost(uint64, optional, tag = "7")]
    pub compression_factor: ::core::option::Option<u64>,
}
/// Definition has been duplicated from
/// $ANDROID_BUILD_TOP/build/tools/releasetools/ota_metadata.proto. Keep in sync.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApexInfo {
    #[prost(string, optional, tag = "1")]
    pub package_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub version: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "3")]
    pub is_compressed: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "4")]
    pub decompressed_size: ::core::option::Option<i64>,
}
/// Definition has been duplicated from
/// $ANDROID_BUILD_TOP/build/tools/releasetools/ota_metadata.proto. Keep in sync.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApexMetadata {
    #[prost(message, repeated, tag = "1")]
    pub apex_info: ::prost::alloc::vec::Vec<ApexInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaArchiveManifest {
    /// (At time of writing) usually 4096
    #[prost(uint32, optional, tag = "3", default = "4096")]
    pub block_size: ::core::option::Option<u32>,
    /// If signatures are present, the offset into the blobs, generally
    /// tacked onto the end of the file, and the length. We use an offset
    /// rather than a bool to allow for more flexibility in future file formats.
    /// If either is absent, it means signatures aren't supported in this
    /// file.
    #[prost(uint64, optional, tag = "4")]
    pub signatures_offset: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub signatures_size: ::core::option::Option<u64>,
    /// The minor version, also referred as "delta version", of the payload.
    /// Minor version 0 is full payload, everything else is delta payload.
    #[prost(uint32, optional, tag = "12", default = "0")]
    pub minor_version: ::core::option::Option<u32>,
    /// Only present in major version >= 2. List of partitions that will be
    /// updated, in the order they will be updated. This field replaces the
    /// |install_operations|, |kernel_install_operations| and the
    /// |{old,new}_{kernel,rootfs}_info| fields used in major version = 1. This
    /// array can have more than two partitions if needed, and they are identified
    /// by the partition name.
    #[prost(message, repeated, tag = "13")]
    pub partitions: ::prost::alloc::vec::Vec<PartitionUpdate>,
    /// The maximum timestamp of the OS allowed to apply this payload.
    /// Can be used to prevent downgrading the OS.
    #[prost(int64, optional, tag = "14")]
    pub max_timestamp: ::core::option::Option<i64>,
    /// Metadata related to all dynamic partitions.
    #[prost(message, optional, tag = "15")]
    pub dynamic_partition_metadata: ::core::option::Option<DynamicPartitionMetadata>,
    /// If the payload only updates a subset of partitions on the device.
    #[prost(bool, optional, tag = "16")]
    pub partial_update: ::core::option::Option<bool>,
    /// Information on compressed APEX to figure out how much space is required for
    /// their decompression
    #[prost(message, repeated, tag = "17")]
    pub apex_info: ::prost::alloc::vec::Vec<ApexInfo>,
    /// Security patch level of the device, usually in the format of
    /// yyyy-mm-dd
    #[prost(string, optional, tag = "18")]
    pub security_patch_level: ::core::option::Option<::prost::alloc::string::String>,
}
