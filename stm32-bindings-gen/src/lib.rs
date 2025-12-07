use bindgen::callbacks::{ItemInfo, ItemKind, ParseCallbacks};
use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

const STD_TO_CORE_REPLACEMENTS: &[(&str, &str)] = &[
    ("::std::mem::", "::core::mem::"),
    ("::std::os::raw::", "::core::ffi::"),
    ("::std::option::", "::core::option::"),
    ("::std::ptr::", "::core::ptr::"),
    (":: std :: mem ::", ":: core :: mem ::"),
    (":: std :: os :: raw ::", ":: core :: ffi ::"),
    (":: std :: option ::", ":: core :: option ::"),
    (":: std :: ptr ::", ":: core :: ptr ::"),
];

const NEWLIB_SHARED_OPAQUES: &[&str] = &["_reent", "__sFILE", "__sFILE64"];

#[derive(Debug, Clone, Copy)]
struct BindingSpec {
    module: &'static str,
    header: &'static str,
    include_dirs: &'static [&'static str],
    clang_args: &'static [&'static str],
    allowlist: &'static [&'static str],
    library_artifacts: &'static [LibraryArtifact],
}

#[derive(Debug, Clone, Copy)]
struct LibraryArtifact {
    source: &'static str,
    destination: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct CrateSpec {
    name: &'static str,
    template: &'static str,
    bindings: &'static [&'static [BindingSpec]],
}

const LINKLAYER_BINDINGS: &[BindingSpec] = &[BindingSpec {
    module: "wba_link_layer",
    header: "stm32-bindings-gen/inc/link_layer.h",
    include_dirs: &[
        "Middlewares/ST/STM32_WPAN",
        "Middlewares/ST/STM32_WPAN/mac_802_15_4/core/inc",
        "Middlewares/ST/STM32_WPAN/mac_802_15_4/mac_utilities/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_sys/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/_40nm_reg_files",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/ot_inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config/ieee_15_4_basic",
        "Drivers/CMSIS/Core/Include",
    ],
    clang_args: &[
        "-DSUPPORT_MAC=1",
        "-DSUPPORT_BLE=1",
        "-DMAC=1",
        "-DBLE=1",
        "-DBLE_LL=1",
        "-DMAC_LAYER=1",
        "-DSUPPORT_MAC=1",
        "-DSUPPORT_CONFIG_LIB=1",
        "-DSUPPORT_OPENTHREAD_1_2=1",
        "-DSUPPORT_ANT_DIV=1",
        "-DEXT_ADDRESS_LENGTH=8",
    ],
    allowlist: &[],
    library_artifacts: &[LibraryArtifact {
        source: "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/lib",
        destination: "src/lib/link_layer",
    }],
}];

const BLE_BINDINGS: &[BindingSpec] = &[BindingSpec {
    module: "wba_ble_stack",
    header: "stm32-bindings-gen/inc/wba_ble.h",
    include_dirs: &[
        "Middlewares/ST/STM32_WPAN",
        "Middlewares/ST/STM32_WPAN/ble/stack/include",
        "Middlewares/ST/STM32_WPAN/ble/stack/include/auto",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_sys/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/_40nm_reg_files",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/ot_inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config/ble_basic_plus",
        "Middlewares/ST/STM32_WPAN/ble/audio/Inc",
        "Middlewares/ST/STM32_WPAN/ble/codec/codec_manager/Inc",
        "Middlewares/ST/STM32_WPAN/ble/codec/lc3/Inc",
        "Drivers/CMSIS/Core/Include",
    ],
    clang_args: &[
        "-DBLE=1",
        "-DBLE_LL=1",
        "-DSUPPORT_BLE=1",
        "-DMAC=1",
        "-DMAC_LAYER=1",
        "-DSUPPORT_MAC=1",
        "-DSUPPORT_CONFIG_LIB=1",
        "-DSUPPORT_OPENTHREAD_1_2=1",
        "-DSUPPORT_ANT_DIV=1",
        "-DEXT_ADDRESS_LENGTH=8",
    ],
    allowlist: &[],
    library_artifacts: &[
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/ble/stack/lib",
            destination: "src/lib/ble/stack",
        },
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/ble/audio/lib",
            destination: "src/lib/ble/audio",
        },
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/ble/codec/codec_manager/Lib",
            destination: "src/lib/ble/codec_manager",
        },
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/ble/codec/lc3/Lib",
            destination: "src/lib/ble/lc3",
        },
    ],
}];

const MAC_BINDINGS: &[BindingSpec] = &[BindingSpec {
    module: "wba_wpan_mac",
    header: "stm32-bindings-gen/inc/wba_wpan_mac.h",
    include_dirs: &[
        "Middlewares/ST/STM32_WPAN",
        "Middlewares/ST/STM32_WPAN/mac_802_15_4/core/inc",
        "Middlewares/ST/STM32_WPAN/mac_802_15_4/mac_utilities/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_sys/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/_40nm_reg_files",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/inc/ot_inc",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config",
        "Middlewares/ST/STM32_WPAN/link_layer/ll_cmd_lib/config/ieee_15_4_basic",
        "Drivers/CMSIS/Core/Include",
    ],
    clang_args: &["-DSUPPORT_MAC=1", "-DMAC=1", "-DMAC_LAYER=1"],
    allowlist: &[],
    library_artifacts: &[
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/mac_802_15_4/lib",
            destination: "src/lib/mac",
        },
        LibraryArtifact {
            source: "Middlewares/ST/STM32_WPAN/mac_802_15_4/lib/wba_mac_lib.a",
            destination: "src/lib/wba_mac_lib.a",
        },
    ],
}];

const CRATE_SPECS: &[CrateSpec] = &[CrateSpec {
    name: "linklayer-bindings",
    template: "linklayer",
    bindings: &[LINKLAYER_BINDINGS, BLE_BINDINGS, MAC_BINDINGS],
}];

#[derive(Debug)]
struct UppercaseCallbacks;

impl ParseCallbacks for UppercaseCallbacks {
    fn item_name(&self, item: ItemInfo<'_>) -> Option<String> {
        if matches!(item.kind, ItemKind::Var) {
            Some(item.name.to_ascii_uppercase())
        } else {
            None
        }
    }
}

pub struct Options {
    pub out_dir: PathBuf,
    pub sources_dir: PathBuf,
    pub target_triple: String,
}

pub struct Gen {
    opts: Options,
}

impl Gen {
    pub fn new(opts: Options) -> Self {
        Self { opts }
    }

    pub fn run_gen(&mut self) {
        let base_out_dir = self.compute_base_out_dir();
        println!(
            "Generating bindings into {} for target {}",
            base_out_dir.display(),
            self.opts.target_triple
        );

        // Remove the legacy single-crate output if it still exists.
        let _ = fs::remove_dir_all(&self.opts.out_dir);

        for crate_spec in CRATE_SPECS {
            println!("  -> emitting `{}` crate", crate_spec.name);
            let crate_out_dir = base_out_dir.join(crate_spec.name);
            self.prepare_crate_out_dir(crate_spec, &crate_out_dir);

            let mut modules = Vec::new();
            for binding_group in crate_spec.bindings {
                for binding in *binding_group {
                    println!("     - generating `{}` bindings", binding.module);
                    self.generate_bindings_for_spec(binding, &crate_out_dir);
                    self.copy_artifacts_for_spec(binding, &crate_out_dir);
                    modules.push(binding.module);
                }
            }

            self.write_bindings_mod(&crate_out_dir, &modules);
        }
    }

    fn compute_base_out_dir(&self) -> PathBuf {
        let out_dir = &self.opts.out_dir;
        if out_dir
            .file_name()
            .map(|name| name == "stm32-bindings")
            .unwrap_or(false)
        {
            out_dir
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| out_dir.clone())
        } else {
            out_dir.clone()
        }
    }

    fn prepare_crate_out_dir(&self, spec: &CrateSpec, dest: &Path) {
        if let Some(parent) = dest.parent() {
            self.create_dir(parent);
        }

        let template_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("res")
            .join(spec.template);

        if !template_root.exists() {
            panic!(
                "Template directory `{}` is missing for crate `{}`",
                template_root.display(),
                spec.name
            );
        }

        let _ = fs::remove_dir_all(dest);
        self.copy_dir_recursive(&template_root, dest)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to copy template `{}` into `{}`: {err}",
                    template_root.display(),
                    dest.display()
                )
            });

        self.create_dir(dest.join("src/bindings"));
        self.create_dir(dest.join("src/lib"));
    }

    fn generate_bindings_for_spec(&self, spec: &BindingSpec, crate_dir: &Path) {
        let mut builder = bindgen::Builder::default()
            .layout_tests(false)
            .parse_callbacks(Box::new(UppercaseCallbacks))
            .header(spec.header)
            .clang_arg(format!("--target={}", self.opts.target_triple));

        for arg in host_isystem_args() {
            builder = builder.clang_arg(arg);
        }

        let crate_inc = Path::new(env!("CARGO_MANIFEST_DIR")).join("inc");
        builder = builder.clang_arg(format!("-iquote{}", crate_inc.display()));
        builder = builder.clang_arg(format!("-I{}", crate_inc.display()));

        if Self::is_thumb_target(&self.opts.target_triple) {
            builder = builder.clang_arg("-mthumb");
        }

        for dir in spec.include_dirs {
            let include_path = Path::new(dir);
            let resolved = if include_path.is_absolute() {
                include_path.to_path_buf()
            } else {
                self.opts.sources_dir.join(include_path)
            };
            builder = builder.clang_arg(format!("-I{}", resolved.display()));
        }

        for arg in spec.clang_args {
            builder = builder.clang_arg(*arg);
        }

        for ty in NEWLIB_SHARED_OPAQUES {
            builder = builder.opaque_type(ty);
        }

        for arg in arm_sysroot_args() {
            builder = builder.clang_arg(arg);
        }

        if !spec.allowlist.is_empty() {
            for pattern in spec.allowlist {
                builder = builder
                    .allowlist_type(pattern)
                    .allowlist_var(pattern)
                    .allowlist_function(pattern);
            }
        }

        let bindings = builder.generate().unwrap_or_else(|err| {
            panic!(
                "Unable to generate bindings for module `{}`: {err}",
                spec.module
            )
        });

        let mut contents = bindings.to_string();
        contents = Self::normalize_bindings(contents);

        let out_path = crate_dir
            .join("src/bindings")
            .join(format!("{}.rs", spec.module));
        self.write_string_path(&out_path, contents);
    }

    fn copy_artifacts_for_spec(&self, spec: &BindingSpec, crate_dir: &Path) {
        for artifact in spec.library_artifacts {
            let src = self.opts.sources_dir.join(artifact.source);
            let dst = crate_dir.join(artifact.destination);

            if src.is_file() {
                self.copy_lib(&src, &dst)
                    .unwrap_or_else(|err| panic!("Failed to copy file {}: {err}", src.display()));
            } else if src.is_dir() {
                self.copy_lib_dir(&src, &dst)
                    .unwrap_or_else(|err| panic!("Failed to copy dir {}: {err}", src.display()));
            } else {
                panic!(
                    "Artifact source {} is neither file nor directory",
                    src.display()
                );
            }
        }
    }

    fn write_bindings_mod(&self, crate_dir: &Path, modules: &[&str]) {
        let mut body = String::new();
        for module in modules {
            body.push_str("pub mod ");
            body.push_str(module);
            body.push_str(";\n");
        }

        let mod_path = crate_dir.join("src/bindings/mod.rs");
        self.write_string_path(&mod_path, body);
    }

    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> io::Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let target = dst.join(entry.file_name());

            if path.is_dir() {
                self.copy_dir_recursive(&path, &target)?;
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &target)?;
            }
        }

        Ok(())
    }

    fn write_string_path(&self, path: &Path, mut contents: String) {
        if !contents.ends_with('\n') {
            contents.push('\n');
        }
        if let Some(parent) = path.parent() {
            self.create_dir(parent);
        }
        fs::write(path, contents).expect("Unable to write string");
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            fs::create_dir_all(path_ref).expect("Unable to create directory");
        }
    }

    fn copy_lib(&self, src: &Path, dst: &Path) -> io::Result<()> {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }

        let file_name = "lib".to_string()
            + dst
                .file_name()
                .ok_or(io::Error::new(io::ErrorKind::InvalidFilename, ""))?
                .to_str()
                .ok_or(io::Error::new(io::ErrorKind::InvalidFilename, ""))?;

        let dst = dst
            .parent()
            .unwrap_or(&Path::new(""))
            .join(file_name.to_ascii_lowercase());

        fs::copy(src, dst)?;
        Ok(())
    }

    fn copy_lib_dir(&self, src: &Path, dst: &Path) -> io::Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let target = dst.join(entry.file_name());
            if path.is_dir() {
                self.copy_lib_dir(&path, &target)?;
            } else {
                self.copy_lib(&path, &target)?;
            }
        }
        Ok(())
    }

    fn normalize_bindings(mut contents: String) -> String {
        for (from, to) in STD_TO_CORE_REPLACEMENTS {
            contents = contents.replace(from, to);
        }

        contents
            .lines()
            .map(|line| {
                if let Some(rest) = line.strip_prefix("pub const ") {
                    if let Some((name, tail)) = rest.split_once(':') {
                        let upper = name.trim().to_ascii_uppercase();
                        return format!("pub const {}:{}", upper, tail);
                    }
                }
                line.to_owned()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn is_thumb_target(triple: &str) -> bool {
        triple.trim().to_ascii_lowercase().starts_with("thumb")
    }
}

fn host_isystem_args() -> Vec<String> {
    let mut args = Vec::new();
    if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("xcrun").arg("--show-sdk-path").output() {
            if output.status.success() {
                if let Ok(path) = String::from_utf8(output.stdout) {
                    let trimmed = path.trim();
                    if !trimmed.is_empty() {
                        args.push(format!("-isystem{}/usr/include", trimmed));
                    }
                }
            }
        }
    }
    args
}

fn arm_sysroot_args() -> Vec<String> {
    let mut args = Vec::new();
    let mut system_include_paths = BTreeSet::new();

    let mut push_sysroot = |path: &Path| {
        system_include_paths.insert(path.join("include"));
        system_include_paths.insert(path.join("include-fixed"));
        system_include_paths.insert(path.join("usr/include"));
        system_include_paths.insert(path.join("usr/include/newlib"));
        system_include_paths.insert(path.join("arm-none-eabi/include"));

        let arg = format!("--sysroot={}", path.display());
        if !args.iter().any(|existing| existing == &arg) {
            args.push(arg);
        }
    };

    if let Some(sysroot_os) = env::var_os("ARM_NONE_EABI_SYSROOT") {
        let sysroot_path = PathBuf::from(&sysroot_os);
        if sysroot_path.exists() {
            push_sysroot(sysroot_path.as_path());
        }
    }

    if let Some(sysroot) = gcc_query(&["-print-sysroot"]) {
        let sysroot = sysroot.trim();
        if !sysroot.is_empty() {
            push_sysroot(Path::new(sysroot));
        }
    }

    if let Some(include_dir) = gcc_query(&["-print-file-name=include"]) {
        let include_dir = include_dir.trim();
        if !include_dir.is_empty() && include_dir != "include" {
            system_include_paths.insert(PathBuf::from(include_dir));
        }
    }

    if let Some(libgcc) = gcc_query(&["-print-libgcc-file-name"]) {
        let libgcc_path = Path::new(libgcc.trim());
        if let Some(version_dir) = libgcc_path.parent() {
            system_include_paths.insert(version_dir.join("include"));
            system_include_paths.insert(version_dir.join("include-fixed"));

            if let Some(toolchain_root) = version_dir.parent() {
                if let Some(version) = version_dir.file_name().and_then(|name| name.to_str()) {
                    system_include_paths
                        .insert(toolchain_root.join("include").join("c++").join(version));
                    system_include_paths.insert(
                        toolchain_root
                            .join("include")
                            .join("c++")
                            .join(version)
                            .join("arm-none-eabi"),
                    );
                }
            }
        }
    }

    for path in gcc_include_search_paths() {
        system_include_paths.insert(path);
    }

    if let Some(extra) = env::var_os("ARM_NONE_EABI_INCLUDE") {
        for path in env::split_paths(&extra) {
            system_include_paths.insert(path);
        }
    }

    for path in system_include_paths {
        if path.exists() {
            let flag = format!("-isystem{}", path.display());
            if !args.contains(&flag) {
                args.push(flag);
            }
        }
    }

    args
}

fn gcc_include_search_paths() -> Vec<PathBuf> {
    let mut command = Command::new("arm-none-eabi-gcc");
    command.args(["-xc", "-E", "-Wp,-v", "-"]);
    command.stdin(Stdio::piped());
    command.stdout(Stdio::null());
    command.stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => return Vec::new(),
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(b"\n");
    }

    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    if !output.status.success() {
        return Vec::new();
    }

    let stderr = match String::from_utf8(output.stderr) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };

    let mut paths = Vec::new();
    let mut capture = false;

    for line in stderr.lines() {
        if line.contains("#include <...> search starts here:") {
            capture = true;
            continue;
        }
        if capture {
            if line.contains("End of search list.") {
                break;
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let trimmed = trimmed.trim_start_matches("(framework directory) ");
            let trimmed = trimmed.trim_end_matches(" (framework directory)");
            if trimmed.is_empty() {
                continue;
            }
            let candidate = PathBuf::from(trimmed);
            if candidate.is_relative() {
                continue;
            }
            paths.push(candidate);
        }
    }

    paths
}

fn gcc_query(args: &[&str]) -> Option<String> {
    let mut command = Command::new("arm-none-eabi-gcc");
    for arg in args {
        command.arg(arg);
    }
    command.output().ok().and_then(|output| {
        if output.status.success() {
            String::from_utf8(output.stdout).ok()
        } else {
            None
        }
    })
}
