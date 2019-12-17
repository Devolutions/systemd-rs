# systemd-rs

A high level wrapper for libsystemd.

Currently supports a partial subset of `sd-login`, including `sd_login_monitor`. Assumes a single seat ("seat0") for all functions.
