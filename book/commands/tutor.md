---
title: tutor
version: 0.63.0
usage: |
  Run the tutorial. To begin, run: tutor
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> tutor (search) --find```

## Parameters

 -  `search`: item to search for, or 'list' to list available tutorials
 -  `--find {string}`: Search tutorial for a phrase

## Examples

Begin the tutorial
```shell
> tutor begin
```

Search a tutorial by phrase
```shell
> tutor -f "$in"
```