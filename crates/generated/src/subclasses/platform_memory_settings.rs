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
