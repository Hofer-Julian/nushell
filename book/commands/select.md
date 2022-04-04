---
title: select
layout: command
version: 0.60.0
usage: |
  Down-select table to only these columns.
---

# `{{ $frontmatter.title }}`

<div style='white-space: pre-wrap;'>{{ $frontmatter.usage }}</div>

## Signature

`> select ...rest`

## Parameters

- `...rest`: the columns to select from the table

## Examples

Select just the name column

```shell
> ls | select name
```

Select the name and size columns

```shell
> ls | select name size
```