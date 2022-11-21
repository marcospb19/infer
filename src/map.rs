use super::{matchers, Matcher, Type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatcherType {
    App,
    Archive,
    Audio,
    Book,
    Doc,
    Font,
    Image,
    Text,
    Video,
    Custom,
}

// This is needed until function pointers can be used in `const fn`.
// See trick and discussion at https://github.com/rust-lang/rust/issues/63997#issuecomment-616666309
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct WrapMatcher(pub Matcher);

macro_rules! matcher_map {
    ($(($mtype:expr, $mime_type:literal, $extension:literal, $matcher:expr)),*) => {
        pub const MATCHER_MAP: &[Type] = &[
            $(Type::new_static($mtype, $mime_type, $extension, WrapMatcher($matcher)),)*
        ];
    };
}

// Order: Application, Image, Video, Audio, Font, Document, Archive, Text.
// The above order should be preserved when adding new types since
// it may affect match result and/or performances.
matcher_map!(
    // Application
    (
        MatcherType::App,
        "application/wasm",
        "wasm",
        matchers::app::is_wasm
    ),
    (
        MatcherType::App,
        "application/x-executable",
        "elf",
        matchers::app::is_elf
    ),
    (
        MatcherType::App,
        "application/vnd.microsoft.portable-executable",
        "exe",
        matchers::app::is_exe
    ),
    (
        MatcherType::App,
        "application/vnd.microsoft.portable-executable",
        "dll",
        matchers::app::is_dll
    ),
    (
        MatcherType::App,
        "application/java",
        "class",
        matchers::app::is_java
    ),
    (
        MatcherType::App,
        "application/x-llvm",
        "bc",
        matchers::app::is_llvm
    ),
    (
        MatcherType::App,
        "application/x-mach-binary",
        "mach",
        matchers::app::is_mach
    ),
    (
        MatcherType::App,
        "application/vnd.android.dex",
        "dex",
        matchers::app::is_dex
    ),
    (
        MatcherType::App,
        "application/vnd.android.dey",
        "dey",
        matchers::app::is_dey
    ),
    (
        MatcherType::App,
        "application/x-x509-ca-cert",
        "der",
        matchers::app::is_der
    ),
    (
        MatcherType::App,
        "application/x-executable",
        "obj",
        matchers::app::is_coff
    ),
    (
        MatcherType::App,
        "application/x-x509-ca-cert",
        "pem",
        matchers::app::is_pem
    ),
    // Book
    (
        MatcherType::Book,
        "application/epub+zip",
        "epub",
        matchers::book::is_epub
    ),
    (
        MatcherType::Book,
        "application/x-mobipocket-ebook",
        "mobi",
        matchers::book::is_mobi
    ),
    // Image
    (
        MatcherType::Image,
        "image/jpeg",
        "jpg",
        matchers::image::is_jpeg
    ),
    (
        MatcherType::Image,
        "image/jp2",
        "jp2",
        matchers::image::is_jpeg2000
    ),
    (
        MatcherType::Image,
        "image/png",
        "png",
        matchers::image::is_png
    ),
    (
        MatcherType::Image,
        "image/gif",
        "gif",
        matchers::image::is_gif
    ),
    (
        MatcherType::Image,
        "image/webp",
        "webp",
        matchers::image::is_webp
    ),
    (
        MatcherType::Image,
        "image/x-canon-cr2",
        "cr2",
        matchers::image::is_cr2
    ),
    (
        MatcherType::Image,
        "image/tiff",
        "tif",
        matchers::image::is_tiff
    ),
    (
        MatcherType::Image,
        "image/bmp",
        "bmp",
        matchers::image::is_bmp
    ),
    (
        MatcherType::Image,
        "image/vnd.ms-photo",
        "jxr",
        matchers::image::is_jxr
    ),
    (
        MatcherType::Image,
        "image/vnd.adobe.photoshop",
        "psd",
        matchers::image::is_psd
    ),
    (
        MatcherType::Image,
        "image/vnd.microsoft.icon",
        "ico",
        matchers::image::is_ico
    ),
    (
        MatcherType::Image,
        "image/heif",
        "heif",
        matchers::image::is_heif
    ),
    (
        MatcherType::Image,
        "image/avif",
        "avif",
        matchers::image::is_avif
    ),
    (
        MatcherType::Image,
        "image/jxl",
        "jxl",
        matchers::image::is_jxl
    ),
    // Video
    (
        MatcherType::Video,
        "video/mp4",
        "mp4",
        matchers::video::is_mp4
    ),
    (
        MatcherType::Video,
        "video/x-m4v",
        "m4v",
        matchers::video::is_m4v
    ),
    (
        MatcherType::Video,
        "video/x-matroska",
        "mkv",
        matchers::video::is_mkv
    ),
    (
        MatcherType::Video,
        "video/webm",
        "webm",
        matchers::video::is_webm
    ),
    (
        MatcherType::Video,
        "video/quicktime",
        "mov",
        matchers::video::is_mov
    ),
    (
        MatcherType::Video,
        "video/x-msvideo",
        "avi",
        matchers::video::is_avi
    ),
    (
        MatcherType::Video,
        "video/x-ms-wmv",
        "wmv",
        matchers::video::is_wmv
    ),
    (
        MatcherType::Video,
        "video/mpeg",
        "mpg",
        matchers::video::is_mpeg
    ),
    (
        MatcherType::Video,
        "video/x-flv",
        "flv",
        matchers::video::is_flv
    ),
    // Audio
    (
        MatcherType::Audio,
        "audio/midi",
        "midi",
        matchers::audio::is_midi
    ),
    (
        MatcherType::Audio,
        "audio/mpeg",
        "mp3",
        matchers::audio::is_mp3
    ),
    (
        MatcherType::Audio,
        "audio/m4a",
        "m4a",
        matchers::audio::is_m4a
    ),
    // has to come before ogg
    (
        MatcherType::Audio,
        "audio/opus",
        "opus",
        matchers::audio::is_ogg_opus
    ),
    (
        MatcherType::Audio,
        "audio/ogg",
        "ogg",
        matchers::audio::is_ogg
    ),
    (
        MatcherType::Audio,
        "audio/x-flac",
        "flac",
        matchers::audio::is_flac
    ),
    (
        MatcherType::Audio,
        "audio/x-wav",
        "wav",
        matchers::audio::is_wav
    ),
    (
        MatcherType::Audio,
        "audio/amr",
        "amr",
        matchers::audio::is_amr
    ),
    (
        MatcherType::Audio,
        "audio/aac",
        "aac",
        matchers::audio::is_aac
    ),
    (
        MatcherType::Audio,
        "audio/x-aiff",
        "aiff",
        matchers::audio::is_aiff
    ),
    (
        MatcherType::Audio,
        "audio/x-dsf",
        "dsf",
        matchers::audio::is_dsf
    ),
    (
        MatcherType::Audio,
        "audio/x-ape",
        "ape",
        matchers::audio::is_ape
    ),
    // Font
    (
        MatcherType::Font,
        "application/font-woff",
        "woff",
        matchers::font::is_woff
    ),
    (
        MatcherType::Font,
        "application/font-woff",
        "woff2",
        matchers::font::is_woff2
    ),
    (
        MatcherType::Font,
        "application/font-sfnt",
        "ttf",
        matchers::font::is_ttf
    ),
    (
        MatcherType::Font,
        "application/font-sfnt",
        "otf",
        matchers::font::is_otf
    ),
    // Document
    (
        MatcherType::Doc,
        "application/msword",
        "doc",
        matchers::doc::is_doc
    ),
    (
        MatcherType::Doc,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "docx",
        matchers::doc::is_docx
    ),
    (
        MatcherType::Doc,
        "application/vnd.ms-excel",
        "xls",
        matchers::doc::is_xls
    ),
    (
        MatcherType::Doc,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xlsx",
        matchers::doc::is_xlsx
    ),
    (
        MatcherType::Doc,
        "application/vnd.ms-powerpoint",
        "ppt",
        matchers::doc::is_ppt
    ),
    (
        MatcherType::Doc,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "pptx",
        matchers::doc::is_pptx
    ),
    // OpenDocument
    (
        MatcherType::Doc,
        "application/vnd.oasis.opendocument.text",
        "odt",
        matchers::odf::is_odt
    ),
    (
        MatcherType::Doc,
        "application/vnd.oasis.opendocument.spreadsheet",
        "ods",
        matchers::odf::is_ods
    ),
    (
        MatcherType::Doc,
        "application/vnd.oasis.opendocument.presentation",
        "odp",
        matchers::odf::is_odp
    ),
    // Archive
    (
        MatcherType::Archive,
        "application/epub+zip",
        "epub",
        matchers::archive::is_epub
    ),
    (
        MatcherType::Archive,
        "application/zip",
        "zip",
        matchers::archive::is_zip
    ),
    (
        MatcherType::Archive,
        "application/x-tar",
        "tar",
        matchers::archive::is_tar
    ),
    (
        MatcherType::Archive,
        "application/vnd.rar",
        "rar",
        matchers::archive::is_rar
    ),
    (
        MatcherType::Archive,
        "application/gzip",
        "gz",
        matchers::archive::is_gz
    ),
    (
        MatcherType::Archive,
        "application/x-bzip2",
        "bz2",
        matchers::archive::is_bz2
    ),
    (
        MatcherType::Archive,
        "application/x-7z-compressed",
        "7z",
        matchers::archive::is_7z
    ),
    (
        MatcherType::Archive,
        "application/x-xz",
        "xz",
        matchers::archive::is_xz
    ),
    (
        MatcherType::Archive,
        "application/pdf",
        "pdf",
        matchers::archive::is_pdf
    ),
    (
        MatcherType::Archive,
        "application/x-shockwave-flash",
        "swf",
        matchers::archive::is_swf
    ),
    (
        MatcherType::Archive,
        "application/rtf",
        "rtf",
        matchers::archive::is_rtf
    ),
    (
        MatcherType::Archive,
        "application/octet-stream",
        "eot",
        matchers::archive::is_eot
    ),
    (
        MatcherType::Archive,
        "application/postscript",
        "ps",
        matchers::archive::is_ps
    ),
    (
        MatcherType::Archive,
        "application/vnd.sqlite3",
        "sqlite",
        matchers::archive::is_sqlite
    ),
    (
        MatcherType::Archive,
        "application/x-nintendo-nes-rom",
        "nes",
        matchers::archive::is_nes
    ),
    (
        MatcherType::Archive,
        "application/x-google-chrome-extension",
        "crx",
        matchers::archive::is_crx
    ),
    (
        MatcherType::Archive,
        "application/vnd.ms-cab-compressed",
        "cab",
        matchers::archive::is_cab
    ),
    (
        MatcherType::Archive,
        "application/vnd.debian.binary-package",
        "deb",
        matchers::archive::is_deb
    ),
    (
        MatcherType::Archive,
        "application/x-unix-archive",
        "ar",
        matchers::archive::is_ar
    ),
    (
        MatcherType::Archive,
        "application/x-compress",
        "Z",
        matchers::archive::is_z
    ),
    (
        MatcherType::Archive,
        "application/x-lzip",
        "lz",
        matchers::archive::is_lz
    ),
    (
        MatcherType::Archive,
        "application/x-rpm",
        "rpm",
        matchers::archive::is_rpm
    ),
    (
        MatcherType::Archive,
        "application/dicom",
        "dcm",
        matchers::archive::is_dcm
    ),
    (
        MatcherType::Archive,
        "application/zstd",
        "zst",
        matchers::archive::is_zst
    ),
    (
        MatcherType::Archive,
        "application/x-ole-storage",
        "msi",
        matchers::archive::is_msi
    ),
    (
        MatcherType::Archive,
        "application/x-cpio",
        "cpio",
        matchers::archive::is_cpio
    ),
    // Text
    (
        MatcherType::Text,
        "text/html",
        "html",
        matchers::text::is_html
    ),
    (MatcherType::Text, "text/xml", "xml", matchers::text::is_xml),
    (
        MatcherType::Text,
        "text/x-shellscript",
        "sh",
        matchers::text::is_shellscript
    )
);
