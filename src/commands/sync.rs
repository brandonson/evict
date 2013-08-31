use file_manager::*;
use merge::mergeIssues;
use vcs_status;
use config;

pub fn syncIssues(_:~[~str], _:config::Config) -> int {
  let branchOpt = vcs_status::currentBranch();
  do branchOpt.map_move_default(2) |branch| {
    let incoming = readCommittedIssues();
    let mergeInto = readCommittableIssues(branch);
    
    let merged = mergeIssues(incoming, mergeInto);

    let success1 = writeCommittableIssues(branch, merged);
    let success2 = commitIssues(merged);
    if(success1 && success2){0}else{1}
  }
}
