
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
pub const TIME_QUANTUM: u8 = 3;

pub struct TaskManager {
    pub ready_queue: [Option<Task>; MAX_TASKS],
    pub front: usize,
    pub rear: usize,
    pub count: usize,
    pub current_task: Option<Task>,
    pub quantum_remaining: u8,
}

impl TaskManager {
    pub const fn new() -> Self {
        Self {
            ready_queue: [None; MAX_TASKS],
            front: 0,
            rear: 0,
            count: 0,
            current_task: None,
            quantum_remaining: 0,
        }
    }

    fn enqueue(&mut self, task: Task) -> Result<(), Task> {
        if self.count >= MAX_TASKS {
            return Err(task);
        }

        self.ready_queue[self.rear] = Some(task);
        self.rear = (self.rear + 1) % MAX_TASKS;
        self.count += 1;

        Ok(())
    }

    pub fn create_task(
        &mut self,
        id: u32,
        name: &'static str,
    ) -> Result<(), Task> {
        self.create_task_with_priority(id, name, 1)
    }

    pub fn create_task_with_priority(
        &mut self,
        id: u32,
        name: &'static str,
        priority: u8,
    ) -> Result<(), Task> {
        let total = self.count
            + if self.current_task.is_some() { 1 } else { 0 };

        let task = Task {
            id,
            name,
            state: TaskState::Ready,
            priority,
        };

        if total >= MAX_TASKS || priority == 0 {
            return Err(task);
        }

        self.enqueue(task)
    }

    pub fn next_task(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        let mut best_offset = 0;
        let mut best_priority = 0;

        for offset in 0..self.count {
            let index = (self.front + offset) % MAX_TASKS;

            if let Some(task) = self.ready_queue[index] {
                if task.priority > best_priority {
                    best_priority = task.priority;
                    best_offset = offset;
                }
            }
        }

        let selected_index =
            (self.front + best_offset) % MAX_TASKS;

        let selected = self.ready_queue[selected_index].take();

        for offset in best_offset..self.count - 1 {
            let current = (self.front + offset) % MAX_TASKS;
            let next = (self.front + offset + 1) % MAX_TASKS;

            self.ready_queue[current] = self.ready_queue[next].take();
        }

        self.rear = (self.rear + MAX_TASKS - 1) % MAX_TASKS;
        self.ready_queue[self.rear] = None;
        self.count -= 1;

        selected
    }

    pub fn start_next_task(&mut self) -> Option<Task> {
        if let Some(task) = self.current_task {
            return Some(task);
        }

        let mut task = self.next_task()?;

        task.state = TaskState::Running;
        self.quantum_remaining = TIME_QUANTUM;
        self.current_task = Some(task);

        Some(task)
    }

    pub fn tick(&mut self) -> Option<Task> {
        if self.current_task.is_none() {
            self.start_next_task()?;
        }

        if self.quantum_remaining > 0 {
            self.quantum_remaining -= 1;
        }

        if self.quantum_remaining == 0 {
            if let Some(mut task) = self.current_task.take() {
                task.state = TaskState::Ready;

                if self.enqueue(task).is_err() {
                    self.current_task = Some(task);
                    return self.current_task;
                }
            }

            self.start_next_task();
        }

        self.current_task
    }

    pub fn complete_current_task(&mut self) -> Option<Task> {
        let mut task = self.current_task.take()?;

        task.state = TaskState::Terminated;
        self.quantum_remaining = 0;

        Some(task)
    }
}
