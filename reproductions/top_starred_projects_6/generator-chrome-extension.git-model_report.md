# Model report for file:///tmp/top-repos-quality-repos-kgq856p0/generator-chrome-extension.git HEAD 3304eefb1a7d04e9e1a487b8b476d51a72d42aff

### Dump

```json
{'created_at': '2021-08-29 21:48:59',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.4 kB',
 'tags': [],
 'uuid': '495233be-bdb1-48bf-a8be-4ddfb37c1309',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kgq856p0/generator-chrome-extension.git 3304eefb1a7d04e9e1a487b8b476d51a72d42aff

# javascript
20 rules, avg.len. 3.2
## train
PPCR: 0.510753
### report
macro
{'f1-score': 0.2744550810928698,
 'precision': 0.2730042016806723,
 'recall': 0.2767857142857143,
 'support': 1615}
micro
{'f1-score': 0.9250773993808049,
 'precision': 0.9250773993808049,
 'recall': 0.9250773993808049,
 'support': 1615}
weighted
{'f1-score': 0.8944535397557627,
 'precision': 0.8673470224002914,
 'recall': 0.9250773993808049,
 'support': 1615}
### report_full
macro
{'f1-score': 0.21545853947759364,
 'precision': 0.2730042016806723,
 'recall': 0.184053804509716,
 'support': 3162}
micro
{'f1-score': 0.6254971739585513,
 'precision': 0.9250773993808049,
 'recall': 0.47248576850094876,
 'support': 3162}
weighted
{'f1-score': 0.5256560023601783,
 'precision': 0.6113803158834691,
 'recall': 0.47248576850094876,
 'support': 3162}
## test
PPCR: 0.391204
### report
macro
{'f1-score': 0.2785186859553948,
 'precision': 0.27751756440281034,
 'recall': 0.27988338192419826,
 'support': 169}
micro
{'f1-score': 0.9585798816568046,
 'precision': 0.9585798816568047,
 'recall': 0.9585798816568047,
 'support': 169}
weighted
{'f1-score': 0.9442753975981825,
 'precision': 0.9313706470074694,
 'recall': 0.9585798816568047,
 'support': 169}
### report_full
macro
{'f1-score': 0.20177078048540986,
 'precision': 0.27751756440281034,
 'recall': 0.16029284818476475,
 'support': 432}
micro
{'f1-score': 0.5391014975041597,
 'precision': 0.9585798816568047,
 'recall': 0.375,
 'support': 432}
weighted
{'f1-score': 0.4636769516681305,
 'precision': 0.6174294171220401,
 'recall': 0.375,
 'support': 432}
```

## javascript
### Summary
13 rules, avg.len. 2.4

| | |
|-|-|
|Min support|143|
|Max support|560|
|Min confidence|0.9475524425506592|
|Max confidence|0.9976303577423096|

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
                     'min_samples_leaf': 93,
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.998. Support: 211.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 520.` |
| 3 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.988. Support: 560.` |
| 5 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 517.` |
| 6 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 167.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 547.` |
| 8 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 552.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 177.` |
| 10 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 527.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 532.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 143.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 505.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.3846153846153846, "max_conf": 0.9976303577423096, "max_support": 560, "min_conf": 0.9475524425506592, "min_support": 143, "num_rules": 13}}
```
</details>
