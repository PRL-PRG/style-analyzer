# Model report for file:///tmp/top-repos-quality-repos-k10zevp7/node_django.git HEAD 8fb39b22445a0362d48a88a10a8c38aa67ecadc2

### Dump

```json
{'created_at': '2021-08-21 23:25:49',
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
 'size': '12.4 kB',
 'tags': [],
 'uuid': 'c7c5da44-9321-4d90-9790-cc0efa7eb081',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-k10zevp7/node_django.git 8fb39b22445a0362d48a88a10a8c38aa67ecadc2

# javascript
3 rules, avg.len. 2.0
## train
PPCR: 0.510592
### report
macro
{'f1-score': 0.5800904977375566,
 'precision': 0.5809322999772054,
 'recall': 0.5804342934293429,
 'support': 699}
micro
{'f1-score': 0.8969957081545065,
 'precision': 0.8969957081545065,
 'recall': 0.8969957081545065,
 'support': 699}
weighted
{'f1-score': 0.883508438040122,
 'precision': 0.8716584602771943,
 'recall': 0.8969957081545065,
 'support': 699}
### report_full
macro
{'f1-score': 0.39815991949647794,
 'precision': 0.5809322999772054,
 'recall': 0.3132097301362335,
 'support': 1369}
micro
{'f1-score': 0.6063829787234043,
 'precision': 0.8969957081545065,
 'recall': 0.4579985390796202,
 'support': 1369}
weighted
{'f1-score': 0.5685630320468229,
 'precision': 0.7872284938417061,
 'recall': 0.4579985390796202,
 'support': 1369}
## test
PPCR: 0.422680
### report
macro
{'f1-score': 0.5263522515321076,
 'precision': 0.5513611615245009,
 'recall': 0.5128205128205128,
 'support': 164}
micro
{'f1-score': 0.8963414634146342,
 'precision': 0.8963414634146342,
 'recall': 0.8963414634146342,
 'support': 164}
weighted
{'f1-score': 0.8746144547776414,
 'precision': 0.8606768182019389,
 'recall': 0.8963414634146342,
 'support': 164}
### report_full
macro
{'f1-score': 0.3137931034482759,
 'precision': 0.5513611615245009,
 'recall': 0.23676244790524237,
 'support': 388}
micro
{'f1-score': 0.5326086956521738,
 'precision': 0.8963414634146342,
 'recall': 0.3788659793814433,
 'support': 388}
weighted
{'f1-score': 0.48431389975115546,
 'precision': 0.739756207083653,
 'recall': 0.3788659793814433,
 'support': 388}
```

## javascript
### Summary
1 rules, avg.len. 2.0

| | |
|-|-|
|Min support|188|
|Max support|188|
|Min confidence|0.9867021441459656|
|Max confidence|0.9867021441459656|

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
                     'max_depth': 5,
                     'min_samples_leaf': 91,
                     'min_samples_split': 214,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 188.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.0, "max_conf": 0.9867021441459656, "max_support": 188, "min_conf": 0.9867021441459656, "min_support": 188, "num_rules": 1}}
```
</details>
