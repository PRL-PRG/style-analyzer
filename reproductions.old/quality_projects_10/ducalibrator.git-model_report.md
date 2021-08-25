# Model report for file:///tmp/top-repos-quality-repos-4lpsneg4/ducalibrator.git HEAD 56a9eb2aef79ee473b8cc2e8157f5cd9967b92ad

### Dump

```json
{'created_at': '2021-08-24 22:29:36',
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
 'size': '15.4 kB',
 'tags': [],
 'uuid': 'a90b3d42-7b0d-49b9-8d52-2f93d48c773c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4lpsneg4/ducalibrator.git 56a9eb2aef79ee473b8cc2e8157f5cd9967b92ad

# javascript
40 rules, avg.len. 5.0
## train
PPCR: 0.781829
### report
macro
{'f1-score': 0.2824936153279008,
 'precision': 0.2887285811634252,
 'recall': 0.2789615626520044,
 'support': 7762}
micro
{'f1-score': 0.8736150476681268,
 'precision': 0.8736150476681268,
 'recall': 0.8736150476681268,
 'support': 7762}
weighted
{'f1-score': 0.8581353461632985,
 'precision': 0.8483783381782675,
 'recall': 0.8736150476681268,
 'support': 7762}
### report_full
macro
{'f1-score': 0.24896333978122706,
 'precision': 0.2887285811634252,
 'recall': 0.22748096391525627,
 'support': 9928}
micro
{'f1-score': 0.766647823629169,
 'precision': 0.8736150476681268,
 'recall': 0.6830177276390008,
 'support': 9928}
weighted
{'f1-score': 0.7179590149293908,
 'precision': 0.7845799004153504,
 'recall': 0.6830177276390008,
 'support': 9928}
## test
PPCR: 0.735094
### report
macro
{'f1-score': 0.269996124042247,
 'precision': 0.2911584196394323,
 'recall': 0.2621307087973755,
 'support': 1948}
micro
{'f1-score': 0.8526694045174538,
 'precision': 0.8526694045174538,
 'recall': 0.8526694045174538,
 'support': 1948}
weighted
{'f1-score': 0.8142077413602113,
 'precision': 0.7958016011228655,
 'recall': 0.8526694045174538,
 'support': 1948}
### report_full
macro
{'f1-score': 0.2201373764249516,
 'precision': 0.2911584196394323,
 'recall': 0.19597358022836295,
 'support': 2650}
micro
{'f1-score': 0.7224880382775121,
 'precision': 0.8526694045174538,
 'recall': 0.6267924528301887,
 'support': 2650}
weighted
{'f1-score': 0.6567379330924502,
 'precision': 0.750276353214495,
 'recall': 0.6267924528301887,
 'support': 2650}
```

## javascript
### Summary
31 rules, avg.len. 4.8

| | |
|-|-|
|Min support|138|
|Max support|2464|
|Min confidence|0.9224519729614258|
|Max confidence|0.9983388781547546|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2404.` |
| 2 | `  -2.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 588.` |
| 3 | `  -2.label not in {<space>}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 321.` |
| 4 | `  +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 138.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1454.` |
| 6 | `  +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.990. Support: 148.` |
| 7 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.977. Support: 2458.` |
| 8 | `  +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.982. Support: 139.` |
| 9 | `  -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1190.` |
| 10 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 301.` |
| 11 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 200.` |
| 12 | `  +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 462.` |
| 13 | `  -2.label in {<space>}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 423.` |
| 14 | `  +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.985. Support: 168.` |
| 15 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2464.` |
| 16 | `  •••start_col ≤ 41<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 634.` |
| 17 | `  +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.983. Support: 150.` |
| 18 | `  -1.length ≤ 4<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 415.` |
| 19 | `  •••start_col ≤ 41<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 677.` |
| 20 | `  +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 167.` |
| 21 | `  +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 166.` |
| 22 | `  +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 414.` |
| 23 | `  +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 387.` |
| 24 | `  •••start_col ≥ 16<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 1180.` |
| 25 | `  •••start_col ≥ 16<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 277.` |
| 26 | `  •••start_col ≥ 16<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 185.` |
| 27 | `  +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 413.` |
| 28 | `  +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 394.` |
| 29 | `  +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.991. Support: 170.` |
| 30 | `  •••start_col ≥ 13<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≤ 4<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 348.` |
| 31 | `  •••start_col ≥ 13<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 1508.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.838709677419355, "max_conf": 0.9983388781547546, "max_support": 2464, "min_conf": 0.9224519729614258, "min_support": 138, "num_rules": 31}}
```
</details>
