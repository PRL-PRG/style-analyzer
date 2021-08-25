# Model report for file:///tmp/top-repos-quality-repos-2j1qajhf/brickpop.git HEAD ff32b9b01266029e9d4237df5188fe4aec1b1a1e

### Dump

```json
{'created_at': '2021-08-22 04:05:35',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': '6a764a05-89c4-445f-9249-d5baf87ac164',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2j1qajhf/brickpop.git ff32b9b01266029e9d4237df5188fe4aec1b1a1e

# javascript
7 rules, avg.len. 4.4
## train
PPCR: 0.753870
### report
macro
{'f1-score': 0.26503551776536727,
 'precision': 0.2564899954012857,
 'recall': 0.27490263385994007,
 'support': 2435}
micro
{'f1-score': 0.9047227926078029,
 'precision': 0.9047227926078029,
 'recall': 0.9047227926078029,
 'support': 2435}
weighted
{'f1-score': 0.8805330749628278,
 'precision': 0.8596149895609226,
 'recall': 0.9047227926078029,
 'support': 2435}
### report_full
macro
{'f1-score': 0.24308551684844784,
 'precision': 0.2564899954012857,
 'recall': 0.23127275887345658,
 'support': 3230}
micro
{'f1-score': 0.7777581641659311,
 'precision': 0.9047227926078029,
 'recall': 0.6820433436532508,
 'support': 3230}
weighted
{'f1-score': 0.714640891480965,
 'precision': 0.7514108912831199,
 'recall': 0.6820433436532508,
 'support': 3230}
## test
PPCR: 0.754717
### report
macro
{'f1-score': 0.25825473023867607,
 'precision': 0.23897451695820857,
 'recall': 0.28092577813248204,
 'support': 280}
micro
{'f1-score': 0.8321428571428572,
 'precision': 0.8321428571428572,
 'recall': 0.8321428571428572,
 'support': 280}
weighted
{'f1-score': 0.7663769407188388,
 'precision': 0.7102560338105186,
 'recall': 0.8321428571428572,
 'support': 280}
### report_full
macro
{'f1-score': 0.22812754806162003,
 'precision': 0.23897451695820857,
 'recall': 0.22122099571975667,
 'support': 371}
micro
{'f1-score': 0.7158218125960061,
 'precision': 0.8321428571428572,
 'recall': 0.628032345013477,
 'support': 371}
weighted
{'f1-score': 0.6303085962544198,
 'precision': 0.6401683550794702,
 'recall': 0.628032345013477,
 'support': 371}
```

## javascript
### Summary
3 rules, avg.len. 5.7

| | |
|-|-|
|Min support|120|
|Max support|673|
|Min confidence|0.9279346466064453|
|Max confidence|0.9972527623176575|

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
                     'min_samples_leaf': 93,
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
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 673.` |
| 2 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 3 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 120.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.666666666666667, "max_conf": 0.9972527623176575, "max_support": 673, "min_conf": 0.9279346466064453, "min_support": 120, "num_rules": 3}}
```
</details>
