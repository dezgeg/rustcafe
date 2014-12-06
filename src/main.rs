#![feature(macro_rules)]
#![feature(slicing_syntax)]
extern crate serialize;
extern crate hyper;

use serialize::json;
use serialize::json::Json;
use hyper::Url;
use hyper::client::Request;

static API_URL: &'static str = "http://messi.hyyravintolat.fi/publicapi";

macro_rules! try_result(
    ($expr:expr) => {
        match $expr {
            Ok(r) => r,
            _ => return None,
        }
    }
)

macro_rules! try_option(
    ($expr:expr) => {
        match $expr {
            Some(r) => r,
            _ => return None,
        }
    }
)

trait JsonWalk {
    fn as_string(&self) -> Option<String>;
    fn as_array(&self) -> Option<json::Array>;
    fn as_uint(&self) -> Option<uint>;

    fn get(&self, key: &str) -> Self;
    fn nth(&self, index: uint) -> Self;
}

impl JsonWalk for Option<json::Json> {
    fn as_string(&self) -> Option<String> {
        match *self {
            Some(Json::String(ref s)) => Some(s.clone()),   // TODO: clone
            _ => None,
        }
    }

    fn as_array(&self) -> Option<json::Array> {
        match *self {
            Some(Json::Array(ref a)) => Some(a.clone()),   // TODO: clone
            _ => None,
        }
    }

    // TODO: Why don't these work?
    // fn as_uint(&self) -> Option<uint> {
    //     return self.and_then(|some| some.as_u64().map(|v| v as uint));
    // }
    //
    // fn get(&self, key: &str) -> Option<json::Json> {
    //     return self.and_then(|some| some.as_object().and_then(|obj| obj.get(key))).map(|ptr| *ptr);
    // }
    //
    // fn nth(&self, index: uint) -> Option<json::Json> {
    //     return self.and_then(|some| some.as_array().and_then(|arr| arr.get(index))).map(|ptr| *ptr);
    // }

    fn as_uint(&self) -> Option<uint> {
        match *self {
            Some(Json::U64(n)) => Some(n as uint),
            _ => None,
        }
    }

    fn get(&self, key: &str) -> Option<json::Json> {
        match *self {
            Some(Json::Object(ref o)) => o.get(key).map(|ptr| ptr.clone()), // TODO: clone
            _ => None,
        }
    }

    fn nth(&self, index: uint) -> Option<json::Json> {
        match *self {
            Some(Json::Array(ref a)) => a.get(index).map(|ptr| ptr.clone()), // TODO: clone
            _ => None,
        }
    }
}

fn restaurants() -> Option<Vec<(uint, String)>> {
    let url = try_result!(Url::parse(format!("{}/restaurants/", API_URL)[]));
    let req = try_result!(Request::get(url));
    let response = req.start().unwrap().send().unwrap().read_to_string().unwrap();

    let o = json::from_str(response[]).ok();
    let restaurants = try_option!(o.get("data").as_array());
    let mut v = Vec::new();
    for o in restaurants.iter() {
        let id = try_option!(Some(o.clone()).get("id").as_uint());          // TODO: clone
        let name = try_option!(Some(o.clone()).get("name").as_string());    // TODO: clone
        v.push((id, name.to_string()));
    };
    Some(v)
}

fn menus(id: u64) -> Option<Vec<(String, Vec<String>)>> {
    let url = try_result!(Url::parse(format!("{}/restaurant/{}", API_URL, id)[]));
    let req = Request::get(url).unwrap();
    let response = req.start().unwrap().send().unwrap().read_to_string().unwrap();

    let o = json::from_str(response[]).ok();
    let menus = try_option!(o.get("data").as_array());
    let res : Option<Vec<(String, Vec<String>)>> = menus.iter().map(|menu| {
        let opt_date = Some(menu.clone()).get("date_en").as_string();
        let opt_foods : Option<Vec<String>> = try_option!(Some(menu.clone()).get("data").as_array())
            .iter().map(|o| Some(o.clone()).get("name").as_string()).collect();
        Some((try_option!(opt_date), try_option!(opt_foods)))
    }).collect();
    res
}

fn main() {
    println!("{}", restaurants());
    println!("{}", menus(1));
}

