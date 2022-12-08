use yew::prelude::*;
use lazy_static::lazy_static;
use regex::*;
use itertools::{Itertools, EitherOrBoth};

pub(super) fn task_value_as_html(value: &str) -> Html {
    html! {
        <>
        // { for value.lines().map(|l| html!(<>{l.to_string()}<br/></>)) }
        { for value.lines().map(|l| line_to_html(l)) }
        </>
    }
}

fn line_to_html(value: &str) -> Html {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"https?://[\w!\?/\+\-_~=;\.,\*&@#\$%\(\)'\[\]]+"#).unwrap();
    }

    let normal = RE.split(value).map(move |value| html!{ <>{ value }</> }).collect::<Vec<_>>();
    let urls = RE.find_iter(value).map(move |mat| mat.as_str().to_string()); //.collect::<Vec<_>>();
    let links = urls.map(move |url| html!{ <a href={ url.clone() }>{ url }</a> }).collect::<Vec<_>>();

    let mut nodes = vec![];
    for pair in normal.into_iter().zip_longest(links.into_iter()) {
        use EitherOrBoth::*;
        match pair {
            Both(normal, link) => {
                nodes.push(normal);
                nodes.push(link);
            },
            Left(normal) => {
                nodes.push(normal);
            },
            Right(link) => {
                nodes.push(link);
            },
        }
    }

    html! {
        <>
        { for nodes } <br/>
        </>
    }
}

