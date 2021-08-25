# Model report for file:///tmp/top-repos-quality-repos-ak4ld70i/weatherstation-project.git HEAD 4df3967867dacf430f636b03b56f4b5c4b3d21a9

### Dump

```json
{'created_at': '2021-08-22 06:24:04',
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
 'size': '15.0 kB',
 'tags': [],
 'uuid': 'd01d29a4-7904-4bd9-9071-f860b6fee130',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ak4ld70i/weatherstation-project.git 4df3967867dacf430f636b03b56f4b5c4b3d21a9

# javascript
7 rules, avg.len. 3.7
## train
PPCR: 0.642941
### report
macro
{'f1-score': 0.7304253636009377,
 'precision': 0.7589845772178229,
 'recall': 0.7078620550030785,
 'support': 2186}
micro
{'f1-score': 0.9062214089661482,
 'precision': 0.9062214089661482,
 'recall': 0.9062214089661482,
 'support': 2186}
weighted
{'f1-score': 0.9011431993402327,
 'precision': 0.9013032543292012,
 'recall': 0.9062214089661482,
 'support': 2186}
### report_full
macro
{'f1-score': 0.5500802939855308,
 'precision': 0.7589845772178229,
 'recall': 0.46518551054684476,
 'support': 3400}
micro
{'f1-score': 0.7092731829573934,
 'precision': 0.9062214089661482,
 'recall': 0.5826470588235294,
 'support': 3400}
weighted
{'f1-score': 0.6681261616291451,
 'precision': 0.8698099223430932,
 'recall': 0.5826470588235294,
 'support': 3400}
## test
PPCR: 0.651422
### report
macro
{'f1-score': 0.6030309831119548,
 'precision': 0.572089683383543,
 'recall': 0.682855200124937,
 'support': 527}
micro
{'f1-score': 0.8121442125237192,
 'precision': 0.8121442125237192,
 'recall': 0.8121442125237192,
 'support': 527}
weighted
{'f1-score': 0.8256257212200638,
 'precision': 0.8626830574798217,
 'recall': 0.8121442125237192,
 'support': 527}
### report_full
macro
{'f1-score': 0.48793166841378094,
 'precision': 0.572089683383543,
 'recall': 0.4706459396819809,
 'support': 809}
micro
{'f1-score': 0.6407185628742514,
 'precision': 0.8121442125237192,
 'recall': 0.5290482076637825,
 'support': 809}
weighted
{'f1-score': 0.6256805376382867,
 'precision': 0.7951522303467525,
 'recall': 0.5290482076637825,
 'support': 809}
```

## javascript
### Summary
3 rules, avg.len. 2.3

| | |
|-|-|
|Min support|100|
|Max support|526|
|Min confidence|0.9420152306556702|
|Max confidence|0.9950000047683716|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 526.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 100.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 109.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.3333333333333335, "max_conf": 0.9950000047683716, "max_support": 526, "min_conf": 0.9420152306556702, "min_support": 100, "num_rules": 3}}
```
</details>
