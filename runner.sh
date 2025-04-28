#!/bin/bash

cd outputs
mkdir bins

for fname in *.rs; do
	if [ -f "$fname" ]; then
		echo "Compiling $fname..."
		rustc --out-dir bins "$fname"
	fi
done

rm -rf bins

