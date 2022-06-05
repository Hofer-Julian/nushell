---
title: du
version: 0.63.0
usage: |
  Find disk usage sizes of specified items.
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> du (path) --all --deref --exclude --max-depth --min-size```

## Parameters

 -  `path`: starting directory
 -  `--all`: Output file sizes as well as directory sizes
 -  `--deref`: Dereference symlinks to their targets for size
 -  `--exclude {glob}`: Exclude these file names
 -  `--max-depth {int}`: Directory recursion limit
 -  `--min-size {int}`: Exclude files below this size

## Examples

Disk usage of the current directory
```shell
> du
```