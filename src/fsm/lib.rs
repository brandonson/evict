#[link(name = "fsm", vers="0.1", author="Brandon Sanderson")];
#[crate_type="lib"];
pub enum NextState<S,I>{
  ChangeState(Executor<S,I>, S),
  Continue(S),
  End(S)
}

pub type Executor<S,I> = ~fn(S, I) -> NextState<S,I>;

pub struct StateMachine<S, I> {
  priv nextExecutor:Option<Executor<S,I>>,
  ///State should only ever be None when the process
  ///method is running.
  priv currentState:Option<S>
}

impl<S,I> StateMachine<S,I>{
  pub fn process(&mut self, pVal:I){
    let mut nStateOpt:Option<NextState<S,I>> = None;
    let oldState = self.currentState.take_unwrap();
    match self.nextExecutor {
      None => {} 
      Some(ref executor) => 
        nStateOpt = Some((*executor)(oldState,pVal))
    };
    match nStateOpt {
      None => {}
      Some(nState) =>
        match nState{
          ChangeState(exec, state) => {
              self.nextExecutor = Some(exec);
              self.currentState = Some(state);
            }
          Continue(state) => 
            self.currentState = Some(state),
          End(state) => {
            self.nextExecutor = None;
            self.currentState = Some(state);
          } 
        }
    }
  }
  pub fn isComplete(&self) -> bool{
    self.nextExecutor.is_none()
  }
  pub fn consumeToState(self) -> S {
    self.currentState.unwrap()
  }
  pub fn new(exec:Executor<S,I>, state:S) -> StateMachine<S,I>{
    StateMachine{nextExecutor:Some(exec), currentState:Some(state)}
  }
}
//provide a way to get state out while processing
//if state is cloneable
impl<S:Clone, I> StateMachine<S,I>{
  pub fn copyCurrentState(&self) -> S {
    self.currentState.clone().unwrap()
  }
}

#[test]
fn simpleStorage(){
  let storer:Executor<Option<int>, int> = |state:Option<int>, input:int| -> NextState<Option<int>,int>{
    if (input == 0){
      End(state)
    }else if(input < 0){
      Continue(None)
    }else{
      Continue(Some(input))
    }
  };
  let mut stateM:StateMachine<Option<int>, int> = StateMachine::new(storer, None);
  assert!(stateM.copyCurrentState() == None);
  stateM.process(2);
  assert!(stateM.copyCurrentState() == Some(2));
  stateM.process(0);
  assert!(stateM.isComplete());
}
