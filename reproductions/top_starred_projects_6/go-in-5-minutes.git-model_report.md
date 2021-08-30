# Model report for file:///tmp/top-repos-quality-repos-r2apbz83/go-in-5-minutes.git HEAD 5d8b5800e21970eb83e4c7b30199f73963315580

### Dump

```json
{'created_at': '2021-08-29 22:25:31',
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
 'size': '14.6 kB',
 'tags': [],
 'uuid': '7a63fbf9-b65f-4c1d-bf52-d542b31c2ad5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-r2apbz83/go-in-5-minutes.git 5d8b5800e21970eb83e4c7b30199f73963315580

# javascript
5 rules, avg.len. 3.6
## train
PPCR: 0.634665
### report
macro
{'f1-score': 0.45884314362712625,
 'precision': 0.4545638998682477,
 'recall': 0.4671922673531655,
 'support': 1732}
micro
{'f1-score': 0.8897228637413395,
 'precision': 0.8897228637413395,
 'recall': 0.8897228637413395,
 'support': 1732}
weighted
{'f1-score': 0.8602797100594546,
 'precision': 0.8395981159116013,
 'recall': 0.8897228637413395,
 'support': 1732}
### report_full
macro
{'f1-score': 0.39749090664730186,
 'precision': 0.4545638998682477,
 'recall': 0.36722531591999097,
 'support': 2729}
micro
{'f1-score': 0.6908764850930286,
 'precision': 0.8897228637413395,
 'recall': 0.5646757053865885,
 'support': 2729}
weighted
{'f1-score': 0.6278298440816386,
 'precision': 0.7348194201643307,
 'recall': 0.5646757053865885,
 'support': 2729}
## test
PPCR: 0.631313
### report
macro
{'f1-score': 0.4591930922863791,
 'precision': 0.45247248997248996,
 'recall': 0.46925287356321843,
 'support': 375}
micro
{'f1-score': 0.9013333333333333,
 'precision': 0.9013333333333333,
 'recall': 0.9013333333333333,
 'support': 375}
weighted
{'f1-score': 0.8759583024906571,
 'precision': 0.8570012138012137,
 'recall': 0.9013333333333333,
 'support': 375}
### report_full
macro
{'f1-score': 0.39264864980665615,
 'precision': 0.45247248997248996,
 'recall': 0.36136020818974,
 'support': 594}
micro
{'f1-score': 0.6976264189886481,
 'precision': 0.9013333333333333,
 'recall': 0.569023569023569,
 'support': 594}
weighted
{'f1-score': 0.6486274097379853,
 'precision': 0.7799563416987659,
 'recall': 0.569023569023569,
 'support': 594}
```

## javascript
### Summary
2 rules, avg.len. 1.5

| | |
|-|-|
|Min support|121|
|Max support|206|
|Min confidence|0.9958677887916565|
|Max confidence|0.9975728392601013|

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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.998. Support: 206.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>⇒ y = "<br>Confidence: 0.996. Support: 121.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.5, "max_conf": 0.9975728392601013, "max_support": 206, "min_conf": 0.9958677887916565, "min_support": 121, "num_rules": 2}}
```
</details>
