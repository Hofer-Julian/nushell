---
title: str capitalize
layout: command
version: 0.63.0
usage: |
  Capitalize first letter of text
---

# `{{ $frontmatter.title }}`

<div style='white-space: pre-wrap;'>{{ $frontmatter.usage }}</div>

## Signature

```> str capitalize ...rest```

## Parameters

 -  `...rest`: optionally capitalize text by column paths

## Examples

Capitalize contents
```shell
> 'good day' | str capitalize
```

Capitalize contents
```shell
> 'anton' | str capitalize
```

Capitalize a column in a table
```shell
> [[lang, gems]; [nu_test, 100]] | str capitalize lang
```