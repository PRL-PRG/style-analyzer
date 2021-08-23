# Model report for file:///tmp/top-repos-quality-repos-x13ev0qx/open-energy-view.git HEAD e4265dc4c240ec35d8fa4d3d3fc94c98e18276bf

### Dump

```json
{'created_at': '2021-08-21 22:40:16',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'ef854e1d-684e-4716-9bac-82f67527960d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-x13ev0qx/open-energy-view.git e4265dc4c240ec35d8fa4d3d3fc94c98e18276bf

# javascript
67 rules, avg.len. 5.8
## train
PPCR: 0.878273
### report
macro
{'f1-score': 0.39572903746294263,
 'precision': 0.3968713634461443,
 'recall': 0.3972294153256657,
 'support': 8117}
micro
{'f1-score': 0.898977454724652,
 'precision': 0.898977454724652,
 'recall': 0.898977454724652,
 'support': 8117}
weighted
{'f1-score': 0.8706644164408216,
 'precision': 0.8456747671388337,
 'recall': 0.898977454724652,
 'support': 8117}
### report_full
macro
{'f1-score': 0.386078211673667,
 'precision': 0.3968713634461443,
 'recall': 0.37891301539878475,
 'support': 9242}
micro
{'f1-score': 0.840716631142347,
 'precision': 0.898977454724652,
 'recall': 0.7895477169443843,
 'support': 9242}
weighted
{'f1-score': 0.7799791993265072,
 'precision': 0.7724901064761182,
 'recall': 0.7895477169443843,
 'support': 9242}
## test
PPCR: 0.843750
### report
macro
{'f1-score': 0.3891117884322139,
 'precision': 0.39102872787533094,
 'recall': 0.3900433644401125,
 'support': 1539}
micro
{'f1-score': 0.8784925276153346,
 'precision': 0.8784925276153346,
 'recall': 0.8784925276153346,
 'support': 1539}
weighted
{'f1-score': 0.8502838347570301,
 'precision': 0.8258441102530666,
 'recall': 0.8784925276153346,
 'support': 1539}
### report_full
macro
{'f1-score': 0.37766505930938876,
 'precision': 0.39102872787533094,
 'recall': 0.36702767609228165,
 'support': 1824}
micro
{'f1-score': 0.8040440083258995,
 'precision': 0.8784925276153346,
 'recall': 0.7412280701754386,
 'support': 1824}
weighted
{'f1-score': 0.7437898459754402,
 'precision': 0.7476820487090663,
 'recall': 0.7412280701754386,
 'support': 1824}
```

## javascript
### Summary
33 rules, avg.len. 4.8

| | |
|-|-|
|Min support|136|
|Max support|1325|
|Min confidence|0.9246894121170044|
|Max confidence|0.9984848499298096|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1120.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 212.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 316.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1262.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 386.` |
| 6 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1126.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.998. Support: 237.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 336.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1325.` |
| 10 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1173.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.998. Support: 227.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ∅<br>Confidence: 0.925. Support: 644.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 422.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 216.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 174.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 18 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.996. Support: 136.` |
| 19 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 354.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.998. Support: 217.` |
| 21 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 357.` |
| 22 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.998. Support: 200.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1239.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 201.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 330.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 162.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1241.` |
| 28 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.998. Support: 203.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 367.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1242.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 341.` |
| 32 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 334.` |
| 33 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.818181818181818, "max_conf": 0.9984848499298096, "max_support": 1325, "min_conf": 0.9246894121170044, "min_support": 136, "num_rules": 33}}
```
</details>
