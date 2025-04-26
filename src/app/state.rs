


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    window_pos: (i32, i32),
    window_size: (u32, u32),
    state_machine: StateMachine,
}

impl State {
    pub fn new() -> Self {
        State {
            window_pos: (0, 0),
            window_size: (800, 600),
            state_machine: StateMachine::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StateMachine {
    Init,
    Running,
    Shutdown,
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine::Init
    }
}

impl std::fmt::Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachine::Init => write!(f, "Init"),
            StateMachine::Running => write!(f, "Running"),
            StateMachine::Shutdown => write!(f, "Shutdown"),
        }
    }
}

impl std::fmt::Debug for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachine::Init => write!(f, "Init"),
            StateMachine::Running => write!(f, "Running"),
            StateMachine::Shutdown => write!(f, "Shutdown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert_eq!(state.window_pos, (0, 0));
        assert_eq!(state.window_size, (800, 600));
        assert_eq!(state.state_machine, StateMachine::Init);
    }
}


