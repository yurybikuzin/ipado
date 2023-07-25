#/usr/bin/env bash
sudo journalctl -f -u {{ App }}_{{ Kind }} | cut -c 47-
