# Model report for file:///tmp/top-repos-quality-repos-5n7_0qpf/webrtc-book-cn.git HEAD 8de08ace70827edffa4f4c1aa9c600b53b9975f3

### Dump

```json
{'created_at': '2021-08-30 03:11:51',
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
 'size': '15.6 kB',
 'tags': [],
 'uuid': 'e6a9561a-1440-498c-b229-259e6e220346',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5n7_0qpf/webrtc-book-cn.git 8de08ace70827edffa4f4c1aa9c600b53b9975f3

# javascript
9 rules, avg.len. 6.3
## train
PPCR: 0.694854
### report
macro
{'f1-score': 0.5426580035809858,
 'precision': 0.5368666859456334,
 'recall': 0.5489680510776908,
 'support': 2498}
micro
{'f1-score': 0.9527622097678142,
 'precision': 0.9527622097678142,
 'recall': 0.9527622097678142,
 'support': 2498}
weighted
{'f1-score': 0.9434746075783585,
 'precision': 0.9347654966078126,
 'recall': 0.9527622097678142,
 'support': 2498}
### report_full
macro
{'f1-score': 0.4353582988403658,
 'precision': 0.5368666859456334,
 'recall': 0.3797959845000945,
 'support': 3595}
micro
{'f1-score': 0.7812243558181519,
 'precision': 0.9527622097678142,
 'recall': 0.6620305980528511,
 'support': 3595}
weighted
{'f1-score': 0.7347737890545375,
 'precision': 0.863885226332119,
 'recall': 0.6620305980528511,
 'support': 3595}
## test
PPCR: 0.820569
### report
macro
{'f1-score': 0.5292087912087912,
 'precision': 0.5186754472468758,
 'recall': 0.5408703719165437,
 'support': 375}
micro
{'f1-score': 0.9306666666666666,
 'precision': 0.9306666666666666,
 'recall': 0.9306666666666666,
 'support': 375}
weighted
{'f1-score': 0.923163487179487,
 'precision': 0.9166702806702806,
 'recall': 0.9306666666666666,
 'support': 375}
### report_full
macro
{'f1-score': 0.4633525967275528,
 'precision': 0.5186754472468758,
 'recall': 0.43434102934102936,
 'support': 457}
micro
{'f1-score': 0.8389423076923076,
 'precision': 0.9306666666666666,
 'recall': 0.7636761487964989,
 'support': 457}
weighted
{'f1-score': 0.7950706541948906,
 'precision': 0.8460802869993241,
 'recall': 0.7636761487964989,
 'support': 457}
```

## javascript
### Summary
7 rules, avg.len. 5.7

| | |
|-|-|
|Min support|108|
|Max support|683|
|Min confidence|0.9597364664077759|
|Max confidence|0.997890293598175|

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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.997. Support: 164.` |
| 2 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 683.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 108.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 237.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 151.` |
| 6 | `  -1.reserved = (<br>	∧ -1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 115.` |
| 7 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ +1.internal_type = CommentLine<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 109.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.714285714285714, "max_conf": 0.997890293598175, "max_support": 683, "min_conf": 0.9597364664077759, "min_support": 108, "num_rules": 7}}
```
</details>
