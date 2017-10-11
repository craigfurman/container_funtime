# container_funtime

A toy container CLI, written so that I can mess about with Rust.

# TODO

1. Do I have to bind-mount /proc?
1. Moar namespaces
1. cgroups
1. Exec user proc rather than fork+exec.
1. Enable colour rspec output from docker tests
1. Sensible default PATH. What to do about other env vars?
1. Backfill explicit PID NS test?
