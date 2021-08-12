#!/bin/bash

python3 -m lookout.style.format \
    --log-level INFO quality-report \
    -o $QUALITY_REPORT_DIR \
    -i $QUALITY_REPORT_REPOS \
    2>&1 | tee -a log.txt
