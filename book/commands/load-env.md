---
title: load-env
version: 0.63.0
usage: |
  Loads an environment update from a record.
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> load-env (update)```

## Parameters

 -  `update`: the record to use for updates

## Examples

Load variables from an input stream
```shell
> {NAME: ABE, AGE: UNKNOWN} | load-env; echo $env.NAME
```

Load variables from an argument
```shell
> load-env {NAME: ABE, AGE: UNKNOWN}; echo $env.NAME
```