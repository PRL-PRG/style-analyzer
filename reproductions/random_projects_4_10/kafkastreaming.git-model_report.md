# Model report for file:///tmp/top-repos-quality-repos-q95g38_n/kafkastreaming.git HEAD 4c41a27b6eb0944b862b13e808415445a50adfcd

### Dump

```json
{'created_at': '2021-08-21 22:51:55',
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
 'uuid': '6433e0fd-7168-40d0-8bc5-67525f849ac6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-q95g38_n/kafkastreaming.git 4c41a27b6eb0944b862b13e808415445a50adfcd

# javascript
1 rules, avg.len. 1.0
## train
PPCR: 0.212095
### report
macro
{'f1-score': 0.24739039665970772,
 'precision': 0.24483471074380164,
 'recall': 0.25,
 'support': 242}
micro
{'f1-score': 0.9793388429752066,
 'precision': 0.9793388429752066,
 'recall': 0.9793388429752066,
 'support': 242}
weighted
{'f1-score': 0.9691160993115823,
 'precision': 0.9591045693600163,
 'recall': 0.9793388429752066,
 'support': 242}
### report_full
macro
{'f1-score': 0.13079470198675494,
 'precision': 0.24483471074380164,
 'recall': 0.08923192771084337,
 'support': 1141}
micro
{'f1-score': 0.34273318872017355,
 'precision': 0.9793388429752066,
 'recall': 0.2077125328659071,
 'support': 1141}
weighted
{'f1-score': 0.30446163757828315,
 'precision': 0.5699219910039766,
 'recall': 0.2077125328659071,
 'support': 1141}
## test
PPCR: 0.275304
### report
macro
{'f1-score': 0.24242424242424243,
 'precision': 0.23529411764705882,
 'recall': 0.25,
 'support': 68}
micro
{'f1-score': 0.9411764705882353,
 'precision': 0.9411764705882353,
 'recall': 0.9411764705882353,
 'support': 68}
weighted
{'f1-score': 0.912655971479501,
 'precision': 0.8858131487889274,
 'recall': 0.9411764705882353,
 'support': 68}
### report_full
macro
{'f1-score': 0.14035087719298245,
 'precision': 0.23529411764705882,
 'recall': 0.1,
 'support': 247}
micro
{'f1-score': 0.40634920634920635,
 'precision': 0.9411764705882353,
 'recall': 0.2591093117408907,
 'support': 247}
weighted
{'f1-score': 0.3636621919170395,
 'precision': 0.6096689688020958,
 'recall': 0.2591093117408907,
 'support': 247}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|196|
|Max support|196|
|Min confidence|0.9719387888908386|
|Max confidence|0.9719387888908386|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 196.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9719387888908386, "max_support": 196, "min_conf": 0.9719387888908386, "min_support": 196, "num_rules": 1}}
```
</details>
