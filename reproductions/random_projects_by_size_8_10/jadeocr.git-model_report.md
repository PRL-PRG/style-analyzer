# Model report for file:///tmp/top-repos-quality-repos-ii0_4zya/jadeocr.git HEAD 98f16bcc5b44375aff7c7ca6d1888f5590628076

### Dump

```json
{'created_at': '2021-08-20 23:58:05',
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
 'size': '13.3 kB',
 'tags': [],
 'uuid': '6bf0a83b-2766-4d30-bce0-f307ef210e15',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ii0_4zya/jadeocr.git 98f16bcc5b44375aff7c7ca6d1888f5590628076

# javascript
11 rules, avg.len. 2.6
## train
PPCR: 0.532814
### report
macro
{'f1-score': 0.18661190965092403,
 'precision': 0.17490377213240954,
 'recall': 0.2,
 'support': 1299}
micro
{'f1-score': 0.8745188606620478,
 'precision': 0.8745188606620478,
 'recall': 0.8745188606620478,
 'support': 1299}
weighted
{'f1-score': 0.8159781730694753,
 'precision': 0.7647832376536461,
 'recall': 0.8745188606620478,
 'support': 1299}
### report_full
macro
{'f1-score': 0.17537630258587417,
 'precision': 0.17490377213240954,
 'recall': 0.1758513931888545,
 'support': 2438}
micro
{'f1-score': 0.607974310944608,
 'precision': 0.8745188606620478,
 'recall': 0.46595570139458575,
 'support': 2438}
weighted
{'f1-score': 0.4646968477049825,
 'precision': 0.4634447776765241,
 'recall': 0.46595570139458575,
 'support': 2438}
## test
PPCR: 0.500000
### report
macro
{'f1-score': 0.13333333333333333, 'precision': 0.1, 'recall': 0.2, 'support': 6}
micro
{'f1-score': 0.5, 'precision': 0.5, 'recall': 0.5, 'support': 6}
weighted
{'f1-score': 0.3333333333333333, 'precision': 0.25, 'recall': 0.5, 'support': 6}
### report_full
macro
{'f1-score': 0.12, 'precision': 0.1, 'recall': 0.15, 'support': 12}
micro
{'f1-score': 0.3333333333333333,
 'precision': 0.5,
 'recall': 0.25,
 'support': 12}
weighted
{'f1-score': 0.19999999999999998,
 'precision': 0.16666666666666666,
 'recall': 0.25,
 'support': 12}
```

## javascript
### Summary
3 rules, avg.len. 2.3

| | |
|-|-|
|Min support|144|
|Max support|411|
|Min confidence|0.9355230927467346|
|Max confidence|0.9970930218696594|

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
                     'min_samples_leaf': 95,
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 411.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.3333333333333335, "max_conf": 0.9970930218696594, "max_support": 411, "min_conf": 0.9355230927467346, "min_support": 144, "num_rules": 3}}
```
</details>
