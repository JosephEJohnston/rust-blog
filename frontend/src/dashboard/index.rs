use stylist::Style;
use yew::{Component, Context, Html, html};
use crate::css::DASHBOARD_CSS;

pub struct DashboardIndex {
    js_code: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DashboardIndexMsg {
    RenderJS,
}

impl Component for DashboardIndex {
    type Message = DashboardIndexMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DashboardIndex {
            js_code: "feather.replace();".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardIndexMsg::RenderJS => {
                self.js_code = "feather.replace();".to_string();

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dashboard_css = Style::new(DASHBOARD_CSS).unwrap();

        html! {
            <>
                <div id="page" class={ dashboard_css }>
                    <aside class="page-column-left">
                        <div class="user-info">
                            <div class="user-info-detail">
                                <img class="user-img" src="../resource/img/dashboard-img.jpg" alt="" />
                                <div class="user-name">{"admin"}</div>
                                <div class="user-email">{"admin@pigjian.com"}</div>
                            </div>
                            <div class="user-info-button">
                                <button class="for-user-info-button">
                                    <i class="user-info-icon" data-feather="home"></i>
                                </button>
                                <button class="for-user-info-button">
                                    <i class="user-info-icon" data-feather="user"></i>
                                </button>
                                <button class="for-user-info-button">
                                    <i class="user-info-icon" data-feather="settings"></i>
                                </button>
                            </div>
                        </div>
                        <div class="function">
                            <hr/>
                            <div class="module">
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="layout"></i>
                                    {"面板"}
                                </div>
                            </div>
                            <div class="module">
                                <div class="module-title">{"内容模块"}</div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="book"></i>
                                    {"文章管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="message-square"></i>
                                    {"讨论管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="message-circle"></i>
                                    {"评论管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="tag"></i>
                                    {"标签管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="table"></i>
                                    {"分类管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="link"></i>
                                    {"友链管理"}
                                </div>
                            </div>
                            <div class="module">
                                <div class="module-title">{"基础模块"}</div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="users"></i>
                                    {"用户管理"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="file"></i>
                                    {"文件管理"}
                                </div>
                            </div>
                            <div class="module">
                                <div class="module-title">{"系统模块"}</div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="fast-forward"></i>
                                    {"访问列表"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="key"></i>
                                    {"角色列表"}
                                </div>
                                <div class="for-each-module">
                                    <i class="module-icon" data-feather="server"></i>
                                    {"系统配置"}
                                </div>
                            </div>
                        </div>
                    </aside>
                    <div id="content" class="page-column-right">
                        <div class="page-right-header">
                            <i class="header-icon" data-feather="align-justify"></i>
                        </div>
                        <article>
                            <div class="statistics-info">
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"用户数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="users"></i>
                                        <div class="each-data">{"12"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"访问者"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="eye"></i>
                                        <div class="each-data">{"4775"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"文章数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="book"></i>
                                        <div class="each-data">{"20"}</div>
                                    </div>
                                </div>
                                <div class="for-each-statistics">
                                    <div class="each-statistics-title">
                                        <div class="each-title-detail">{"评论数"}</div>
                                        <div class="each-tag-all">{"全部"}</div>
                                    </div>
                                    <div class="each-statistics-data">
                                        <i class="each-icon" data-feather="message-circle"></i>
                                        <div class="each-data">{"50"}</div>
                                    </div>
                                </div>
                            </div>
                        </article>
                    </div>
                    <footer>

                    </footer>
                </div>
                <script src="feather/feather.min.js">
                </script>
                <script>
                    { "setTimeout(function() { feather.replace(); }, 50)" }
                </script>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(DashboardIndexMsg::RenderJS);
        }
    }
}