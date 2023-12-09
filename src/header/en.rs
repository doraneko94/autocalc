use yew::prelude::*;
use yew_router::prelude::*;

use crate::create_header;
use crate::router::{make_path, encode_query, parse_query};

create_header!("AutoCalc by Ushitora Lab.", "Languages");