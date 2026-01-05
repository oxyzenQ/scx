# scx_wolf

This is a single user-defined scheduler used within [`sched_ext`](https://github.com/sched-ext/scx/tree/main), which is a Linux kernel feature which enables implementing kernel thread schedulers in BPF and dynamically loading them. [Read more about `sched_ext`](https://github.com/sched-ext/scx/tree/main).

## Overview

A scheduler that focuses on aggressive throughput while keeping behavior bounded,
predictable, and stable.

It operates using an earliest deadline first (EDF) policy. scx_wolf extends the
base EDF behavior with explicit two-lane scheduling:

- **Latency lane**: favors bursty / interactive workloads
- **Bulk lane**: isolates sustained CPU-bound workloads

Lane selection is hybrid:

- behavior-based defaults (sleep/burst vs sustained running)
- cgroup overrides (`wolf.latency` / `wolf.bulk`)
- weak hints from RT policies (`SCHED_FIFO` / `SCHED_RR`)

Lane switches are constrained with hysteresis and a minimum dwell time to avoid
oscillation.

## Typical Use Case

The combination of EDF and explicit lanes aims to deliver consistent
responsiveness under load, while preserving throughput for bulk work.

This makes the scheduler well-suited for mixed workloads such as desktop +
gaming + background compilation/mining.

## Presets

scx_wolf intentionally supports only two presets and presets only affect BPF
rodata knobs (they do not change scheduling logic).

- `adaptive-wolf` (default)
  - `lane_min_dwell_ns = 300ms`
  - `lane_lat_nvcsw_hi = 6`
  - `lane_lat_nvcsw_lo = 3`
  - `lane_bulk_exec_ns = 8ms`
- `gaming`
  - `lane_min_dwell_ns = 120ms`
  - `lane_lat_nvcsw_hi = 4`
  - `lane_lat_nvcsw_lo = 2`
  - `lane_bulk_exec_ns = 4ms`

CLI:

- `--preset adaptive-wolf|gaming`
- Alias flags (recommended for clarity):
  - `--adaptive-wolf`
  - `--gaming`

## Cgroup overrides

To force lane assignment by cgroup name, place workloads under a cgroup named:

- `wolf.latency` to force Latency lane
- `wolf.bulk` to force Bulk lane

The name match applies to the cgroup and its ancestors.

## Stats / observability

Continuous:

- `--stats <seconds>` prints periodic deltas.

One-shot:

- `--dump-stats` prints a single snapshot from the running scheduler and exits.

Lane-related counters:

- `lane_switch_total`
- `latency_starvation_events`
- `bulk_run_ns_total`

## Attribution

scx_wolf is based on scx_flash (sched-ext/scx) by Andrea Righi and is modified
and extended by Rezky Nightky.

## Production Ready?

Yes.
