---
title: to text
version: 0.69.1
formats: |
  Converts data into simple text.
usage: |
  Converts data into simple text.
---

# <code>{{ $frontmatter.title }}</code> for formats

<div class='command-title'>{{ $frontmatter.formats }}</div>

## Signature

```> to text ```

## Examples

Outputs data as simple text
```shell
> 1 | to text
```

Outputs external data as simple text
```shell
> git help -a | lines | find -r '^ ' |  to text
```

Outputs records as simple text
```shell
> ls |  to text
```