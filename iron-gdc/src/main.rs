#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("🔧🧲 serving iron http on http://localhost:3000 🤖🔩");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title=GCD Calc</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="type" name="n"/>
            <button type="submit">Compute! 🖥️</button>
        </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    dbg!(&form_data);

    let unparse_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' param\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = vec![];
    for n in unparse_numbers {
        match u64::from_str(&n) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("value for 'n' param not a number\n"));
                return Ok(response);
            }
            Ok(num) => { numbers.push(num) }
        }
    }

    let mut divisor = numbers[0];

    for x in &numbers[1..] {
        divisor = gcd(divisor, *x);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of {:?} is {}",
                numbers, divisor));
    
    Ok(response)

}


fn gcd(mut x: u64, mut y: u64) -> u64 {
    assert!(x != 0, y != 0);
    loop {
        if x > y {
            let temp_var = y;
            y = x;
            x = temp_var;
        }
        y %= x;
        if y == 0 { break }
    }
    x
}
