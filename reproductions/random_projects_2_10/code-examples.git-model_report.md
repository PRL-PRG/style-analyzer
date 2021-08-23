# Model report for file:///tmp/top-repos-quality-repos-nstzg0kk/code-examples.git HEAD 0084688cf3513eeec12072e877036fc9ace15faa

### Dump

```json
{'created_at': '2021-08-22 05:03:57',
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
 'size': '13.1 kB',
 'tags': [],
 'uuid': 'c3b44119-ab58-4fe3-a8e6-01fb704ac7ab',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-nstzg0kk/code-examples.git 0084688cf3513eeec12072e877036fc9ace15faa

# javascript
11 rules, avg.len. 2.6
## train
PPCR: 0.478802
### report
macro
{'f1-score': 0.45287013523888264,
 'precision': 0.441815641832077,
 'recall': 0.4661341040269576,
 'support': 1039}
micro
{'f1-score': 0.888354186717998,
 'precision': 0.888354186717998,
 'recall': 0.888354186717998,
 'support': 1039}
weighted
{'f1-score': 0.8693923233769597,
 'precision': 0.8539937024939426,
 'recall': 0.888354186717998,
 'support': 1039}
### report_full
macro
{'f1-score': 0.3249721795991835,
 'precision': 0.441815641832077,
 'recall': 0.25732242313267717,
 'support': 2170}
micro
{'f1-score': 0.5752570894359613,
 'precision': 0.888354186717998,
 'recall': 0.4253456221198157,
 'support': 2170}
weighted
{'f1-score': 0.5388517141180815,
 'precision': 0.7365105754893131,
 'recall': 0.4253456221198157,
 'support': 2170}
## test
PPCR: 0.577697
### report
macro
{'f1-score': 0.4151593453919035,
 'precision': 0.3930611559139785,
 'recall': 0.4455964325529543,
 'support': 316}
micro
{'f1-score': 0.8164556962025317,
 'precision': 0.8164556962025317,
 'recall': 0.8164556962025317,
 'support': 316}
weighted
{'f1-score': 0.7833164338904697,
 'precision': 0.7599243313597387,
 'recall': 0.8164556962025317,
 'support': 316}
### report_full
macro
{'f1-score': 0.3121960826926734,
 'precision': 0.3930611559139785,
 'recall': 0.26142467961016036,
 'support': 547}
micro
{'f1-score': 0.5979142526071842,
 'precision': 0.8164556962025317,
 'recall': 0.4716636197440585,
 'support': 547}
weighted
{'f1-score': 0.5757811146288688,
 'precision': 0.7478646969786323,
 'recall': 0.4716636197440585,
 'support': 547}
```

## javascript
### Summary
4 rules, avg.len. 1.8

| | |
|-|-|
|Min support|302|
|Max support|482|
|Min confidence|0.9533194899559021|
|Max confidence|0.9985590577125549|

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
                     'max_depth': 10,
                     'min_samples_leaf': 104,
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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 339.` |
| 2 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 347.` |
| 3 | `  ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 482.` |
| 4 | `  ^1.roles in {IDENTIFIER} and not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 302.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.75, "max_conf": 0.9985590577125549, "max_support": 482, "min_conf": 0.9533194899559021, "min_support": 302, "num_rules": 4}}
```
</details>
