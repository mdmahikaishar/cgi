use std::process::ExitCode;

pub fn help() -> ExitCode {
  println!(
      "
      Usage:

      <init|-i>                           : Initilize repo.
      <uninit|-u>                         : Uninitilze repo.

      <commit|-c>                         : Commit.
      commit <--message|-m> <MESSAGE>     : Commit with message.
      commit <--random|-r>                : Commit random message.

      <add|-a>                            : Add.
      add <...FILES>                      : Add files.

      <origin|-o>                         : Origin.
      origin                              : Get origin address.
      origin <ADDRESS>                    : Add origin.

      <branch|-b>                         : Branch.
      branch                              : Get current branch name.
      branch <BRANCH_NAME>                : Go to branch.
      branch <--create|-c> <BRANCH_NAME>  : Create new branch.
      branch <--list|-l>                  : Branch lists.

      <push|-p>                           : Push.
      push <SRC> <DEST>                   : Push sourse to destination.

      <status|-s>                         : Status.
      status                              : Show stats.

      <log|-l>                            : Log
      log                                 : Show logs.

      <merge|-m>                          : Merge.
      merge                               : Do merge.
  "
  );

  ExitCode::SUCCESS
}
