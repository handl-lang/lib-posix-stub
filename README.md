# lib-os-stub

Stub backend for the `os` ports.

## Layout

```text
src/             the .hnl sources (path.hnl, env.hnl, time.hnl + tests)
pkg.hnl          package manifest — modules, deps, description, license
ci.sh            bupha entry → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
