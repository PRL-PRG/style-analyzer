# Model report for file:///tmp/top-repos-quality-repos-vk_0mqpi/seen.git HEAD d8946b3b97b9814e78f79334b9fd6349b9022289

### Dump

```json
{'created_at': '2021-08-29 13:35:50',
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
 'size': '15.5 kB',
 'tags': [],
 'uuid': '23b17614-f13b-49dc-ae39-bb1269e055cc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vk_0mqpi/seen.git d8946b3b97b9814e78f79334b9fd6349b9022289

# javascript
11 rules, avg.len. 4.2
## train
PPCR: 0.729955
### report
macro
{'f1-score': 0.6381404996322415,
 'precision': 0.6608889004739537,
 'recall': 0.6291357862203307,
 'support': 11307}
micro
{'f1-score': 0.9390642964535244,
 'precision': 0.9390642964535244,
 'recall': 0.9390642964535244,
 'support': 11307}
weighted
{'f1-score': 0.9370485908003191,
 'precision': 0.9373261913769985,
 'recall': 0.9390642964535244,
 'support': 11307}
### report_full
macro
{'f1-score': 0.5663756006516801,
 'precision': 0.6608889004739537,
 'recall': 0.5202459607831493,
 'support': 15490}
micro
{'f1-score': 0.7924767697876628,
 'precision': 0.9390642964535244,
 'recall': 0.6854744996772111,
 'support': 15490}
weighted
{'f1-score': 0.7525986754017987,
 'precision': 0.8635228521305115,
 'recall': 0.6854744996772111,
 'support': 15490}
## test
PPCR: 0.814286
### report
macro
{'f1-score': 0.479322033898305,
 'precision': 0.5473562939607082,
 'recall': 0.4453484096341239,
 'support': 114}
micro
{'f1-score': 0.9385964912280702,
 'precision': 0.9385964912280702,
 'recall': 0.9385964912280702,
 'support': 114}
weighted
{'f1-score': 0.9338051343046881,
 'precision': 0.9426845977422333,
 'recall': 0.9385964912280702,
 'support': 114}
### report_full
macro
{'f1-score': 0.454496916137347,
 'precision': 0.5473562939607082,
 'recall': 0.39853123067408786,
 'support': 140}
micro
{'f1-score': 0.84251968503937,
 'precision': 0.9385964912280702,
 'recall': 0.7642857142857142,
 'support': 140}
weighted
{'f1-score': 0.8049975671019581,
 'precision': 0.8628516856657774,
 'recall': 0.7642857142857142,
 'support': 140}
```

## javascript
### Summary
6 rules, avg.len. 4.5

| | |
|-|-|
|Min support|114|
|Max support|3201|
|Min confidence|0.9242424368858337|
|Max confidence|0.998789370059967|

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
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 138.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.996. Support: 114.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 3201.` |
| 4 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 883.` |
| 5 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 413.` |
| 6 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 231.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.998789370059967, "max_support": 3201, "min_conf": 0.9242424368858337, "min_support": 114, "num_rules": 6}}
```
</details>
