# Model report for file:///tmp/top-repos-quality-repos-dcsc9gu1/react-query-devtools.git HEAD d8cd2cb107a7cfaabb42b0fbc9fb144b343ec51c

### Dump

```json
{'created_at': '2021-08-30 02:38:37',
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
 'size': '14.4 kB',
 'tags': [],
 'uuid': '4965859b-21ad-4138-8158-af41132d776b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dcsc9gu1/react-query-devtools.git d8cd2cb107a7cfaabb42b0fbc9fb144b343ec51c

# javascript
4 rules, avg.len. 3.2
## train
PPCR: 0.173887
### report
macro
{'f1-score': 0.4884615384615385,
 'precision': 0.4784172661870503,
 'recall': 0.5,
 'support': 859}
micro
{'f1-score': 0.9790454016298021,
 'precision': 0.9790454016298021,
 'recall': 0.9790454016298021,
 'support': 859}
weighted
{'f1-score': 0.9692934539267485,
 'precision': 0.9608043483722917,
 'recall': 0.9790454016298021,
 'support': 859}
### report_full
macro
{'f1-score': 0.3111867213418739,
 'precision': 0.4784172661870503,
 'recall': 0.2642738396317212,
 'support': 4940}
micro
{'f1-score': 0.2900500086221762,
 'precision': 0.9790454016298021,
 'recall': 0.17024291497975708,
 'support': 4940}
weighted
{'f1-score': 0.2055108316561758,
 'precision': 0.3904945679084263,
 'recall': 0.17024291497975708,
 'support': 4940}
## test
PPCR: 0.142308
### report
macro
{'f1-score': 0.45390070921985815,
 'precision': 0.42777777777777776,
 'recall': 0.5,
 'support': 111}
micro
{'f1-score': 0.7657657657657657,
 'precision': 0.7657657657657657,
 'recall': 0.7657657657657657,
 'support': 111}
weighted
{'f1-score': 0.681042744872532,
 'precision': 0.633033033033033,
 'recall': 0.7657657657657657,
 'support': 111}
### report_full
macro
{'f1-score': 0.2731838816278458,
 'precision': 0.42777777777777776,
 'recall': 0.23126332622601278,
 'support': 780}
micro
{'f1-score': 0.19079685746352412,
 'precision': 0.7657657657657657,
 'recall': 0.10897435897435898,
 'support': 780}
weighted
{'f1-score': 0.1457965553365368,
 'precision': 0.29982905982905983,
 'recall': 0.10897435897435898,
 'support': 780}
```

## javascript
### Summary
3 rules, avg.len. 2.7

| | |
|-|-|
|Min support|156|
|Max support|240|
|Min confidence|0.9967948794364929|
|Max confidence|0.9979166388511658|

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
                     'min_samples_leaf': 92,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 240.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 186.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {MAP} and not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 156.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.6666666666666665, "max_conf": 0.9979166388511658, "max_support": 240, "min_conf": 0.9967948794364929, "min_support": 156, "num_rules": 3}}
```
</details>
