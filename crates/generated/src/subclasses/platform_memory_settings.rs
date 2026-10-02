//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformMemorySettings {
    pub m_main_allocator_block_size: i64,
    pub m_thread_allocator_block_size: i64,
    pub m_main_gfx_block_size: i64,
    pub m_thread_gfx_block_size: i64,
    pub m_cache_block_size: i64,
    pub m_typetree_block_size: i64,
    pub m_profiler_block_size: i64,
    pub m_profiler_editor_block_size: i64,
    pub m_bucket_allocator_granularity: i64,
    pub m_bucket_allocator_buckets_count: i64,
    pub m_bucket_allocator_block_size: i64,
    pub m_bucket_allocator_block_count: i64,
    pub m_profiler_bucket_allocator_granularity: i64,
    pub m_profiler_bucket_allocator_buckets_count: i64,
    pub m_profiler_bucket_allocator_block_size: i64,
    pub m_profiler_bucket_allocator_block_count: i64,
    pub m_temp_allocator_size_main: i64,
    pub m_job_temp_allocator_block_size: i64,
    pub m_background_job_temp_allocator_block_size: i64,
    pub m_job_temp_allocator_reduced_block_size: i64,
    pub m_temp_allocator_size_gi_baking_worker: i64,
    pub m_temp_allocator_size_nav_mesh_worker: i64,
    pub m_temp_allocator_size_audio_worker: i64,
    pub m_temp_allocator_size_cloud_worker: i64,
    pub m_temp_allocator_size_gfx: i64,
    pub m_temp_allocator_size_job_worker: i64,
    pub m_temp_allocator_size_background_worker: i64,
    pub m_temp_allocator_size_preload_manager: i64,
}

impl crate::Parse for PlatformMemorySettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_main_allocator_block_size = rdr.read_i64()?;
        let m_thread_allocator_block_size = rdr.read_i64()?;
        let m_main_gfx_block_size = rdr.read_i64()?;
        let m_thread_gfx_block_size = rdr.read_i64()?;
        let m_cache_block_size = rdr.read_i64()?;
        let m_typetree_block_size = rdr.read_i64()?;
        let m_profiler_block_size = rdr.read_i64()?;
        let m_profiler_editor_block_size = rdr.read_i64()?;
        let m_bucket_allocator_granularity = rdr.read_i64()?;
        let m_bucket_allocator_buckets_count = rdr.read_i64()?;
        let m_bucket_allocator_block_size = rdr.read_i64()?;
        let m_bucket_allocator_block_count = rdr.read_i64()?;
        let m_profiler_bucket_allocator_granularity = rdr.read_i64()?;
        let m_profiler_bucket_allocator_buckets_count = rdr.read_i64()?;
        let m_profiler_bucket_allocator_block_size = rdr.read_i64()?;
        let m_profiler_bucket_allocator_block_count = rdr.read_i64()?;
        let m_temp_allocator_size_main = rdr.read_i64()?;
        let m_job_temp_allocator_block_size = rdr.read_i64()?;
        let m_background_job_temp_allocator_block_size = rdr.read_i64()?;
        let m_job_temp_allocator_reduced_block_size = rdr.read_i64()?;
        let m_temp_allocator_size_gi_baking_worker = rdr.read_i64()?;
        let m_temp_allocator_size_nav_mesh_worker = rdr.read_i64()?;
        let m_temp_allocator_size_audio_worker = rdr.read_i64()?;
        let m_temp_allocator_size_cloud_worker = rdr.read_i64()?;
        let m_temp_allocator_size_gfx = rdr.read_i64()?;
        let m_temp_allocator_size_job_worker = rdr.read_i64()?;
        let m_temp_allocator_size_background_worker = rdr.read_i64()?;
        let m_temp_allocator_size_preload_manager = rdr.read_i64()?;

        Ok(PlatformMemorySettings {
            m_main_allocator_block_size,
            m_thread_allocator_block_size,
            m_main_gfx_block_size,
            m_thread_gfx_block_size,
            m_cache_block_size,
            m_typetree_block_size,
            m_profiler_block_size,
            m_profiler_editor_block_size,
            m_bucket_allocator_granularity,
            m_bucket_allocator_buckets_count,
            m_bucket_allocator_block_size,
            m_bucket_allocator_block_count,
            m_profiler_bucket_allocator_granularity,
            m_profiler_bucket_allocator_buckets_count,
            m_profiler_bucket_allocator_block_size,
            m_profiler_bucket_allocator_block_count,
            m_temp_allocator_size_main,
            m_job_temp_allocator_block_size,
            m_background_job_temp_allocator_block_size,
            m_job_temp_allocator_reduced_block_size,
            m_temp_allocator_size_gi_baking_worker,
            m_temp_allocator_size_nav_mesh_worker,
            m_temp_allocator_size_audio_worker,
            m_temp_allocator_size_cloud_worker,
            m_temp_allocator_size_gfx,
            m_temp_allocator_size_job_worker,
            m_temp_allocator_size_background_worker,
            m_temp_allocator_size_preload_manager,
        })
    }
}

impl crate::ToValue for PlatformMemorySettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_MainAllocatorBlockSize".to_string(),
                self.m_main_allocator_block_size.to_value(),
            )),
            Some((
                "m_ThreadAllocatorBlockSize".to_string(),
                self.m_thread_allocator_block_size.to_value(),
            )),
            Some((
                "m_MainGfxBlockSize".to_string(),
                self.m_main_gfx_block_size.to_value(),
            )),
            Some((
                "m_ThreadGfxBlockSize".to_string(),
                self.m_thread_gfx_block_size.to_value(),
            )),
            Some((
                "m_CacheBlockSize".to_string(),
                self.m_cache_block_size.to_value(),
            )),
            Some((
                "m_TypetreeBlockSize".to_string(),
                self.m_typetree_block_size.to_value(),
            )),
            Some((
                "m_ProfilerBlockSize".to_string(),
                self.m_profiler_block_size.to_value(),
            )),
            Some((
                "m_ProfilerEditorBlockSize".to_string(),
                self.m_profiler_editor_block_size.to_value(),
            )),
            Some((
                "m_BucketAllocatorGranularity".to_string(),
                self.m_bucket_allocator_granularity.to_value(),
            )),
            Some((
                "m_BucketAllocatorBucketsCount".to_string(),
                self.m_bucket_allocator_buckets_count.to_value(),
            )),
            Some((
                "m_BucketAllocatorBlockSize".to_string(),
                self.m_bucket_allocator_block_size.to_value(),
            )),
            Some((
                "m_BucketAllocatorBlockCount".to_string(),
                self.m_bucket_allocator_block_count.to_value(),
            )),
            Some((
                "m_ProfilerBucketAllocatorGranularity".to_string(),
                self.m_profiler_bucket_allocator_granularity.to_value(),
            )),
            Some((
                "m_ProfilerBucketAllocatorBucketsCount".to_string(),
                self.m_profiler_bucket_allocator_buckets_count.to_value(),
            )),
            Some((
                "m_ProfilerBucketAllocatorBlockSize".to_string(),
                self.m_profiler_bucket_allocator_block_size.to_value(),
            )),
            Some((
                "m_ProfilerBucketAllocatorBlockCount".to_string(),
                self.m_profiler_bucket_allocator_block_count.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeMain".to_string(),
                self.m_temp_allocator_size_main.to_value(),
            )),
            Some((
                "m_JobTempAllocatorBlockSize".to_string(),
                self.m_job_temp_allocator_block_size.to_value(),
            )),
            Some((
                "m_BackgroundJobTempAllocatorBlockSize".to_string(),
                self.m_background_job_temp_allocator_block_size.to_value(),
            )),
            Some((
                "m_JobTempAllocatorReducedBlockSize".to_string(),
                self.m_job_temp_allocator_reduced_block_size.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeGIBakingWorker".to_string(),
                self.m_temp_allocator_size_gi_baking_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeNavMeshWorker".to_string(),
                self.m_temp_allocator_size_nav_mesh_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeAudioWorker".to_string(),
                self.m_temp_allocator_size_audio_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeCloudWorker".to_string(),
                self.m_temp_allocator_size_cloud_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeGfx".to_string(),
                self.m_temp_allocator_size_gfx.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeJobWorker".to_string(),
                self.m_temp_allocator_size_job_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizeBackgroundWorker".to_string(),
                self.m_temp_allocator_size_background_worker.to_value(),
            )),
            Some((
                "m_TempAllocatorSizePreloadManager".to_string(),
                self.m_temp_allocator_size_preload_manager.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_MainAllocatorBlockSize" => {
                    (self.m_main_allocator_block_size).inspect_value(path, offset, limit)
                }
                "m_ThreadAllocatorBlockSize" => {
                    (self.m_thread_allocator_block_size).inspect_value(path, offset, limit)
                }
                "m_MainGfxBlockSize" => {
                    (self.m_main_gfx_block_size).inspect_value(path, offset, limit)
                }
                "m_ThreadGfxBlockSize" => {
                    (self.m_thread_gfx_block_size).inspect_value(path, offset, limit)
                }
                "m_CacheBlockSize" => (self.m_cache_block_size).inspect_value(path, offset, limit),
                "m_TypetreeBlockSize" => {
                    (self.m_typetree_block_size).inspect_value(path, offset, limit)
                }
                "m_ProfilerBlockSize" => {
                    (self.m_profiler_block_size).inspect_value(path, offset, limit)
                }
                "m_ProfilerEditorBlockSize" => {
                    (self.m_profiler_editor_block_size).inspect_value(path, offset, limit)
                }
                "m_BucketAllocatorGranularity" => {
                    (self.m_bucket_allocator_granularity).inspect_value(path, offset, limit)
                }
                "m_BucketAllocatorBucketsCount" => {
                    (self.m_bucket_allocator_buckets_count).inspect_value(path, offset, limit)
                }
                "m_BucketAllocatorBlockSize" => {
                    (self.m_bucket_allocator_block_size).inspect_value(path, offset, limit)
                }
                "m_BucketAllocatorBlockCount" => {
                    (self.m_bucket_allocator_block_count).inspect_value(path, offset, limit)
                }
                "m_ProfilerBucketAllocatorGranularity" => (self
                    .m_profiler_bucket_allocator_granularity)
                    .inspect_value(path, offset, limit),
                "m_ProfilerBucketAllocatorBucketsCount" => (self
                    .m_profiler_bucket_allocator_buckets_count)
                    .inspect_value(path, offset, limit),
                "m_ProfilerBucketAllocatorBlockSize" => {
                    (self.m_profiler_bucket_allocator_block_size).inspect_value(path, offset, limit)
                }
                "m_ProfilerBucketAllocatorBlockCount" => (self
                    .m_profiler_bucket_allocator_block_count)
                    .inspect_value(path, offset, limit),
                "m_TempAllocatorSizeMain" => {
                    (self.m_temp_allocator_size_main).inspect_value(path, offset, limit)
                }
                "m_JobTempAllocatorBlockSize" => {
                    (self.m_job_temp_allocator_block_size).inspect_value(path, offset, limit)
                }
                "m_BackgroundJobTempAllocatorBlockSize" => (self
                    .m_background_job_temp_allocator_block_size)
                    .inspect_value(path, offset, limit),
                "m_JobTempAllocatorReducedBlockSize" => (self
                    .m_job_temp_allocator_reduced_block_size)
                    .inspect_value(path, offset, limit),
                "m_TempAllocatorSizeGIBakingWorker" => {
                    (self.m_temp_allocator_size_gi_baking_worker).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeNavMeshWorker" => {
                    (self.m_temp_allocator_size_nav_mesh_worker).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeAudioWorker" => {
                    (self.m_temp_allocator_size_audio_worker).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeCloudWorker" => {
                    (self.m_temp_allocator_size_cloud_worker).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeGfx" => {
                    (self.m_temp_allocator_size_gfx).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeJobWorker" => {
                    (self.m_temp_allocator_size_job_worker).inspect_value(path, offset, limit)
                }
                "m_TempAllocatorSizeBackgroundWorker" => (self
                    .m_temp_allocator_size_background_worker)
                    .inspect_value(path, offset, limit),
                "m_TempAllocatorSizePreloadManager" => {
                    (self.m_temp_allocator_size_preload_manager).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_main_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MainAllocatorBlockSize".to_string(), v)),
            ((self.m_thread_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ThreadAllocatorBlockSize".to_string(), v)),
            ((self.m_main_gfx_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MainGfxBlockSize".to_string(), v)),
            ((self.m_thread_gfx_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ThreadGfxBlockSize".to_string(), v)),
            ((self.m_cache_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CacheBlockSize".to_string(), v)),
            ((self.m_typetree_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_TypetreeBlockSize".to_string(), v)),
            ((self.m_profiler_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerBlockSize".to_string(), v)),
            ((self.m_profiler_editor_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerEditorBlockSize".to_string(), v)),
            ((self.m_bucket_allocator_granularity).inspect_value(path, offset, limit))
                .map(|v| ("m_BucketAllocatorGranularity".to_string(), v)),
            ((self.m_bucket_allocator_buckets_count).inspect_value(path, offset, limit))
                .map(|v| ("m_BucketAllocatorBucketsCount".to_string(), v)),
            ((self.m_bucket_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_BucketAllocatorBlockSize".to_string(), v)),
            ((self.m_bucket_allocator_block_count).inspect_value(path, offset, limit))
                .map(|v| ("m_BucketAllocatorBlockCount".to_string(), v)),
            ((self.m_profiler_bucket_allocator_granularity).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerBucketAllocatorGranularity".to_string(), v)),
            ((self.m_profiler_bucket_allocator_buckets_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerBucketAllocatorBucketsCount".to_string(), v)),
            ((self.m_profiler_bucket_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerBucketAllocatorBlockSize".to_string(), v)),
            ((self.m_profiler_bucket_allocator_block_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ProfilerBucketAllocatorBlockCount".to_string(), v)),
            ((self.m_temp_allocator_size_main).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeMain".to_string(), v)),
            ((self.m_job_temp_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_JobTempAllocatorBlockSize".to_string(), v)),
            ((self.m_background_job_temp_allocator_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_BackgroundJobTempAllocatorBlockSize".to_string(), v)),
            ((self.m_job_temp_allocator_reduced_block_size).inspect_value(path, offset, limit))
                .map(|v| ("m_JobTempAllocatorReducedBlockSize".to_string(), v)),
            ((self.m_temp_allocator_size_gi_baking_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeGIBakingWorker".to_string(), v)),
            ((self.m_temp_allocator_size_nav_mesh_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeNavMeshWorker".to_string(), v)),
            ((self.m_temp_allocator_size_audio_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeAudioWorker".to_string(), v)),
            ((self.m_temp_allocator_size_cloud_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeCloudWorker".to_string(), v)),
            ((self.m_temp_allocator_size_gfx).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeGfx".to_string(), v)),
            ((self.m_temp_allocator_size_job_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeJobWorker".to_string(), v)),
            ((self.m_temp_allocator_size_background_worker).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizeBackgroundWorker".to_string(), v)),
            ((self.m_temp_allocator_size_preload_manager).inspect_value(path, offset, limit))
                .map(|v| ("m_TempAllocatorSizePreloadManager".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
