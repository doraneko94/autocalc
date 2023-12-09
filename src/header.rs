pub mod ja;
pub mod en;

#[macro_export]
macro_rules! create_header {
    ($title: expr, $lang_menu: expr) => {
        #[function_component(Header)]
        pub fn header() -> Html {
            let params = parse_query(use_location().unwrap().query_str());
            let mut params_ja = params.clone();
            let mut params_en = params.clone();
            params_ja.1 = Some("ja".to_string());
            params_en.1 = Some("en".to_string());
            let query_ja = encode_query(params_ja);
            let query_en = encode_query(params_en);

            let lang = match params.1 {
                Some(s) => {
                    if s == "ja".to_string() { "?lang=ja" } else { "?lang=en" }
                },
                _ => "?lang=en",
            };
            html! {
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <div class="container">
                        <a href={make_path(lang)} class="navbar-brand">{$title}</a>
                        <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle navbar-brand" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                                {$lang_menu}
                            </a>
                            <ul class="dropdown-menu">
                                <li><a class="dropdown-item" href={make_path(query_en.as_str())}>{"🇬🇧 English"}</a></li>
                                <li><a class="dropdown-item" href={make_path(query_ja.as_str())}>{"🇯🇵 日本語"}</a></li>
                            </ul>
                        </li>
                    </div>
                </nav>
            }
        }
    };
}