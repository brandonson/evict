Evict-BT is a issue tracker written in Rust that integrates loosely with git.  In the future, 
it will hopefully support additional version control systems.

Currently Evict-BT has minimal support for just about everything,
but the hope is that while updates may cause strange state, they
won't break any issues currently being tracked.

<dl>
<dt>Features supported by Evict-BT:</dt>
<dd> Evict-BT repo initialization/de-initialization -- `evict init/clear`</dd>
<dd> Issue creation -- `evict create`</dd>
<dd> Issue deletion -- `evict delete` (limited to non-committed issues)</dd>
<dd> Branch-based issue location</dd>
<dd> Issue committing -- `evict sync`</dd>
<dd> Merging across branches -- `evict merge`</dd>
<dd> Issue authors</dd>
<dd> Default issue author -- `evict default-author`</dd>
<dd> Issue listing -- `evict list`</dd>
<dd> Issue statuses -- `evict set-status`</dd>
<dd> Default status for new issues -- `evict default-status`</dd>
<dd> User-defined issue states -- `evict new-status`</dd>
<dd> Issue commenting -- `evict comment`</dd>
<dd> List filtering by status</dd>

<dt>Features to be supported:</dt>
<dd> Default issue statuses for new repos.</dd>
<dd> Issue status removal</dd>
<dd> Assigning issues</dd>
<dd> Issue tags</dd>
<dd> More filter options for `evict list`</dd>
<dl>
Installation
------------
Evict-BT is written in Rust (https://github.com/mozilla/rust/).  To install,
you will need a working version of rust from the latest master branch.

NOTE: The following probably only works on linux and other systems where /usr/local/bin
is on the system path.

Clone the git repository into a location of your choice.  Then cd into the root directory
of the repo.

Run the `build` script.  The evict binary should now be located in <current-dir>/bin.

Run the `install` script.  This copies the binary into /usr/local/bin and therefore
probably needs root user privileges.  (It doesn't seem to print anything if it fails,
either, which needs looking into)

Run `evict list`.  If you get a bunch of output that looks like issues, then you've
got a working install.  (Evict-BT uses `less` to paginate input, so hit q to terminate
it)


Commands
--------
#### General

All commands will ignore unknown arguments, except the default-xxx commands
which take only 0 or 1 argument.

For commands that take an <issue-id> argument, that argument is the last
digits of an issue id.  If an issue requires a single id and the given digits
match more than one issue, the matching issues will be listed with their titles
and no action will be taken.

#### init/clear

`evict init` and `evict clear` create/delete all folders/files  needed for 
Evict-BT to work in a given directory.

Currently, all this means is creating the `.evict` directory.

#### create

`evict create` creates a new issue.  It prompts for a title, an author if needed
and then opens a file for editing using the text editor specified by the EDITOR environment variable.

By default `evict create` uses the default author as set by `evict default-author`,
unless no author has been set or the `--author <auth-name>` option is used.

Passing `--no-body` will cause `evict create` to skip launching the
text editor and use an empty body.

Passing `--title <title-text>` will use <title-text> as the title and skip prompting for
a title.

Passing `--author <auth-name>` will use <auth-name> as the author, overriding the
default author and skipping prompting.

Passing `--body-file <file-name>` will use <file-name> as the body of the issue.  This
has not been tested and may be buggy.  If there are bugs, try putting the
desired body in a file named `ISSUE_MSG` in the directory where evict will be run.
The editor will still be launched, but should already have the desired issue body.
(Note: `ISSUE_MSG` is deleted each time `evict create` runs)

#### delete

`evict delete <issue-id>` deletes the issue specified by <issue-id>, if that issue
has not yet been committed.

#### list

`evict list` lists all issues for the current branch, or a subset of those as specified by options.

Passing `--short` or `-s` will list in short mode, which prints only the title and
ID of issues.

Passing `--committed` will list only issues which have been committed.

Passing `--nocomment` lists issue info and body only, not comments.

Passing `--status <status-name>` lists issues with the status <status-name>.

Passing `--id <issue-id>` lists issues which have an id ending in <issue-id>.

#### comment

`evict comment <issue-id>` launches an editor to write a comment for the specified issue.  Takes only
a single issue.

#### default-author

`evict default-author [author-name]` prints the current default author if no [author-name] argument is
given, and sets the default to [author-name] otherwise.  This is global for the current user, not evict
repository based.

#### new-status

`evict new-status <status-name>` creates a new status that can be used for issues

#### set-status

`evict set-status <issue-id> <status-name>` sets the status of the given issue to the given status.  The status
given must have been created using `evict new-status`.

#### default-status

`evict default-status [status-name]` print the current default status if no [status-name] argument is
given, and sets the default to [status-name] otherwise.  This is local to the current repo.  [status-name] must
have been created using `evict new-status`.

