# Model report for file:///tmp/top-repos-quality-repos-sfowcawa/ts-node.git HEAD aaf60523ac0f77dc52b3c729f1f179a85dcac2c0

### Dump

```json
{'created_at': '2021-08-29 21:47:08',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'fc6eae60-7f19-4127-89f6-3cb62d89e27b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sfowcawa/ts-node.git aaf60523ac0f77dc52b3c729f1f179a85dcac2c0

# javascript
18 rules, avg.len. 6.1
## train
PPCR: 0.919812
### report
macro
{'f1-score': 0.8711343594014263,
 'precision': 0.8994371814720814,
 'recall': 0.849816924381351,
 'support': 16036}
micro
{'f1-score': 0.9344599650785732,
 'precision': 0.9344599650785732,
 'recall': 0.9344599650785732,
 'support': 16036}
weighted
{'f1-score': 0.9332566162213544,
 'precision': 0.9338756850786651,
 'recall': 0.9344599650785732,
 'support': 16036}
### report_full
macro
{'f1-score': 0.7604516713958905,
 'precision': 0.8994371814720814,
 'recall': 0.6937091550635275,
 'support': 17434}
micro
{'f1-score': 0.8954287421571557,
 'precision': 0.9344599650785732,
 'recall': 0.8595273603303889,
 'support': 17434}
weighted
{'f1-score': 0.8827291000717842,
 'precision': 0.9291590199182946,
 'recall': 0.8595273603303889,
 'support': 17434}
## test
PPCR: 0.896049
### report
macro
{'f1-score': 0.8011733401227717,
 'precision': 0.822754517308881,
 'recall': 0.7896195300842467,
 'support': 3379}
micro
{'f1-score': 0.8464042616158627,
 'precision': 0.8464042616158627,
 'recall': 0.8464042616158627,
 'support': 3379}
weighted
{'f1-score': 0.8498488996707474,
 'precision': 0.8685789314257163,
 'recall': 0.8464042616158627,
 'support': 3379}
### report_full
macro
{'f1-score': 0.7008546217936161,
 'precision': 0.822754517308881,
 'recall': 0.650106481057759,
 'support': 3771}
micro
{'f1-score': 0.7999999999999999,
 'precision': 0.8464042616158627,
 'recall': 0.758419517369398,
 'support': 3771}
weighted
{'f1-score': 0.7932470099271162,
 'precision': 0.8666570765714109,
 'recall': 0.758419517369398,
 'support': 3771}
```

## javascript
### Summary
11 rules, avg.len. 4.5

| | |
|-|-|
|Min support|93|
|Max support|3555|
|Min confidence|0.9359503984451294|
|Max confidence|0.9991452693939209|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.999. Support: 395.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 434.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 321.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 3555.` |
| 5 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.952. Support: 93.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 585.` |
| 7 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 474.` |
| 8 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 337.` |
| 9 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 266.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.936. Support: 242.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 172.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.454545454545454, "max_conf": 0.9991452693939209, "max_support": 3555, "min_conf": 0.9359503984451294, "min_support": 93, "num_rules": 11}}
```
</details>
