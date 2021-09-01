# Model report for file:///tmp/top-repos-quality-repos-axerpbcy/fcqa.git HEAD a3b72620e7447076e3a3bd70015021d404e51661

### Dump

```json
{'created_at': '2021-08-30 08:02:02',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.0 kB',
 'tags': [],
 'uuid': 'cdca7009-aafe-40c0-8151-52522ede6d31',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-axerpbcy/fcqa.git a3b72620e7447076e3a3bd70015021d404e51661

# javascript
203 rules, avg.len. 7.0
## train
PPCR: 1.000000
### report
macro
{'f1-score': 0.9897465034740817,
 'precision': 0.9906935294362721,
 'recall': 0.9889833297470852,
 'support': 26548}
micro
{'f1-score': 0.9935588368238663,
 'precision': 0.9935588368238663,
 'recall': 0.9935588368238663,
 'support': 26548}
weighted
{'f1-score': 0.9935555031974196,
 'precision': 0.9936143366846654,
 'recall': 0.9935588368238663,
 'support': 26548}
### report_full
macro
{'f1-score': 0.9897465034740817,
 'precision': 0.9906935294362721,
 'recall': 0.9889833297470852,
 'support': 26548}
micro
{'f1-score': 0.9935588368238663,
 'precision': 0.9935588368238663,
 'recall': 0.9935588368238663,
 'support': 26548}
weighted
{'f1-score': 0.9935555031974196,
 'precision': 0.9936143366846654,
 'recall': 0.9935588368238663,
 'support': 26548}
## test
PPCR: 1.000000
### report
macro
{'f1-score': 0.9899102560282976,
 'precision': 0.9912643264991093,
 'recall': 0.9886913201641967,
 'support': 6581}
micro
{'f1-score': 0.9939218963683331,
 'precision': 0.9939218963683331,
 'recall': 0.9939218963683331,
 'support': 6581}
weighted
{'f1-score': 0.993918377751926,
 'precision': 0.9939553935116533,
 'recall': 0.9939218963683331,
 'support': 6581}
### report_full
macro
{'f1-score': 0.9899102560282976,
 'precision': 0.9912643264991093,
 'recall': 0.9886913201641967,
 'support': 6581}
micro
{'f1-score': 0.9939218963683331,
 'precision': 0.9939218963683331,
 'recall': 0.9939218963683331,
 'support': 6581}
weighted
{'f1-score': 0.993918377751926,
 'precision': 0.9939553935116533,
 'recall': 0.9939218963683331,
 'support': 6581}
```

## javascript
### Summary
181 rules, avg.len. 6.9

| | |
|-|-|
|Min support|146|
|Max support|3906|
|Min confidence|0.9222972989082336|
|Max confidence|0.999834418296814|

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
                     'min_samples_split': 238,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -3.diff_offset ≥ 5<br>	∧ +1.roles in {KEY}<br>	∧ +5.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 446.` |
| 2 | `  -3.diff_offset ≤ 4<br>	∧ +1.roles in {KEY}<br>	∧ +5.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 603.` |
| 3 | `  +1.roles in {KEY}<br>	∧ +5.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 562.` |
| 4 | `  -1.reserved = :<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1601.` |
| 5 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ +1.roles not in {KEY}<br>⇒ y = '<br>Confidence: 1.000. Support: 2107.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -3.roles not in {CALL}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1938.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {STRING}<br>	∧ -3.roles not in {CALL}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 2924.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -3.roles not in {CALL}<br>	∧ +1.roles in {CALL} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 921.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 460.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.956. Support: 446.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.internal_type = Identifier<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 897.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 8<br>	∧ -3.roles not in {CALL}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 441.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:, return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 8<br>	∧ -3.roles not in {CALL}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 3796.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 59<br>	∧ -2.diff_offset ≤ 7<br>	∧ -3.roles not in {CALL}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.length ≤ 2<br>	∧ +1.roles not in {CALL, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.922. Support: 444.` |
| 15 | `  -3.roles in {VALUE}<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 1096.` |
| 16 | `  -3.label in {<newline>}<br>	∧ -3.roles not in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 2264.` |
| 17 | `  -1.roles in {CALL}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {VALUE}<br>⇒ y = '<br>Confidence: 0.980. Support: 1752.` |
| 18 | `  -1.roles in {VALUE} and not in {CALL}<br>	∧ -1.length ≥ 9<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {VALUE}<br>⇒ y = '<br>Confidence: 0.999. Support: 933.` |
| 19 | `  -1.roles not in {CALL, VALUE}<br>	∧ -1.length ≥ 9<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 3732.` |
| 20 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {VALUE}<br>	∧ -5.label in {<-space>}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 524.` |
| 21 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {'}<br>	∧ -3.roles not in {VALUE}<br>	∧ -5.label not in {<-space>}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1287.` |
| 22 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {'}<br>	∧ -3.roles not in {VALUE}<br>	∧ -5.label not in {<-space>}<br>	∧ +4.internal_type not in {Identifier}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 406.` |
| 23 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles in {MAP} and not in {VALUE}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = '<br>Confidence: 1.000. Support: 1226.` |
| 24 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -2.label in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 863.` |
| 25 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -2.label not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = '<br>Confidence: 1.000. Support: 1713.` |
| 26 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ -5.roles in {VALUE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 548.` |
| 27 | `  -1.diff_col ≥ 2<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ -4.diff_col ≥ 11<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 867.` |
| 28 | `  -1.diff_col ≤ 1<br>	∧ -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ -4.diff_col ≥ 11<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 500.` |
| 29 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ -4.diff_col ≤ 10<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 897.` |
| 30 | `  -1.roles not in {CALL}<br>	∧ -1.length ≤ 8<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label not in {<newline>}<br>	∧ -3.roles not in {MAP, VALUE}<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +4.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 451.` |
| 31 | `  -1.reserved = )<br>	∧ -2.reserved = '<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 399.` |
| 32 | `  -1.reserved not in {)}<br>	∧ -2.reserved = '<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 1035.` |
| 33 | `  -2.reserved = '<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 1371.` |
| 34 | `  -1.reserved = {<br>	∧ -2.reserved not in {'}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 1.000. Support: 1040.` |
| 35 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles in {KEY}<br>	∧ -4.diff_line ≥ 1<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1570.` |
| 36 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_line ≥ 1<br>	∧ +2.roles in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 556.` |
| 37 | `  -1.reserved not in {{}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_line ≥ 1<br>	∧ +2.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1558.` |
| 38 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_line ≥ 1<br>	∧ +2.roles not in {KEY}<br>	∧ +4.reserved = :<br>⇒ y = ∅<br>Confidence: 0.999. Support: 556.` |
| 39 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type = Identifier<br>	∧ +2.roles not in {KEY}<br>	∧ +4.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 382.` |
| 40 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {KEY}<br>	∧ -4.diff_col ≥ 62<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +2.roles not in {KEY}<br>	∧ +4.reserved not in {:}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.964. Support: 430.` |
| 41 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label in {<space>}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2200.` |
| 42 | `  -1.reserved = ||<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 517.` |
| 43 | `  -1.reserved not in {{, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ ^1.roles in {CALLEE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1274.` |
| 44 | `  -1.reserved not in {{, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ +3.roles in {FUNCTION}<br>	∧ ^1.roles not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 425.` |
| 45 | `  -1.reserved not in {{, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ +3.roles not in {FUNCTION}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.roles in {BOOLEAN}<br>⇒ y = '<br>Confidence: 0.973. Support: 2355.` |
| 46 | `  -1.reserved not in {{, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +3.roles not in {FUNCTION}<br>	∧ ^1.roles not in {CALLEE}<br>	∧ ^2.roles not in {BOOLEAN}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 529.` |
| 47 | `  -1.reserved not in {{, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {FUNCTION}<br>	∧ ^1.roles in {VALUE} and not in {CALLEE}<br>	∧ ^2.roles not in {BOOLEAN}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 459.` |
| 48 | `  -1.reserved not in {;, {, ||}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +3.roles not in {FUNCTION}<br>	∧ ^1.roles not in {CALLEE, VALUE}<br>	∧ ^2.roles not in {BOOLEAN}<br>⇒ y = '<br>Confidence: 0.960. Support: 3797.` |
| 49 | `  -3.label in {<newline>}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 2231.` |
| 50 | `  -3.label not in {<newline>}<br>	∧ -4.label in {<space>}<br>	∧ +5.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 436.` |
| 51 | `  -1.length ≥ 8<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label in {<space>}<br>	∧ +5.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2732.` |
| 52 | `  -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label in {<space>}<br>	∧ +3.length ≥ 3<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 948.` |
| 53 | `  -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label in {<space>}<br>	∧ +3.length ≤ 2<br>	∧ ^1.roles in {BOOLEAN}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 457.` |
| 54 | `  -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 872.` |
| 55 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 1.000. Support: 2814.` |
| 56 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≥ 23<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 431.` |
| 57 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≥ 23<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {RIGHT} and not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.999. Support: 453.` |
| 58 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≥ 23<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, RIGHT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 511.` |
| 59 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≥ 23<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, RIGHT}<br>⇒ y = '<br>Confidence: 0.959. Support: 1437.` |
| 60 | `  -2.label in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1573.` |
| 61 | `  -2.internal_type = Identifier<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1327.` |
| 62 | `  -1.roles in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 655.` |
| 63 | `  -1.reserved = ||<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 463.` |
| 64 | `  -1.reserved not in {||}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.length ≥ 11<br>⇒ y = ␣<br>Confidence: 0.927. Support: 456.` |
| 65 | `  -1.reserved not in {||}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.length ≤ 10<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 528.` |
| 66 | `  -1.roles not in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.955. Support: 478.` |
| 67 | `  -1.roles in {LITERAL} and not in {ARGUMENT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {EXPRESSION, LITERAL}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.999. Support: 529.` |
| 68 | `  -1.roles in {LITERAL}<br>	∧ -4.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 1.000. Support: 1979.` |
| 69 | `  -1.roles not in {LITERAL}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 818.` |
| 70 | `  -1.roles not in {LITERAL}<br>	∧ -3.reserved = '<br>	∧ -4.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.964. Support: 430.` |
| 71 | `  -1.roles not in {LITERAL}<br>	∧ -3.reserved not in {'}<br>	∧ -4.roles in {ARGUMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 421.` |
| 72 | `  -4.roles not in {EXPRESSION}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 1058.` |
| 73 | `  -4.roles not in {EXPRESSION}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 584.` |
| 74 | `  -2.diff_col ≥ 12<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 404.` |
| 75 | `  -2.diff_col ≥ 12<br>	∧ -3.diff_line ≥ 1<br>	∧ -3.diff_offset ≥ 8<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = FunctionExpression<br>⇒ y = ␣<br>Confidence: 0.999. Support: 420.` |
| 76 | `  -2.diff_col ≥ 12<br>	∧ -3.diff_line ≥ 1<br>	∧ -3.diff_offset ≥ 8<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 807.` |
| 77 | `  -2.diff_col ≥ 12<br>	∧ -3.diff_line ≥ 1<br>	∧ -3.diff_offset ≤ 8<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 408.` |
| 78 | `  -1.roles in {LITERAL}<br>	∧ -2.diff_col ≥ 12<br>	∧ -2.diff_offset ≥ 15<br>	∧ -3.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>⇒ y = '<br>Confidence: 0.999. Support: 377.` |
| 79 | `  -1.roles not in {LITERAL}<br>	∧ -2.diff_col ≥ 12<br>	∧ -2.diff_offset ≥ 15<br>	∧ -3.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 917.` |
| 80 | `  -2.diff_col ≥ 12<br>	∧ -2.diff_offset ≤ 14<br>	∧ -3.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 3227.` |
| 81 | `  -2.diff_col ≤ 11<br>	∧ -3.label in {<newline>}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1745.` |
| 82 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1245.` |
| 83 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.roles not in {KEY}<br>⇒ y = '<br>Confidence: 0.944. Support: 1228.` |
| 84 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≥ 26<br>	∧ -5.diff_line = 0<br>	∧ +1.roles not in {KEY}<br>⇒ y = '<br>Confidence: 0.966. Support: 1385.` |
| 85 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 26<br>	∧ -5.diff_line = 0<br>	∧ -5.label in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ ^2.roles in {RETURN}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 220.` |
| 86 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 26<br>	∧ -5.diff_line = 0<br>	∧ -5.label in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {BOOLEAN}<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 212.` |
| 87 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 26<br>	∧ -5.diff_line = 0<br>	∧ -5.label in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {BOOLEAN}<br>	∧ ^2.roles not in {RETURN}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 391.` |
| 88 | `  -2.diff_col ≤ 11<br>	∧ -3.label not in {<newline>}<br>	∧ -4.diff_line = 0<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 26<br>	∧ -5.diff_line = 0<br>	∧ -5.label not in {<space>}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1496.` |
| 89 | `  -3.internal_type = Identifier<br>	∧ +2.reserved = ,<br>	∧ ^2.internal_type not in {CallExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1016.` |
| 90 | `  -3.internal_type not in {Identifier}<br>	∧ +2.reserved = ,<br>	∧ ^2.internal_type not in {CallExpression}<br>	∧ ^2.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 656.` |
| 91 | `  -3.internal_type not in {Identifier}<br>	∧ +2.reserved = ,<br>	∧ ^2.internal_type not in {CallExpression}<br>	∧ ^2.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1028.` |
| 92 | `  -3.internal_type not in {Identifier}<br>	∧ +2.reserved = ,<br>	∧ ^2.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 195.` |
| 93 | `  -1.roles in {STRING}<br>	∧ +2.reserved not in {,}<br>⇒ y = '<br>Confidence: 1.000. Support: 2749.` |
| 94 | `  -1.roles not in {STRING}<br>	∧ +2.internal_type = StringLiteral<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 687.` |
| 95 | `  -1.diff_col ≥ 2<br>	∧ -1.roles not in {STRING}<br>	∧ +2.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 481.` |
| 96 | `  •••start_col ≥ 67<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = ,<br>	∧ +2.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles in {BINARY}<br>⇒ y = '<br>Confidence: 0.999. Support: 439.` |
| 97 | `  •••start_col ≥ 67<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {,}<br>	∧ +2.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles in {BINARY}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 929.` |
| 98 | `  -1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ ^1.roles in {QUALIFIED} and not in {BINARY}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1947.` |
| 99 | `  -1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 641.` |
| 100 | `  -1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type = StringLiteral<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 564.` |
| 101 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 479.` |
| 102 | `  -1.diff_offset ≥ 9<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 2132.` |
| 103 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label in {<newline>}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1137.` |
| 104 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 859.` |
| 105 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ -3.label not in {<newline>}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +3.reserved = function<br>	∧ ^1.roles not in {BINARY, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 444.` |
| 106 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {function}<br>	∧ ^1.roles in {LIST} and not in {BINARY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 544.` |
| 107 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {function}<br>	∧ ^1.roles in {LIST} and not in {BINARY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 373.` |
| 108 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {function}<br>	∧ ^1.roles in {LIST} and not in {BINARY, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.960. Support: 434.` |
| 109 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 4<br>	∧ -3.diff_offset ≥ 15<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {function}<br>	∧ ^1.roles not in {BINARY, LIST, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 427.` |
| 110 | `  -1.label in {<space>}<br>⇒ y = '<br>Confidence: 1.000. Support: 2112.` |
| 111 | `  -1.label not in {<space>}<br>	∧ -1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 1.000. Support: 2930.` |
| 112 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1872.` |
| 113 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +2.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2025.` |
| 114 | `  -1.diff_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +2.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1434.` |
| 115 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -3.diff_offset ≥ 19<br>	∧ -4.internal_type = Identifier<br>	∧ +2.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 848.` |
| 116 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 19<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.roles in {KEY}<br>	∧ +2.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 570.` |
| 117 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -3.internal_type = StringLiteral<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 234.` |
| 118 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 585.` |
| 119 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved = )<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.956. Support: 422.` |
| 120 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 463.` |
| 121 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles in {LITERAL} and not in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 122 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -3.reserved = ;<br>	∧ -4.roles not in {ARGUMENT, LITERAL}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 378.` |
| 123 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -3.reserved not in {;}<br>	∧ -4.roles not in {ARGUMENT, LITERAL}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 418.` |
| 124 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved = return<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 466.` |
| 125 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 274.` |
| 126 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -1.length ≥ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1336.` |
| 127 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 2055.` |
| 128 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.internal_type = Identifier<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 1022.` |
| 129 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ +5.reserved = {<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 384.` |
| 130 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 18<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.reserved = (<br>	∧ +2.internal_type not in {Identifier}<br>	∧ +2.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {return}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 856.` |
| 131 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2020.` |
| 132 | `  -1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 3020.` |
| 133 | `  -1.roles in {IDENTIFIER} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3372.` |
| 134 | `  -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 883.` |
| 135 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.diff_offset ≥ 27<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 851.` |
| 136 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -5.diff_offset ≤ 26<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1151.` |
| 137 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 2131.` |
| 138 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 984.` |
| 139 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {ARGUMENT}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 902.` |
| 140 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 991.` |
| 141 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 506.` |
| 142 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.length ≥ 8<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARGUMENT}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 917.` |
| 143 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.length ≤ 7<br>	∧ -5.diff_col ≥ 24<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 1012.` |
| 144 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.length ≤ 7<br>	∧ -5.diff_col ≤ 24<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARGUMENT}<br>	∧ ^2.internal_type = LogicalExpression<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 145 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≥ 16<br>	∧ -3.length ≤ 7<br>	∧ -5.diff_col ≤ 24<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARGUMENT}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 346.` |
| 146 | `  -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -3.length ≤ 7<br>	∧ -5.diff_col ≤ 24<br>	∧ +1.roles in {ARGUMENT} and not in {LITERAL}<br>	∧ ^1.roles not in {ARGUMENT}<br>	∧ ^2.internal_type not in {LogicalExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 438.` |
| 147 | `  -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 904.` |
| 148 | `  -1.reserved = ,<br>	∧ -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ⏎<br>Confidence: 1.000. Support: 1029.` |
| 149 | `  -1.reserved not in {,}<br>	∧ -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 414.` |
| 150 | `  -2.reserved = '<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 503.` |
| 151 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.roles in {LIST}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 526.` |
| 152 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.roles not in {LIST}<br>⇒ y = '<br>Confidence: 1.000. Support: 1027.` |
| 153 | `  -1.diff_offset ≥ 10<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 428.` |
| 154 | `  -1.diff_offset ≤ 10<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles not in {LIST}<br>⇒ y = '<br>Confidence: 0.999. Support: 415.` |
| 155 | `  -1.diff_offset ≤ 10<br>	∧ -1.length ≤ 3<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 471.` |
| 156 | `  -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ +3.internal_type not in {Identifier}<br>⇒ y = '<br>Confidence: 0.952. Support: 1812.` |
| 157 | `  -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles in {LIST}<br>⇒ y = '<br>Confidence: 0.999. Support: 496.` |
| 158 | `  -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ +5.roles in {NAME}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = '<br>Confidence: 0.999. Support: 445.` |
| 159 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = '<br>Confidence: 0.999. Support: 702.` |
| 160 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ +5.roles not in {NAME}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3906.` |
| 161 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.diff_offset ≥ 33<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +3.internal_type not in {Identifier}<br>⇒ y = '<br>Confidence: 0.999. Support: 859.` |
| 162 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.diff_offset ≤ 32<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = :<br>	∧ +3.internal_type not in {Identifier}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 543.` |
| 163 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.diff_offset ≤ 32<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {:}<br>	∧ +3.internal_type not in {Identifier}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1295.` |
| 164 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.diff_offset ≤ 32<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {:}<br>	∧ +3.internal_type not in {Identifier}<br>⇒ y = '<br>Confidence: 0.997. Support: 187.` |
| 165 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles in {MAP}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 161.` |
| 166 | `  -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {'}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label not in {<newline>}<br>	∧ -5.reserved = '<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.932. Support: 923.` |
| 167 | `  -2.roles not in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +3.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 387.` |
| 168 | `  -2.roles not in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +3.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2742.` |
| 169 | `  -1.roles in {STRING}<br>	∧ -4.label not in {<space>}<br>⇒ y = '<br>Confidence: 1.000. Support: 2924.` |
| 170 | `  -1.roles not in {STRING}<br>	∧ -4.label not in {<space>}<br>	∧ ^2.roles in {ARGUMENT}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 1092.` |
| 171 | `  -1.roles not in {STRING}<br>	∧ -3.label in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 2292.` |
| 172 | `  -1.roles not in {STRING}<br>	∧ -2.reserved = :<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = '<br>Confidence: 1.000. Support: 1129.` |
| 173 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_offset ≥ 26<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = '<br>Confidence: 1.000. Support: 1669.` |
| 174 | `  -1.roles not in {STRING}<br>	∧ -2.reserved not in {:}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -5.diff_offset ≤ 26<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 874.` |
| 175 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved = ;<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved = '<br>	∧ +1.roles not in {STRING}<br>	∧ +5.roles not in {KEY}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 385.` |
| 176 | `  -1.diff_offset ≥ 5<br>	∧ -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≥ 11<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 452.` |
| 177 | `  -1.diff_offset ≤ 4<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 6<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≥ 11<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.999. Support: 484.` |
| 178 | `  -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≤ 10<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≥ 17<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 444.` |
| 179 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≤ 10<br>	∧ -5.diff_offset ≥ 9<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≤ 16<br>	∧ ^1.roles in {OR}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 186.` |
| 180 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≤ 10<br>	∧ -5.diff_offset ≥ 9<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≤ 16<br>	∧ ^1.roles not in {OR}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 3678.` |
| 181 | `  -1.roles not in {STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≤ 10<br>	∧ -5.diff_offset ≤ 8<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {'}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.length ≤ 7<br>	∧ +3.length ≤ 16<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.900552486187845, "max_conf": 0.999834418296814, "max_support": 3906, "min_conf": 0.9222972989082336, "min_support": 146, "num_rules": 181}}
```
</details>
