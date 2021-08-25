#!/bin/bash

if [ $# -eq 0 ]
then
    # If no arguments were given, do evereything ireturned by the queries
    SELECTIONS=$(find "$(pwd)/selections/selections/" -iname '*.csv' | cut -f 1 -d.)
else
    SELECTIONS="$@"
fi

time for selection in $SELECTIONS
do
   QUALITY_REPORT_REPOS="$(pwd)/selections/selections/$selection.csv"
   QUALITY_REPORT_DIR="$(pwd)/reproductions/$selection"

   mkdir -p ${QUALITY_REPORT_DIR};   
   chmod a+rw ${QUALITY_REPORT_DIR}; 

   echo "STARTING ${selection}"

   time python3 -m lookout.style.format \
      --log-level DEBUG quality-report \
      -o ${QUALITY_REPORT_DIR} \
      -i $QUALITY_REPORT_REPOS \
      2>&1 | tee -a ${QUALITY_REPORT_DIR}/logs.txt

   echo "FINISHED ${selection}"
done