# Here's the plugins directory used for plugin system

## Usage

Add as many plugins as you want and add the name of it to the nyx's config file. The name of the plugin must match the name of the file without file extension. 

## Exampled

Simple example for a plugin file:

```toml
[plugin]
name = "rust"
plugin_type = "Simple"
version = "0.1.0"
enabled = true

[core]
build_command = true
clean_command = true
run_command = true

[commands]
init_command = ["cargo", "init", "--bin"]
build_command = []
clean_command = []
run_command = []
```
