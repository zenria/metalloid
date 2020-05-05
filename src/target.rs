#[derive(Clone, Copy, Debug)]
pub enum OsType {
    Linux,
    MacOs,
    Windows,
    FreeBSD,
}

pub struct Os {
    os_type: OsType,
    /// Ubuntu, Debian, Microsoft, Apple
    vendor: String,
    /// 18.04, 10.15.4 ...
    version: String,
}

impl From<(OsType, &str, &str)> for Os {
    fn from((os_type, vendor, version): (OsType, &str, &str)) -> Self {
        Self {
            os_type,
            vendor: vendor.into(),
            version: version.into(),
        }
    }
}

impl Os {
    pub fn os_type(&self) -> OsType {
        self.os_type
    }
    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

/// Target system properties like hostname, os type, version, vendor, network interfaces and custom data.
pub trait Target {
    fn hostname(&self) -> &str;

    fn os(&self) -> &Os;
}
