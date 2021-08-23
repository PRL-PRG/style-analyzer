# Model report for file:///tmp/top-repos-quality-repos-4einmve6/esfera.git HEAD 163be1282f73bccc57b2df48f51df66cde3e3b24

### Dump

```json
{'created_at': '2021-08-22 01:41:28',
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
 'size': '14.3 kB',
 'tags': [],
 'uuid': 'de5f9a6b-36d5-47e4-8c4c-11fab605fef5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4einmve6/esfera.git 163be1282f73bccc57b2df48f51df66cde3e3b24

# javascript
4 rules, avg.len. 4.0
## train
PPCR: 0.646109
### report
macro
{'f1-score': 0.2778294377051721,
 'precision': 0.31901352426412094,
 'recall': 0.25877192982456143,
 'support': 3122}
micro
{'f1-score': 0.9192825112107623,
 'precision': 0.9192825112107623,
 'recall': 0.9192825112107623,
 'support': 3122}
weighted
{'f1-score': 0.8981785674144047,
 'precision': 0.8945072233544681,
 'recall': 0.9192825112107623,
 'support': 3122}
### report_full
macro
{'f1-score': 0.20133427094143133,
 'precision': 0.31901352426412094,
 'recall': 0.18547400364369415,
 'support': 4832}
micro
{'f1-score': 0.7216494845360825,
 'precision': 0.9192825112107623,
 'recall': 0.5939569536423841,
 'support': 4832}
weighted
{'f1-score': 0.6105493381484841,
 'precision': 0.7776246068374717,
 'recall': 0.5939569536423841,
 'support': 4832}
## test
PPCR: 0.715729
### report
macro
{'f1-score': 0.26913580246913577,
 'precision': 0.31690140845070425,
 'recall': 0.25,
 'support': 1629}
micro
{'f1-score': 0.9054634745242481,
 'precision': 0.905463474524248,
 'recall': 0.905463474524248,
 'support': 1629}
weighted
{'f1-score': 0.8743560011822749,
 'precision': 0.8613769788775626,
 'recall': 0.905463474524248,
 'support': 1629}
### report_full
macro
{'f1-score': 0.19449192265737278,
 'precision': 0.31690140845070425,
 'recall': 0.17786521041165004,
 'support': 2276}
micro
{'f1-score': 0.7554417413572343,
 'precision': 0.905463474524248,
 'recall': 0.6480667838312829,
 'support': 2276}
weighted
{'f1-score': 0.6635596897270535,
 'precision': 0.8063070868090795,
 'recall': 0.6480667838312829,
 'support': 2276}
```

## javascript
### Summary
3 rules, avg.len. 3.0

| | |
|-|-|
|Min support|152|
|Max support|1041|
|Min confidence|0.9927954077720642|
|Max confidence|0.9967319965362549|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1041.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 3 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 153.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0, "max_conf": 0.9967319965362549, "max_support": 1041, "min_conf": 0.9927954077720642, "min_support": 152, "num_rules": 3}}
```
</details>
