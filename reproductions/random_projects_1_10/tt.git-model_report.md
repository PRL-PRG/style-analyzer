# Model report for file:///tmp/top-repos-quality-repos-8lvps42i/tt.git HEAD 0a5d7c2520e9cbd93b2677f39cbe8e1b42cc611f

### Dump

```json
{'created_at': '2021-08-22 11:43:58',
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
 'size': '15.5 kB',
 'tags': [],
 'uuid': 'c9b3be10-e95b-4300-99de-8b59f0b0384b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8lvps42i/tt.git 0a5d7c2520e9cbd93b2677f39cbe8e1b42cc611f

# javascript
3 rules, avg.len. 3.0
## train
PPCR: 0.497886
### report
macro
{'f1-score': 0.36677934821136476,
 'precision': 0.3962339295459091,
 'recall': 0.35320325746334713,
 'support': 1649}
micro
{'f1-score': 0.8568829593693147,
 'precision': 0.8568829593693147,
 'recall': 0.8568829593693147,
 'support': 1649}
weighted
{'f1-score': 0.8257671236181068,
 'precision': 0.8225235111226622,
 'recall': 0.8568829593693147,
 'support': 1649}
### report_full
macro
{'f1-score': 0.24704955071488932,
 'precision': 0.3962339295459091,
 'recall': 0.19432346194854605,
 'support': 3312}
micro
{'f1-score': 0.569643217093328,
 'precision': 0.8568829593693147,
 'recall': 0.4266304347826087,
 'support': 3312}
weighted
{'f1-score': 0.5116723298578486,
 'precision': 0.7531632227736623,
 'recall': 0.4266304347826087,
 'support': 3312}
## test
PPCR: 0.533887
### report
macro
{'f1-score': 0.3430548545103055,
 'precision': 0.37831928138050586,
 'recall': 0.33472949549939585,
 'support': 386}
micro
{'f1-score': 0.7694300518134715,
 'precision': 0.7694300518134715,
 'recall': 0.7694300518134715,
 'support': 386}
weighted
{'f1-score': 0.7228126662602835,
 'precision': 0.7336418183013298,
 'recall': 0.7694300518134715,
 'support': 386}
### report_full
macro
{'f1-score': 0.24899064884999086,
 'precision': 0.37831928138050586,
 'recall': 0.20973834368325245,
 'support': 723}
micro
{'f1-score': 0.5356176735798016,
 'precision': 0.7694300518134715,
 'recall': 0.4107883817427386,
 'support': 723}
weighted
{'f1-score': 0.460418285420385,
 'precision': 0.6695879764937264,
 'recall': 0.4107883817427386,
 'support': 723}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|97|
|Max support|158|
|Min confidence|0.9398733973503113|
|Max confidence|0.9948453903198242|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.995. Support: 97.` |
| 2 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 5<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 158.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9948453903198242, "max_support": 158, "min_conf": 0.9398733973503113, "min_support": 97, "num_rules": 2}}
```
</details>
