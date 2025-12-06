`cardauth` allows for system authentication by magstripe card.
The software is currently usable, but could use better documentation and error
handling.

# Hardware compatibility
`cardauth` is compatible with the Deftun MSR605x and MSRx6 magstripe
reader/encoders.

# Installation
TODO
## Permissions and security
TODO
## PAM configuration
PAM (Pluggable Authentication Modules) is how different services perform local
authentication.
For general information about PAM configuration, see the
[`pam(8)`](https://man.archlinux.org/man/pam.8),
[`pam.conf(5)`](https://man.archlinux.org/man/pam.conf.5), and
[`pam_exec(8)`](https://man.archlinux.org/man/pam_exec.8) `man` pages.

To use `cardauth` for system authentication, you must look in `/etc/pam.d` for
configuration files for each service with which you want to use `cardauth`.
(I recommend only using `cardauth` for services used by local users, such as
`sddm`, `login`, and `i3lock`.)
This may be made easier by a catch-all `system-local-login` file or the like,
depending on how your distro configures things.
In each file you will find something like:
```
#%PAM-1.0
auth include system-auth
```
or
```
#%PAM-1.0

auth       required   pam_shells.so
auth       requisite  pam_nologin.so
auth       include    system-auth

account    required   pam_access.so
account    required   pam_nologin.so
account    include    system-auth

password   include    system-auth

session    optional   pam_loginuid.so
session    optional   pam_keyinit.so       force revoke
session    include    system-auth
session    optional   pam_lastlog2.so      silent
session    optional   pam_motd.so
session    optional   pam_mail.so          dir=/var/spool/mail standard quiet
session    optional   pam_umask.so
-session   optional   pam_systemd.so
session    required   pam_env.so
```
You must add something like
```
auth sufficient pam_exec.so expose_authtok /usr/local/bin/cardauth auth -ef /usr/local/etc/cardauth-users.toml
```
to the top of the block of `auth` directives in each of these files.
Replace `/usr/local/bin/cardauth` with the path to `cardauth` and
`/usr/local/etc/cardauth-users.toml` with the path to `users.toml`.

# Usage
## Enrolling credentials
### Using `cardauth`
`cardauth read -H`
### Manually
TODO
## Authentication
To test authentication outside of PAM, run `cardauth auth <user>`.
The card reader should indicate that it is ready to read a card by illuminating
the yellow LED (for an MSR605x) or quickly flashing the LED green (for a MSRx6).
Swipe your card.
`cardauth` will print `Success` for a successful authentication or `Failure` for
an unsuccessful authentication.

To authenticate with a system service (when properly configured),
enter `C` as your password.
The reader will indicate that it is ready to read a card;
when it does, swipe your card.
