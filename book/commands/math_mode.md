---
title: math mode
version: 0.63.0
usage: |
  Gets the most frequent element(s) from a list of numbers or tables
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> math mode ```

## Examples

Get the mode(s) of a list of numbers
```shell
> [3 3 9 12 12 15] | math mode
```