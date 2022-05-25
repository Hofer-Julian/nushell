---
title: from ics
layout: command
version: 0.63.0
usage: |
  Parse text as .ics and create table.
---

# `{{ $frontmatter.title }}`

<div style='white-space: pre-wrap;'>{{ $frontmatter.usage }}</div>

## Signature

```> from ics ```

## Examples

Converts ics formatted string to table
```shell
> 'BEGIN:VCALENDAR
END:VCALENDAR' | from ics
```