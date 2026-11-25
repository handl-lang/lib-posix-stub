# lib-posix-stub

Stub backend for the `posix` ports (path, env, time, fs).

## Layout

```text
src/             the .hnl sources (path.hnl, env.hnl, time.hnl, fs.hnl + tests)
pkg.hnl          package manifest — modules, deps, description, license
workflows/gate.yaml            bupha entry (handl-gates) → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
