# Why?
Pacman is aware of only the most recent packages (pacman -Q), while older packages are not easily
accessible by pacman.

Let's say a bluez package was updated and now bluetooth isn't working on your pc, to downgrade you
have to go to /var/cache/pacman/pkg/ and find previous version, But with this app you can now just
type:

```pacbase -Q <package name>```

and get all the versions available on your pc! Setup a script to pipe it
through fzf and easily downgrade!
comes with optional builtin fzf integration!
