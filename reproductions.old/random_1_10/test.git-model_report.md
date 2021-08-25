# Model report for file:///tmp/top-repos-quality-repos-damb09ci/test.git HEAD de6368ad894f2cbf0b37665a803550ef21c31255

### Dump

```json
{'created_at': '2021-08-20 11:36:22',
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
 'size': '12.7 kB',
 'tags': [],
 'uuid': 'c3ccbdb7-ae21-458c-9fc4-d2569a23f77f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-damb09ci/test.git de6368ad894f2cbf0b37665a803550ef21c31255

# javascript
1 rules, avg.len. 1.0
## train
PPCR: 0.313426
### report
macro
{'f1-score': 0.24865900383141765,
 'precision': 0.24733231707317074,
 'recall': 0.25,
 'support': 656}
micro
{'f1-score': 0.989329268292683,
 'precision': 0.989329268292683,
 'recall': 0.989329268292683,
 'support': 656}
weighted
{'f1-score': 0.9840225212596954,
 'precision': 0.9787724011005354,
 'recall': 0.989329268292683,
 'support': 656}
### report_full
macro
{'f1-score': 0.1593028964162985,
 'precision': 0.24733231707317074,
 'recall': 0.11748732802317162,
 'support': 2093}
micro
{'f1-score': 0.47217169879956344,
 'precision': 0.989329268292683,
 'recall': 0.3100812231247014,
 'support': 2093}
weighted
{'f1-score': 0.4204439559501352,
 'precision': 0.6527776968524582,
 'recall': 0.3100812231247014,
 'support': 2093}
## test
PPCR: 0.282967
### report
macro
{'f1-score': 0.24754901960784315,
 'precision': 0.24514563106796117,
 'recall': 0.25,
 'support': 103}
micro
{'f1-score': 0.9805825242718447,
 'precision': 0.9805825242718447,
 'recall': 0.9805825242718447,
 'support': 103}
weighted
{'f1-score': 0.9709689701123169,
 'precision': 0.9615420869073428,
 'recall': 0.9805825242718447,
 'support': 103}
### report_full
macro
{'f1-score': 0.16185897435897437,
 'precision': 0.24514563106796117,
 'recall': 0.12081339712918661,
 'support': 364}
micro
{'f1-score': 0.4325481798715204,
 'precision': 0.9805825242718447,
 'recall': 0.2774725274725275,
 'support': 364}
weighted
{'f1-score': 0.3717420400112708,
 'precision': 0.5630267790461965,
 'recall': 0.2774725274725275,
 'support': 364}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|519|
|Max support|519|
|Min confidence|0.9874759316444397|
|Max confidence|0.9874759316444397|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 519.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9874759316444397, "max_support": 519, "min_conf": 0.9874759316444397, "min_support": 519, "num_rules": 1}}
```
</details>
