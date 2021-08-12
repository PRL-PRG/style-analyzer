![Format man](doc/logo.png)
# style-analyzer
Fix code style faults using 🤖


[![Read the Docs](https://img.shields.io/readthedocs/style-analyzer.svg)](https://readthedocs.org/projects/style-analyzer/)
[![Travis build status](https://travis-ci.com/src-d/style-analyzer.svg?branch=master)](https://travis-ci.com/src-d/style-analyzer)
[![Code coverage](https://codecov.io/github/src-d/style-analyzer/coverage.svg)](https://codecov.io/github/src-d/style-analyzer)
[![Docker build status](https://img.shields.io/docker/build/srcd/style-analyzer.svg)](https://hub.docker.com/r/srcd/style-analyzer)
[![PyPi package status](https://img.shields.io/pypi/v/lookout-style.svg)](https://pypi.python.org/pypi/lookout-style)
![stability: beta](https://svg-badge.appspot.com/badge/stability/beta?color=ff8000)
[![AGPL 3.0 license](https://img.shields.io/badge/license-AGPL%203.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)

[Overview](#overview) • [How To Use](#how-to-use) • [Science](#science) • [Contributions](#contributions) • [License](#license)

## Overview

This is a collection of analyzers for [Lookout](https://github.com/src-d/lookout) - the open source framework for code review intelligence.
You can run them directly on your Git repositories, but most likely you don't want that and instead
just use the upcoming code review product from [source{d}](https://sourced.tech).
Overall, this project is a mix of research ideas and their applications to solving real problems.
Consider it as an experiment at this stage.

Currently, there is the "format" analyzer working and the one "typos" incubating. All the current and the future
ones are based on machine learning and never contain any hidden domain knowledge such as static
code analysis rules or human-written pattern matchers.

* [`lookout.style.format`](lookout/style/format) - mine "white box" code formatting rules with machine learning and validate new code against them.
* [`lookout.style.typos`](lookout/style/typos) - find typos in identifier names, using the dataset of 60 million identifiers already present in open source repositories on GitHub.

"format" analyzer supports only JavaScript for now, though it is not nailed to that language and
is based on the language-agnostic [Babelfish](https://doc.bblf.sh/) parser. Everything is written in Python.

## How To Use

There are several ways to run style-analyzer:

* [Developer's setup](doc/how-to/developer.md)
* [Demonstration setup](doc/how-to/demo.md)
* [Reports](doc/how-to/reports.md)

## Science

The implemented analyzers are driven by bleeding edge research. One day we will write papers about them,
but first we want to focus on making them work. Below are brief descriptions of how the analyzers
are designed.

#### format

The core of the format analyzer is a language model: we learn without labeled data, just by modeling the existing format in a repository given the current code at a given point in a file. We then check whether the proposed changes follow those learnt formatting conventions.
The training algorithm is summarized below.

1. Represent a file as a linear sequence of "virtual" nodes. Some nodes correspond to the UAST nodes, and some are inserted to mirror the real tokens in the code which are not present in the UAST (e.g. white spaces, keywords, quotes or braces).
2. Identify the nodes which we use as labels - that is, identify Y-s in the (X, Y) training samples. We have around 50 classes at the moment. Some of the classes are sequences of nodes, e.g. four space indentation increase. We also predict NOOP-s: the empty gaps between non-Y nodes.
3. Extract features from the nodes surrounding the Y nodes. We take a fixed-size window and record the internal types, roles, positions and unique identifiers (for tokens which are not present in the UAST) for the left and right siblings and the parent hierarchy (2-3 levels). The features for the left and for the right siblings are different so that we avoid the information "leakage". For example, the difference in offsets between the left and the right neighbor defines the exact length of the predicted token in between.
4. We train the random forest model on the collected (X, Y) dataset. We fine-tune it with bayesian optimization.
5. We extract the rules - the branches of the trees. We prune them in several steps: first we exclude the rules which do not improve the accuracy, second we remove the rule parts which are redundant.
6. We put 93% rule confidence threshold - that is, precision on the training set - and discard the rest. This gives ~95% joint precision.
7. The rules which are left is our model - the training result.

The application algorithm is much simpler - we take the rules and apply them. However, there are several quirks:
1. In case several rules are triggered, the rule with the highest confidence wins.
2. There are paired tokens which we predict such as quotes. It is possible that there are two rules which contradict each other - the left and the right quotes are predicted to be different. We pick the most confident prediction and change the second quote accordingly.
3. We check that the prediction does not break the code. For example, it can insert a newline in the middle of the expression which can change the AST. We run Babelfish on each changed line to see if the AST remains the same.
4. There is a huge chunk of code to represent the triggered rule in a human-readable format and generate the code for fixes.

#### typos

We take the dataset with identifiers extracted from [Public Git Archive](https://github.com/src-d/datasets/tree/master/PublicGitArchive).
We split them (blog post is pending early November). There are frequencies present for each "atom",
so we consider top frequent ones as ground truth. For each checked "atom", we take it's embedding
computed with [fasttext](https://github.com/facebookresearch/fastText), refine it with a deep
fully-connected neural network, generate candidates with [symspell](https://github.com/wolfgarbe/SymSpell)
and rank them with [XGBoost](https://github.com/dmlc/xgboost).

## Contributions

Contributions are very welcome and desired! Please follow the [code of conduct](doc/code_of_conduct.md)
and read the [contribution guidelines](doc/contributing.md). If you want to add a new cool style
fixer backed by machine learning, it is always a good idea to discuss it on
[Slack](https://sourced.tech/community/#talk).

## License

AGPL-3.0, see [LICENSE.md](LICENSE.md).

## Notes for paper:

They say they stick to JavaScript.

## Reproduction notes

Installing dependencies: (Python 3.7)

The package requires PyStemmer 1.3, which looks like is not maintained right now and errors out. The src-d people made a fork.

```
pip3 install pip install git+git://github.com/snowballstem/pystemmer
sudo apt install --reinstall python3
pip3 install bblfsh
pip3 install jinja
```

Misc tools (superset):

```
pip3 install --ignore-installed PyYAML
pip3 install pylint pytest flake8 mypy pre-commit isort black
```

Installing dependencies:

```
pip3 install -e .
```

Installing things:

```
pip3 install lookout
pip3 install lookout-sdk-ml
pip3 install lookout-style
```

They talk about their datasets in here: https://github.com/src-d/datasets

### Projects used for the evaluation:

```
repository
https://github.com/nodejs/node.git
https://github.com/webpack/webpack.git
https://github.com/meteor/meteor.git
https://github.com/facebook/react.git
https://github.com/atom/atom.git
https://github.com/facebook/react-native.git
https://github.com/jquery/jquery.git
https://github.com/storybookjs/storybook.git
https://github.com/freeCodeCamp/freeCodeCamp.git
https://github.com/expressjs/express.git
https://github.com/30-seconds/30-seconds-of-code.git
https://github.com/evergreen-ci/evergreen.git
https://github.com/nodejs/citgm.git
https://github.com/axios/axios.git
https://github.com/facebook/create-react-app.git
https://github.com/reduxjs/redux.git
https://github.com/hakimel/reveal.js.git
https://github.com/GoogleChromeLabs/carlo.git
https://github.com/laravel/telescope.git
```

Running the docker container:

```bash
docker run --rm -it --network host \
    -v /home/kondziu/Workspace/style-analyzer/.git:/style-analyzer/.git \
    -v /home/kondziu/Workspace/style-analyzer/lookout/core/server:/style-analyzer/lookout/core/server \
    --entrypoint /bin/sh \
    -v/home/kondziu/Workspace/style-analyzer/sdk:/style-analyzer/sdk \
    -w /style-analyzer \
    srcd/style-analyzer:latest
```

Variables:

```bash
REPORTS_DIR="$(pwd)/lookout/style/format/benchmarks/reports"
REPORT_VERSION=untagged
REPORT_DIR="$REPORTS_DIR/$REPORT_VERSION"
SMOKE_REPORT_DIR="$REPORT_DIR/js_smoke"
NOISY_REPORT_DIR="$REPORT_DIR/noise"
QUALITY_REPORT_DIR="$REPORT_DIR/quality"
SMOKE_INIT="./lookout/style/format/benchmarks/data/js_smoke_init.tar.xz"
QUALITY_REPORT_REPOS="./lookout/style/format/benchmarks/data/quality_report_repos_small.csv"
QUALITY_REPORT_REPOS_WITH_VNODE="./lookout/style/format/benchmarks/data/quality_report_repos_with_vnodes_number.csv"
BASE_REPORT_VERSION="0.1.0"
```

```bash
python3 -m lookout.style.format --log-level DEBUG quality-report -o $QUALITY_REPORT_DIR -i $QUALITY_REPORT_REPOS 2>&1 | tee -a $QUALITY_REPORT_DIR/logs.txt
```

```bash
python3 -m lookout.style.format --log-level DEBUG quality-report -o ${QUALITY_REPORT_DIR}.new -i $QUALITY_REPORT_REPOS 2>&1 | tee -a ${QUALITY_REPORT_DIR}.new/logs.txt
```

### Results

Interesting bits of the log:

```
DEBUG:FeaturesExtractor:Removed 9/17 labels by support 80 
DEBUG:FeaturesExtractor:12800 out of 25405 are labeled and saved after filtering
...
INFO:QualityReportAnalyzer:extracted 12800 samples to train, searching for the best hyperparameters
```

This gives us
```
            in log      in log      paper       paper 
project     labels      samples     labels      samples
carlo       9/17 (8)    12800       8           5529
reveal.js   37/47 (10)  52262       14          9974
```

There's more interesting stuff in there. TODO

### Reports

It looks like the report used in the paper is deposited at:

```bash
$QUALITY_REPORT_DIR/telescope-model_report.md
```

rules and avg rules are in the `Summary` section



### Stuff

### Other notes

I suspect in the Makefile there might be a typo and `srcd/style-analyzer:test` should be `srcd/style-analyzer:latest`

I had a problem with apt_pkg. Making a manual link as advised here helped (36 to 37): https://stackoverflow.com/questions/13708180/python-dev-installation-error-importerror-no-module-named-apt-pkg