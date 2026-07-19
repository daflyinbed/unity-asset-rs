//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_package_requirement::SerializedPackageRequirement;

#[derive(Debug, Clone)]
pub struct SerializedPackageRequirements {
    pub m_requirements: Vec<SerializedPackageRequirement>,
    pub m_status_message: String,
    pub m_combined_status: i8,
}

impl crate::Parse for SerializedPackageRequirements {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_requirements = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| SerializedPackageRequirement::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_status_message = rdr.read_aligned_string()?;
        let m_combined_status = rdr.read_i8()?;

        Ok(SerializedPackageRequirements {
            m_requirements,
            m_status_message,
            m_combined_status,
        })
    }
}
