---
title: merge
version: 0.63.0
usage: |
  Merge a table into an input table
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> merge (block)```

## Parameters

 -  `block`: the block to run and merge into the table

## Examples

Merge an index column into the input table
```shell
> [a b c] | wrap name | merge { [1 2 3] | wrap index }
```

Merge two records
```shell
> {a: 1, b: 2} | merge { {c: 3} }
```