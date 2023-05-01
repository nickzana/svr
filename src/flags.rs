use std::path::PathBuf;

xflags::xflags! {
    cmd svr {
        optional -c, --config config_path: PathBuf
    }
}

