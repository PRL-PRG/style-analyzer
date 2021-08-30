# Model report for file:///tmp/top-repos-quality-repos-ufywvx10/masteringnode.git HEAD 6a8a2e78284f1ccdd18b468959e10bcf1b5807ef

### Dump

```json
{'created_at': '2021-08-30 02:26:36',
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
 'size': '12.9 kB',
 'tags': [],
 'uuid': 'c52a4729-6710-465e-812e-532bc4b4c38e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ufywvx10/masteringnode.git 6a8a2e78284f1ccdd18b468959e10bcf1b5807ef

# javascript
3 rules, avg.len. 4.0
## train
PPCR: 0.630545
### report
macro
{'f1-score': 0.46838025295797636,
 'precision': 0.4410207029369283,
 'recall': 0.5,
 'support': 995}
micro
{'f1-score': 0.914572864321608,
 'precision': 0.914572864321608,
 'recall': 0.914572864321608,
 'support': 995}
weighted
{'f1-score': 0.8740069174923988,
 'precision': 0.8372628670555474,
 'recall': 0.914572864321608,
 'support': 995}
### report_full
macro
{'f1-score': 0.3725964953059827,
 'precision': 0.4410207029369283,
 'recall': 0.3275125628140704,
 'support': 1578}
micro
{'f1-score': 0.7073455110765643,
 'precision': 0.914572864321608,
 'recall': 0.5766793409378961,
 'support': 1578}
weighted
{'f1-score': 0.627296457765498,
 'precision': 0.6940417500379863,
 'recall': 0.5766793409378961,
 'support': 1578}
## test
PPCR: 0.574163
### report
macro
{'f1-score': 0.4369501466275659,
 'precision': 0.3888888888888889,
 'recall': 0.5,
 'support': 120}
micro
{'f1-score': 0.8166666666666667,
 'precision': 0.8166666666666667,
 'recall': 0.8166666666666667,
 'support': 120}
weighted
{'f1-score': 0.7347996089931573,
 'precision': 0.6685185185185186,
 'recall': 0.8166666666666667,
 'support': 120}
### report_full
macro
{'f1-score': 0.341791199667912,
 'precision': 0.3888888888888889,
 'recall': 0.30662393162393164,
 'support': 209}
micro
{'f1-score': 0.5957446808510639,
 'precision': 0.8166666666666667,
 'recall': 0.4688995215311005,
 'support': 209}
weighted
{'f1-score': 0.5080648524969165,
 'precision': 0.5563530037214247,
 'recall': 0.4688995215311005,
 'support': 209}
```

## javascript
### Summary
1 rules, avg.len. 5.0

| | |
|-|-|
|Min support|267|
|Max support|267|
|Min confidence|0.9906367063522339|
|Max confidence|0.9906367063522339|

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
| 1 | `  -1.roles not in {LITERAL}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 267.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9906367063522339, "max_support": 267, "min_conf": 0.9906367063522339, "min_support": 267, "num_rules": 1}}
```
</details>
