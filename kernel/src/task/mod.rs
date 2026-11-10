#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub state: TaskState,
    pub priority: u8,
}

pub const MAX_TASKS: usize = 16;

pub struct TaskManager {
    pub ready_queue: [Option<Task>; MAX_TASKS],
    pub front: usize,
    pub rear: usize,
    pub count: usize,
    pub current_task: Option<Task>,
}

impl TaskManager {
    pub const fn new() -> Self {
        Self {
            ready_queue: [None; MAX_TASKS],
            front: 0,
            rear: 0,
            count: 0,
            current_task: None,
        }
    }

    pub fn create_task(
        &mut self,
        id: u32,
        name: &'static str,
    ) -> Result<(), Task> {
        if self.count == MAX_TASKS {
            return Err(Task {
                id,
                name,
                state: TaskState::Ready,
                priority: 1,
            });
        }

        let task = Task {
            id,
            name,
            state: TaskState::Ready,
            priority: 1,
        };

        self.ready_queue[self.rear] = Some(task);
        self.rear = (self.rear + 1) % MAX_TASKS;
        self.count += 1;

        Ok(())
    }

    pub fn next_task(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        let task = self.ready_queue[self.front].take();

        self.front = (self.front + 1) % MAX_TASKS;
        self.count -= 1;

        task
    }
}





