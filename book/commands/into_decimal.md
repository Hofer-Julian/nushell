---
title: into decimal
version: 0.63.0
usage: |
  Convert text into a decimal
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> into decimal ...rest```

## Parameters

 -  `...rest`: optionally convert text into decimal by column paths

## Examples

Convert string to integer in table
```shell
> [[num]; ['5.01']] | into decimal num
```

Convert string to integer
```shell
> '1.345' | into decimal
```

Convert decimal to integer
```shell
> '-5.9' | into decimal
```