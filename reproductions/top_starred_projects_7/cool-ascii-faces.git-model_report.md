# Model report for file:///tmp/top-repos-quality-repos-6epgs1o4/cool-ascii-faces.git HEAD c7c04b4c7ce2877bf663efdfac4dcc182b53fe4e

### Dump

```json
{'created_at': '2021-08-31 12:58:15',
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
 'size': '10.2 kB',
 'tags': [],
 'uuid': '2c2df865-c09f-418e-b34f-f6cc4349b72d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-6epgs1o4/cool-ascii-faces.git c7c04b4c7ce2877bf663efdfac4dcc182b53fe4e

# javascript
10 rules, avg.len. 1.0
## train
PPCR: 1.000000
### report
macro
{'f1-score': 0.9986166487594819,
 'precision': 0.9990825688073395,
 'recall': 0.9981549815498155,
 'support': 815}
micro
{'f1-score': 0.9987730061349693,
 'precision': 0.9987730061349693,
 'recall': 0.9987730061349693,
 'support': 815}
weighted
{'f1-score': 0.9987724354876135,
 'precision': 0.9987752574998593,
 'recall': 0.9987730061349693,
 'support': 815}
### report_full
macro
{'f1-score': 0.9986166487594819,
 'precision': 0.9990825688073395,
 'recall': 0.9981549815498155,
 'support': 815}
micro
{'f1-score': 0.9987730061349693,
 'precision': 0.9987730061349693,
 'recall': 0.9987730061349693,
 'support': 815}
weighted
{'f1-score': 0.9987724354876135,
 'precision': 0.9987752574998593,
 'recall': 0.9987730061349693,
 'support': 815}
## test
PPCR: 1.000000
### report
macro
{'f1-score': 0.4166666666666667,
 'precision': 0.6666666666666666,
 'recall': 0.6,
 'support': 7}
micro
{'f1-score': 0.42857142857142855,
 'precision': 0.42857142857142855,
 'recall': 0.42857142857142855,
 'support': 7}
weighted
{'f1-score': 0.380952380952381,
 'precision': 0.8095238095238095,
 'recall': 0.42857142857142855,
 'support': 7}
### report_full
macro
{'f1-score': 0.4166666666666667,
 'precision': 0.6666666666666666,
 'recall': 0.6,
 'support': 7}
micro
{'f1-score': 0.42857142857142855,
 'precision': 0.42857142857142855,
 'recall': 0.42857142857142855,
 'support': 7}
weighted
{'f1-score': 0.380952380952381,
 'precision': 0.8095238095238095,
 'recall': 0.42857142857142855,
 'support': 7}
```

## javascript
### Summary
10 rules, avg.len. 1.0

| | |
|-|-|
|Min support|216|
|Max support|436|
|Min confidence|0.9976851940155029|
|Max confidence|0.9988532066345215|

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
                     'min_samples_leaf': 118,
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
| 1 | `  -2.diff_offset ≥ 3<br>⇒ y = "<br>Confidence: 0.999. Support: 427.` |
| 2 | `  -2.diff_offset ≤ 3<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 225.` |
| 3 | `  -1.diff_offset ≥ 2<br>⇒ y = "<br>Confidence: 0.999. Support: 417.` |
| 4 | `  -1.diff_offset ≤ 2<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 235.` |
| 5 | `  -2.reserved = "<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 225.` |
| 6 | `  -2.reserved not in {"}<br>⇒ y = "<br>Confidence: 0.999. Support: 427.` |
| 7 | `  -1.length ≥ 2<br>⇒ y = "<br>Confidence: 0.999. Support: 412.` |
| 8 | `  -1.length ≤ 1<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 240.` |
| 9 | `  -4.label in {"}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 216.` |
| 10 | `  -4.label not in {"}<br>⇒ y = "<br>Confidence: 0.999. Support: 436.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9988532066345215, "max_support": 436, "min_conf": 0.9976851940155029, "min_support": 216, "num_rules": 10}}
```
</details>
