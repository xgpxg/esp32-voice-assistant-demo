use serde_json::{json, Value};

pub mod llm;
pub mod mic;
pub mod speaker;
pub mod stt;
pub mod tts;

#[derive(Debug, PartialEq)]
enum WsEvent<T> {
    Connected,
    Disconnected,
    TaskStarted,
    ResultGenerated(T),
    TaskFinished,
    #[allow(unused)]
    TaskFailed,
}

trait Action {
    fn run_task(task_id: &str) -> String;
    fn run_task_with_payload(task_id: &str, payload: Value) -> String {
        let json = json!({
            "header": {
                "action": "run-task",
                "task_id": task_id,
                "streaming": "duplex"
            },
            "payload": payload
        });
        json.to_string()
    }
    fn finish_task(task_id: &str) -> String {
        let json = json!({
            "header": {
                "action": "finish-task",
                "task_id": task_id,
                "streaming": "duplex"
            },
             "payload": {
                "input": {}
            }
        });
        json.to_string()
    }
}
