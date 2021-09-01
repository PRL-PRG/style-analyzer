# Model report for file:///tmp/top-repos-quality-repos-vwwtzv2c/project3.git HEAD 5cdbd162477789cd187071a8c056d2f1531018d3

### Dump

```json
{'created_at': '2021-08-30 08:12:58',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.5 kB',
 'tags': [],
 'uuid': '4b8c5f4e-8093-451a-9fcb-6439cab317ea',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vwwtzv2c/project3.git 5cdbd162477789cd187071a8c056d2f1531018d3

# javascript
13 rules, avg.len. 6.0
## train
PPCR: 0.780149
### report
macro
{'f1-score': 0.421206078442516,
 'precision': 0.42708277479473933,
 'recall': 0.42276752762202296,
 'support': 7239}
micro
{'f1-score': 0.8852051388313303,
 'precision': 0.8852051388313303,
 'recall': 0.8852051388313303,
 'support': 7239}
weighted
{'f1-score': 0.8675142546946086,
 'precision': 0.8575980405271321,
 'recall': 0.8852051388313303,
 'support': 7239}
### report_full
macro
{'f1-score': 0.3317227027747325,
 'precision': 0.42708277479473933,
 'recall': 0.2867685015309839,
 'support': 9279}
micro
{'f1-score': 0.77588085724664,
 'precision': 0.8852051388313303,
 'recall': 0.6905916585838991,
 'support': 9279}
weighted
{'f1-score': 0.7355480168345614,
 'precision': 0.8146722706441403,
 'recall': 0.6905916585838991,
 'support': 9279}
## test
PPCR: 0.794581
### report
macro
{'f1-score': 0.41294235748949737,
 'precision': 0.43981183067509244,
 'recall': 0.4054100628557093,
 'support': 1965}
micro
{'f1-score': 0.8824427480916031,
 'precision': 0.8824427480916031,
 'recall': 0.8824427480916031,
 'support': 1965}
weighted
{'f1-score': 0.8654749518869068,
 'precision': 0.8584722791538917,
 'recall': 0.8824427480916031,
 'support': 1965}
### report_full
macro
{'f1-score': 0.33241418194890815,
 'precision': 0.43981183067509244,
 'recall': 0.28547107104687314,
 'support': 2473}
micro
{'f1-score': 0.7814330779630464,
 'precision': 0.8824427480916031,
 'recall': 0.7011726647796199,
 'support': 2473}
weighted
{'f1-score': 0.7412272130309052,
 'precision': 0.8155809642306663,
 'recall': 0.7011726647796199,
 'support': 2473}
```

## javascript
### Summary
5 rules, avg.len. 5.2

| | |
|-|-|
|Min support|110|
|Max support|185|
|Min confidence|0.9270270466804504|
|Max confidence|0.9959999918937683|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -4.length ≤ 1<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 125.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 1<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 110.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 176.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 128.` |
| 5 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 185.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.2, "max_conf": 0.9959999918937683, "max_support": 185, "min_conf": 0.9270270466804504, "min_support": 110, "num_rules": 5}}
```
</details>
