use std::{
    error,
    ffi::{c_int, CStr},
    fmt,
    marker::PhantomData,
    ops,
    path::Path,
};

pub mod compaction_filter {
    use {crate::CompactionDecision, std::ffi::CStr};

    pub trait CompactionFilter {
        fn name(&self) -> &CStr;

        fn filter(&mut self, level: u32, key: &[u8], value: &[u8]) -> CompactionDecision;
    }
}

pub mod compaction_filter_factory {
    use {crate::compaction_filter::CompactionFilter, std::ffi::CStr};

    pub struct CompactionFilterContext;
    pub trait CompactionFilterFactory {
        type Filter: CompactionFilter;

        fn name(&self) -> &CStr;

        fn create(&mut self, context: CompactionFilterContext) -> Self::Filter;
    }
}

pub mod perf {
    pub enum PerfMetric {
        UserKeyComparisonCount,
        BlockCacheHitCount,
        BlockReadCount,
        BlockReadByte,
        BlockReadTime,
        BlockChecksumTime,
        BlockDecompressTime,
        GetReadBytes,
        MultigetReadBytes,
        IterReadBytes,
        InternalKeySkippedCount,
        InternalDeleteSkippedCount,
        InternalRecentSkippedCount,
        InternalMergeCount,
        GetSnapshotTime,
        GetFromMemtableTime,
        GetFromMemtableCount,
        GetPostProcessTime,
        GetFromOutputFilesTime,
        SeekOnMemtableTime,
        SeekOnMemtableCount,
        NextOnMemtableCount,
        PrevOnMemtableCount,
        SeekChildSeekTime,
        SeekChildSeekCount,
        SeekMinHeapTime,
        SeekMaxHeapTime,
        SeekInternalSeekTime,
        FindNextUserEntryTime,
        WriteWalTime,
        WriteMemtableTime,
        WriteDelayTime,
        WritePreAndPostProcessTime,
        DbMutexLockNanos,
        DbConditionWaitNanos,
        MergeOperatorTimeNanos,
        ReadIndexBlockNanos,
        ReadFilterBlockNanos,
        NewTableBlockIterNanos,
        NewTableIteratorNanos,
        BlockSeekNanos,
        FindTableNanos,
        BloomMemtableHitCount,
        BloomMemtableMissCount,
        BloomSstHitCount,
        BloomSstMissCount,
        KeyLockWaitTime,
        KeyLockWaitCount,
        EnvNewSequentialFileNanos,
        EnvNewRandomAccessFileNanos,
        EnvNewWritableFileNanos,
        EnvReuseWritableFileNanos,
        EnvNewRandomRwFileNanos,
        EnvNewDirectoryNanos,
        EnvFileExistsNanos,
        EnvGetChildrenNanos,
        EnvGetChildrenFileAttributesNanos,
        EnvDeleteFileNanos,
        EnvCreateDirNanos,
        EnvCreateDirIfMissingNanos,
        EnvDeleteDirNanos,
        EnvGetFileSizeNanos,
        EnvGetFileModificationTimeNanos,
        EnvRenameFileNanos,
        EnvLinkFileNanos,
        EnvLockFileNanos,
        EnvUnlockFileNanos,
        EnvNewLoggerNanos,
        TotalMetricCount,
    }
    pub enum PerfStatsLevel {
        EnableTime,
        Disable,
    }

    pub fn set_perf_stats(level: PerfStatsLevel) {
        unreachable!();
    }
}

pub mod properties {
    use std::ffi::CStr;

    pub const ACTUAL_DELAYED_WRITE_RATE: &CStr = c"actual_delayed_write_rate";
    pub const BACKGROUND_ERRORS: &CStr = c"background_errors";
    pub const BLOCK_CACHE_CAPACITY: &CStr = c"block_cache_capacity";
    pub const BLOCK_CACHE_PINNED_USAGE: &CStr = c"block_cache_pinned_usage";
    pub const COMPACTION_PENDING: &CStr = c"compaction_pending";
    pub const BLOCK_CACHE_USAGE: &CStr = c"block_cache_usage";
    pub const ESTIMATE_OLDEST_KEY_TIME: &CStr = c"estimate_oldest_key_time";
    pub const ESTIMATE_TABLE_READERS_MEM: &CStr = c"estimate_table_readers_mem";
    pub const ESTIMATE_READERS_LEN: &CStr = c"estimate_readers_len";
    pub const IS_WRITE_STOPPED: &CStr = c"is_write_stopped";
    pub const MEM_TABLE_FLUSH_PENDING: &CStr = c"mem_table_flush_pending";
    pub const NUM_RUNNING_COMPACTIONS: &CStr = c"num_running_compactions";
    pub const NUM_RUNNING_FLUSHES: &CStr = c"num_running_flushes";
    pub const NUM_SNAPSHOTS: &CStr = c"num_snapshots";
    pub const OLDEST_SNAPSHOT_TIME: &CStr = c"oldest_snapshot_time";
    pub const SIZE_ALL_MEM_TABLES: &CStr = c"size_all_mem_tables";
    pub const TOTAL_SST_FILES_SIZE: &CStr = c"total_sst_files_size";
}

pub struct ColumnFamily;
pub struct ColumnFamilyDescriptor;
pub enum CompactionDecision {
    Keep,
    Remove,
}
#[derive(Debug)]
pub struct DB;
pub enum DBCompressionType {
    None,
    Snappy,
    Lz4,
    Zlib,
}
pub struct DBIterator;
pub struct DBPinnableSlice<'a>(PhantomData<&'a ()>);
pub struct DBRawIterator;
pub enum DBRecoveryMode {
    AbsoluteConsistency,
    PointInTime,
    SkipAnyCorruptedRecord,
    TolerateCorruptedTailRecords,
}
pub enum Direction {
    Forward,
    Reverse,
}
pub struct Env;
#[derive(Debug)]
pub enum Error {}
pub enum IteratorMode<'a> {
    Start,
    From(&'a [u8], Direction),
    End,
}
pub struct LiveFile {
    pub column_family_name: String,
    pub num_entries: u64,
}
#[derive(Default)]
pub struct Options;
#[derive(Default)]
pub struct PerfContext;
#[derive(Default)]
pub struct WriteBatch;

impl ColumnFamilyDescriptor {
    pub fn new<N: Into<String>>(name: N, options: Options) -> Self {
        unreachable!();
    }

    pub fn name(&self) -> &str {
        unreachable!();
    }
}

impl DB {
    pub fn batched_multi_get_cf<I: IntoIterator<Item: AsRef<[u8]>>>(
        &self,
        cf: &ColumnFamily,
        keys: I,
        sorted_input: bool,
    ) -> Vec<Result<Option<DBPinnableSlice>, Error>> {
        unreachable!();
    }
    pub fn open_cf_descriptors<P: AsRef<Path>, I: IntoIterator<Item = ColumnFamilyDescriptor>>(
        opts: &Options,
        path: P,
        cfs: I,
    ) -> Result<Self, Error> {
        unreachable!();
    }
    pub fn open_cf_descriptors_as_secondary<
        P: AsRef<Path>,
        I: IntoIterator<Item = ColumnFamilyDescriptor>,
    >(
        opts: &Options,
        path: P,
        secondary_path: P,
        cfs: I,
    ) -> Result<Self, Error> {
        unreachable!();
    }
    pub fn list_cf<P: AsRef<Path>>(options: &Options, path: P) -> Result<Vec<String>, Error> {
        unreachable!();
    }
    pub fn set_options_cf(&self, cf: &ColumnFamily, opts: &[(&str, &str)]) -> Result<(), Error> {
        unreachable!();
    }
    pub fn destroy<P: AsRef<Path>>(opts: &Options, path: P) -> Result<(), Error> {
        unreachable!();
    }
    pub fn cf_handle(&self, name: &str) -> Result<&ColumnFamily, Error> {
        unreachable!();
    }
    pub fn get_cf<K: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
    ) -> Result<Option<Vec<u8>>, Error> {
        unreachable!();
    }
    pub fn get_pinned_cf<K: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
    ) -> Result<Option<DBPinnableSlice<'_>>, Error> {
        unreachable!();
    }
    pub fn put_cf<K: AsRef<[u8]>, V: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        key: K,
        value: V,
    ) -> Result<(), Error> {
        unreachable!();
    }
    pub fn delete_cf<K: AsRef<[u8]>>(&self, cf: &ColumnFamily, key: K) -> Result<(), Error> {
        unreachable!();
    }
    pub fn delete_file_in_range_cf<K: AsRef<[u8]>, L: AsRef<[u8]>>(
        &self,
        cf: &ColumnFamily,
        from: K,
        to: L,
    ) -> Result<(), Error> {
        unreachable!();
    }
    pub fn iterator_cf(&self, cf: &ColumnFamily, mode: IteratorMode<'_>) -> DBIterator {
        unreachable!();
    }
    pub fn raw_iterator_cf(&self, cf: &ColumnFamily) -> DBRawIterator {
        unreachable!();
    }
    pub fn write(&self, batch: WriteBatch) -> Result<(), Error> {
        unreachable!();
    }
    pub fn property_int_value_cf(
        &self,
        cf: &ColumnFamily,
        name: &CStr,
    ) -> Result<Option<c_int>, Error> {
        unreachable!();
    }
    pub fn live_files(&self) -> Result<Vec<LiveFile>, Error> {
        unreachable!();
    }
}

impl Iterator for DBIterator {
    type Item = Result<(Box<[u8]>, Box<[u8]>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unreachable!();
    }
}

impl ops::Deref for DBPinnableSlice<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unreachable!();
    }
}

impl DBRawIterator {
    pub fn key(&self) -> Option<&[u8]> {
        unreachable!();
    }
    pub fn valid(&self) -> bool {
        unreachable!();
    }
    pub fn value(&self) -> Option<&[u8]> {
        unreachable!();
    }

    pub fn next(&mut self) {
        unreachable!();
    }
    pub fn seek<K: AsRef<[u8]>>(&mut self, key: K) {
        unreachable!();
    }
}

impl Env {
    pub fn new() -> Result<Self, Error> {
        unreachable!();
    }

    pub fn set_high_priority_background_threads(&mut self, n: c_int) {
        unreachable!();
    }
    pub fn set_low_priority_background_threads(&mut self, n: c_int) {
        unreachable!();
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!();
    }
}

impl error::Error for Error {}

impl Options {
    pub fn set_max_write_buffer_number(&mut self, n: c_int) {
        unreachable!();
    }
    pub fn set_level_zero_file_num_compaction_trigger(&mut self, n: c_int) {
        unreachable!();
    }
    pub fn set_max_bytes_for_level_base(&mut self, size: u64) {
        unreachable!();
    }
    pub fn set_target_file_size_base(&mut self, size: u64) {
        unreachable!();
    }
    pub fn set_compaction_filter_factory<F: compaction_filter_factory::CompactionFilterFactory>(
        &mut self,
        factory: F,
    ) {
        unreachable!();
    }
    pub fn set_compression_type(&mut self, ty: DBCompressionType) {
        unreachable!();
    }
    pub fn create_if_missing(&mut self, create: bool) {
        unreachable!();
    }
    pub fn set_max_open_files(&mut self, n: c_int) {
        unreachable!();
    }
    pub fn set_max_background_jobs(&mut self, n: c_int) {
        unreachable!();
    }
    pub fn set_max_total_wal_size(&mut self, size: u64) {
        unreachable!();
    }
    pub fn set_env(&mut self, env: &Env) {
        unreachable!();
    }
    pub fn create_missing_column_families(&mut self, create: bool) {
        unreachable!();
    }
    pub fn set_disable_auto_compactions(&mut self, disable: bool) {
        unreachable!();
    }
    pub fn set_wal_recovery_mode(&mut self, mode: DBRecoveryMode) {
        unreachable!();
    }
    pub fn set_write_buffer_size(&mut self, size: usize) {
        unreachable!();
    }
}

impl PerfContext {
    pub fn metric(&self, metric: perf::PerfMetric) -> u64 {
        unreachable!();
    }

    pub fn reset(&mut self) {
        unreachable!();
    }
}

impl WriteBatch {
    pub fn delete_cf<K: AsRef<[u8]>>(&mut self, cf: &ColumnFamily, key: K) {
        unreachable!();
    }

    pub fn delete_range_cf<K: AsRef<[u8]>, L: AsRef<[u8]>>(
        &mut self,
        cf: &ColumnFamily,
        from: K,
        to: L,
    ) {
        unreachable!();
    }

    pub fn put_cf<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, cf: &ColumnFamily, key: K, value: V) {
        unreachable!();
    }
}
