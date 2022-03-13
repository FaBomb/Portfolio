use crate::compornents::{footer::Footer, header::Header};
use yew::{function_component, html};

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <Header/>
            <div class="profile">
                <h1>{ "- Profile -" }</h1>
                <p class="small-text">{"プロフィール"}</p>
                <img src="images/profile.jpg" alt="プロフィール画像" />
                <h2>{ "豊見 悠太" }</h2>
                <p class="small-text">{"Yuta Toyomi"}</p>
                <p class="p-detail">
                    {"1997年12月31日生まれ"}<br/>
                    {"2016年： 法政大学 デザイン工学部 建築学科 卒業"}<br/>
                    {"2022年： 同大学大学院 デザイン工学研究科 建築学専攻 修士課程修了（工学）"}<br/>
                    {"現在： Webエンジニアとして勤務"}<br/><br/>
                    {"大学では、数値流体解析や音・光・日射解析等のCAE解析を通して建築の形態を最適化する
                    アルゴリズミックデザインの研究をしていました。
                    他にも統計や機械学習を用いたデータ分析を趣味で行っていました。"}<br/>
                </p>
                <h2>{ "- Vision -" }</h2>
                <h3>{"複雑な仕組みをシンプルに..."}<br/>{"今まで見えなかった“つながり”を生み出す"}</h3>
                <p class="p-detail">
                    {"情報があふれ、様々な仕組みが複雑化し交錯している現代において、
                    一人ひとりに適した情報や人、モノと出会うことはむずかしい..."}<br/>
                    {"そのような複雑な仕組みをテクノロジーによりひも解いていき、
                    今まで交錯していて見えなかった様々なつながりを生み出していきたい"}
                </p>
                <h2>{ "- As Engineer -" }</h2>
                <h3>{"とことん好奇心を追及する"}</h3>
                <p class="p-detail">
                    {"CSの基礎から着実に学び、日々発展していく技術にキャッチアップすることを心がけ、
                    自分が面白いと思える興味の幅を広く持ちながら探求していくエンジニアを目指す"}
                </p>
                <h2>{ "- About -" }</h2>
                <div class="about">
                    <a href="https://twitter.com/FaBombLab" target="_blank">
                        <i class="fa-brands fa-twitter"></i>
                        <p>{"Twitter"}</p>
                    </a>
                    <a href="https://www.instagram.com/fabomb_lab" target="_blank">
                        <i class="fa-brands fa-instagram"></i>
                        <p>{"Instagram"}</p>
                    </a>
                    <a href="https://github.com/FaBomb" target="_blank">
                        <i class="fa-brands fa-github"></i>
                        <p>{"Github"}</p>
                    </a>
                </div>
            </div>
            <Footer/>
        </>
    }
}
