# Model report for file:///tmp/top-repos-quality-repos-t0kdfbrc/savia2html.git HEAD a5091cbedf6049d6b5429f976c631a26b985cd2f

### Dump

```json
{'created_at': '2021-08-22 05:36:33',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '25.9 kB',
 'tags': [],
 'uuid': 'c2293a94-d6d9-400f-b187-b854a6e1a23c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t0kdfbrc/savia2html.git a5091cbedf6049d6b5429f976c631a26b985cd2f

# javascript
260 rules, avg.len. 6.9
## train
PPCR: 1.000000
### report
macro
{'f1-score': 0.7506463710796377,
 'precision': 0.7895837731763672,
 'recall': 0.720711517367501,
 'support': 345872}
micro
{'f1-score': 0.9935843317759171,
 'precision': 0.9935843317759171,
 'recall': 0.9935843317759171,
 'support': 345872}
weighted
{'f1-score': 0.9931507696870225,
 'precision': 0.9930815279508732,
 'recall': 0.9935843317759171,
 'support': 345872}
### report_full
macro
{'f1-score': 0.7506463710796377,
 'precision': 0.7895837731763672,
 'recall': 0.720711517367501,
 'support': 345872}
micro
{'f1-score': 0.9935843317759171,
 'precision': 0.9935843317759171,
 'recall': 0.9935843317759171,
 'support': 345872}
weighted
{'f1-score': 0.9931507696870225,
 'precision': 0.9930815279508732,
 'recall': 0.9935843317759171,
 'support': 345872}
## test
PPCR: 1.000000
### report
macro
{'f1-score': 0.5931079840923144,
 'precision': 0.6643736440392211,
 'recall': 0.6066359368497338,
 'support': 60877}
micro
{'f1-score': 0.9651428289830314,
 'precision': 0.9651428289830314,
 'recall': 0.9651428289830314,
 'support': 60877}
weighted
{'f1-score': 0.965017629942163,
 'precision': 0.9740068572398958,
 'recall': 0.9651428289830314,
 'support': 60877}
### report_full
macro
{'f1-score': 0.5931079840923144,
 'precision': 0.6643736440392211,
 'recall': 0.6066359368497338,
 'support': 60877}
micro
{'f1-score': 0.9651428289830314,
 'precision': 0.9651428289830314,
 'recall': 0.9651428289830314,
 'support': 60877}
weighted
{'f1-score': 0.965017629942163,
 'precision': 0.9740068572398958,
 'recall': 0.9651428289830314,
 'support': 60877}
```

## javascript
### Summary
229 rules, avg.len. 6.8

| | |
|-|-|
|Min support|128|
|Max support|61899|
|Min confidence|0.921521008014679|
|Max confidence|0.9999960064888|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -2.diff_offset ≥ 4<br>	∧ -3.reserved = {<br>	∧ +3.roles in {KEY}<br>⇒ y = "<br>Confidence: 0.976. Support: 145.` |
| 2 | `  -2.diff_offset ≤ 3<br>	∧ +3.roles in {KEY}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.935. Support: 2224.` |
| 3 | `  -2.roles in {MAP}<br>	∧ -3.length ≥ 3<br>	∧ +3.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2223.` |
| 4 | `  -2.roles in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +3.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 534.` |
| 5 | `  -2.roles not in {MAP}<br>	∧ -3.reserved = "<br>	∧ -4.roles in {MAP}<br>	∧ +3.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 202.` |
| 6 | `  -2.roles not in {MAP}<br>	∧ -3.reserved not in {"}<br>	∧ -4.roles in {MAP}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.981. Support: 289.` |
| 7 | `  -2.roles not in {MAP}<br>	∧ -4.roles in {MAP}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>⇒ y = ␣␣<br>Confidence: 0.947. Support: 2297.` |
| 8 | `  -2.roles not in {MAP}<br>	∧ -3.label in {<space>}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2183.` |
| 9 | `  -2.roles not in {MAP}<br>	∧ +1.roles in {RIGHT}<br>⇒ y = "<br>Confidence: 0.991. Support: 274.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 1.000. Support: 1107.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = ]<br>	∧ -4.diff_line ≥ 1<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 2109.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {]}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 13927.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {LITERAL}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 55607.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = -<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4360.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ -5.length ≥ 3<br>	∧ +3.roles not in {KEY}<br>	∧ +4.length ≥ 2<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 16 | `  •••start_col ≤ 79<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ -5.length ≤ 2<br>	∧ +3.roles not in {KEY}<br>	∧ +4.length ≥ 2<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1654.` |
| 17 | `  •••start_col ≥ 74<br>	∧ •••start_line ≥ 36<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_col ≤ 3<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +4.length ≥ 2<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 1580.` |
| 18 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≤ 3<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +3.roles not in {KEY}<br>	∧ +4.length ≥ 2<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 13615.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +3.roles not in {KEY}<br>	∧ +4.length ≤ 1<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 7033.` |
| 20 | `  •••start_col ≤ 78<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +2.length ≥ 2<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1011.` |
| 21 | `  •••start_col ≤ 75<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {-}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 4<br>	∧ -4.roles not in {MAP}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2275.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -3.roles not in {NUMBER}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 15246.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 18<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles in {LEFT}<br>⇒ y = "<br>Confidence: 0.963. Support: 177.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ -2.roles in {IDENTIFIER} and not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 18<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles not in {LEFT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 225.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ -2.roles in {IDENTIFIER} and not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 18<br>	∧ -4.roles not in {MAP}<br>	∧ +3.roles not in {KEY}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles not in {LEFT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 290.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 18<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>⇒ y = "<br>Confidence: 0.935. Support: 409.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER} and not in {MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≤ 17<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1092.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {IDENTIFIER, MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≤ 2<br>	∧ -4.diff_line = 0<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles in {MODULE}<br>⇒ y = ⏎⏎<br>Confidence: 0.938. Support: 234.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {IDENTIFIER, MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.label in {"}<br>	∧ +1.roles not in {RIGHT, STRING}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles in {LEFT} and not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 194.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {IDENTIFIER, MAP}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {MAP}<br>	∧ -5.label not in {"}<br>	∧ +1.roles not in {RIGHT, STRING}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 10202.` |
| 32 | `  +5.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 2200.` |
| 33 | `  +1.roles in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 25261.` |
| 34 | `  -1.internal_type = NumericLiteral<br>	∧ -5.length ≥ 2<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 26017.` |
| 35 | `  •••start_col ≥ 73<br>	∧ -1.internal_type not in {NumericLiteral}<br>	∧ -5.length ≥ 2<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.roles in {EXPRESSION, MAP}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 344.` |
| 36 | `  •••start_col ≤ 72<br>	∧ -1.internal_type not in {NumericLiteral}<br>	∧ -5.length ≥ 2<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 14543.` |
| 37 | `  -5.length ≤ 1<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ +3.length ≥ 4<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.957. Support: 1918.` |
| 38 | `  -5.length ≤ 1<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 4<br>	∧ +3.length ≤ 3<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 47252.` |
| 39 | `  +3.roles in {COMMENT}<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2172.` |
| 40 | `  -3.label in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2051.` |
| 41 | `  -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≥ 27<br>	∧ +3.internal_type = Identifier<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.951. Support: 461.` |
| 42 | `  -1.diff_col ≥ 15<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≥ 27<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.985. Support: 362.` |
| 43 | `  -1.reserved = [<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≤ 26<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.999. Support: 411.` |
| 44 | `  -1.reserved not in {[}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≤ 26<br>	∧ +2.reserved = .<br>	∧ +3.roles not in {COMMENT, STRING}<br>	∧ +4.length ≥ 1<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {FILE, MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 394.` |
| 45 | `  -1.reserved not in {[}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≤ 26<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved = [<br>	∧ +3.roles not in {COMMENT, STRING}<br>	∧ +4.length ≥ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {FILE, MAP}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 235.` |
| 46 | `  -1.reserved not in {[}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≤ 26<br>	∧ -5.length ≥ 7<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved not in {[}<br>	∧ +3.roles not in {COMMENT, STRING}<br>	∧ +4.length ≥ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {FILE, MAP}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 178.` |
| 47 | `  -1.reserved not in {[}<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_col ≥ 15<br>	∧ -5.diff_col ≤ 26<br>	∧ +2.reserved not in {.}<br>	∧ +3.roles not in {COMMENT, STRING}<br>	∧ +4.length ≤ 2<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles not in {FILE, MAP}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 552.` |
| 48 | `  -3.reserved not in {,}<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type = NumericLiteral<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣␣<br>Confidence: 0.962. Support: 2249.` |
| 49 | `  •••start_col ≥ 8<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.998. Support: 293.` |
| 50 | `  •••start_col ≥ 8<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ +3.roles not in {COMMENT}<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 256.` |
| 51 | `  •••start_col ≥ 8<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ +3.roles not in {COMMENT}<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 11455.` |
| 52 | `  •••start_col ≤ 7<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎⏎<br>Confidence: 0.951. Support: 295.` |
| 53 | `  •••start_col ≤ 7<br>	∧ -4.diff_col ≤ 14<br>	∧ -4.internal_type not in {NumericLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.roles not in {COMMENT}<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 627.` |
| 54 | `  +5.roles in {COMMENT}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 2177.` |
| 55 | `  -1.roles in {COMMENT}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +5.roles not in {COMMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 2207.` |
| 56 | `  -1.roles not in {COMMENT}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +2.length ≥ 13<br>	∧ +4.internal_type = NumericLiteral<br>⇒ y = ␣␣<br>Confidence: 0.998. Support: 1648.` |
| 57 | `  -1.roles not in {COMMENT}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +2.length ≥ 13<br>	∧ +4.internal_type not in {NumericLiteral}<br>	∧ +5.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 535.` |
| 58 | `  -1.roles not in {COMMENT}<br>	∧ -2.roles in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +5.roles in {NUMBER}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 2338.` |
| 59 | `  -1.roles not in {COMMENT}<br>	∧ -2.roles in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +2.length ≤ 12<br>	∧ +3.length ≤ 1<br>	∧ +5.roles not in {COMMENT, NUMBER}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 321.` |
| 60 | `  -1.roles not in {COMMENT}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type = Identifier<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.length ≤ 12<br>	∧ +5.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1027.` |
| 61 | `  -1.roles not in {COMMENT}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type = Identifier<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +4.length ≥ 3<br>⇒ y = "<br>Confidence: 0.922. Support: 618.` |
| 62 | `  -1.roles not in {COMMENT}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 7<br>⇒ y = ␣␣<br>Confidence: 0.949. Support: 420.` |
| 63 | `  -1.diff_offset ≥ 8<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved = ]<br>⇒ y = "<br>Confidence: 0.999. Support: 490.` |
| 64 | `  -1.diff_offset ≥ 8<br>	∧ -1.roles in {IDENTIFIER} and not in {COMMENT}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.reserved not in {]}<br>	∧ +2.length ≤ 12<br>	∧ +5.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1119.` |
| 65 | `  -1.diff_offset ≥ 8<br>	∧ -1.roles not in {COMMENT, IDENTIFIER}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {]}<br>⇒ y = "<br>Confidence: 0.966. Support: 540.` |
| 66 | `  -1.diff_offset ≤ 7<br>	∧ -1.roles not in {COMMENT, VALUE}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +2.reserved not in {)}<br>	∧ +2.length ≤ 12<br>	∧ +5.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ +5.length ≥ 1<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ∅<br>Confidence: 0.949. Support: 2774.` |
| 67 | `  -1.diff_offset ≤ 7<br>	∧ -1.roles not in {COMMENT, VALUE}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {)}<br>	∧ +2.length ≤ 12<br>	∧ +5.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ +5.length ≥ 1<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 2866.` |
| 68 | `  -1.diff_offset ≤ 7<br>	∧ -1.roles not in {COMMENT, VALUE}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {KEY}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +2.reserved not in {)}<br>	∧ +2.length ≤ 12<br>	∧ +5.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ +5.length ≥ 1<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 18399.` |
| 69 | `  -4.diff_offset ≤ 6<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 50730.` |
| 70 | `  -4.diff_offset ≤ 6<br>	∧ +1.internal_type = Identifier<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^1.internal_type = ExpressionStatement<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 231.` |
| 71 | `  •••start_col ≥ 66<br>	∧ -1.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 5411.` |
| 72 | `  •••start_col ≥ 66<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles in {CALL} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 73 | `  •••start_col ≤ 78<br>	∧ •••start_line ≥ 23<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.diff_col ≥ 7<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 74 | `  •••start_col ≥ 74<br>	∧ •••start_line ≥ 184<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -5.roles in {LITERAL}<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ +2.length ≥ 2<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 949.` |
| 75 | `  •••start_col ≤ 73<br>	∧ •••start_line ≥ 184<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -5.roles in {LITERAL}<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ +2.length ≥ 2<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 663.` |
| 76 | `  •••start_col ≥ 74<br>	∧ •••start_line ≤ 184<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -5.roles in {LITERAL}<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 662.` |
| 77 | `  •••start_col ≤ 73<br>	∧ •••start_line ≤ 184<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -5.roles in {LITERAL}<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 621.` |
| 78 | `  •••start_col ≥ 66<br>	∧ •••start_line ≤ 22<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type = NumericLiteral<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 79 | `  •••start_col ≥ 66<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {CALL, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 311.` |
| 80 | `  •••start_col ≥ 66<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.965. Support: 1869.` |
| 81 | `  •••start_col ≥ 66<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 7986.` |
| 82 | `  •••start_col ≤ 65<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +5.reserved = [<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.969. Support: 145.` |
| 83 | `  •••start_col ≤ 65<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +5.reserved not in {[}<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 33612.` |
| 84 | `  -2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2200.` |
| 85 | `  -2.roles not in {EXPRESSION}<br>	∧ -4.roles in {NUMBER}<br>	∧ ^1.internal_type = File<br>⇒ y = ␣␣<br>Confidence: 1.000. Support: 2177.` |
| 86 | `  -2.roles not in {EXPRESSION}<br>	∧ -4.roles not in {NUMBER}<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4238.` |
| 87 | `  -2.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 3<br>	∧ -4.roles not in {NUMBER}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2117.` |
| 88 | `  -2.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 2<br>	∧ -4.roles not in {NUMBER}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type = File<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 2137.` |
| 89 | `  -2.internal_type = NumericLiteral<br>	∧ +4.reserved = :<br>	∧ ^1.internal_type not in {File}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 2078.` |
| 90 | `  -1.diff_col ≥ 8<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1135.` |
| 91 | `  -1.diff_col ≥ 8<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.diff_offset ≥ 7<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = "<br>Confidence: 0.975. Support: 965.` |
| 92 | `  -1.diff_col ≥ 8<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≥ 133<br>	∧ -5.diff_offset ≤ 6<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 230.` |
| 93 | `  -1.diff_col ≥ 8<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.diff_offset ≤ 6<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 241.` |
| 94 | `  -1.diff_col ≤ 7<br>	∧ -2.label in {<newline>}<br>	∧ -2.length ≥ 9<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 752.` |
| 95 | `  -1.diff_col ≤ 7<br>	∧ -2.length ≤ 8<br>	∧ +1.roles in {KEY}<br>	∧ +4.reserved not in {:}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 292.` |
| 96 | `  -1.diff_col ≤ 7<br>	∧ -2.length ≤ 8<br>	∧ -4.diff_col ≥ 41<br>	∧ +1.roles in {KEY}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 2115.` |
| 97 | `  -1.diff_col ≤ 7<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = "<br>Confidence: 0.996. Support: 342.` |
| 98 | `  •••start_col ≤ 73<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.reserved = ,<br>	∧ -2.length ≤ 8<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved = [<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 25721.` |
| 99 | `  -1.diff_col ≤ 7<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≤ 8<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved = [<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 408.` |
| 100 | `  -1.diff_col ≤ 7<br>	∧ -2.length ≤ 8<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved not in {[}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 347.` |
| 101 | `  -1.diff_col ≤ 7<br>	∧ -1.roles not in {VALUE}<br>	∧ -2.length ≤ 8<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved not in {[}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {ARGUMENT} and not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 452.` |
| 102 | `  -1.diff_col ≤ 7<br>	∧ -2.length ≤ 8<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved not in {[}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {INITIALIZATION} and not in {ARGUMENT, MODULE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 19821.` |
| 103 | `  -1.diff_col ≤ 7<br>	∧ -2.length ≤ 8<br>	∧ -4.roles not in {RIGHT}<br>	∧ +1.reserved not in {[}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {ARGUMENT, INITIALIZATION, MODULE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3404.` |
| 104 | `  +3.length ≥ 4<br>	∧ ^2.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1043.` |
| 105 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ]<br>	∧ +3.length ≥ 4<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 2125.` |
| 106 | `  -4.reserved = ,<br>	∧ +1.reserved not in {]}<br>	∧ +3.length ≥ 4<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 2193.` |
| 107 | `  -1.roles in {LITERAL}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved not in {]}<br>	∧ +3.length ≥ 4<br>	∧ ^2.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.981. Support: 130.` |
| 108 | `  -1.roles not in {LITERAL}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved not in {]}<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≥ 4<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.internal_type not in {Program}<br>	∧ ^2.roles in {ARITHMETIC} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 267.` |
| 109 | `  +3.length ≤ 3<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 60161.` |
| 110 | `  +2.roles in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ +5.reserved = ,<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ␣␣<br>Confidence: 1.000. Support: 2260.` |
| 111 | `  +2.roles in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ +5.reserved not in {,}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 216.` |
| 112 | `  •••start_col ≤ 73<br>	∧ -5.diff_col ≥ 6<br>	∧ +1.reserved = [<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14903.` |
| 113 | `  -5.diff_col ≤ 5<br>	∧ +1.reserved = [<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 11189.` |
| 114 | `  +1.reserved not in {[}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 21878.` |
| 115 | `  -3.label in {<space>}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2033.` |
| 116 | `  -1.roles in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = "<br>Confidence: 0.999. Support: 645.` |
| 117 | `  -1.roles not in {STRING}<br>	∧ +1.length ≥ 4<br>	∧ +3.length ≤ 3<br>	∧ +5.roles in {COMMENT}<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 2116.` |
| 118 | `  -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = "<br>Confidence: 0.994. Support: 736.` |
| 119 | `  -1.roles not in {STRING}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.reserved = [<br>	∧ +3.length ≤ 3<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 120 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 4<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ +5.roles not in {COMMENT}<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 2568.` |
| 121 | `  -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≤ 3<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 14422.` |
| 122 | `  -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 129<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label not in {<space>}<br>	∧ +1.length ≤ 3<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 123 | `  +2.internal_type = CommentLine<br>⇒ y = ␣␣<br>Confidence: 0.997. Support: 2130.` |
| 124 | `  -3.diff_col ≥ 45<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 2040.` |
| 125 | `  -2.label in {"}<br>	∧ -3.diff_col ≤ 45<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved = :<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 126 | `  +1.internal_type = StringLiteral<br>	∧ +2.internal_type not in {CommentLine}<br>⇒ y = "<br>Confidence: 0.998. Support: 1141.` |
| 127 | `  -1.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 1.000. Support: 1208.` |
| 128 | `  -1.roles not in {STRING}<br>	∧ -2.internal_type = NumericLiteral<br>	∧ +2.length ≥ 2<br>	∧ +3.length ≥ 4<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2169.` |
| 129 | `  -1.roles not in {STRING}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {:}<br>	∧ +2.length ≥ 2<br>	∧ +3.length ≥ 4<br>⇒ y = ∅<br>Confidence: 0.971. Support: 226.` |
| 130 | `  -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles in {BLOCK}<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≥ 4<br>⇒ y = ⏎⏎<br>Confidence: 0.996. Support: 141.` |
| 131 | `  -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {:}<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≥ 4<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1893.` |
| 132 | `  -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +2.length ≤ 1<br>	∧ +3.length ≥ 4<br>	∧ ^1.roles in {VALUE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 2149.` |
| 133 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {:}<br>	∧ +3.length ≤ 3<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 59754.` |
| 134 | `  -1.roles not in {STRING}<br>	∧ -5.label in {<space>}<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2070.` |
| 135 | `  -1.roles not in {STRING}<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {:}<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 32307.` |
| 136 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.reserved not in {:}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 36420.` |
| 137 | `  +3.roles in {COMMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 2191.` |
| 138 | `  -2.label in {<space>}<br>	∧ +2.roles in {COMMENT}<br>	∧ +3.roles not in {COMMENT}<br>⇒ y = ␣␣<br>Confidence: 1.000. Support: 2156.` |
| 139 | `  -2.label in {<space>}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2278.` |
| 140 | `  -2.label not in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles in {VALUE}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 2074.` |
| 141 | `  -2.label not in {<space>}<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles not in {VALUE}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 142 | `  -2.label not in {<space>}<br>	∧ -3.label in {<space>}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2194.` |
| 143 | `  -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ +5.roles in {BINARY}<br>⇒ y = "<br>Confidence: 0.984. Support: 274.` |
| 144 | `  -2.label not in {<space>}<br>	∧ -3.diff_col ≥ 18<br>	∧ -3.label not in {<space>}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {COMMENT}<br>	∧ +5.reserved = ]<br>	∧ +5.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 145 | `  -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ -5.length ≥ 3<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles in {KEY} and not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 294.` |
| 146 | `  -2.label not in {<space>}<br>	∧ -2.roles in {NUMBER}<br>	∧ -3.label not in {<space>}<br>	∧ -5.length ≤ 2<br>	∧ +3.roles in {KEY}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 2031.` |
| 147 | `  -1.roles in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>⇒ y = "<br>Confidence: 0.999. Support: 526.` |
| 148 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles in {STRING}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ +5.length ≤ 4<br>	∧ ^1.roles in {ADD}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 223.` |
| 149 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {ADD}<br>⇒ y = "<br>Confidence: 0.998. Support: 205.` |
| 150 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles in {IDENTIFIER}<br>	∧ +1.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.929. Support: 177.` |
| 151 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {CALL, MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 345.` |
| 152 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {CALL, MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ +3.length ≥ 2<br>	∧ ^1.roles not in {ADD}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 772.` |
| 153 | `  •••start_col ≥ 73<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = ]<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.label in {"}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {ADD}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 154 | `  •••start_col ≤ 72<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = ]<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {ADD}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 25832.` |
| 155 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {]}<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {ADD}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 30526.` |
| 156 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -5.diff_col ≤ 1<br>	∧ +1.internal_type = Identifier<br>⇒ y = ⏎⏎<br>Confidence: 0.992. Support: 298.` |
| 157 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.label not in {<space>}<br>	∧ -5.diff_col ≤ 1<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {COMMENT, KEY}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 712.` |
| 158 | `  -4.diff_col ≤ 10<br>	∧ +3.length ≥ 4<br>	∧ +4.reserved = [<br>⇒ y = ␣<br>Confidence: 0.954. Support: 2208.` |
| 159 | `  -1.reserved = .<br>	∧ +3.length ≥ 4<br>	∧ +4.reserved not in {[}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1478.` |
| 160 | `  -1.reserved not in {.}<br>	∧ -2.internal_type = NumericLiteral<br>	∧ +3.length ≥ 4<br>	∧ +4.reserved not in {[}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.957. Support: 2151.` |
| 161 | `  -1.reserved not in {.}<br>	∧ -1.roles in {LITERAL}<br>	∧ -2.diff_offset ≥ 5<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ +3.length ≥ 4<br>	∧ +4.reserved not in {[}<br>⇒ y = "<br>Confidence: 0.965. Support: 557.` |
| 162 | `  -1.reserved not in {.}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.diff_offset ≥ 5<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.length ≤ 2<br>	∧ +3.length ≥ 4<br>	∧ +4.reserved not in {[}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 174.` |
| 163 | `  +3.length ≤ 3<br>	∧ +5.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 2073.` |
| 164 | `  -3.diff_col ≥ 4<br>	∧ -5.length ≥ 5<br>	∧ +1.roles in {BINARY}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 165 | `  -1.roles in {LITERAL}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.diff_offset ≥ 18<br>	∧ -5.length ≥ 5<br>	∧ +1.roles not in {BINARY}<br>	∧ +3.length ≤ 3<br>⇒ y = "<br>Confidence: 0.997. Support: 152.` |
| 166 | `  -1.roles not in {LITERAL}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.diff_offset ≥ 18<br>	∧ -5.length ≥ 5<br>	∧ +1.roles not in {BINARY}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 145.` |
| 167 | `  -3.diff_col ≥ 4<br>	∧ -3.diff_offset ≤ 17<br>	∧ -5.length ≥ 5<br>	∧ +1.roles not in {BINARY}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 4761.` |
| 168 | `  -3.diff_col ≤ 3<br>	∧ -5.length ≥ 5<br>	∧ +2.length ≥ 2<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ␣␣<br>Confidence: 0.982. Support: 2310.` |
| 169 | `  -3.diff_col ≤ 3<br>	∧ -5.length ≥ 5<br>	∧ +2.length ≤ 2<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5473.` |
| 170 | `  -2.internal_type = Identifier<br>	∧ -4.length ≥ 5<br>	∧ -5.length ≤ 4<br>	∧ +1.roles in {EXPRESSION, LITERAL}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = "<br>Confidence: 0.990. Support: 460.` |
| 171 | `  -2.internal_type = Identifier<br>	∧ -4.length ≥ 5<br>	∧ -5.length ≤ 4<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 172 | `  -2.internal_type = Identifier<br>	∧ -5.length ≤ 4<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 212.` |
| 173 | `  -1.diff_col ≥ 10<br>	∧ -1.roles in {LITERAL}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.diff_offset ≥ 14<br>	∧ -5.length ≤ 4<br>	∧ +3.length ≤ 3<br>⇒ y = "<br>Confidence: 0.998. Support: 296.` |
| 174 | `  -1.diff_col ≤ 9<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -5.length ≤ 4<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.roles in {LIST}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 53938.` |
| 175 | `  -1.diff_col ≤ 9<br>	∧ -5.length ≤ 4<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ ^1.internal_type = File<br>	∧ ^2.roles not in {LIST}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 196.` |
| 176 | `  •••start_col ≥ 73<br>	∧ -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1420.` |
| 177 | `  •••start_col ≥ 73<br>	∧ •••start_line ≤ 24<br>	∧ -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 483.` |
| 178 | `  •••start_col ≤ 72<br>	∧ -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 27835.` |
| 179 | `  -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.internal_type = ObjectProperty<br>⇒ y = ∅<br>Confidence: 1.000. Support: 22299.` |
| 180 | `  •••start_col ≥ 74<br>	∧ -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles in {LITERAL}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 166.` |
| 181 | `  -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.internal_type = NumericLiteral<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4885.` |
| 182 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type = Identifier<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.reserved not in {]}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.internal_type not in {NumericLiteral}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1415.` |
| 183 | `  -1.diff_col ≤ 9<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -5.length ≤ 4<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.internal_type not in {NumericLiteral}<br>	∧ +3.length ≤ 3<br>	∧ +5.internal_type not in {CommentLine}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1875.` |
| 184 | `  -1.diff_col ≥ 10<br>	∧ +3.reserved = ,<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.965. Support: 1994.` |
| 185 | `  -1.diff_col ≥ 10<br>	∧ -1.length ≤ 2<br>	∧ +3.reserved not in {,}<br>	∧ +3.roles in {EXPRESSION}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 200.` |
| 186 | `  -1.diff_col ≤ 9<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.932. Support: 1296.` |
| 187 | `  -1.diff_col ≤ 9<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = :<br>⇒ y = ␣␣<br>Confidence: 0.967. Support: 2142.` |
| 188 | `  -1.diff_col ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = :<br>	∧ +1.roles not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 555.` |
| 189 | `  -1.diff_col ≤ 9<br>	∧ +3.roles in {COMMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 2210.` |
| 190 | `  -1.diff_col ≤ 9<br>	∧ -2.diff_offset ≥ 9<br>	∧ -3.length ≥ 9<br>	∧ -5.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.985. Support: 434.` |
| 191 | `  -1.diff_col ≤ 9<br>	∧ -4.diff_col ≥ 35<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 2028.` |
| 192 | `  -1.diff_col ≤ 9<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≥ 35<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ +3.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4904.` |
| 193 | `  -1.diff_col ≤ 9<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≥ 35<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {EXPRESSION, KEY, STRING}<br>	∧ +3.roles not in {COMMENT}<br>	∧ ^1.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 4553.` |
| 194 | `  -1.diff_col ≤ 9<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -5.diff_col ≥ 9<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles in {MAP} and not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 502.` |
| 195 | `  -1.diff_col ≤ 9<br>	∧ -4.diff_col ≤ 34<br>	∧ -5.diff_col ≤ 8<br>	∧ +3.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 2127.` |
| 196 | `  -1.diff_col ≤ 9<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {COMMENT, MAP}<br>	∧ ^2.roles in {VALUE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 56052.` |
| 197 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {LITERAL}<br>	∧ -4.diff_col ≤ 34<br>	∧ -5.diff_col ≥ 6<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 198 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {LITERAL}<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -4.roles not in {BINARY}<br>	∧ -5.diff_col ≥ 6<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {COMMENT, MAP}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 24204.` |
| 199 | `  -1.diff_col ≤ 9<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -4.roles in {EXPRESSION} and not in {BINARY}<br>	∧ -5.diff_col ≥ 6<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {COMMENT, MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 22060.` |
| 200 | `  -1.diff_col ≤ 9<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.label in {<space>}<br>	∧ -4.diff_col ≤ 34<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≥ 6<br>	∧ -5.length ≤ 1<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 182.` |
| 201 | `  -1.diff_col ≤ 9<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -4.roles not in {BINARY, EXPRESSION}<br>	∧ -5.diff_col ≥ 6<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ -5.length ≤ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {COMMENT, MAP}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 5492.` |
| 202 | `  -1.diff_col ≤ 9<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 34<br>	∧ -4.roles not in {BINARY}<br>	∧ -5.diff_col ≤ 5<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {COMMENT, MAP}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 56500.` |
| 203 | `  -2.length ≥ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {MAP}<br>	∧ ^2.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.997. Support: 176.` |
| 204 | `  -2.length ≤ 2<br>	∧ +3.roles in {MAP}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.941. Support: 2204.` |
| 205 | `  +1.roles in {MAP}<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 2098.` |
| 206 | `  +1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 31417.` |
| 207 | `  •••start_col ≥ 74<br>	∧ -2.diff_col ≥ 3<br>	∧ -5.internal_type = NumericLiteral<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved = ,<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 208 | `  •••start_col ≥ 74<br>	∧ -5.internal_type = NumericLiteral<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {,}<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2267.` |
| 209 | `  •••start_col ≥ 74<br>	∧ -5.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.length ≤ 1<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 1807.` |
| 210 | `  •••start_col ≤ 73<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 15901.` |
| 211 | `  -3.roles not in {STRING}<br>	∧ +3.roles not in {MAP}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 61899.` |
| 212 | `  -3.reserved = :<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣␣<br>Confidence: 0.948. Support: 2250.` |
| 213 | `  -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {:}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 1.000. Support: 1097.` |
| 214 | `  -1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {NUMBER}<br>	∧ +4.roles in {NUMBER}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 1.000. Support: 2217.` |
| 215 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {:}<br>	∧ +2.roles in {NUMBER}<br>	∧ +4.roles not in {NUMBER}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 216 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {NUMBER}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.986. Support: 315.` |
| 217 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {KEY}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2188.` |
| 218 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {BLOCK}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 227.` |
| 219 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {BLOCK, KEY}<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.roles in {EXPRESSION}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 6310.` |
| 220 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {BLOCK, KEY}<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +2.roles in {STRING} and not in {NUMBER}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 532.` |
| 221 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {BLOCK, KEY}<br>	∧ -3.reserved not in {:}<br>	∧ +1.reserved = .<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 222 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {BLOCK, KEY}<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_col ≤ 12<br>	∧ +2.roles not in {NUMBER}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 128.` |
| 223 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {NUMBER}<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2387.` |
| 224 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {NUMBER}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles in {BINARY} and not in {MAP}<br>⇒ y = "<br>Confidence: 0.990. Support: 250.` |
| 225 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {NUMBER}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.997. Support: 159.` |
| 226 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {NUMBER}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {:}<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {BINARY, MAP}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 251.` |
| 227 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {NUMBER}<br>	∧ -3.reserved not in {:}<br>	∧ -4.internal_type = Identifier<br>	∧ -5.diff_offset ≥ 19<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 228 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, [}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -3.reserved not in {:}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_offset ≥ 19<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 1881.` |
| 229 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, [}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -3.reserved not in {:}<br>	∧ -5.diff_offset ≤ 18<br>	∧ +2.roles not in {NUMBER}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 3394.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.764192139737991, "max_conf": 0.9999960064888, "max_support": 61899, "min_conf": 0.921521008014679, "min_support": 128, "num_rules": 229}}
```
</details>
