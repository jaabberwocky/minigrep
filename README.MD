# minigrep

`minigrep` utility to search for text within files.

## Usage
`./minigrep <query> <path/to/filename>`

You can include case-insensitive search by using the `CASE_INSENSITIVE` environment variable:
`EXPORT CASE_INSENSITIVE=1`

## Examples

```
>>> ./minigrep to examples/poem.txt
Searching for to
In file examples/poem.txt
Are you nobody, too?
How dreary to be somebody!
```

##
