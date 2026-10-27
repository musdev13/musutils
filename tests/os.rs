#[cfg(feature = "os-v1")]
mod v1 {
    use musutils::os::{Arch, OS, get_arch, get_os};

    #[test]
    fn get_os_matches_target_cfg() {
        if cfg!(target_os = "linux") {
            assert!(matches!(get_os(), OS::Linux));
        } else if cfg!(target_os = "windows") {
            assert!(matches!(get_os(), OS::Windows));
        } else if cfg!(target_os = "macos") {
            assert!(matches!(get_os(), OS::MacOs));
        } else {
            assert!(matches!(get_os(), OS::Unknown));
        }
    }

    #[test]
    fn get_arch_matches_target_cfg() {
        if cfg!(target_arch = "x86_64") {
            assert!(matches!(get_arch(), Arch::X64));
        } else if cfg!(target_arch = "x86") {
            assert!(matches!(get_arch(), Arch::X86));
        } else if cfg!(target_arch = "aarch64") {
            assert!(matches!(get_arch(), Arch::Arm64));
        } else if cfg!(target_arch = "arm") {
            assert!(matches!(get_arch(), Arch::Arm));
        } else {
            assert!(matches!(get_arch(), Arch::Unknown));
        }
    }

    #[test]
    fn get_os_deterministic() {
        let a = get_os();
        let b = get_os();
        assert!(matches!(
            (&a, &b),
            (OS::Linux, OS::Linux)
                | (OS::Windows, OS::Windows)
                | (OS::MacOs, OS::MacOs)
                | (OS::Unknown, OS::Unknown)
        ));
    }
}

#[cfg(feature = "os-v2")]
mod v2 {
    use musutils::os::{Arch, OS, get_arch, get_os};

    const ALL_OS: &[OS] = &[OS::Windows, OS::Linux, OS::MacOs, OS::Unknown];
    const ALL_ARCH: &[Arch] = &[Arch::X86, Arch::X64, Arch::Arm, Arch::Arm64, Arch::Unknown];

    #[test]
    fn os_as_str_values() {
        assert_eq!(OS::Windows.as_str(), "windows");
        assert_eq!(OS::Linux.as_str(), "linux");
        assert_eq!(OS::MacOs.as_str(), "macos");
        assert_eq!(OS::Unknown.as_str(), "unknown");
    }

    #[test]
    fn arch_as_str_values() {
        assert_eq!(Arch::X86.as_str(), "x86");
        assert_eq!(Arch::X64.as_str(), "x86_64");
        assert_eq!(Arch::Arm.as_str(), "arm");
        assert_eq!(Arch::Arm64.as_str(), "aarch64");
        assert_eq!(Arch::Unknown.as_str(), "unknown");
    }

    #[test]
    fn os_display_matches_as_str() {
        for &o in ALL_OS {
            assert_eq!(o.to_string(), o.as_str());
        }
    }

    #[test]
    fn arch_display_matches_as_str() {
        for &a in ALL_ARCH {
            assert_eq!(a.to_string(), a.as_str());
        }
    }

    #[test]
    fn os_and_arch_unique_strs() {
        let mut s: Vec<&'static str> = Vec::new();
        for &o in ALL_OS {
            assert!(!s.contains(&o.as_str()));
            s.push(o.as_str());
        }
        let mut s: Vec<&'static str> = Vec::new();
        for &a in ALL_ARCH {
            assert!(!s.contains(&a.as_str()));
            s.push(a.as_str());
        }
    }

    #[test]
    fn os_and_arch_eq_hash() {
        use std::collections::HashSet;
        let mut set: HashSet<OS> = HashSet::new();
        for &o in ALL_OS {
            set.insert(o);
            set.insert(o);
        }
        assert_eq!(set.len(), ALL_OS.len());
        let mut set: HashSet<Arch> = HashSet::new();
        for &a in ALL_ARCH {
            set.insert(a);
            set.insert(a);
        }
        assert_eq!(set.len(), ALL_ARCH.len());
    }

    #[test]
    fn get_os_matches_target_cfg() {
        if cfg!(target_os = "linux") {
            assert_eq!(get_os(), OS::Linux);
        } else if cfg!(target_os = "windows") {
            assert_eq!(get_os(), OS::Windows);
        } else if cfg!(target_os = "macos") {
            assert_eq!(get_os(), OS::MacOs);
        } else {
            assert_eq!(get_os(), OS::Unknown);
        }
    }

    #[test]
    fn get_arch_matches_target_cfg() {
        if cfg!(target_arch = "x86_64") {
            assert_eq!(get_arch(), Arch::X64);
        } else if cfg!(target_arch = "x86") {
            assert_eq!(get_arch(), Arch::X86);
        } else if cfg!(target_arch = "aarch64") {
            assert_eq!(get_arch(), Arch::Arm64);
        } else if cfg!(target_arch = "arm") {
            assert_eq!(get_arch(), Arch::Arm);
        } else {
            assert_eq!(get_arch(), Arch::Unknown);
        }
    }
}
