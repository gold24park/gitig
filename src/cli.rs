use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gitignore",
    about = "generates gitignore for specified project",
    author = "gold24park"
)]
pub struct CmdArgs {
    #[structopt(flatten)]
    pub write_opts: WriteOpts,

    /*
        OsString
            Owned, Mutable한 운영체제 네이티브 문자열 형식.
            - Unix: 문자열은 종종 0이아닌 임의의 바이트 시퀀스로 표현됨, 많은 경우 UTF-8로 해석됨.
            - Windows: 문자열은 종종 0이아닌 16비트 값의 임의의 시퀀스로 표현됨, 유효한 경우 UTF-16으로도 해석됨.
            Rust에서는 문자열은 항상 UTF-8로 구성되어 있고, 이 문자열에는 0이 포함될 수도 있다.
            각 운영체제에서 문자열 인코딩 방식이 다를 수 있으므로, 플랫폼별 문자열 표현을 다룰 때 사용된다.
            시스템 경로, 환경 변수등을 나타내는데 유용하다.

        OsStr
            String에게 str가 있다면 OsString에는 OsStr가 있음.
            Borrowed References to an OsString. Immutable


        PathBuf
            OsString을 가지고 있음.
            Owned, Mutable Path. (원래 Path를 써보려고 했는데 쓸 수 없었음. 이유는 Path의 설명에)
            파일 시스템에서 경로를 표현, 조작하기 위해 사용된다.
            - PathBuf::as_path로 Path Slice로 변환 가능

        Path
            compile time에는 크기를 알 수 없는 "unsized" 타입. (항상 &나 Box와 함께 써야함을 의미)
            - Path::join으로 PathBuf를 "생성"할 수 있다.
            파일 경로를 조작할때 PathBuf를 복제해서 PathBuff::push를 해야한다면,
            차라리 Path::join을 써서 새로운 PathBuf를 만드는 것을 권장한다.

            왜?
            https://youtu.be/rDoqT-a6UFg?t=1653

    */
    /// 지정한 경로에 .gitignore 생성
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    pub project: String,
}

#[derive(Debug, StructOpt)]
pub struct WriteOpts {
    /// path에 파일이 있을 경우 replace
    #[structopt(short, long)]
    pub replace: bool,

    /// path에 파일이 있을 경우 append
    #[structopt(short, long)]
    pub append: bool,
}
