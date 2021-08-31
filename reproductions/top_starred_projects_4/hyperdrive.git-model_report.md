# Model report for file:///tmp/top-repos-quality-repos-7fw4_obx/hyperdrive.git HEAD 4071b1e5f7a777e325910501ea72fdd892b00418

### Dump

```json
{'created_at': '2021-08-30 08:25:20',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'df737c4f-0241-47da-b854-84b530fcbb09',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7fw4_obx/hyperdrive.git 4071b1e5f7a777e325910501ea72fdd892b00418

# javascript
21 rules, avg.len. 7.1
## train
PPCR: 0.961714
### report
macro
{'f1-score': 0.9061681148897837,
 'precision': 0.9690978978509798,
 'recall': 0.8802305955376729,
 'support': 35694}
micro
{'f1-score': 0.9759623466128761,
 'precision': 0.9759623466128761,
 'recall': 0.9759623466128761,
 'support': 35694}
weighted
{'f1-score': 0.9747438849847653,
 'precision': 0.9763999150146223,
 'recall': 0.9759623466128761,
 'support': 35694}
### report_full
macro
{'f1-score': 0.8452651642784901,
 'precision': 0.9690978978509798,
 'recall': 0.802192075341995,
 'support': 37115}
micro
{'f1-score': 0.9569146671427982,
 'precision': 0.9759623466128761,
 'recall': 0.9385962548834703,
 'support': 37115}
weighted
{'f1-score': 0.9509259302754944,
 'precision': 0.9753122709320645,
 'recall': 0.9385962548834703,
 'support': 37115}
## test
PPCR: 0.976053
### report
macro
{'f1-score': 0.8691130584030898,
 'precision': 0.9617401095654671,
 'recall': 0.8589486863317576,
 'support': 8437}
micro
{'f1-score': 0.9777171980561811,
 'precision': 0.9777171980561811,
 'recall': 0.9777171980561811,
 'support': 8437}
weighted
{'f1-score': 0.9745800256146018,
 'precision': 0.9789030902003678,
 'recall': 0.9777171980561811,
 'support': 8437}
### report_full
macro
{'f1-score': 0.8377690842844865,
 'precision': 0.9617401095654671,
 'recall': 0.8155244333951774,
 'support': 8644}
micro
{'f1-score': 0.9658685088695039,
 'precision': 0.9777171980561811,
 'recall': 0.9543035631652013,
 'support': 8644}
weighted
{'f1-score': 0.9590875426298014,
 'precision': 0.9778530020039494,
 'recall': 0.9543035631652013,
 'support': 8644}
```

## javascript
### Summary
17 rules, avg.len. 6.3

| | |
|-|-|
|Min support|108|
|Max support|6149|
|Min confidence|0.9394210577011108|
|Max confidence|0.9995905160903931|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 4513.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1007.` |
| 3 | `  -1.roles in {IDENTIFIER, NAME}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 282.` |
| 4 | `  -1.roles in {IDENTIFIER} and not in {NAME}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 5 | `  -1.roles in {IDENTIFIER} and not in {NAME}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5326.` |
| 6 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 573.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2693.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1221.` |
| 9 | `  •••start_col ≤ 51<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.982. Support: 953.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 1006.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 977.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 568.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 600.` |
| 14 | `  •••start_col ≤ 19<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), >, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 699.` |
| 15 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +2.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.977. Support: 108.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 239.` |
| 17 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 6149.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.294117647058823, "max_conf": 0.9995905160903931, "max_support": 6149, "min_conf": 0.9394210577011108, "min_support": 108, "num_rules": 17}}
```
</details>
