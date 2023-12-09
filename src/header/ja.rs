use yew::prelude::*;
use yew_router::prelude::*;

use crate::create_header;
use crate::router::{make_path, encode_query, parse_query};

create_header!("自動計算サイト by 艮電算術研究所", "言語設定");