use std::*;

enum VCS{
  Git
}

impl VCS {
  fn currentBranchCmdOutput(&self) -> ~str{
    match self {
      &Git =>
        str::from_utf8(run::process_output("git", [~"rev-parse", 
                                                    ~"--abbrev-ref", 
                                                    ~"HEAD"]).output)
    }
  }
}
fn currentVCS() -> VCS{
  Git //TODO actually detect a VCS
}

pub fn currentBranch() -> Option<~str> {
  let output = currentVCS().currentBranchCmdOutput(); 
  let mut line:~str = ~"";
  for branch in output.any_line_iter() {
    line = branch.to_owned();
    break;
  }
  if (line == ~"") {
    None
  }else{
    Some(line)
  }
}

