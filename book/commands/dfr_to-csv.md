---
title: dfr to-csv
version: 0.63.0
usage: |
  Saves dataframe to csv file
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> dfr to-csv (file) --delimiter --no-header```

## Parameters

 -  `file`: file path to save dataframe
 -  `--delimiter {string}`: file delimiter character
 -  `--no-header`: Indicates if file doesn't have header

## Examples

Saves dataframe to csv file
```shell
> [[a b]; [1 2] [3 4]] | dfr to-df | dfr to-csv test.csv
```

Saves dataframe to csv file using other delimiter
```shell
> [[a b]; [1 2] [3 4]] | dfr to-df | dfr to-csv test.csv -d '|'
```