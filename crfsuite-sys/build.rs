extern crate gcc;

extern crate bindgen;

use std::env;
use std::path::Path;
use std::io::{Write, Read};
use std::fs::File;

fn main() {
    gcc::Build::new()
        .include("c/include")
        //.define("USE_SSE", "1") // TODO check if target supports SSE and enable if so

        // lbfgs
        //.file("c/lbfgs/arithmetic_ansi.h")
        //.file("c/lbfgs/arithmetic_sse_double.h")
        //.file("c/lbfgs/arithmetic_sse_float.h")
        //.file("c/include/lbfgs.h")
        .file("c/lbfgs/lbfgs.c")
        // cqdb
        .file("c/cqdb/lookup3.c")
        // .file("c/include/cqdb.h")
        .file("c/cqdb/cqdb.c")
        // crf
        .file("c/crf/dictionary.c")
        .file("c/crf/logging.c")
        //.file("c/crf/logging.h")
        .file("c/crf/params.c")
        //.file("c/crf/params.h")
        .file("c/crf/quark.c")
        //.file("c/crf/quark.h")
        .file("c/crf/rumavl.c")
        //.file("c/crf/rumavl.h")
        //.file("c/crf/vecmath.h")
        //.file("c/crf/crfsuite_internal.h")
        .file("c/crf/dataset.c")
        .file("c/crf/holdout.c")
        .file("c/crf/train_arow.c")
        .file("c/crf/train_averaged_perceptron.c")
        .file("c/crf/train_l2sgd.c")
        .file("c/crf/train_lbfgs.c")
        .file("c/crf/train_passive_aggressive.c")
        //.file("c/crf/crf1d.h")
        .file("c/crf/crf1d_context.c")
        .file("c/crf/crf1d_model.c")
        .file("c/crf/crf1d_feature.c")
        .file("c/crf/crf1d_encode.c")
        .file("c/crf/crf1d_tag.c")
        .file("c/crf/crfsuite_train.c")
        .file("c/crf/crfsuite.c")
        .compile("libcrfsuite.a");

    let out_dir = env::var("OUT_DIR").unwrap();

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    let mut builder = bindgen::builder();

    if target != host {
        if let Ok(sysroot) = env::var("TARGET_SYSROOT") {
            builder = builder
                .clang_arg(format!("--target={}", target))
                .clang_arg(format!("--sysroot={}", sysroot));

            // Add a path to the private headers for the target compiler. Borderline,
            // as we are likely using a gcc header with clang frontend.
            let target_compiler = gcc::Build::new().get_compiler();
            let target_compiler_include = target_compiler.to_command()
                .arg("--print-file-name=include").output();
            if let Ok(output) = target_compiler_include {
                if output.status.success() {
                    let path = String::from_utf8(output.stdout)
                        .expect("toolchain path shoud be utf8 friendly");
                    builder = builder.clang_arg(format!("-I{}", path.trim()));
                }
            }

            if target.contains("apple") && target.contains("aarch64") {
                // The official Apple tools use "-arch arm64" instead of specifying
                // -target directly; -arch only works when the default target is
                // Darwin-based to put Clang into "Apple mode" as it were. But it does
                // sort of explain why arm64 works better than aarch64, which is the
                // preferred name everywhere else.
                builder = builder
                    .clang_arg(format!("-arch"))
                    .clang_arg(format!("arm64"));
            }
            // ProTip : if some include are missing from your sysroot, (for example GCC include like
            // stddef.h) you can add them to the clang search path by using the CPATH env var
        } else {
            panic!("Cross compiling detected, please provide a sysroot in TARGET_SYSROOT env var")
        }
    }
    let p = Path::new(&out_dir).join("crfsuite_orig.rs");

    builder.clang_arg("-v")
        .header("c/include/crfsuite.h")
        .generate().unwrap()
        .write_to_file(&p)
        .expect("Couldn't write bindings!");

    let mut file = File::open(p).unwrap();

    // bindgen generate a compile error when building for arm android...
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read file");
    let contents = if target == "armv7-linux-androideabi" || target == "arm-linux-androideabi" || target == "aarch64-linux-android" {
        contents
            // the generated code will have a space or not depending if rustfmt in installed...
            .replace("pub type __va_list = __builtin_va_list;", "")
            .replace("pub type __va_list = __builtin_va_list ;", "")
    } else {
        contents
    };

    let mut patched_file = File::create(Path::new(&out_dir).join("crfsuite.rs")).expect("couln't create crfsuite.rs");
    patched_file.write_all(contents.as_bytes()).expect("couldn't wrote to patched crfsuite.rs");
}
