# System Setup

Before you can get started building games with Manatee, there's a few things we'll need to get set
up on your computer first. This section will cover setting up Zig and cloning the Manatee source
code.

> **A Quick Note on Command Line Notation:**
>
> In this page (and the rest of the book), you'll be asked to enter some commands in your system's
> terminal. Any command that you'll need to enter will be denoted by a `$` character. **Do not type
> the `$` character when entering these commands, that character is only there to denote that the
> thing on the screen is, in fact, a command you need to enter.

## Installing Zig

If you've never installed a programming language before, installing Zig can seem a little daunting
at first. The following steps install the latest stable version of Zig, as well as a system package
management tool to help you keep Zig up to date.

### Installing Zig on Windows

Before we can install Zig we'll want to install a package manager. By installing a package manager,
you'll greatly simplify the process of installing and updating your Zig installation. Think of a
package manager kind of like an app store, but you install your apps using the command line rather
than a UI. This guide uses Chocolatey as its package manager for Windows. To install Chocolatey,
do the following:

1. Open your start menu and find "Command Prompt". Once you've found it, right click the tile and
   click "Run as Administrator". From here forward, all commands will be entered in Command Prompt.
2. Once Command Prompt is open, enter the following command:
   ```bat
   $ @"%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -InputFormat None -ExecutionPolicy Bypass -Command "[System.Net.ServicePointManager]::SecurityProtocol = 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))" && SET "PATH=%PATH%;%ALLUSERSPROFILE%\chocolatey\bin"
   ```
3. Let's verify that Chocolatey was successfully installed by entering the following command:
   ```bat
   $ choco --version
   ```

Chocolatey is installed, yay! Now that we have that out of the way, let's install Zig by doing the
following:

1. Enter the following command:
   ```bat
   $ choco install zig
   ```
2. Let's verify that Zig was successfully installed by entering the following command:
   ```bat
   zig version
   ```

Congratulations, both Chocolatey and Zig have been successfully installed, now you can move on to
actually using Manatee!

### Installing Zig on MacOS

Before we can install Zig we'll want to install a package manager. By installing a package manager,
you'll greatly simplify the process of installing and updating your Zig installation. Think of a
package manager kind of like an app store, but you install your apps using the command line rather
than a UI. This guide uses Homebrew as its package manager for MacOS. To install Homebrew,  do the
following:

1. Open applications and find "Terminal". Once you've found it, open it. From here forward, all
   commands will be entered in Terminal.
2. Once Terminal is open, enter the following command
   ```zsh
   $ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```
3. Let's verify that Homebrew was successfully installed by entering the following command:
   ```zsh
   brew --version
   ```

Homebrew is installed, yay! Now that we have that out of the way, let's install Zig by doing the
following:

1. Enter the following command:
   ```bat
   $ brew install zig
   ```
2. Let's verify that Zig was successfully installed by entering the following command:
   ```bat
   zig version
   ```

### Installing Zig on Linux

If you're using Linux I'm going to assume two things about you

1. You have a pretty damn good understanding of how computers, terminals, and package managers work
2. You probably don't need help installing Zig.

Given the fact that Linux has a seemingly infinite number of package managers, and I have no way to
tell which distro / package manager you're using, I'm going to link you to the
[Zig docs](https://github.com/ziglang/zig/wiki/Install-Zig-from-a-Package-Manager) for instructions
on installing Zig with your package manager of choice.

## Setting Up Manatee

Now that we have Zig set up, it's time to install Manatee.

> TODO: I need to write instructions on how to setup Git, this will come soon but I really should
> actually be building this game engine instead of writing a book on using it when it doesn't
> exist yet. For now I'll assume you have git installed

Let's start off by cloning Manatee. I personally prefer to clone Manatee into my Documents folder,
let's do that!

### Cloning Manatee on Windows

```bat
$ cd %userprofile%\Documents
$ git clone https://github.com/jrkienle/manatee.git
$ cd manatee
```

### Cloning Manatee on MacOS / Linux

```zsh
$ cd ~/Documents
$ git clone https://github.com/jrkienle/manatee.git
$ cd manatee
```

Now that we've cloned Manatee, let's compile it! Compiling Manatee will allow you to actually run
the editor and start building games! Luckily for us, Zig makes compiling code a breeze. In the same
terminal you used to clone Manatee, run the following command:

```zsh
$ zig build
```

Once that command completes (it may take awhile depending on how powerful your machine is), you
should see a new folder named `zig-out` appear inside of `Documents/manatee`. Open up that folder,
then open the `bin` folder inside it, and you should see `manatee-editor.exe` if you're on windows,
or `manatee-editor` if you're on MacOS or Linux. Double click that file, and congratulations,
you're now officially running the Manatee editor!
