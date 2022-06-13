---
title: str length
version: 0.63.0
usage: |
  Output the length of any strings in the pipeline
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> str length ...rest```

## Parameters

 -  `...rest`: optionally find length of text by column paths

## Examples

Return the lengths of multiple strings
```shell
> 'hello' | str length
```

Return the lengths of multiple strings
```shell
> ['hi' 'there'] | str length
```