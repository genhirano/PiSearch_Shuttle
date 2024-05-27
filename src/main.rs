use crate::ycd::YCD;
use num_format::{Locale, ToFormattedString};
use rocket::form::Form;
use rocket::serde::Serialize;
use rocket::{get, post, routes, FromForm};
use rocket_dyn_templates::Template;

mod ycd;
mod ycdfile;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, search])
        .attach(Template::fairing());
    Ok(rocket.into())
}

#[derive(Serialize)]
struct Context {
    result_target: String,
    result_pos: String,
}

#[get("/")]
async fn index() -> Template {
    let context = Context {
        result_target: "".to_string(),
        result_pos: "".to_string(),
    };
    Template::render("index", &context)
}

#[derive(FromForm)]
struct PostData {
    target: String,
}

#[post("/", data = "<arg_target>")]
async fn search(arg_target: Form<PostData>) -> Template {
    let target = &arg_target.target;

    if !is_valid_number(target.as_str()) || target.is_empty() {
        let context = Context {
            result_target: "".to_string(),
            result_pos: "".to_string(),
        };
        return Template::render("index", &context);
    }

    let context = match search_pi(target.clone()).await {
        Some(pos) => {
            let formatted_result = pos.to_formatted_string(&Locale::en);
            Context {
                result_target: target.clone(),
                result_pos: formatted_result,
            }
        }
        None => Context {
            result_target: target.clone(),
            result_pos: "not found...".to_string(),
        },
    };

    return Template::render("index", &context);
}

fn is_valid_number(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

async fn search_pi(target: String) -> Option<i64> {
    let mut ycd: YCD = YCD::new("pi/1000000");

    loop {
        match ycd.get_next_block() {
            Some(block) => match block.data().find(target.as_str()) {
                Some(pos) => {
                    println!(
                        "'{}' found at position {}",
                        target,
                        pos as i64 + block.pos()
                    );
                    return Some(pos as i64 + block.pos());
                }
                None => continue,
            },
            None => {
                return None;
            }
        }
    }
}

//1415926535897932384

//Last Decimal Digits:  Pi
//0315614033 3212728491 9441843715 0696552087 5424505989  :  999,950
//5678796130 3311646283 9963464604 2209010610 5779458151  :  1,000,000

//8バイト（19桁）読むと、ファイルの有効データ末端を超えて読むため、このファイルの有効桁より後の桁は切る
//Long readEndDigit = this.currentBlock * 19;
//if (readEndDigit > this.digit_Length) {
//このファイルに格納されている桁数よりオーバーして読み込んだ場合はオーバー分を切り捨てる
//Long over = readEndDigit - this.digit_Length;
//numStr = numStr.substring(0, (int) (19 - over));  //左を残し、右からオーバー分を切り捨てる
//}
