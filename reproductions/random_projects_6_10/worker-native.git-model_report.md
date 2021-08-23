# Model report for file:///tmp/top-repos-quality-repos-ioaweb38/worker-native.git HEAD f2cd49bbb85f6c59decc71363ee86d483f05cf5f

### Dump

```json
{'created_at': '2021-08-21 15:54:43',
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
 'size': '14.2 kB',
 'tags': [],
 'uuid': 'ba90c134-9067-4d08-8adc-949c641faa3a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ioaweb38/worker-native.git f2cd49bbb85f6c59decc71363ee86d483f05cf5f

# javascript
1 rules, avg.len. 1.0
## train
PPCR: 0.172973
### report
macro
{'f1-score': 0.16377649325626206,
 'precision': 0.16098484848484848,
 'recall': 0.16666666666666666,
 'support': 352}
micro
{'f1-score': 0.9659090909090909,
 'precision': 0.9659090909090909,
 'recall': 0.9659090909090909,
 'support': 352}
weighted
{'f1-score': 0.9491592222806097,
 'precision': 0.9329803719008265,
 'recall': 0.9659090909090909,
 'support': 352}
### report_full
macro
{'f1-score': 0.07821486082355648,
 'precision': 0.16098484848484848,
 'recall': 0.05165603160133698,
 'support': 2035}
micro
{'f1-score': 0.284876413908672,
 'precision': 0.9659090909090909,
 'recall': 0.16707616707616707,
 'support': 2035}
weighted
{'f1-score': 0.2529779921084269,
 'precision': 0.5206890775072593,
 'recall': 0.16707616707616707,
 'support': 2035}
## test
PPCR: 0.210909
### report
macro
{'f1-score': 0.16374269005847952,
 'precision': 0.16091954022988506,
 'recall': 0.16666666666666666,
 'support': 58}
micro
{'f1-score': 0.9655172413793104,
 'precision': 0.9655172413793104,
 'recall': 0.9655172413793104,
 'support': 58}
weighted
{'f1-score': 0.9485783424077435,
 'precision': 0.9322235434007135,
 'recall': 0.9655172413793104,
 'support': 58}
### report_full
macro
{'f1-score': 0.08333333333333333,
 'precision': 0.16091954022988506,
 'recall': 0.05622489959839357,
 'support': 275}
micro
{'f1-score': 0.3363363363363363,
 'precision': 0.9655172413793104,
 'recall': 0.20363636363636364,
 'support': 275}
weighted
{'f1-score': 0.3018181818181818,
 'precision': 0.58282131661442,
 'recall': 0.20363636363636362,
 'support': 275}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|285|
|Max support|285|
|Min confidence|0.9596491456031799|
|Max confidence|0.9596491456031799|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 285.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9596491456031799, "max_support": 285, "min_conf": 0.9596491456031799, "min_support": 285, "num_rules": 1}}
```
</details>
