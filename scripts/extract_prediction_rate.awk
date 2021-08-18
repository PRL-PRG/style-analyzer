#!/usr/bin/awk -f 
# Collects prediction rate from style-analyzer's log files.
# Usage: <path/to/log.txt extract_prediction_rate.awk >path/to/output.csv

# Project URL, starts a project section.
# This is how the section looks:
#
#   ================================================================================
#   = https://github.com/laravel/telescope                                         =
#   =  1 / 20                                                                      =
#   = Now:  2021-08-13 15:55:00.899852                                             =
#   = Left: None                                                                   =
#   = Ends: None                                                                   =
#   ================================================================================
#
$0 ~ /^=\s+http/ {
    # URL is the second word. Stick it into a variable so it persists in
    # all subsequent rules until a new URL is found
    url = $2;

    # Make two maps, one for samples, one for samples without predictions.
    # Both initially zero.
    samples[url] = 0;
    uncovered[url] = 0;

    # Each project is trained first, so set training flag to true
    training = "t";
}

# When we encounter a review event, the project switches from training to testing
$0 == "INFO:EventListener:new ReviewEvent" {
    training = "f";
}

# When we encounter a prediction message, we can harvest a sample size
# The line looks like this:
#
#   DEBUG:Rules:predicting 464 samples using 3 rules
#
# We can match on the first word
$1 == "DEBUG:Rules:predicting" {
    # We only do this after training, so if training is true, we move to the next line.
    if (training == "t") {
        next;
    }

    found_sample = $2 # sample size is the second word
    samples[url] += found_sample;
}

# When we encounter a missed predictions message, we can harvest the number of 
# uncovered samples. 
# 
# The line looks like this:
# 
#   DEBUG:Rules:No rule was triggered in 139 cases
#
# We have to match by regex
$0 ~ /^DEBUG:Rules:No rule was triggered/ {
    # We only do this after training, so if training is true, we move to the next line.
    if (training == "t") {
        next;
    }

    found_uncovered = $6 # number of interest is the sixth word
    uncovered[url] += found_uncovered;
}

# When the file is process, output all collected data to stdout in CSV format
END {
    # Comma separated format, will have fields separated by commas
    OFS = ", ";

    # Print out header
    print "url", "samples", "predictions", "uncovered", "prediction rate";

    # Go over collected data and print it out, we iterate over the samples map, 
    # which should have the samne URL keys as the missed map.
    for (url in samples) {
        # Calculate predictions from the number of samples without predictions
        predictions = samples[url] - uncovered[url];

        # Calculate prediction rate, or set to NA if number of samples is zero
        if (samples[url] == 0) {
            prediction_rate = "NA";
        } else {
            prediction_rate = predictions / samples[url];
        }

        # Print out one line of data per each collected URL
        print url, samples[url], predictions, uncovered[url], prediction_rate;
    }
}

