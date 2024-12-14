use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, path::Path};

use askama::Template;
use include_dir::{include_dir, Dir};
use strum::{EnumIter, IntoEnumIterator};

use crate::utils::create_file;

use dialoguer::{Input, Select};

use super::TerraformCommandError;

static SRC_DIR: Dir = include_dir!("resources/terraform/");

pub(crate) fn init_terraform_project(
    overwrite_conflict_files: bool,
    skip_conflict_files: bool,
) -> Result<(), TerraformCommandError> {
    copy_dir_recursive(
        Path::new("./"),
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    let mut project_name: String = String::from("");
    while project_name.is_empty() {
        project_name = match Input::new().with_prompt("Project Name").interact_text() {
            Ok(val) => val,
            Err(err) => match err {
                dialoguer::Error::IO(e) => return Err(TerraformCommandError::IOError(e)),
            },
        }
    }
    let mut regions: Vec<String> = vec![];
    for region in AwsRegion::iter() {
        regions.push(format!("{}", region));
    }
    let choice: usize = match Select::new()
        .with_prompt("Region Name")
        .items(&regions)
        .default(15)
        .interact()
    {
        Ok(val) => val,
        Err(err) => match err {
            dialoguer::Error::IO(e) => return Err(TerraformCommandError::IOError(e)),
        },
    };
    let region_name: &str = &regions[choice];
    let envs = ["dev", "test", "prod"];
    for env in envs {
        generate_provider_template(
            &project_name,
            env,
            region_name,
            overwrite_conflict_files,
            skip_conflict_files,
        )?;
    }
    Ok(())
}

fn copy_dir_recursive(
    dst: &Path,
    overwrite_all: bool,
    ignore_conflict_config_file: bool,
) -> Result<(), std::io::Error> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }
    let glob = "**/*";
    for file in SRC_DIR.find(glob).unwrap() {
        let dst_path = dst.join(file.path());
        match file {
            include_dir::DirEntry::Dir(d) => {
                fs::create_dir_all(d.path().as_os_str().to_str().unwrap()).unwrap();
            }
            include_dir::DirEntry::File(f) => {
                let file_path = f.path().as_os_str().to_str().unwrap();
                let file_buf: PathBuf = PathBuf::from_str(file_path).unwrap();
                if let Some(parent) = file_buf.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                let _ = create_file(
                    dst_path.as_os_str().to_str().unwrap(),
                    f.contents(),
                    overwrite_all,
                    ignore_conflict_config_file,
                );
            }
        }
    }
    Ok(())
}

fn generate_provider_template(
    project_name: &str,
    env_name: &str,
    region_name: &str,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), std::io::Error> {
    let template = ProviderTemplate {
        project_name,
        env_name,
        region_name,
    };
    let render_res = template.render().unwrap();
    let file_name = format!("envs/{}/provider.tf", env_name,);
    create_file(
        &file_name,
        render_res.as_bytes(),
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    Ok(())
}

#[derive(Template)]
#[template(path = "terraform/envs/provider.tf", escape = "none")]
pub(super) struct ProviderTemplate<'a> {
    pub(super) project_name: &'a str,
    pub(super) env_name: &'a str,
    pub(super) region_name: &'a str,
}

#[derive(Debug, EnumIter)]
enum AwsRegion {
    UsEast1,      // 米国東部 (バージニア北部)
    UsEast2,      // 米国東部 (オハイオ)
    UsWest1,      // 米国西部 (北カリフォルニア)
    UsWest2,      // 米国西部 (オレゴン)
    AfSouth1,     // アフリカ (ケープタウン)
    ApEast1,      // アジアパシフィック (香港)
    ApSouth2,     // アジアパシフィック (ハイデラバード)
    ApSoutheast3, // アジアパシフィック (ジャカルタ)
    ApSoutheast5, // アジアパシフィック (マレーシア)
    ApSoutheast4, // アジアパシフィック (メルボルン)
    ApSouth1,     // アジアパシフィック (ムンバイ)
    ApNortheast3, // アジアパシフィック (大阪)
    ApNortheast2, // アジアパシフィック (ソウル)
    ApSoutheast1, // アジアパシフィック (シンガポール)
    ApSoutheast2, // アジアパシフィック (シドニー)
    ApNortheast1, // アジアパシフィック (東京)
    CaCentral1,   // カナダ (中部)
    CaWest1,      // カナダ西部 (カルガリー)
    CnNorth1,     // 中国 (北京)
    CnNorthwest1, // 中国 (寧夏)
    EuCentral1,   // 欧州 (フランクフルト)
    EuWest1,      // 欧州 (アイルランド)
    EuWest2,      // 欧州 (ロンドン)
    EuSouth1,     // 欧州 (ミラノ)
    EuWest3,      // 欧州 (パリ)
    EuSouth2,     // 欧州 (スペイン)
    EuNorth1,     // 欧州 (ストックホルム)
    EuCentral2,   // 欧州 (チューリッヒ)
    IlCentral1,   // イスラエル (テルアビブ)
    MeSouth1,     // 中東 (バーレーン)
    MeCentral1,   // 中東 (アラブ首長国連邦)
    SaEast1,      // 南米 (サンパウロ)
}

impl fmt::Display for AwsRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            AwsRegion::UsEast1 => "us-east-1",
            AwsRegion::UsEast2 => "us-east-2",
            AwsRegion::UsWest1 => "us-west-1",
            AwsRegion::UsWest2 => "us-west-2",
            AwsRegion::AfSouth1 => "af-south-1",
            AwsRegion::ApEast1 => "ap-east-1",
            AwsRegion::ApSouth2 => "ap-south-2",
            AwsRegion::ApSoutheast3 => "ap-southeast-3",
            AwsRegion::ApSoutheast5 => "ap-southeast-5",
            AwsRegion::ApSoutheast4 => "ap-southeast-4",
            AwsRegion::ApSouth1 => "ap-south-1",
            AwsRegion::ApNortheast3 => "ap-northeast-3",
            AwsRegion::ApNortheast2 => "ap-northeast-2",
            AwsRegion::ApSoutheast1 => "ap-southeast-1",
            AwsRegion::ApSoutheast2 => "ap-southeast-2",
            AwsRegion::ApNortheast1 => "ap-northeast-1",
            AwsRegion::CaCentral1 => "ca-central-1",
            AwsRegion::CaWest1 => "ca-west-1",
            AwsRegion::CnNorth1 => "cn-north-1",
            AwsRegion::CnNorthwest1 => "cn-northwest-1",
            AwsRegion::EuCentral1 => "eu-central-1",
            AwsRegion::EuWest1 => "eu-west-1",
            AwsRegion::EuWest2 => "eu-west-2",
            AwsRegion::EuSouth1 => "eu-south-1",
            AwsRegion::EuWest3 => "eu-west-3",
            AwsRegion::EuSouth2 => "eu-south-2",
            AwsRegion::EuNorth1 => "eu-north-1",
            AwsRegion::EuCentral2 => "eu-central-2",
            AwsRegion::IlCentral1 => "il-central-1",
            AwsRegion::MeSouth1 => "me-south-1",
            AwsRegion::MeCentral1 => "me-central-1",
            AwsRegion::SaEast1 => "sa-east-1",
        };
        write!(f, "{}", s)
    }
}
