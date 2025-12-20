// =======================================================
// Rust学習100本ノック 011
// テーマ: if を match に置き換える（match 強制）
//
// 【目的】
// - if / if let に逃げず、match で全分岐を書く
// - enum の exhaustiveness を身体に入れる
// - 「分岐の抜け」をコンパイルエラーで潰す
//
// 【制約】
// - if / if let / while let 使用禁止
// - unwrap / expect / panic! 禁止
// - match は必ず exhaustively 書く（_ 乱用禁止）
//
// 【狙い】
// - enum 設計 → match → 安全性、の流れを体感
// - OS / VM / パーサで必要な分岐思考の基礎
// =======================================================
enum State {
    Init,
    Running,
    Done,
}

enum Event {
    Start,
    Finish,
    Reset,
}

fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Init, Event::Start) => State::Running,
        (State::Running, Event::Finish) => State::Done,
        (State::Done, Event::Reset) => State::Init,
        (s, _) => s, 
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_start_goes_to_running() {
        let next = transition(State::Init, Event::Start);
        assert!(matches!(next, State::Running));
    }

    #[test]
    fn running_finish_goes_to_done() {
        let next = transition(State::Running, Event::Finish);
        assert!(matches!(next, State::Done));
    }

    #[test]
    fn done_reset_goes_to_init() {
        let next = transition(State::Done, Event::Reset);
        assert!(matches!(next, State::Init));
    }

    #[test]
    fn invalid_transition_keeps_state_init() {
        let next = transition(State::Init, Event::Finish);
        assert!(matches!(next, State::Init));
    }

    #[test]
    fn invalid_transition_keeps_state_running() {
        let next = transition(State::Running, Event::Reset);
        assert!(matches!(next, State::Running));
    }

    #[test]
    fn invalid_transition_keeps_state_done() {
        let next = transition(State::Done, Event::Start);
        assert!(matches!(next, State::Done));
    }
}