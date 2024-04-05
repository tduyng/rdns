# Build your own DNS server

[![progress-banner](https://backend.codecrafters.io/progress/dns-server/fb66e20e-06fa-4037-b84b-53d9ca3a788b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

Welcome to the "Build Your Own DNS Server" Challenge repository for Rust solutions!

## Overview
This repository serves as a my Rust solutions to the
["Build Your Own DNS server" Challenge](https://app.codecrafters.io/courses/dns-server/overview).

You'll build a DNS server that's capable of parsing and
creating DNS packets, responding to DNS queries, handling various record types
and doing recursive resolve. Along the way we'll learn about the DNS protocol,
DNS packet format, root servers, authoritative servers, forwarding servers,
various record types (A, AAAA, CNAME, etc) and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Getting started

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.70)` installed locally
1. Run `./your_server.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.
