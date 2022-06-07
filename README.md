# The real CGI

TODO LOGO

## Spec
` `

- Routes `->` Subpaths
- Subpath names `->` Route parameter
  - Svelte style, e.g. `/[userid]/` sets `$userid`
  - (case sensitive)
- HTTP method GET `->` `METHOD=GET`
- Headers `->` Environment Variables
- Request Body `->` stdin
- stdout `->` Response Body
- Return codes:
  - `0 => 200`
  - `_ => 500`
  - `404` if route does not exist

## TODO
- Security features
  - hidden files are not allowed
  - parent paths are not allowed (`..`)
  - existing path variables cant be overwritten
- Debug flag `->` stderr in resp body if ret != 0
