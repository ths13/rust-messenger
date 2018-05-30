extern crate firebase;

use self::firebase::{Firebase, Response};
use super::{error, message};

pub fn get_user(user_id: &str, firebase: &Firebase) -> Result<Response, error::ServerError>{
    let user = match firebase.at(&format!("/users/{}", user_id)) {
        Err(err)            => { return Err(error::handleParseError(err)) }
        Ok(user)            => user
    };

    let res = match user.get() {
        Err(err)    => { return Err(error::handleReqErr(err)) }
        Ok(res)     => {
            if res.body == "null" {
                return Err(error::ServerError::InvalidUserId)
            }
            res
        }
    };

    Ok(res)
}

pub fn get_user_threads(user_id: &str, start_index: u32, end_index: u32, firebase: &Firebase)
                        -> Result<Response, error::ServerError>
{
    let threads = match firebase.at(&format!("/users/{}/threads", user_id)) {
        Err(err)    => { return Err(error::handleParseError(err)) }
        Ok(user)    => user
    };

    let range = end_index - start_index;
    let res = match threads.order_by("\"timestamp\"").start_at(start_index).limit_to_first(range).get() {
        Err(err)    => { return Err(error::handleReqErr(err)) }
        Ok(threads) => threads
    };
    Ok(res)
}

pub fn update_user_threads(user_id: &str, thread_id: &str, new_message: message::Message, firebase: &Firebase)
    -> Result<Response, error::ServerError>
{
    let thread = match firebase.at(&format!("/users/{}/threads/{}", user_id, thread_id)) {
        Err(err)    => { return Err(error::handleParseError(err)) }
        Ok(user)    => user
    };
    let msg = message::new_message_to_user_JSON(new_message);
    let res = match thread.update(&msg) {
        Err(err)    => { return Err(error::handleReqErr(err)) }
        Ok(thread)  => { thread }
    };

    Ok(res)

}