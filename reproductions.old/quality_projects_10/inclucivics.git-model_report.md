# Model report for file:///tmp/top-repos-quality-repos-xg5skbws/inclucivics.git HEAD 1a89419c5b414b0aca7c72316fa9fd5f22d5f1c9

### Dump

```json
{'created_at': '2021-08-24 20:39:27',
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
 'uuid': '1ad24042-ea71-460f-bce7-568624f0ae97',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xg5skbws/inclucivics.git 1a89419c5b414b0aca7c72316fa9fd5f22d5f1c9

# javascript
4 rules, avg.len. 4.5
## train
PPCR: 0.637255
### report
macro
{'f1-score': 0.36237972130125307,
 'precision': 0.3952603239230729,
 'recall': 0.3556390977443609,
 'support': 2665}
micro
{'f1-score': 0.8555347091932458,
 'precision': 0.8555347091932458,
 'recall': 0.8555347091932458,
 'support': 2665}
weighted
{'f1-score': 0.813757044074123,
 'precision': 0.8064026017129171,
 'recall': 0.8555347091932458,
 'support': 2665}
### report_full
macro
{'f1-score': 0.2940208763780017,
 'precision': 0.3952603239230729,
 'recall': 0.2732054098644406,
 'support': 4182}
micro
{'f1-score': 0.6659851029648022,
 'precision': 0.8555347091932458,
 'recall': 0.5451936872309899,
 'support': 4182}
weighted
{'f1-score': 0.570257761621455,
 'precision': 0.7351248660711948,
 'recall': 0.5451936872309899,
 'support': 4182}
## test
PPCR: 0.582492
### report
macro
{'f1-score': 0.3071941741383452,
 'precision': 0.38505323551755805,
 'recall': 0.31266846361185985,
 'support': 346}
micro
{'f1-score': 0.8092485549132948,
 'precision': 0.8092485549132948,
 'recall': 0.8092485549132948,
 'support': 346}
weighted
{'f1-score': 0.7444823604979893,
 'precision': 0.7817674767288164,
 'recall': 0.8092485549132948,
 'support': 346}
### report_full
macro
{'f1-score': 0.2565616215830934,
 'precision': 0.38505323551755805,
 'recall': 0.24944862845310128,
 'support': 594}
micro
{'f1-score': 0.5957446808510639,
 'precision': 0.8092485549132948,
 'recall': 0.4713804713804714,
 'support': 594}
weighted
{'f1-score': 0.475634251821522,
 'precision': 0.7386218990711411,
 'recall': 0.4713804713804714,
 'support': 594}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|131|
|Max support|157|
|Min confidence|0.977707028388977|
|Max confidence|0.9961832165718079|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.996. Support: 131.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 157.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9961832165718079, "max_support": 157, "min_conf": 0.977707028388977, "min_support": 131, "num_rules": 2}}
```
</details>
