# Fantasy Grounds Unity Control

This is a CLI that can help automate some actions for FGU.

## Module Creation

Have you ever wanted to build a simple module, but don't want to deal with
doing it all in the FGU UI?

`fguctl` is able to build modules for you!

For example, in my home game I've ported some components from Grim Hollow,
and I can build the Grim Hollow module with

### init-module

Making yaml is hard, so the `init-module` sub-command creates the basic yaml for you.

```bash
fguctl module init-module --output modules/grimhollow/grim-hollow.yaml --name 'Grim Hollow'
```


### build

The build-subcommand takes a "Module Definition" and outputs a "mod" file. For example

```bash
fguctl module build -m modules/grimhollow/grim-hollow.yaml -o modules/grimhollow/grimhollow.mod
```

### create-spell

Managing all the yaml is hard, so `fguctl` helps with a sub-command to build a sample spell where
you can update to match your needs!

```bash
fguctl module create-spell --output modules/grimhollow/spells/chitinous_shell.yaml --name 'Chitinous Shell'
```

### create-table

Managing all the yaml is hard, so `fguctl` helps with a sub-command to build a sample table where
you can update to match your needs!

```bash
fguctl module create-table --output modules/grimhollow/tables/level_1_unstable_mution.yaml --name 'Level 1 - Unstable Mutation Table'
```
