use yew::prelude::*;

use crate::set_lang;
use crate::breadcrumb::BreadCrumb;
use crate::footer::Footer;
use crate::header::Header;
use crate::home::HomeProps;
use crate::layout::class_text;
use crate::router::{Lang, Route};
use crate::title::Title;

set_lang!(_privacy, "プライバシーポリシー", "Privacy Policy");
set_lang!(_personal, "個人情報の利用目的", "Use of Personal Data");
set_lang!(_personal1,
    "AutoCalc by 艮電算術研究所（以下、当サイト）では、お問い合わせやコメントの際、名前やメールアドレス等の個人情報を入力いただく場合がございます。",
    "AutoCalc by Ushitora Lab. (hereafter referred to as 'this website') may ask you to enter personal information such as your name and email address when making an enquiry or comment."
);
set_lang!(_personal2,
    "取得した個人情報は、お問い合わせに対する回答や必要な情報を電子メールなどでご連絡する場合に利用させていただくものであり、これらの目的以外では利用いたしません。",
    "The personal information obtained will only be used to reply to your enquiry or to contact you by e-mail with necessary information, and will not be used for any other purpose."
);
set_lang!(_adsense, "広告について", "About Advertising");
set_lang!(_adsense1,
    "当サイトでは、第三者配信の広告サービス（Google Adsense）を利用しており、ユーザーの興味に応じた商品やサービスの広告を表示するため、クッキー（Cookie）を使用しております。",
    "This website uses a third-party advertising service (Google Adsense) and uses cookies to display advertisements for products and services that match the interests of users."
);
set_lang!(_adsense2,
    "Cookieを使用することで当サイトは利用者様のコンピュータを識別できるようになりますが、利用者様個人を特定できるものではありません。",
    "Cookies enable this website to identify the user's computer, but they do not identify the individual user."
);
set_lang!(_adsense3,
    "Cookieを無効にする方法やGoogle Adsenseに関する詳細は「",
    "For more information on how to disable cookies and on Google Adsense, please see '."
);
set_lang!(_adsense_policy, "広告-ポリシーと規約-Google", "Advertising - Privacy & Terms - Google");
set_lang!(_adsense_policy_link,
    "https://policies.google.com/technologies/ads?gl=jp",
    "https://policies.google.com/technologies/ads?gl=en&hl=en-GB"
);
set_lang!(_adsense4, "」をご確認下さい。", "'.");
set_lang!(_amazon, "Amazon アソシエイト・プログラムについて", "About the Amazon Associates Program");
set_lang!(_amazon1,
    "当サイトは、Amazon.co.jpを宣伝しリンクすることによってサイトが紹介料を獲得できる手段を提供することを目的に設定されたアフィリエイトプログラムである、Amazonアソシエイト・プログラムの参加者です。",
    "This site is a participant in the Amazon Associates Programme, an affiliate programme set up to provide a means for sites to earn referral fees by promoting and linking to Amazon.co.jp."
);
set_lang!(_analytics,
    "アクセス解析ツールについて",
    "About Access Analysis Tools"
);
set_lang!(_analytics1,
    "当サイトでは、第三者配信のアクセス解析ツール（Google Analytics、Google Search Console）を利用しています。これらのツールはトラフィックデータ収集のためにCookieを使用しております。トラフィックデータは匿名で収集されており、個人を特定するものではありません。",
    "This website uses third-party access analysis tools (Google Analytics, Google Search Console). These tools use cookies for traffic data collection. Traffic data is collected anonymously and does not identify individuals."
);
set_lang!(_comment, "コメントについて", "About Comments");
set_lang!(_comment1,
    "当サイトへのコメントを残す際に、IPアドレスを収集しています。",
    "We collect IP addresses when you leave comments on our website."
);
set_lang!(_comment2,
    "これはサイトの標準機能としてサポートされている機能で、スパムや荒らしへの対応以外にこのIPアドレスを使用することはありません。",
    "This is a standard supported feature of the site and we do not use this IP address except in response to spam or vandalism."
);
set_lang!(_comment3,
    "なお、すべてのコメントは管理人がその内容を確認し、承認した上での掲載となります。あらかじめご了承ください。",
    "Please note that all comments are posted only after the administrator has reviewed and approved their content. Please be aware of this in advance."
);
set_lang!(_disclaimer, "免責事項", "Disclaimer");
set_lang!(_contents, "コンテンツについて", "About Contents");
set_lang!(_contents1, 
    "当サイトからのリンクやバナー等で移動したサイトで提供される情報、サービス等について一切の責任を負いません。",
    "We accept no responsibility for the information, services or other content provided on sites to which you have moved via links, banners or other means from this website."
);
set_lang!(_contents2,
    "また当サイトのコンテンツ・情報について、できる限り正確な情報を提供するように努めておりますが、正確性や安全性を保証するものではありません。情報が古くなっている場合もございます。",
    "While every effort has been made to ensure that the content and information on this website is as accurate as possible, we cannot guarantee its accuracy or safety. Information may be out of date."
);
set_lang!(_contents3,
    "当サイトに掲載された内容によって生じた損害等の",
    "We "
);
set_lang!(_contents_strong,
    "一切の責任を負いかねます",
    "accept no liability"
);
set_lang!(_contents4,
    "のでご了承ください。",
    " for any damages or other losses caused by the content of this website."
);
set_lang!(_copylight, "著作権について", "Copyrights");
set_lang!(_copylight1, 
    "当サイトの閲覧は自由ですが、文字コンテンツや計算ツール・計算結果の利用については、以下を遵守してください。",
    "You are free to browse this website, but please observe the following regarding the use of textual content, calculation tools and calculation results."
);
set_lang!(_about_text, "文字コンテンツについて", "About Textual Contents");
set_lang!(_about_text1,
    "当サイトの記載内容の引用・翻訳は自由ですが、必ず出典を明記してください。",
    "You are free to quote or translate the content of the descriptions on this website, but you must always state the source."
);
set_lang!(_about_text2,
    "また、以下の行為を禁止します。",
    "Additionally, the following acts are prohibited"
);
set_lang!(_about_text_list1_1,
    "無断転載（転載を希望される場合は、「",
    "Unauthorised reproduction (if you wish to reproduce an content, please contact us via the '"
);
set_lang!(_contact_us, "お問い合わせ", "Contact Us");
set_lang!(_contact_us_link, "https://ushitora.net/contact", "https://ushitora.net/en-GB/contact");
set_lang!(_about_text_list1_2,
    "」からその旨を連絡してください）",
    "' form)."
);
set_lang!(_about_text_list2,
    "当サイトの記載内容をそのまま、または一部改変して無償・有償にて配布すること",
    "Distributing the contents of this website as they are or partially modified for free or for a fee."
);
set_lang!(_about_text_list3,
    "当サイトの記載内容をそのまま、または一部改変して大学等の学術機関にて「課題」等として提出すること",
    "Submission of the content of this website as it is or with partial modification as an 'assignment' at universities or other academic institutions."
);
set_lang!(_about_text_list4,
    "当サイトの記載内容を、誤解を招く形で引用すること",
    "Quoting the contents of this website in a misleading manner."
);
set_lang!(_about_text_list5,
    "その他、管理人が不適切と判断した利用",
    "Any other use deemed inappropriate by the administrator."
);
set_lang!(_about_calcs,
    "計算ツール・計算結果について",
    "About Calculation Tools and Results"
);
set_lang!(_about_calcs1,
    "当サイトで実行された計算の結果を利用する際、当サイトまで連絡したり出典を明記したりする必要は特にありません。",
    "When using the results of calculations performed on this website, there is no particular need to contact the website or state the source."
);
set_lang!(_about_calcs2,
    "計算プログラムの性質上、バグが存在したり、利用者に何らかの損害を与えたりする可能性が考えられますが、その場合であっても当サイト並びに管理人は一切の責任を負いません。",
    "Due to the nature of the calculation programme, it is possible that bugs may exist or damage may be caused to the user, for which neither the website nor its administrators accept any liability."
);
set_lang!(_about_calcs3,
    "また、当サイトで使用している計算プログラムを逆解析したり、商用・非商用を問わず流用したりすることを禁止します。",
    "It is forbidden to reverse-analyses the calculation programmes used on this website or to divert them for commercial or non-commercial use."
);
set_lang!(_unclear, "不明点について", "About Unclear Things");
set_lang!(_unclear1,
    "当サイトの利用について不明な点がある場合は、「",
    "If you have any questions about the use of this website, please contact us via '"
);
set_lang!(_unclear2,
    "」より連絡してください。",
    "'."
);
set_lang!(_infringement,
    "当サイトによる著作権侵害事例に関する対応",
    "Response to Cases of Copyright Infringement by This Website"
);
set_lang!(_infringement1,
    "当サイトは著作権や肖像権の侵害を目的としたものではありません、著作権や肖像権に関して問題がございましたら、「",
    "This website is not intended to infringe copyright or portrait rights. If you have any problems regarding copyright or portrait rights, please contact us via '"
);
set_lang!(_infringement2,
    "」よりご連絡ください。迅速に対応いたします。",
    "' form. We will respond promptly."
);
set_lang!(_link, "リンクについて", "About Linking to This Website");
set_lang!(_link1,
    "当サイトは基本的にリンクフリーです。リンクを行う場合の許可や連絡は不要です。",
    "This website is basically link-free. No permission or contact is required to link to this website."
);
set_lang!(_link2,
    "ただし、インラインフレームの使用や画像の直リンクはご遠慮ください。",
    "However, please refrain from using inline frames and direct links to images."
);

#[function_component(Privacy)]
pub fn privacy(props: &HomeProps) -> Html {
    let lang = props.lang;
    html! {
        <>
        <Header route={Route::Privacy} {lang} />
        <BreadCrumb route={Route::Privacy} {lang} />
        <main class="container mt-2">
        <Title route={Route::Privacy} {lang} />
        <div class="row justify-content-md-center">
        <div class={class_text("")}>
            <h2 class="mt-5">{_privacy(lang)}</h2>
            
            <h3 class="mt-4">{_personal(lang)}</h3>
            <p>{_personal1(lang)}</p>
            <p>{_personal2(lang)}</p>
            
            <h3 class="mt-4">{_adsense(lang)}</h3>
            <p>{_adsense1(lang)}</p>
            <p>{_adsense2(lang)}</p>
            <p>{_adsense3(lang)}<a href={_adsense_policy_link(lang)}>{_adsense_policy(lang)}</a>{_adsense4(lang)}</p>

            <h3 class="mt-4">{_amazon(lang)}</h3>
            <p>{_amazon1(lang)}</p>

            <h3 class="mt-4">{_analytics(lang)}</h3>
            <p>{_analytics1(lang)}</p>

            <h3 class="mt-4">{_comment(lang)}</h3>
            <p>{_comment1(lang)}</p>
            <p>{_comment2(lang)}</p>
            <p>{_comment3(lang)}</p>
            
            <h2 class="mt-5">{_disclaimer(lang)}</h2>

            <h3 class="mt-4">{_contents(lang)}</h3>
            <p>{_contents1(lang)}</p>
            <p>{_contents2(lang)}</p>
            <p>{_contents3(lang)}<strong>{_contents_strong(lang)}</strong>{_contents4(lang)}</p>
            
            <h3 class="mt-4">{_copylight(lang)}</h3>
            <p>{_copylight1(lang)}</p>
            
            <h4 class="mt-4">{_about_text(lang)}</h4>
            <p>{_about_text1(lang)}</p>
            <p>{_about_text2(lang)}</p>
            <ul>
                <li>{_about_text_list1_1(lang)}<a href={_contact_us_link(lang)}>{_contact_us(lang)}</a>{_about_text_list1_2(lang)}</li>
                <li>{_about_text_list2(lang)}</li>
                <li>{_about_text_list3(lang)}</li>
                <li>{_about_text_list4(lang)}</li>
                <li>{_about_text_list5(lang)}</li>
            </ul>

            <h4 class="mt-4">{_about_calcs(lang)}</h4>
            <p>{_about_calcs1(lang)}</p>
            <p>{_about_calcs2(lang)}</p>
            <p>{_about_calcs3(lang)}</p>

            <h4 class="mt-4">{_unclear(lang)}</h4>
            <p>{_unclear1(lang)}<a href={_contact_us_link(lang)}>{_contact_us(lang)}</a>{_unclear2(lang)}</p>

            <h4 class="mt-4">{_infringement(lang)}</h4>
            <p>{_infringement1(lang)}<a href={_contact_us_link(lang)}>{_contact_us(lang)}</a>{_infringement2(lang)}</p>

            <h3 class="mt-4">{_link(lang)}</h3>
            <p>{_link1(lang)}</p>
            <p>{_link2(lang)}</p>
        </div>
        </div>
        </main>
        <Footer {lang} />
        </>
    }
}