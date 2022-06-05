---
title: help
version: 0.63.0
usage: |
  Display help information about commands.
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> help ...rest --find```

## Parameters

 -  `...rest`: the name of command to get help on
 -  `--find {string}`: string to find in command names, usage, and search terms

## Examples

show all commands and sub-commands
```shell
> help commands
```

show help for single command
```shell
> help match
```

show help for single sub-command
```shell
> help str lpad
```

search for string in command names, usage and search terms
```shell
> help --find char
```