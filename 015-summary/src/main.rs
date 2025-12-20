// =======================================================
// Rust学習100本ノック 015
// テーマ: Option / Result / enum 総まとめ
//
// 【目的】
// - Option / Result / enum を混ぜて設計する
// - 「何を Option にするか」「何を Result にするか」を判断する
// - 状態遷移を enum + match で表現する
//
// 【制約】
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止
// - 状態管理は enum で行う
// - エラーは Result で表現する
// - match は exhaustively 書く
//
// =======================================================
#[derive(Debug, Clone, PartialEq)]
enum TaskState {
    Ready,
    Running,
    Done(i32),
    Failed(TaskError),
}
#[derive(Debug, Clone, PartialEq)]
enum TaskError{
    AlreadyRunning,
    DivisionByZero,
}

struct Task{
    state: TaskState,
}

impl Task {
    fn new() -> Self{
        Self { state: TaskState::Ready }
    }
    
    fn start(&mut self) -> Result<(), TaskError>{
        match self.state {
            TaskState::Ready => {
                self.state = TaskState::Running;
                Ok(())
            },
            TaskState::Running
            | TaskState::Done(_)
            | TaskState::Failed(_) => Err(TaskError::AlreadyRunning),
        }
    }
    
    fn finish(&mut self, value: i32) -> Result<(), TaskError> {
        match self.state {
            TaskState::Running => {
                self.state = TaskState::Done(value);
                Ok(())
            },
            TaskState::Ready
            | TaskState::Done(_)
            | TaskState::Failed(_) => Err(TaskError::AlreadyRunning),
        }
    }
    
    fn divide(&mut self, divisor: i32) -> Result<i32, TaskError> {
        match self.state{
            TaskState::Running => {
                match divisor {
                    0 => {
                        self.state = TaskState::Failed(TaskError::DivisionByZero);
                        Err(TaskError::DivisionByZero)
                    },
                    _ => Ok(100 / divisor),
                }
            },
            TaskState::Ready
            | TaskState::Done(_)
            | TaskState::Failed(_) => Err(TaskError::AlreadyRunning),
        }
    }
    
    fn take_result(&mut self) -> Option<i32> {
        match self.state {
            TaskState::Done(n) => {
                self.state = TaskState::Ready;
                Some(n)
            },
            _ => None
        }
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_is_ready() {
        let t = Task::new();
        match t.state {
            TaskState::Ready => {}
            _ => panic!("new task must be Ready"),
        }
    }

    #[test]
    fn start_from_ready_succeeds() {
        let mut t = Task::new();
        assert!(t.start().is_ok());

        match t.state {
            TaskState::Running => {}
            _ => panic!("task must be Running after start"),
        }
    }

    #[test]
    fn start_twice_fails_and_state_unchanged() {
        let mut t = Task::new();
        assert!(t.start().is_ok());
        assert!(t.start().is_err());

        match t.state {
            TaskState::Running => {}
            _ => panic!("state must remain Running"),
        }
    }

    #[test]
    fn finish_only_works_when_running() {
        let mut t = Task::new();
        assert!(t.finish(10).is_err());

        t.start().unwrap();
        assert!(t.finish(10).is_ok());

        match t.state {
            TaskState::Done(v) => assert_eq!(v, 10),
            _ => panic!("task must be Done after finish"),
        }
    }

    #[test]
    fn divide_when_running_returns_result() {
        let mut t = Task::new();
        t.start().unwrap();

        let r = t.divide(2);
        assert_eq!(r, Ok(50));

        match t.state {
            TaskState::Running => {}
            _ => panic!("state must remain Running after successful divide"),
        }
    }

    #[test]
    fn divide_by_zero_moves_to_failed() {
        let mut t = Task::new();
        t.start().unwrap();

        let r = t.divide(0);
        assert!(matches!(r, Err(TaskError::DivisionByZero)));

        match t.state {
            TaskState::Failed(TaskError::DivisionByZero) => {}
            _ => panic!("state must be Failed(DivisionByZero)"),
        }
    }

    #[test]
    fn take_result_from_done_returns_value_and_resets_state() {
        let mut t = Task::new();
        t.start().unwrap();
        t.finish(42).unwrap();

        let v = t.take_result();
        assert_eq!(v, Some(42));

        match t.state {
            TaskState::Ready => {}
            _ => panic!("state must return to Ready after take_result"),
        }
    }

    #[test]
    fn take_result_from_non_done_returns_none() {
        let mut t = Task::new();

        assert_eq!(t.take_result(), None);

        t.start().unwrap();
        assert_eq!(t.take_result(), None);

        match t.state {
            TaskState::Running => {}
            _ => panic!("state must remain unchanged"),
        }
    }
}