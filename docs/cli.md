# Command Line Interface

`pytokei` can be used from the console (at least for a subset of the options that [`tokei`](https://github.com/XAMPPRocky/tokei) offers). 

The CLI is built using [`typer`](LINK) and [`rich`](LINK), so it should be self explanatory:

![pytokei-help](./assets/pytokei-help.png)

Only the compact report has been implemented:

![pytokei-itself](./assets/pytokei-itself.png)

With options to sort by the different variables, and ignore paths:

![pytokei-no-color](./assets/pytokei-no-color.png)

For the full detailed CLI you should use `tokei` directly.
