use anyhow::Result;
use package::Package;

pub mod package;

pub enum Strategy {
    Package(Package),
}

impl Strategy {
    pub fn package(package_id: &str, registry_id: &str) -> Result<Self> {
        Ok(Strategy::Package(Package::new(package_id, registry_id)?))
    }
}
