# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""Shared Rerun display helpers; not an executable example.

Imported by sensor/current scripts. start() opens/connects a viewer unless --no-display is set; missing rerun-sdk falls back to printing. Running this module directly starts no viewer or robot connection."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

import tyro
from typing_extensions import Annotated


@dataclass
class ViewerOptions:
    """Display options shared by sensor examples."""

    viewer_url: Annotated[
        Optional[str],
        tyro.conf.arg(
            help="stream to a Rerun viewer at HOST:PORT instead of spawning one (run `rerun` on your laptop, then point this at it)"
        ),
    ] = None
    no_display: Annotated[
        bool, tyro.conf.arg(help="print values instead of visualizing them")
    ] = False


def start(name: str, args: ViewerOptions) -> bool:
    """Starts a Rerun recording unless the caller asked for plain output.

    Returns whether anything should be logged, so an example can branch once
    and keep its loop readable. A missing `rerun-sdk` is reported and treated
    as `--no-display` rather than raised: the example still does something
    useful, which matters when the point of running it is to find out whether
    the sensor works at all.
    """
    if args.no_display:
        return False
    try:
        import rerun as rr
    except ImportError:
        print("rerun-sdk is not installed; printing instead (pip install rerun-sdk)")
        return False

    rr.init(f"dexcontrol-{name}")
    if args.viewer_url:
        rr.connect_grpc(f"rerun+http://{args.viewer_url}/proxy")
    else:
        _spawn_attached(rr)
    return True


_spawned_viewer_pid: Optional[int] = None


def _spawn_attached(rr) -> None:
    """Spawns a viewer whose lifetime is tied to this example.

    `rr.spawn()` detaches the viewer and forgets its pid, which is right for
    a session kept open across runs, but a live monitor stopped with Ctrl-C
    would leave a window behind showing a recording nothing feeds any more.
    The viewer always lands in its own process group, so a terminal Ctrl-C
    never reaches it directly; `close()` ends it when the example is
    interrupted. An example that finishes on its own (a bounded `--samples
    N`) leaves the window open for inspection.
    """
    global _spawned_viewer_pid
    try:
        from rerun._spawn import _spawn_viewer
    except ImportError:  # an SDK without the helper: detached spawn
        rr.spawn()
        return
    _spawned_viewer_pid = _spawn_viewer()
    rr.connect_grpc("rerun+http://127.0.0.1:9876/proxy")


def close() -> None:
    """Ends the viewer this process spawned, if any. A viewer that was
    already running (or reached through --viewer-url) is left alone."""
    global _spawned_viewer_pid
    pid, _spawned_viewer_pid = _spawned_viewer_pid, None
    if pid is None:
        return
    import os
    import signal
    import time

    # The `rerun` entry point is a wrapper script running the real viewer as
    # its child, in a process group of its own: signal the group so the
    # binary goes with the wrapper. A viewer that holds a blueprint does not
    # exit on the first SIGTERM (observed with rerun 0.36), so escalate.
    try:
        group = os.getpgid(pid)
    except OSError:
        return
    for sig in (signal.SIGTERM, signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(group, sig)
        except OSError:
            return
        deadline = time.monotonic() + 1.0
        while time.monotonic() < deadline:
            try:
                os.killpg(group, 0)
            except OSError:
                return  # nothing left in the group
            time.sleep(0.05)


def show_time_series(prefix: str, groups: list[str]) -> None:
    """Lays out one time-series panel per group with every series visible.

    Without a blueprint the viewer picks its own layout and selection, and a
    plot with many joints starts with some of them hidden. `prefix/group/*`
    is the entity layout the current monitor logs.
    """
    import rerun as rr
    import rerun.blueprint as rrb

    panels = [
        rrb.TimeSeriesView(name=group, origin=f"/{prefix}/{group}") for group in groups
    ]
    rr.send_blueprint(rrb.Blueprint(rrb.Grid(*panels), collapse_panels=True))


def set_sample_time(index: int, seconds: float) -> None:
    """Places the next log call on the timeline.

    Two timelines, because the useful question differs: `sample` for stepping
    through readings one at a time, `time` for anything whose shape over time
    is the point, like a current trace.
    """
    import rerun as rr

    rr.set_time("sample", sequence=index)
    rr.set_time("time", duration=seconds)


def log_frame(path: str, frame: dict) -> None:
    """Logs one camera frame, choosing the entity type from its shape.

    Colour arrives as `uint8 (H, W, 3)` and depth as `float32 (H, W)` metres.
    They are logged as different Rerun types on purpose: depth then gets a
    colour map and a readable metre value under the cursor, instead of being
    rendered as a nearly-black grayscale image.
    """
    import rerun as rr

    data = frame["data"]
    if data.ndim == 3:
        rr.log(path, rr.Image(data))
    else:
        rr.log(path, rr.DepthImage(data, meter=1.0))
