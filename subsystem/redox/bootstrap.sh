#!/bin/sh

url=https://static.redox-os.org/img/riscv64gc/redox_minimal-net_riscv64gc_2025-05-05_26_harddrive.img.zst
curl -O $url
unzstd redox_minimal-net_riscv64gc_*_harddrive.img.zst
rm *.zst
mv redox_minimal-net_riscv64gc_*_harddrive.img redox_minimal-net.img
