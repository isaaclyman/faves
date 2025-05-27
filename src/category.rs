use std::collections::HashMap;

use serde_json::Value;
use urlencoding::decode;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{data::get_all_data, icons, Route};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct Category {
    name: String,
    data: HashMap<String, Value>,
    content: Option<Value>,
}

impl Component for Category {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let name = decode_name(&ctx.props().name);
        let data = get_all_data();
        let content = data.get(&name).cloned();

        Self {
            name,
            data,
            content,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.name = decode_name(&ctx.props().name);
        self.content = self.data.get(&self.name).cloned();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { name, content, .. } = self;

        match content {
            None => html! {
                <Redirect<Route> to={Route::NotFound} />
            },
            Some(content) => html! {
                <div class="p-8 flex-1 bg-rust text-black">
                    <h1 class="text-xl font-light pb-4">{ name }</h1>
                    { self.view_entries(content) }
                </div>
            },
        }
    }
}

fn decode_name(name: &str) -> String {
    decode(name).expect("UTF-8").to_string()
}

impl Category {
    fn view_entries(&self, content: &Value) -> Html {
        let arr = content.as_array();

        match arr {
            None => html! {
                <p>{ "#ERR" }</p>
            },
            Some(arr) => html! {
                { for arr.iter().enumerate().map(|(ix, v)| self.view_entry(ix, v)) }
            },
        }
    }

    fn view_entry(&self, ix: usize, item: &Value) -> Html {
        if !item.is_object() {
            panic!("Each entry must be an object.")
        }

        let map = item.as_object().unwrap();
        let title = map
            .get("title")
            .expect("Each entry must have a title.")
            .as_str()
            .expect("Title must be a string.");
        let link_opt = map.get("link");
        let link = link_opt.map(|val| val.as_str().expect("If present, link must be a string."));

        let name_element = match link {
            None => html! {
                <h2 class="text-l pr-2">{ title }</h2>
            },
            Some(link) => html! {
                <a class="flex flex-row items-center mr-2" href={String::from(link)}>
                    <h2 class="text-l pr-2">{ ix + 1 }{ ". " }{ title }</h2>
                    { icons::link_out() }
                </a>
            },
        };

        let valid_soft_tags: Vec<String> = vec![
            String::from("author"),
            String::from("year"),
            String::from("network"),
        ];
        let mut soft_tags: Vec<(&String, String)> = Vec::new();

        for key in map.keys() {
            if !valid_soft_tags.contains(key) {
                continue;
            }

            let val = map
                .get(key)
                .map(|v| v.as_str().unwrap())
                .unwrap_or_else(|| panic!("If present, key {} must contain a string", key));

            soft_tags.push((key, String::from(val)));
        }

        let hard_tags = map.get("tags").map(|val| {
            val.as_array()
                .expect("tags field must be an array.")
                .iter()
                .map(|v| v.as_str().expect("Each tag in tags must be a string."))
        });

        html! {
            <div class="pb-2 flex flex-col items-start">
                <div class="flex flex-row flex-wrap items-center mb-2">
                    { name_element }
                    { for soft_tags.iter().map(|(key, val)|
                        html! {
                            <div class="text-xs py-0.5 px-1.5 mr-2 bg-slate-400 text-white rounded">
                                <span class="">
                                    {key}{": "}
                                </span>
                                <span class="">
                                    {val}
                                </span>
                            </div>
                        }
                    ) }
                </div>
                <div class="flex flex-wrap md:max-w-md max-w-full">
                    { match hard_tags {
                        None => html! {},
                        Some(tags) => html! {
                            { for tags.map(|tag|
                                html! {
                                    <div class="inline-block mb-2 text-xs py-0.5 px-1.5 mr-2 bg-lightgray text-jet rounded">
                                        <span class="">
                                            {"#"}{tag}
                                        </span>
                                    </div>
                                }
                            ) }
                        }
                    } }
                </div>
            </div>
        }
    }
}
