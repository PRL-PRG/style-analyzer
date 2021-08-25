# Model report for file:///tmp/top-repos-quality-repos-z1xfkvat/kmera.git HEAD 77ba2c13238342eb1d768d26bcbf135580f49555

### Dump

```json
{'created_at': '2021-08-22 04:44:31',
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
 'size': '13.6 kB',
 'tags': [],
 'uuid': 'fe75f71d-e570-40a7-bc66-068215e16c79',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-z1xfkvat/kmera.git 77ba2c13238342eb1d768d26bcbf135580f49555

# javascript
3 rules, avg.len. 4.0
## train
PPCR: 0.364761
### report
macro
{'f1-score': 0.3680891964628989,
 'precision': 0.3677580151806956,
 'recall': 0.3684492248649195,
 'support': 855}
micro
{'f1-score': 0.928654970760234,
 'precision': 0.928654970760234,
 'recall': 0.928654970760234,
 'support': 855}
weighted
{'f1-score': 0.9257626958592174,
 'precision': 0.922948749969911,
 'recall': 0.928654970760234,
 'support': 855}
### report_full
macro
{'f1-score': 0.24883981723049428,
 'precision': 0.3677580151806956,
 'recall': 0.18835306923508263,
 'support': 2344}
micro
{'f1-score': 0.4964051266020632,
 'precision': 0.928654970760234,
 'recall': 0.3387372013651877,
 'support': 2344}
weighted
{'f1-score': 0.45209466642194035,
 'precision': 0.6813677860087205,
 'recall': 0.3387372013651877,
 'support': 2344}
## test
PPCR: 0.286822
### report
macro
{'f1-score': 0.37599134823359776,
 'precision': 0.3783783783783784,
 'recall': 0.37393162393162394,
 'support': 111}
micro
{'f1-score': 0.9459459459459459,
 'precision': 0.9459459459459459,
 'recall': 0.9459459459459459,
 'support': 111}
weighted
{'f1-score': 0.9456049416395488,
 'precision': 0.9459459459459459,
 'recall': 0.9459459459459459,
 'support': 111}
### report_full
macro
{'f1-score': 0.23982979347786215,
 'precision': 0.3783783783783784,
 'recall': 0.1755503144654088,
 'support': 387}
micro
{'f1-score': 0.42168674698795183,
 'precision': 0.9459459459459459,
 'recall': 0.2713178294573643,
 'support': 387}
weighted
{'f1-score': 0.3705419342375792,
 'precision': 0.5841888400027935,
 'recall': 0.2713178294573643,
 'support': 387}
```

## javascript
### Summary
2 rules, avg.len. 3.5

| | |
|-|-|
|Min support|107|
|Max support|500|
|Min confidence|0.9390000104904175|
|Max confidence|0.985981285572052|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.986. Support: 107.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 500.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.985981285572052, "max_support": 500, "min_conf": 0.9390000104904175, "min_support": 107, "num_rules": 2}}
```
</details>
