# A Freestanding Rust Binary
Personal notes of Josiah on following Philipp Oppermann's blog "Writing an OS in Rust", located at https://os.phil-opp.com/

So, starting a new repo with git and Github - all code run from shell:
1. Create a directory
2. `git init`
3. Create/Edit files in the directory
4. `git add [filenames]` or `git commit -a" for all with a commit message`
5. `git commit -m "Fix..."`
6. Create a repo in Github
6. `git remote add origin https://github.com/[username]/[repo]`
7. `git pull -r origin master` (To rebase to online repo with README, license, etc) - ie the initial pull
8. `git push origin master` (7 and 8 together get the local and remote properly merged and rebased)

Repeat steps 4, 5, and 8 to make new commits
