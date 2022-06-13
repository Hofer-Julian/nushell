---
title: fetch
version: 0.63.0
usage: |
  Fetch the contents from a URL.
---

<script>
  import { usePageFrontmatter } from '@vuepress/client';
  export default { computed: { frontmatter() { return usePageFrontmatter().value; } } }
</script>

# <code>{{ frontmatter.title }}</code>

<div style='white-space: pre-wrap;'>{{ frontmatter.usage }}</div>

## Signature

```> fetch (URL) --user --password --timeout --headers --raw```

## Parameters

 -  `URL`: the URL to fetch the contents from
 -  `--user {any}`: the username when authenticating
 -  `--password {any}`: the password when authenticating
 -  `--timeout {int}`: timeout period in seconds
 -  `--headers {any}`: custom headers you want to add
 -  `--raw`: fetch contents as text rather than a table

## Notes
```text
Performs HTTP GET operation.
```
## Examples

Fetch content from url.com
```shell
> fetch url.com
```

Fetch content from url.com, with username and password
```shell
> fetch -u myuser -p mypass url.com
```

Fetch content from url.com, with custom header
```shell
> fetch -H [my-header-key my-header-value] url.com
```