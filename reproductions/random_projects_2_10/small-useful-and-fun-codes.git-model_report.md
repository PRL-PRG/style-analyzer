# Model report for file:///tmp/top-repos-quality-repos-f2fbyuei/small-useful-and-fun-codes.git HEAD 4b69d0c4c85ae59e03c342bd2119ad7befece072

### Dump

```json
{'created_at': '2021-08-22 06:12:21',
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
 'size': '14.9 kB',
 'tags': [],
 'uuid': '7c549293-d8da-4ac7-9c05-ca121343be58',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f2fbyuei/small-useful-and-fun-codes.git 4b69d0c4c85ae59e03c342bd2119ad7befece072

# javascript
12 rules, avg.len. 3.3
## train
PPCR: 0.646565
### report
macro
{'f1-score': 0.30669815176422505,
 'precision': 0.3012136928854701,
 'recall': 0.3123880409522015,
 'support': 2616}
micro
{'f1-score': 0.911697247706422,
 'precision': 0.911697247706422,
 'recall': 0.911697247706422,
 'support': 2616}
weighted
{'f1-score': 0.8944436829620654,
 'precision': 0.8778354817115244,
 'recall': 0.911697247706422,
 'support': 2616}
### report_full
macro
{'f1-score': 0.2513162184970655,
 'precision': 0.3012136928854701,
 'recall': 0.21888507733467724,
 'support': 4046}
micro
{'f1-score': 0.7160012008405885,
 'precision': 0.911697247706422,
 'recall': 0.5894710825506674,
 'support': 4046}
weighted
{'f1-score': 0.6650336919567441,
 'precision': 0.7767597832128573,
 'recall': 0.5894710825506674,
 'support': 4046}
## test
PPCR: 0.647858
### report
macro
{'f1-score': 0.31402798575310115,
 'precision': 0.3144784194077542,
 'recall': 0.3141688684263304,
 'support': 2193}
micro
{'f1-score': 0.9375284997720018,
 'precision': 0.9375284997720018,
 'recall': 0.9375284997720018,
 'support': 2193}
weighted
{'f1-score': 0.9256663372352208,
 'precision': 0.9155007512021198,
 'recall': 0.9375284997720018,
 'support': 2193}
### report_full
macro
{'f1-score': 0.2560154212701664,
 'precision': 0.3144784194077542,
 'recall': 0.22259827482521014,
 'support': 3385}
micro
{'f1-score': 0.7371817855862317,
 'precision': 0.9375284997720018,
 'recall': 0.6073855243722305,
 'support': 3385}
weighted
{'f1-score': 0.6833629624474676,
 'precision': 0.8111744570090957,
 'recall': 0.6073855243722305,
 'support': 3385}
```

## javascript
### Summary
4 rules, avg.len. 2.8

| | |
|-|-|
|Min support|257|
|Max support|655|
|Min confidence|0.9474708437919617|
|Max confidence|0.9944968819618225|

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
                     'min_samples_leaf': 98,
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
| 1 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 257.` |
| 2 | `  -1.reserved not in {;}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 623.` |
| 3 | `  -1.reserved not in {;}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 636.` |
| 4 | `  -1.reserved not in {;}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 655.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.75, "max_conf": 0.9944968819618225, "max_support": 655, "min_conf": 0.9474708437919617, "min_support": 257, "num_rules": 4}}
```
</details>
