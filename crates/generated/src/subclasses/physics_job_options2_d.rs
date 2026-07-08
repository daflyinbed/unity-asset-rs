//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PhysicsJobOptions2D {
    pub use_multithreading: bool,
    pub use_consistency_sorting: bool,
    pub m_interpolation_poses_per_job: i32,
    pub m_new_contacts_per_job: i32,
    pub m_collide_contacts_per_job: i32,
    pub m_clear_flags_per_job: i32,
    pub m_clear_body_forces_per_job: i32,
    pub m_sync_discrete_fixtures_per_job: i32,
    pub m_sync_continuous_fixtures_per_job: i32,
    pub m_find_nearest_contacts_per_job: i32,
    pub m_update_trigger_contacts_per_job: i32,
    pub m_island_solver_cost_threshold: i32,
    pub m_island_solver_body_cost_scale: i32,
    pub m_island_solver_contact_cost_scale: i32,
    pub m_island_solver_joint_cost_scale: i32,
    pub m_island_solver_bodies_per_job: i32,
    pub m_island_solver_contacts_per_job: i32,
}

impl crate::Parse for PhysicsJobOptions2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let use_multithreading = rdr.read_bool()?;
        let use_consistency_sorting = rdr.read_bool()?;
        rdr.align(4)?;
        let m_interpolation_poses_per_job = rdr.read_i32()?;
        let m_new_contacts_per_job = rdr.read_i32()?;
        let m_collide_contacts_per_job = rdr.read_i32()?;
        let m_clear_flags_per_job = rdr.read_i32()?;
        let m_clear_body_forces_per_job = rdr.read_i32()?;
        let m_sync_discrete_fixtures_per_job = rdr.read_i32()?;
        let m_sync_continuous_fixtures_per_job = rdr.read_i32()?;
        let m_find_nearest_contacts_per_job = rdr.read_i32()?;
        let m_update_trigger_contacts_per_job = rdr.read_i32()?;
        let m_island_solver_cost_threshold = rdr.read_i32()?;
        let m_island_solver_body_cost_scale = rdr.read_i32()?;
        let m_island_solver_contact_cost_scale = rdr.read_i32()?;
        let m_island_solver_joint_cost_scale = rdr.read_i32()?;
        let m_island_solver_bodies_per_job = rdr.read_i32()?;
        let m_island_solver_contacts_per_job = rdr.read_i32()?;

        Ok(PhysicsJobOptions2D {
            use_multithreading,
            use_consistency_sorting,
            m_interpolation_poses_per_job,
            m_new_contacts_per_job,
            m_collide_contacts_per_job,
            m_clear_flags_per_job,
            m_clear_body_forces_per_job,
            m_sync_discrete_fixtures_per_job,
            m_sync_continuous_fixtures_per_job,
            m_find_nearest_contacts_per_job,
            m_update_trigger_contacts_per_job,
            m_island_solver_cost_threshold,
            m_island_solver_body_cost_scale,
            m_island_solver_contact_cost_scale,
            m_island_solver_joint_cost_scale,
            m_island_solver_bodies_per_job,
            m_island_solver_contacts_per_job,
        })
    }
}
