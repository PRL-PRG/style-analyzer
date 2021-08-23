# Model report for file:///tmp/top-repos-quality-repos-hmj6uupd/pdc.git HEAD 6330d567bd2d77c3a0fbce5d9100300e308aab0b

### Dump

```json
{'created_at': '2021-08-21 11:22:44',
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
 'size': '17.7 kB',
 'tags': [],
 'uuid': '32aa9371-64ae-4470-957b-1f79d3f06e3f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-hmj6uupd/pdc.git 6330d567bd2d77c3a0fbce5d9100300e308aab0b

# javascript
11 rules, avg.len. 5.9
## train
PPCR: 0.643130
### report
macro
{'f1-score': 0.41149254166706045,
 'precision': 0.39891096567781853,
 'recall': 0.4263291073369941,
 'support': 6239}
micro
{'f1-score': 0.9557621413688091,
 'precision': 0.9557621413688091,
 'recall': 0.9557621413688091,
 'support': 6239}
weighted
{'f1-score': 0.9445344109528693,
 'precision': 0.9342729282591292,
 'recall': 0.9557621413688091,
 'support': 6239}
### report_full
macro
{'f1-score': 0.3212211659645556,
 'precision': 0.39891096567781853,
 'recall': 0.2743293931129414,
 'support': 9701}
micro
{'f1-score': 0.7481806775407779,
 'precision': 0.9557621413688091,
 'recall': 0.6146788990825688,
 'support': 9701}
weighted
{'f1-score': 0.7019330845501823,
 'precision': 0.8542880713672818,
 'recall': 0.6146788990825688,
 'support': 9701}
## test
PPCR: 0.652116
### report
macro
{'f1-score': 0.41273607588961037,
 'precision': 0.4011625966196485,
 'recall': 0.4264408968444926,
 'support': 5916}
micro
{'f1-score': 0.9623056118999324,
 'precision': 0.9623056118999324,
 'recall': 0.9623056118999324,
 'support': 5916}
weighted
{'f1-score': 0.9539363339538687,
 'precision': 0.9463423392112199,
 'recall': 0.9623056118999324,
 'support': 5916}
### report_full
macro
{'f1-score': 0.3257800335266601,
 'precision': 0.4011625966196485,
 'recall': 0.2796723507945809,
 'support': 9072}
micro
{'f1-score': 0.75967440619162,
 'precision': 0.9623056118999324,
 'recall': 0.6275352733686067,
 'support': 9072}
weighted
{'f1-score': 0.7138699369182598,
 'precision': 0.8632317963195063,
 'recall': 0.6275352733686067,
 'support': 9072}
```

## javascript
### Summary
7 rules, avg.len. 5.7

| | |
|-|-|
|Min support|158|
|Max support|1823|
|Min confidence|0.9361510872840881|
|Max confidence|0.9982331991195679|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1823.` |
| 2 | `  -1.roles not in {LITERAL}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label in {<space>}<br>	∧ +2.roles in {BINARY}<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 3 | `  -1.roles not in {LITERAL}<br>	∧ -3.label not in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 841.` |
| 4 | `  +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 556.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 266.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 283.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {), ;}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {VARIABLE} and not in {CALL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 308.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.714285714285714, "max_conf": 0.9982331991195679, "max_support": 1823, "min_conf": 0.9361510872840881, "min_support": 158, "num_rules": 7}}
```
</details>
