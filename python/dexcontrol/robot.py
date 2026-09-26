# Copyright (C) 2026 Dexmate Inc.
#
# This software is dual-licensed:
#
# 1. GNU Affero General Public License v3.0 (AGPL-3.0)
#    See LICENSE-AGPL for details
#
# 2. Commercial License
#    For commercial licensing terms, contact: contact@dexmate.ai

"""High-level Robot class: thin Python conveniences over the native runtime.

The native ``_native.Robot`` performs every connection, subscription,
decode, and control operation in Rust. This subclass adds only what reads
better in Python: the sensor accessor and a few small conveniences.
"""

from __future__ import annotations

import weakref
from typing import Any

from dexcontrol import _native


class Robot(_native.Robot):
    """Main robot interface (see the package README for examples).

    Always release the connection: use ``with Robot(...) as robot:`` or call
    :meth:`shutdown` (alias :meth:`close`). A robot that is garbage-collected
    without it still performs a bounded (2 s) native shutdown — stop request
    included — but emits a ``ResourceWarning``; do not rely on it. A robot
    object cannot be used in a forked child process (``RuntimeError``):
    create it in the child, or use the ``spawn`` start method.

    Args:
        profile: Built-in robot profile name. Defaults to the
            ``DEXBOT_PROFILE`` environment variable, then to the profile the
            ``ROBOT_NAME`` environment variable maps to, then ``vega_1``
            (with a warning). A ``ROBOT_NAME`` that names no known robot
            raises :class:`~dexcontrol.ConfigurationError` instead of
            silently selecting ``vega_1``: the wrong profile moves real
            hardware with the wrong limits.
        config_file: Path to a custom robot YAML/JSON file (mutually
            exclusive with ``profile``).
        enable_sensors: Optional configured sensors to enable.
        simulation: When true, runs against the deterministic in-process
            loopback simulation instead of real hardware.
        watchdog: Controls the dedicated-process safety watchdog (one small
            extra Python process supervising heartbeat/E-stop on its own
            session, isolated from this one). ``None`` keeps the default:
            on for production robots, always off for simulation. An
            explicit bool overrides the production default.
        exit_on_safety_termination: When a safety monitor fires the
            configured ``request_process_termination`` action (e.g. a
            heartbeat timeout with that policy), the native layer makes the
            robot safe, emits the safety event, logs critically to stderr,
            and terminates this process with exit code 1 — immediately and
            without running Python cleanup (an unstable network must not
            leave a half-alive controller running). The exit uses the C
            ``_exit``: no ``atexit`` hooks, no ``finally`` blocks, no
            ``__del__``, no static destructors, and unflushed ``stdout``
            buffers are lost. ``None`` keeps the
            default: ``False`` in every language. Safety actions still stop
            the robot and emit events; pass ``True`` only when this application
            explicitly requires whole-process termination. Pausing heartbeat monitoring
            (``robot.heartbeat.pause()``) suspends the whole heartbeat
            reaction chain, including this one.
        context: Optional shared :class:`~dexcontrol.ControlContext`.
        require_version_check: When true, connecting fails closed with
            :class:`~dexcontrol.ConfigurationError` if the robot's versions
            service cannot be reached or verified. By default that only
            downgrades ``robot.health()["version_check"]``.

    Migration notes (from the pre-0.6 pure-Python package):
        * ``component.get_joint_pos(joint_id=3)`` (and the other
          ``get_joint_*`` getters) return a 1-element ``numpy.ndarray`` for an
          integer ``joint_id``; the old API returned a scalar. Index the
          result (``[0]``) or call ``float()`` on it.
        * No SIGTERM/SIGHUP handlers are installed. The old package
          converted them to ``sys.exit`` so ``atexit`` shut the robot down;
          call :func:`dexcontrol.install_signal_handlers` to opt in.
        * ``robot.health()`` additionally reports ``supervision_failed`` (a
          safety monitor task died; nothing is watching) and
          ``stop_undelivered`` (the last safety stop could not be delivered;
          the robot may still be moving).
    """

    def __init__(
        self,
        profile=None,
        *,
        config_file=None,
        enable_sensors=(),
        simulation=False,
        watchdog=None,
        exit_on_safety_termination=None,
        context=None,
        require_version_check=False,
        _sim_event_driven_estop=False,
    ):
        # The native __new__ has already connected using these arguments.
        del profile, config_file, enable_sensors, simulation, watchdog
        del exit_on_safety_termination, context, require_version_check
        del _sim_event_driven_estop
        self._sensor_manager = None

    # ------------------------------------------------------------------
    # Sensors
    # ------------------------------------------------------------------
    @property
    def sensors(self):
        """Every sensor on this robot; see
        :class:`dexcontrol.sensors.manager.Sensors`.

        The manager refers back to this robot weakly (a strong reference
        would form a cycle that keeps an unclosed robot, its threads and its
        connection alive until the cycle collector runs), so keep the robot
        itself referenced while using it.
        """
        if self._sensor_manager is None:
            from dexcontrol.sensors.manager import Sensors

            self._sensor_manager = Sensors(weakref.proxy(self))
        return self._sensor_manager

    # ------------------------------------------------------------------
    # Convenience helpers
    # ------------------------------------------------------------------
    def get_controllable_component_map(self) -> dict[str, Any]:
        """Maps every enabled joint-controllable component to its handle."""
        result: dict[str, Any] = {}
        for name in self.component_names:
            try:
                result[name] = self.joints(name)
            except Exception:  # noqa: BLE001 - non-joint components are skipped
                continue
        return result

    def validate_component_names(self, joint_pos: dict[str, Any]) -> None:
        """Raises ``ValueError`` when a command dictionary names an unknown
        or non-controllable component."""
        controllable = set(self.get_controllable_component_map())
        unknown = [name for name in joint_pos if name not in controllable]
        if unknown:
            raise ValueError(
                f"Unknown or non-controllable component(s): {sorted(unknown)}. "
                f"Controllable components: {sorted(controllable)}"
            )

    def have_hand(self, side: str) -> bool:
        """True when a hand component is configured on the given side."""
        if side not in ("left", "right"):
            raise ValueError("side must be 'left' or 'right'")
        return self.has_component(f"{side}_hand")
