# lib-posix-stub

Stub backend for the `posix` ports (path, env, time, fs) — the
deterministic test implementation of the `lib-posix` contract.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | module index |
| `src/{path,env,time,fs}.hnl` + matching `*.test.hnl` | per-surface stub impls + tests |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / gate / bupha entry |

## Contract

- Satisfies the `posix` contract (`lib-posix`) with stub behavior —
  for tests and platforms without libc.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
