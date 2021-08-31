# Model report for file:///tmp/top-repos-quality-repos-9wjv15g2/github-buttons.git HEAD 4a0bfa75e18619804dfec24770e8add807130c7b

### Dump

```json
{'created_at': '2021-08-30 05:18:18',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.8 kB',
 'tags': [],
 'uuid': 'ec7ff5d3-873d-4af2-8203-4012651b7b83',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9wjv15g2/github-buttons.git 4a0bfa75e18619804dfec24770e8add807130c7b

# javascript
71 rules, avg.len. 5.9
## train
PPCR: 0.939924
### report
macro
{'f1-score': 0.6319957069347167,
 'precision': 0.6557285875323514,
 'recall': 0.6152255698371387,
 'support': 10107}
micro
{'f1-score': 0.8975957257346393,
 'precision': 0.8975957257346393,
 'recall': 0.8975957257346393,
 'support': 10107}
weighted
{'f1-score': 0.8848372361567786,
 'precision': 0.8788043303245426,
 'recall': 0.8975957257346393,
 'support': 10107}
### report_full
macro
{'f1-score': 0.6120279595767363,
 'precision': 0.6557285875323514,
 'recall': 0.580071618968928,
 'support': 10753}
micro
{'f1-score': 0.8697986577181207,
 'precision': 0.8975957257346393,
 'recall': 0.8436715335255277,
 'support': 10753}
weighted
{'f1-score': 0.8429450919175729,
 'precision': 0.8515713089969729,
 'recall': 0.8436715335255277,
 'support': 10753}
## test
PPCR: 0.944591
### report
macro
{'f1-score': 0.5984333079271983,
 'precision': 0.5880767385961484,
 'recall': 0.6126183517240233,
 'support': 1790}
micro
{'f1-score': 0.8245810055865922,
 'precision': 0.8245810055865922,
 'recall': 0.8245810055865922,
 'support': 1790}
weighted
{'f1-score': 0.7947294823797298,
 'precision': 0.77132314635563,
 'recall': 0.8245810055865922,
 'support': 1790}
### report_full
macro
{'f1-score': 0.5883486270323088,
 'precision': 0.5880767385961484,
 'recall': 0.5934990847917039,
 'support': 1895}
micro
{'f1-score': 0.8010854816824965,
 'precision': 0.8245810055865922,
 'recall': 0.7788918205804749,
 'support': 1895}
weighted
{'f1-score': 0.7625009185222938,
 'precision': 0.7540797824600827,
 'recall': 0.7788918205804749,
 'support': 1895}
```

## javascript
### Summary
41 rules, avg.len. 5.2

| | |
|-|-|
|Min support|136|
|Max support|2949|
|Min confidence|0.9212667346000671|
|Max confidence|0.9993780851364136|

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
                     'max_depth': 10,
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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 538.` |
| 2 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 275.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 226.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.948. Support: 339.` |
| 5 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.923. Support: 319.` |
| 6 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 801.` |
| 7 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 8 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.923. Support: 332.` |
| 9 | `  -1.reserved not in {{}<br>	∧ -1.roles in {CALL} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 747.` |
| 10 | `  -1.reserved = (<br>	∧ -1.roles not in {CALL, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 731.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {CALL, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 418.` |
| 12 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 522.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 273.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 249.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 1721.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.928. Support: 325.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1444.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 768.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 1755.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 762.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 740.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {CALL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 408.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1515.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 796.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1732.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 746.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1756.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 804.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 1778.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 777.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 762.` |
| 37 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.982. Support: 136.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 361.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.969. Support: 377.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 1191.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.938. Support: 2949.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.2439024390243905, "max_conf": 0.9993780851364136, "max_support": 2949, "min_conf": 0.9212667346000671, "min_support": 136, "num_rules": 41}}
```
</details>
