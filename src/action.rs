#![allow(dead_code)]

use iced::Task;

pub struct Action<Message, Request> {
    pub task: Task<Message>,
    pub requests: Vec<Request>,
}

impl<Message, Request> Action<Message, Request> {
    pub fn none() -> Self {
        Self {
            task: Task::none(),
            requests: Vec::new(),
        }
    }

    pub fn task(task: Task<Message>) -> Self {
        Self {
            task,
            requests: Vec::new(),
        }
    }

    pub fn requests<I>(requests: I) -> Self
    where
        I: IntoIterator<Item = Request>,
    {
        Self {
            task: Task::none(),
            requests: requests.into_iter().collect(),
        }
    }

    pub fn with_requests(self, requests: Vec<Request>) -> Self {
        Self { requests, ..self }
    }
}

impl<Message, Request> From<Task<Message>> for Action<Message, Request> {
    fn from(value: Task<Message>) -> Self {
        Self {
            task: value,
            requests: Vec::new(),
        }
    }
}
