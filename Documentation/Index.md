# Documentation

Everything here describes Round as it is now. `CHANGELOG.md` is the only
historical record.

---

## Guides

- **[Architecture](Architecture.md)** - the two packages, the module map, the
  startup sequence, the tray → settings → webview event flow, the webview
  contract, and where to change what.
- **[Releasing](Releasing.md)** - the version in four files, the `Round/v*` tag,
  what each workflow does, the opt-in crates.io publish with OIDC trusted
  publishing, and four ways to test a publish without burning a version.

## Diagrams

- **[`Knowledge.dot`](Knowledge.dot)** - the module and dependency graph, source
  of truth.
- **[`Knowledge.svg`](Knowledge.svg)** - the same graph rendered.

Regenerate after editing the dot file:

```sh
dot -Tsvg Documentation/Knowledge.dot -o Documentation/Knowledge.svg
```

## Outside this directory

- **[`README.md`](../README.md)** - what Round is, install paths, the tray menu,
  the repository layout, development commands.
- **[`CHANGELOG.md`](../CHANGELOG.md)** - what changed in each version.
- **[`.github/workflows/`](../.github/workflows)** - `Check.yml`, `Build.yml`,
  `Publish.yml`, `Auto.yml`; `Dependabot.yml` covers GitHub Actions, both npm
  manifests and `Crate/Round`.
