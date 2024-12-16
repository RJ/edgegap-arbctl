# Arbctl - An Edgegap API tool

[![Crates.io](https://img.shields.io/crates/v/arbctl.svg)](https://crates.io/crates/arbctl)

This is a tool for interacting with the Edgegap API.

I've tried to build a scalable way to easily add api commands, but I plan to only really add enough functionality to be useful as part of my automated deployment pipeline. PRs welcome.

A key feature is the ability to patch resources with json pointers, per [RFC 6901](https://tools.ietf.org/html/rfc6901). This should allow an ergonomic way to change values in application versions, create clone-and-modified versions of resources, etc,

## Quick note about JSON Pointers

JSON Pointers are a way to reference a specific value in a json document.
They are specified in the [RFC 6901](https://tools.ietf.org/html/rfc6901) specification, and support for
them is [built in](https://docs.rs/serde_json/latest/serde_json/enum.Value.html#method.pointer) to `serde_json`.

We use them in two ways:

#### Render patches
A quick way to edit the JSON returned from a command, before it's printed to stdout.
This is done via the global `--render-patch` arg, which each take 2 args (pointer and replacement value), and can be specified multiple times.

Eg:
```
$ arbctl application get simple_box

{
  "create_time": "2024-12-12 12:26:39.029223",
  "image": "iVBO...ggg==",
  "is_active": true,
  "is_telemetry_agent_active": false,
  "last_updated": "2024-12-13 08:55:43.373337",
  "name": "simple_box"
}
```

```
# daft example, because image is supposed to be base64 encoded image data
# but this only patches the stdout, we aren't sending a PATCH request

$ arbctl \
    --render-patch /is_active false \
    --render-patch /image '"foo"' \
    application get simple_box

{
  "create_time": "2024-12-12 12:26:39.029223",
  "image": "foo",
  "is_active": false,
  "is_telemetry_agent_active": false,
  "last_updated": "2024-12-13 08:55:43.373337",
  "name": "simple_box"
}
```

#### Patching resources for PATCH commands

Say we want to mutate an existing application version to change some settings. Here we change application `simple_box` version `1`:

**Note:** we must pass literal json strings for replacement strings, so '"quotes"' like this are needed for string values.

```
arbctl application patch-version simple_box 1 \
    --patch /termination_grace_period_seconds 6 \
    --patch /session_config/empty_ttl 15 \
    --patch /docker_tag '"sha-1234567890"'
 ```

This command will `GET` the existing version, apply the patches, and send a `PATCH` request to update it.



## Example Invocations

Get version 1 of the application "simple_box" (json)
```
arbctl application version simple_box 1
```

Modify it by using `--patch` and the `patch-version` application sub-command:
```
arbctl application patch-version simple_box 1 \
    --patch /termination_grace_period_seconds 6 \
    --patch /ports/0/name '"gameserver"'
```

### Creating a new application version based on an existing one

use `application version` to fetch version 1, use `---render-patch` to change the output json, and pipe into `create-version` which reads from stfdin. Anything requiring advanced json edits can just use `jq` or whatever.
```
arbctl --render-patch /name '"2"' --render-patch /docker_tag '"v2"' application version simple_box 1 \
 | arbctl application create-version simple_box 2 --stdin
```

### Create a new application based on an existing one
(copy simple_box to an application called testapp)
```
arbctl --render-patch /name '"testapp"' application get simple_box | arbctl application create testapp
```

# Questions for Edgegap

the `docker_image` field seems to be validated by a mega regex when your submit a patch or create request – 
a regex that is more restrictive than the web dashboard uses.

it has to be lower case.

```
❌ "docker_image": "RJ/lightyear-server",
✅ "docker_image": "rj/lightyear-server",
```


## Notes

* [CLI Struture in Rust](https://kbknapp.dev/cli-structure-01/) - a great intro to writing scalable cli tools in rust using clap.
 