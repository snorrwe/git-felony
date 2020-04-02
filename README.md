## Installation
-   Copy the hooks from `/hooks` to `.git/hooks`
-   Install `git-felony` by either:

    -   Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) `cargo install --path git-felony`
    -   Downloading a pre-built binary (__TBA__)

-   Assert that git-felony works `git-felony --version`

## To skip

If git-felony flagged a file by accident you can either:
- Include `git-felony::skip-next` in the previous line
- Use `git commit --no-verify` to skip verification

## Example output:

```
$ cat offender.txt
"c:\"
'w:/'
exit     (123)

$ git add offender.txt

$ git commit
Looks like you're about to commit a felony here:
    | offender.txt
    | @@ -0,0 +1,3 @@
    | "c:\"

Looks like you're about to commit a felony here:
    | offender.txt
    | @@ -0,0 +1,3 @@
    | 'w:/'

Looks like you're about to commit a felony here:
    | offender.txt
    | @@ -0,0 +1,3 @@
    | exit     (123)

Found 3 felonies
Looks like you're about to commit files containing illegal content
```
