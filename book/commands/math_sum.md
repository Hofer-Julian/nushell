---
title: math sum
version: 0.63.0
usage: |
  Finds the sum of a list of numbers or tables
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> math sum ```

## Examples

Sum a list of numbers
```shell
> [1 2 3] | math sum
```

Get the disk usage for the current directory
```shell
> ls | get size | math sum
```