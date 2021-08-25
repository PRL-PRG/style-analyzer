# Model report for file:///tmp/top-repos-quality-repos-t3cguefe/iotc.git HEAD 4b2b745cb343fe5969218c79f080d98b4bbc02f9

### Dump

```json
{'created_at': '2021-08-21 21:18:02',
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
 'size': '12.3 kB',
 'tags': [],
 'uuid': '217871e0-de4d-4c8a-b203-9821f4f7fbbf',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t3cguefe/iotc.git 4b2b745cb343fe5969218c79f080d98b4bbc02f9

# javascript
3 rules, avg.len. 2.0
## train
PPCR: 0.540614
### report
macro
{'f1-score': 0.6297721755789469,
 'precision': 0.6282493673798021,
 'recall': 0.6337615403304179,
 'support': 792}
micro
{'f1-score': 0.9444444444444444,
 'precision': 0.9444444444444444,
 'recall': 0.9444444444444444,
 'support': 792}
weighted
{'f1-score': 0.9440352776520641,
 'precision': 0.9472228321866003,
 'recall': 0.9444444444444444,
 'support': 792}
### report_full
macro
{'f1-score': 0.4724022665199136,
 'precision': 0.6282493673798021,
 'recall': 0.38485254356880155,
 'support': 1465}
micro
{'f1-score': 0.662826761187417,
 'precision': 0.9444444444444444,
 'recall': 0.510580204778157,
 'support': 1465}
weighted
{'f1-score': 0.6395134776182759,
 'precision': 0.8789081324769112,
 'recall': 0.510580204778157,
 'support': 1465}
## test
PPCR: 0.440476
### report
macro
{'f1-score': 0.5895833333333333,
 'precision': 0.6464646464646465,
 'recall': 0.5555555555555555,
 'support': 37}
micro
{'f1-score': 0.9459459459459459,
 'precision': 0.9459459459459459,
 'recall': 0.9459459459459459,
 'support': 37}
weighted
{'f1-score': 0.9413851351351351,
 'precision': 0.9492219492219494,
 'recall': 0.9459459459459459,
 'support': 37}
### report_full
macro
{'f1-score': 0.3300865800865801,
 'precision': 0.6464646464646465,
 'recall': 0.24343434343434342,
 'support': 84}
micro
{'f1-score': 0.578512396694215,
 'precision': 0.9459459459459459,
 'recall': 0.4166666666666667,
 'support': 84}
weighted
{'f1-score': 0.5429421768707483,
 'precision': 0.9007936507936508,
 'recall': 0.4166666666666667,
 'support': 84}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|98|
|Max support|329|
|Min confidence|0.954081654548645|
|Max confidence|0.9893617033958435|

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
                     'max_depth': 10,
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 98.` |
| 2 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 329.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9893617033958435, "max_support": 329, "min_conf": 0.954081654548645, "min_support": 98, "num_rules": 2}}
```
</details>
