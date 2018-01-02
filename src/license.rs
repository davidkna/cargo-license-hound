use std::{iter, slice};

use itertools;

const LICENSE_BASE_NAMES: &[&str] = &[
    "LICENSE", "COPYING", "LICENCE" // Typo seen in the wild
];

const LICENSE_EXTENSIONS: &[&str] = &["", ".txt"];

#[derive(Debug, Copy, Clone, Serialize)]
pub enum LicenseId {
    Apache2,
    Bsd2Clause,
    Bsd3Clause,
    Mit,
    Mpl2,
    ZlibAck,
}

impl LicenseId {
    pub fn suffixes(&self) -> &'static [&'static str] {
        use LicenseId::*;
        match self {
            &Mit => &["-MIT"],
            _ => &[],
        }
    }

    pub fn guess_filenames<'a>(
        &'a self,
    ) -> itertools::ConsTuples<
        itertools::Product<
            itertools::Product<
                slice::Iter<'a, &str>,
                iter::Chain<slice::Iter<'a, &str>, slice::Iter<'a, &str>>,
            >,
            slice::Iter<'a, &str>,
        >,
        ((&&str, &&str), &&str),
    > {
        iproduct!(
            LICENSE_BASE_NAMES.iter(),
            self.suffixes().iter().chain(&[""]),
            LICENSE_EXTENSIONS.iter()
        )
    }

    pub fn spdx_id(&self) -> &'static str {
        use LicenseId::*;
        match self {
            &Mit => "MIT",
            &Bsd3Clause => "BSD-3-Clause",
            &Mpl2 => "MPL-2.0",
            &Apache2 => "Apache-2.0",
            &Bsd2Clause => "BSD-2-Clause",
            &ZlibAck => "zlib-acknowledgement",
        }
    }
}

#[derive(Debug, Serialize)]
pub enum LicenseSource {
    Crate(String),
    GitHubApi { url: String },
    GitHubRepo { url: String },
}
