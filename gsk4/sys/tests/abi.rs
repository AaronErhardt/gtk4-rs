// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gsk4_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gtk4"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {cmd:?} returned {}", out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {abi_cmd:?} failed, {output:?}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GskBlendMode",
        Layout {
            size: size_of::<GskBlendMode>(),
            alignment: align_of::<GskBlendMode>(),
        },
    ),
    (
        "GskColorStop",
        Layout {
            size: size_of::<GskColorStop>(),
            alignment: align_of::<GskColorStop>(),
        },
    ),
    (
        "GskCorner",
        Layout {
            size: size_of::<GskCorner>(),
            alignment: align_of::<GskCorner>(),
        },
    ),
    (
        "GskGLShaderClass",
        Layout {
            size: size_of::<GskGLShaderClass>(),
            alignment: align_of::<GskGLShaderClass>(),
        },
    ),
    (
        "GskGLUniformType",
        Layout {
            size: size_of::<GskGLUniformType>(),
            alignment: align_of::<GskGLUniformType>(),
        },
    ),
    (
        "GskParseLocation",
        Layout {
            size: size_of::<GskParseLocation>(),
            alignment: align_of::<GskParseLocation>(),
        },
    ),
    (
        "GskRenderNodeType",
        Layout {
            size: size_of::<GskRenderNodeType>(),
            alignment: align_of::<GskRenderNodeType>(),
        },
    ),
    (
        "GskRoundedRect",
        Layout {
            size: size_of::<GskRoundedRect>(),
            alignment: align_of::<GskRoundedRect>(),
        },
    ),
    (
        "GskScalingFilter",
        Layout {
            size: size_of::<GskScalingFilter>(),
            alignment: align_of::<GskScalingFilter>(),
        },
    ),
    (
        "GskSerializationError",
        Layout {
            size: size_of::<GskSerializationError>(),
            alignment: align_of::<GskSerializationError>(),
        },
    ),
    (
        "GskShadow",
        Layout {
            size: size_of::<GskShadow>(),
            alignment: align_of::<GskShadow>(),
        },
    ),
    (
        "GskTransformCategory",
        Layout {
            size: size_of::<GskTransformCategory>(),
            alignment: align_of::<GskTransformCategory>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GSK_BLEND_MODE_COLOR", "12"),
    ("(gint) GSK_BLEND_MODE_COLOR_BURN", "7"),
    ("(gint) GSK_BLEND_MODE_COLOR_DODGE", "6"),
    ("(gint) GSK_BLEND_MODE_DARKEN", "4"),
    ("(gint) GSK_BLEND_MODE_DEFAULT", "0"),
    ("(gint) GSK_BLEND_MODE_DIFFERENCE", "10"),
    ("(gint) GSK_BLEND_MODE_EXCLUSION", "11"),
    ("(gint) GSK_BLEND_MODE_HARD_LIGHT", "8"),
    ("(gint) GSK_BLEND_MODE_HUE", "13"),
    ("(gint) GSK_BLEND_MODE_LIGHTEN", "5"),
    ("(gint) GSK_BLEND_MODE_LUMINOSITY", "15"),
    ("(gint) GSK_BLEND_MODE_MULTIPLY", "1"),
    ("(gint) GSK_BLEND_MODE_OVERLAY", "3"),
    ("(gint) GSK_BLEND_MODE_SATURATION", "14"),
    ("(gint) GSK_BLEND_MODE_SCREEN", "2"),
    ("(gint) GSK_BLEND_MODE_SOFT_LIGHT", "9"),
    ("(gint) GSK_BLEND_NODE", "20"),
    ("(gint) GSK_BLUR_NODE", "23"),
    ("(gint) GSK_BORDER_NODE", "9"),
    ("(gint) GSK_CAIRO_NODE", "2"),
    ("(gint) GSK_CLIP_NODE", "17"),
    ("(gint) GSK_COLOR_MATRIX_NODE", "15"),
    ("(gint) GSK_COLOR_NODE", "3"),
    ("(gint) GSK_CONIC_GRADIENT_NODE", "8"),
    ("(gint) GSK_CONTAINER_NODE", "1"),
    ("(gint) GSK_CORNER_BOTTOM_LEFT", "3"),
    ("(gint) GSK_CORNER_BOTTOM_RIGHT", "2"),
    ("(gint) GSK_CORNER_TOP_LEFT", "0"),
    ("(gint) GSK_CORNER_TOP_RIGHT", "1"),
    ("(gint) GSK_CROSS_FADE_NODE", "21"),
    ("(gint) GSK_DEBUG_NODE", "24"),
    ("(gint) GSK_GL_SHADER_NODE", "25"),
    ("(gint) GSK_GL_UNIFORM_TYPE_BOOL", "4"),
    ("(gint) GSK_GL_UNIFORM_TYPE_FLOAT", "1"),
    ("(gint) GSK_GL_UNIFORM_TYPE_INT", "2"),
    ("(gint) GSK_GL_UNIFORM_TYPE_NONE", "0"),
    ("(gint) GSK_GL_UNIFORM_TYPE_UINT", "3"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC2", "5"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC3", "6"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC4", "7"),
    ("(gint) GSK_INSET_SHADOW_NODE", "11"),
    ("(gint) GSK_LINEAR_GRADIENT_NODE", "4"),
    ("(gint) GSK_NOT_A_RENDER_NODE", "0"),
    ("(gint) GSK_OPACITY_NODE", "14"),
    ("(gint) GSK_OUTSET_SHADOW_NODE", "12"),
    ("(gint) GSK_RADIAL_GRADIENT_NODE", "6"),
    ("(gint) GSK_REPEATING_LINEAR_GRADIENT_NODE", "5"),
    ("(gint) GSK_REPEATING_RADIAL_GRADIENT_NODE", "7"),
    ("(gint) GSK_REPEAT_NODE", "16"),
    ("(gint) GSK_ROUNDED_CLIP_NODE", "18"),
    ("(gint) GSK_SCALING_FILTER_LINEAR", "0"),
    ("(gint) GSK_SCALING_FILTER_NEAREST", "1"),
    ("(gint) GSK_SCALING_FILTER_TRILINEAR", "2"),
    ("(gint) GSK_SERIALIZATION_INVALID_DATA", "2"),
    ("(gint) GSK_SERIALIZATION_UNSUPPORTED_FORMAT", "0"),
    ("(gint) GSK_SERIALIZATION_UNSUPPORTED_VERSION", "1"),
    ("(gint) GSK_SHADOW_NODE", "19"),
    ("(gint) GSK_TEXTURE_NODE", "10"),
    ("(gint) GSK_TEXT_NODE", "22"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D", "3"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D_AFFINE", "4"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D_TRANSLATE", "5"),
    ("(gint) GSK_TRANSFORM_CATEGORY_3D", "2"),
    ("(gint) GSK_TRANSFORM_CATEGORY_ANY", "1"),
    ("(gint) GSK_TRANSFORM_CATEGORY_IDENTITY", "6"),
    ("(gint) GSK_TRANSFORM_CATEGORY_UNKNOWN", "0"),
    ("(gint) GSK_TRANSFORM_NODE", "13"),
];
