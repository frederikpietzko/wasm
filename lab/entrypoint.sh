#!/bin/bash

/usr/bin/supervisord -n >> /dev/null 2>&1 &
nohup dockerd >/dev/null 2>&1 &
# run the command
"$@"