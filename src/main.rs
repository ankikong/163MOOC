#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate reqwest;
extern crate regex;

use reqwest::header;
use reqwest::blocking::Client;
use regex::Regex;
use std::thread;
use std::time;
use std::io;

struct headerGetter {
    value:String
}

fn signForPPT(client:&Client, content:&String, csrf:&str, hder:&headerGetter) {
    let re = Regex::new(r"contentType=3.+?id=(\d+).+?").unwrap();
    let url = format!("https://www.icourse163.org/web/j/courseBean.saveMocContentLearn.rpc?csrfKey={}", csrf);
    for i in re.captures_iter(content) {
        thread::sleep(time::Duration::from_secs(3));
        let id = i.get(1).unwrap().as_str();
        println!("{}", id);
        let data = format!("dto={{\"unitId\":{},\"pageNum\":1,\"finished\":true,\"contentType\":3}}", id);
        let rs:serde_json::Value = client.post(url.as_str())
              .body(data)
              .headers(hder.getHeader("application/x-www-form-urlencoded"))
              .send()
              .expect("request failed")
              .json()
              .expect("json format error");
        if rs.get("result").expect("failed") == true {
            println!("sign success")
        } else {
            println!("{}", rs.get("message").unwrap())
        }
    }
}

fn signForComment(client:&Client, content:&String, csrf:&str, hder:&headerGetter) {
    let re = Regex::new(r"contentType=6.+?id=(\d+).+?").unwrap();
    let url = format!("https://www.icourse163.org/web/j/courseBean.saveMocContentLearn.rpc?csrfKey={}", csrf);
    for i in re.captures_iter(content) {
        thread::sleep(time::Duration::from_secs(3));
        let id = i.get(1).unwrap().as_str();
        println!("{}", id);
        let data = format!("dto={{\"unitId\":{},\"finished\":true,\"contentType\":6}}", id);
        let rs:serde_json::Value = client.post(url.as_str())
              .body(data)
              .headers(hder.getHeader("application/x-www-form-urlencoded"))
              .send()
              .expect("request failed")
              .json()
              .expect("json format error");
        if rs.get("result").expect("failed") == true {
            println!("sign success")
        } else {
            println!("{}", rs.get("message").unwrap())
        }
    }
}

fn signForRichtext(client:&Client, content:&String, csrf:&str, hder:&headerGetter) {
    let re = Regex::new(r"contentType=4.+?id=(\d+).+?").unwrap();
    let url = format!("https://www.icourse163.org/web/j/courseBean.saveMocContentLearn.rpc?csrfKey={}", csrf);
    for i in re.captures_iter(content) {
        thread::sleep(time::Duration::from_secs(3));
        let id = i.get(1).unwrap().as_str();
        println!("{}", id);
        let data = format!("dto={{\"unitId\":{},\"finished\":true,\"contentType\":4}}", id);
        let rs:serde_json::Value = client.post(url.as_str())
              .body(data)
              .headers(hder.getHeader("application/x-www-form-urlencoded"))
              .send()
              .expect("request failed")
              .json()
              .expect("json format error");
        if rs.get("result").expect("failed") == true {
            println!("sign success")
        } else {
            println!("{}", rs.get("message").unwrap())
        }
    }
}

impl headerGetter {
    pub fn getHeader(&self, cType:&str) -> header::HeaderMap {
        let mut hder = header::HeaderMap::new();
        let cookie = self.value.as_str();
        hder.append("origin", "https://www.icourse163.org".parse().unwrap());
        hder.append("referer", "https://www.icourse163.org/spoc/learn/HNNY-1451765171?tid=1452238457".parse().unwrap());
        hder.append("content-type", cType.parse().unwrap());
        hder.append("cookie", cookie.parse().unwrap());
        hder
    }
    pub fn setCookie(&mut self, cookie:String) {
        if cookie.ends_with("\n") {
            self.value = cookie[0..cookie.len()-1].to_string()
        } else {
            self.value = cookie
        }
    }

    pub fn getCookie<'a>(&'a self) -> &'a String {
        &self.value
    }
}

fn main() {

    let mut cookie = String::new();
    println!("input cookie");
    io::stdin().read_line(&mut cookie).expect("input cookie!!!");

    let mut hdgen = headerGetter{value:String::new()};

    hdgen.setCookie(cookie);

    let csrfRe = Regex::new(r"NTESSTUDYSI=(.+?);").unwrap();
    let csrf = csrfRe.captures(hdgen.getCookie()).expect("cookie format error").get(1).expect("no csrf found").as_str();
    println!("{}", csrf);

    let cl = Client::new();
    let tidReg = Regex::new(r"tid=(\d+)").unwrap();

    loop {
        let mut cUrl = String::new();
        println!("input course url");
        io::stdin().read_line(&mut cUrl).expect("input course url");
        let tid = tidReg.captures(cUrl.as_str()).expect("tid not in url").get(1).expect("tid not in url").as_str();
        println!("get tid:{}", tid);
        
        let rs = cl.post("https://www.icourse163.org/dwr/call/plaincall/CourseBean.getLastLearnedMocTermDto.dwr")
          .headers(hdgen.getHeader("text/plain"))
          .body(format!("callCount=1\nscriptSessionId=${{scriptSessionId}}190\nc0-scriptName=CourseBean\nc0-methodName=getLastLearnedMocTermDto\nc0-id=0\nc0-param0=number:{}\nbatchId=1583152407811", tid))
          .send()
          .unwrap()
          .text()
          .unwrap();

        println!("start signing ppt");
        signForPPT(&cl, &rs, csrf, &hdgen);
        println!("start signing comment");
        signForComment(&cl, &rs, csrf, &hdgen);
        println!("start signing richtext");
        signForRichtext(&cl, &rs, csrf, &hdgen);

    }

}
