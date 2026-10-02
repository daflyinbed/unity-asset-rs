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

impl crate::ToValue for PhysicsJobOptions2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "useMultithreading".to_string(),
                self.use_multithreading.to_value(),
            )),
            Some((
                "useConsistencySorting".to_string(),
                self.use_consistency_sorting.to_value(),
            )),
            Some((
                "m_InterpolationPosesPerJob".to_string(),
                self.m_interpolation_poses_per_job.to_value(),
            )),
            Some((
                "m_NewContactsPerJob".to_string(),
                self.m_new_contacts_per_job.to_value(),
            )),
            Some((
                "m_CollideContactsPerJob".to_string(),
                self.m_collide_contacts_per_job.to_value(),
            )),
            Some((
                "m_ClearFlagsPerJob".to_string(),
                self.m_clear_flags_per_job.to_value(),
            )),
            Some((
                "m_ClearBodyForcesPerJob".to_string(),
                self.m_clear_body_forces_per_job.to_value(),
            )),
            Some((
                "m_SyncDiscreteFixturesPerJob".to_string(),
                self.m_sync_discrete_fixtures_per_job.to_value(),
            )),
            Some((
                "m_SyncContinuousFixturesPerJob".to_string(),
                self.m_sync_continuous_fixtures_per_job.to_value(),
            )),
            Some((
                "m_FindNearestContactsPerJob".to_string(),
                self.m_find_nearest_contacts_per_job.to_value(),
            )),
            Some((
                "m_UpdateTriggerContactsPerJob".to_string(),
                self.m_update_trigger_contacts_per_job.to_value(),
            )),
            Some((
                "m_IslandSolverCostThreshold".to_string(),
                self.m_island_solver_cost_threshold.to_value(),
            )),
            Some((
                "m_IslandSolverBodyCostScale".to_string(),
                self.m_island_solver_body_cost_scale.to_value(),
            )),
            Some((
                "m_IslandSolverContactCostScale".to_string(),
                self.m_island_solver_contact_cost_scale.to_value(),
            )),
            Some((
                "m_IslandSolverJointCostScale".to_string(),
                self.m_island_solver_joint_cost_scale.to_value(),
            )),
            Some((
                "m_IslandSolverBodiesPerJob".to_string(),
                self.m_island_solver_bodies_per_job.to_value(),
            )),
            Some((
                "m_IslandSolverContactsPerJob".to_string(),
                self.m_island_solver_contacts_per_job.to_value(),
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
                "useMultithreading" => (self.use_multithreading).inspect_value(path, offset, limit),
                "useConsistencySorting" => {
                    (self.use_consistency_sorting).inspect_value(path, offset, limit)
                }
                "m_InterpolationPosesPerJob" => {
                    (self.m_interpolation_poses_per_job).inspect_value(path, offset, limit)
                }
                "m_NewContactsPerJob" => {
                    (self.m_new_contacts_per_job).inspect_value(path, offset, limit)
                }
                "m_CollideContactsPerJob" => {
                    (self.m_collide_contacts_per_job).inspect_value(path, offset, limit)
                }
                "m_ClearFlagsPerJob" => {
                    (self.m_clear_flags_per_job).inspect_value(path, offset, limit)
                }
                "m_ClearBodyForcesPerJob" => {
                    (self.m_clear_body_forces_per_job).inspect_value(path, offset, limit)
                }
                "m_SyncDiscreteFixturesPerJob" => {
                    (self.m_sync_discrete_fixtures_per_job).inspect_value(path, offset, limit)
                }
                "m_SyncContinuousFixturesPerJob" => {
                    (self.m_sync_continuous_fixtures_per_job).inspect_value(path, offset, limit)
                }
                "m_FindNearestContactsPerJob" => {
                    (self.m_find_nearest_contacts_per_job).inspect_value(path, offset, limit)
                }
                "m_UpdateTriggerContactsPerJob" => {
                    (self.m_update_trigger_contacts_per_job).inspect_value(path, offset, limit)
                }
                "m_IslandSolverCostThreshold" => {
                    (self.m_island_solver_cost_threshold).inspect_value(path, offset, limit)
                }
                "m_IslandSolverBodyCostScale" => {
                    (self.m_island_solver_body_cost_scale).inspect_value(path, offset, limit)
                }
                "m_IslandSolverContactCostScale" => {
                    (self.m_island_solver_contact_cost_scale).inspect_value(path, offset, limit)
                }
                "m_IslandSolverJointCostScale" => {
                    (self.m_island_solver_joint_cost_scale).inspect_value(path, offset, limit)
                }
                "m_IslandSolverBodiesPerJob" => {
                    (self.m_island_solver_bodies_per_job).inspect_value(path, offset, limit)
                }
                "m_IslandSolverContactsPerJob" => {
                    (self.m_island_solver_contacts_per_job).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.use_multithreading).inspect_value(path, offset, limit))
                .map(|v| ("useMultithreading".to_string(), v)),
            ((self.use_consistency_sorting).inspect_value(path, offset, limit))
                .map(|v| ("useConsistencySorting".to_string(), v)),
            ((self.m_interpolation_poses_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_InterpolationPosesPerJob".to_string(), v)),
            ((self.m_new_contacts_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_NewContactsPerJob".to_string(), v)),
            ((self.m_collide_contacts_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_CollideContactsPerJob".to_string(), v)),
            ((self.m_clear_flags_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_ClearFlagsPerJob".to_string(), v)),
            ((self.m_clear_body_forces_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_ClearBodyForcesPerJob".to_string(), v)),
            ((self.m_sync_discrete_fixtures_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_SyncDiscreteFixturesPerJob".to_string(), v)),
            ((self.m_sync_continuous_fixtures_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_SyncContinuousFixturesPerJob".to_string(), v)),
            ((self.m_find_nearest_contacts_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_FindNearestContactsPerJob".to_string(), v)),
            ((self.m_update_trigger_contacts_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateTriggerContactsPerJob".to_string(), v)),
            ((self.m_island_solver_cost_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverCostThreshold".to_string(), v)),
            ((self.m_island_solver_body_cost_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverBodyCostScale".to_string(), v)),
            ((self.m_island_solver_contact_cost_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverContactCostScale".to_string(), v)),
            ((self.m_island_solver_joint_cost_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverJointCostScale".to_string(), v)),
            ((self.m_island_solver_bodies_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverBodiesPerJob".to_string(), v)),
            ((self.m_island_solver_contacts_per_job).inspect_value(path, offset, limit))
                .map(|v| ("m_IslandSolverContactsPerJob".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
