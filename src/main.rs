use crate::ycd::YCD;
use num_format::{Locale, ToFormattedString};
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket::{get, post, routes, FromForm};
use rocket_dyn_templates::Template;

mod ycd;
mod ycdfile;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
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

    let context = match search_pi(target).await {
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

/*
 * 文字列が自然数として有効かどうかを判定する
*/
fn is_valid_number(s: &str) -> bool {
    s.len() > 0 && s.chars().all(|c| c.is_digit(10))
}

async fn search_pi(target: &str) -> Option<i64> {
    let mut ycd: YCD = YCD::new("pi/1000000");

    loop {
        match ycd.get_next_block() {
            Some(block) => match block.data().find(target) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_number() {
        assert_eq!(is_valid_number("0"), true);
        assert_eq!(is_valid_number("1"), true);
        assert_eq!(is_valid_number("123"), true);
        assert_eq!(is_valid_number("123001112222"), true);
        assert_eq!(is_valid_number("123.0"), false);
        assert_eq!(is_valid_number("123.1"), false);
        assert_eq!(is_valid_number(""), false);
        assert_eq!(is_valid_number("\n"), false);
        assert_eq!(is_valid_number("-123"), false);
        assert_eq!(is_valid_number("123a"), false);
        assert_eq!(is_valid_number("a123"), false);
        assert_eq!(is_valid_number("123 "), false);
        assert_eq!(is_valid_number(" 123"), false);
        assert_eq!(is_valid_number("123\n"), false);
    }

    #[tokio::test]
    async fn test_search_pi() {
        assert_eq!(search_pi(&"1415").await, Some(1));
        assert_ne!(search_pi(&"14150").await, Some(1));

        //5779458151|30927562832084531584  1,000,000桁保有ファイルの区切り目
        assert_eq!(search_pi(&"5779458151").await, Some(999991));
        assert_eq!(
            search_pi(&"577945815130927562832084531584").await,
            Some(999991)
        );
        assert_eq!(search_pi(&"130927562832084531584").await, Some(1000000));
        assert_eq!(search_pi(&"30927562832084531584").await, Some(1000001));
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
