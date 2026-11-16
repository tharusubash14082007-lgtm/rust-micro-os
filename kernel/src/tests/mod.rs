
pub fn run_tests() -> (u32, u32) {
    let mut passed = 0;
    let mut failed = 0;

    // TC1: Two-task scheduling
    if test_two_tasks() {
        passed += 1;
    } else {
        failed += 1;
    }

    // TC2: Multiple-task scheduling
    if test_multiple_tasks() {
        passed += 1;
    } else {
        failed += 1;
    }

    // TC3: Queue boundary
    if test_queue_boundary() {
        passed += 1;
    } else {
        failed += 1;
    }

    (passed, failed)
}

fn test_two_tasks() -> bool {
    use crate::task::TaskManager;

    let mut manager = TaskManager::new();

    manager.create_task(1, "A").is_ok()
        && manager.create_task(2, "B").is_ok()
        && manager.tick().is_some()
}

fn test_multiple_tasks() -> bool {
    use crate::task::TaskManager;

    let mut manager = TaskManager::new();

    for id in 1..=5 {
        if manager.create_task(id, "Task").is_err() {
            return false;
        }
    }

    manager.tick().is_some()
}

fn test_queue_boundary() -> bool {
    use crate::task::TaskManager;

    let mut manager = TaskManager::new();

    for id in 1..=16 {
        if manager.create_task(id, "Task").is_err() {
            return false;
        }
    }

    manager.create_task(17, "Extra").is_err()
}
