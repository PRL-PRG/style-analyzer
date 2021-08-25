# Model report for file:///tmp/top-repos-quality-repos-skeutftm/tads.git HEAD a39116a8182e7cf99e23ee1692854f3260646cd0

### Dump

```json
{'created_at': '2021-08-21 04:48:10',
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
 'size': '14.7 kB',
 'tags': [],
 'uuid': 'a870a6aa-9109-4bac-abfd-6e6a30efb55f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-skeutftm/tads.git a39116a8182e7cf99e23ee1692854f3260646cd0

# javascript
6 rules, avg.len. 3.8
## train
PPCR: 0.690122
### report
macro
{'f1-score': 0.44975426663148993,
 'precision': 0.467490366231785,
 'recall': 0.43688388022400154,
 'support': 2089}
micro
{'f1-score': 0.9009095260890378,
 'precision': 0.9009095260890378,
 'recall': 0.9009095260890378,
 'support': 2089}
weighted
{'f1-score': 0.8918864159107633,
 'precision': 0.8913630380801233,
 'recall': 0.9009095260890378,
 'support': 2089}
### report_full
macro
{'f1-score': 0.35802155474547814,
 'precision': 0.467490366231785,
 'recall': 0.3008493358364008,
 'support': 3027}
micro
{'f1-score': 0.7357310398749023,
 'precision': 0.9009095260890378,
 'recall': 0.6217376940865543,
 'support': 3027}
weighted
{'f1-score': 0.6885953111289491,
 'precision': 0.8042693816416808,
 'recall': 0.6217376940865543,
 'support': 3027}
## test
PPCR: 0.678250
### report
macro
{'f1-score': 0.44280442765067246,
 'precision': 0.46489697192370105,
 'recall': 0.4290850231639705,
 'support': 527}
micro
{'f1-score': 0.8785578747628083,
 'precision': 0.8785578747628083,
 'recall': 0.8785578747628083,
 'support': 527}
weighted
{'f1-score': 0.8660883246837235,
 'precision': 0.8698782333402877,
 'recall': 0.8785578747628083,
 'support': 527}
### report_full
macro
{'f1-score': 0.35578820272697825,
 'precision': 0.46489697192370105,
 'recall': 0.3008129623793874,
 'support': 777}
micro
{'f1-score': 0.7101226993865031,
 'precision': 0.8785578747628083,
 'recall': 0.5958815958815958,
 'support': 777}
weighted
{'f1-score': 0.6586106003015624,
 'precision': 0.7812655193476356,
 'recall': 0.5958815958815958,
 'support': 777}
```

## javascript
### Summary
4 rules, avg.len. 3.5

| | |
|-|-|
|Min support|104|
|Max support|438|
|Min confidence|0.9370370507240295|
|Max confidence|0.995192289352417|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 438.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 104.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 135.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 119.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.995192289352417, "max_support": 438, "min_conf": 0.9370370507240295, "min_support": 104, "num_rules": 4}}
```
</details>
