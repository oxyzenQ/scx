# A Quick Start Guide

This guide provides instructions for running the SCX schedulers as a systemd service and checking its logs.

## Getting Started

At the very beginning, configure the /etc/default/scx file:

- in the SCX_SCHEDULER variable, select the scheduler you are interested in

- in the SCX_FLAGS variable, specify the flags you want to add. To do this, execute and read what flags you can add.

```text
scx_SCHEDNAME --help
```

To start the SCX scheduler at boot, you need to run the systemd service as root. Here are the steps:

- Enable the service:

```text
systemctl enable scx.service
```

- Start the service:

```text
systemctl start scx.service
```

Alternatively, you can use a shortened version of these commands:

```text
systemctl enable --now scx.service
```

- To check the status of the service, use the following command:

```text
systemctl status scx.service
```

## Override global configuration

It is possible to override the global scx settings using systemd environment
variables `SCX_SCHEDULER_OVERRIDE` and `SCX_FLAGS_OVERRIDE`.

Example:

```text
systemctl set-environment SCX_SCHEDULER_OVERRIDE=scx_rustland
systemctl set-environment SCX_FLAGS_OVERRIDE="-s 10000"
systemctl restart scx
```

If you want to restore the default value from the `/etc/default/scx` file execute:

```text
systemctl unset-environment SCX_SCHEDULER_OVERRIDE
systemctl unset-environment SCX_FLAGS_OVERRIDE
systemctl restart scx
```

## Checking journald Logs

- To view the logs, use the following command:

```text
journalctl -u scx.service
```

- To view the logs of the current session use the command

```text
journalctl -u scx.service -b 0
```
