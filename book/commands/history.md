---
title: history
version: 0.63.0
usage: |
  Get the command history
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> history --clear```

## Parameters

 -  `--clear`: Clears out the history entries

## Examples

Get current history length
```shell
> history | length
```

Show last 5 commands you have ran
```shell
> history | last 5
```

Search all the commands from history that contains 'cargo'
```shell
> history | wrap cmd | where cmd =~ cargo
```