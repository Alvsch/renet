use crate::DisconnectReason;

impl actix::Message for DisconnectReason {
    type Result = ();
}
