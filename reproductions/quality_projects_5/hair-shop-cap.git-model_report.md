# Model report for file:///tmp/top-repos-quality-repos-pl1imr1i/hair-shop-cap.git HEAD c43ea6866b5e7f76895a194f42c8a1d549cc04bf

### Dump

```json
{'created_at': '2021-08-30 07:32:01',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': '09275a35-8677-4bff-ab32-6d753fc0332d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-pl1imr1i/hair-shop-cap.git c43ea6866b5e7f76895a194f42c8a1d549cc04bf

# javascript
8 rules, avg.len. 5.0
## train
PPCR: 0.769983
### report
macro
{'f1-score': 0.6469639279143361,
 'precision': 0.7413396930308784,
 'recall': 0.6075083762038903,
 'support': 10394}
micro
{'f1-score': 0.9232249374639215,
 'precision': 0.9232249374639215,
 'recall': 0.9232249374639215,
 'support': 10394}
weighted
{'f1-score': 0.9035022209010989,
 'precision': 0.9009699690907197,
 'recall': 0.9232249374639215,
 'support': 10394}
### report_full
macro
{'f1-score': 0.49699802000159,
 'precision': 0.7413396930308784,
 'recall': 0.4407761908272505,
 'support': 13499}
micro
{'f1-score': 0.8032478131670364,
 'precision': 0.9232249374639215,
 'recall': 0.7108674716645678,
 'support': 13499}
weighted
{'f1-score': 0.7349466354390085,
 'precision': 0.8838919529542154,
 'recall': 0.7108674716645678,
 'support': 13499}
## test
PPCR: 0.746808
### report
macro
{'f1-score': 0.6248791622610356,
 'precision': 0.7005583309557416,
 'recall': 0.6257660641584916,
 'support': 2398}
micro
{'f1-score': 0.9132610508757297,
 'precision': 0.9132610508757297,
 'recall': 0.9132610508757297,
 'support': 2398}
weighted
{'f1-score': 0.8886565569879346,
 'precision': 0.8921681904328754,
 'recall': 0.9132610508757297,
 'support': 2398}
### report_full
macro
{'f1-score': 0.44574524586715825,
 'precision': 0.7005583309557416,
 'recall': 0.401386732412435,
 'support': 3211}
micro
{'f1-score': 0.7808878587983598,
 'precision': 0.9132610508757297,
 'recall': 0.6820305200872002,
 'support': 3211}
weighted
{'f1-score': 0.704662048028794,
 'precision': 0.8867599961197787,
 'recall': 0.6820305200872002,
 'support': 3211}
```

## javascript
### Summary
2 rules, avg.len. 5.5

| | |
|-|-|
|Min support|380|
|Max support|6624|
|Min confidence|0.9312348961830139|
|Max confidence|0.9986842274665833|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.999. Support: 380.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 6624.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9986842274665833, "max_support": 6624, "min_conf": 0.9312348961830139, "min_support": 380, "num_rules": 2}}
```
</details>
