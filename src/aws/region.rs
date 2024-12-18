use std::fmt;

use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, PartialEq)]
pub(crate) enum AwsRegion {
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

impl AwsRegion {
    pub(crate) fn get_index(region: &AwsRegion) -> Option<usize> {
        for (index, reg) in AwsRegion::iter().enumerate() {
            if &reg == region {
                return Some(index);
            }
        }
        None
    }
}
