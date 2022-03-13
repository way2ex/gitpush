use gitpush::find_gitpush_command;

#[test]
fn test_find_gitpush_command() {
    let git_string = "\
fatal: The current branch fix/author has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream origin fix/author

";
    let ma = find_gitpush_command(git_string).unwrap();
    assert_eq!(ma, "git push --set-upstream origin fix/author");
}

#[test]
fn failed_find_gitpush_command() {
    let git_string = "\
fatal: The current branch fix/author has no upstream branch.
To push the current branch and set the remote as upstream, use
";
    let ma = find_gitpush_command(git_string).unwrap_or("");
    assert_eq!(ma, "");
}
