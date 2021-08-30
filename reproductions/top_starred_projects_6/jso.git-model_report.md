# Model report for file:///tmp/top-repos-quality-repos-ax5dqr3q/jso.git HEAD b1be102c5c8eedb15c38dea2ba56d83b749eed94

### Dump

```json
{'created_at': '2021-08-29 21:52:45',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '09d61af2-4da7-44ab-a5f6-997182d81272',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ax5dqr3q/jso.git b1be102c5c8eedb15c38dea2ba56d83b749eed94

# javascript
24 rules, avg.len. 5.1
## train
PPCR: 0.706408
### report
macro
{'f1-score': 0.33070664133528377,
 'precision': 0.3381783309607017,
 'recall': 0.3256165645069072,
 'support': 3715}
micro
{'f1-score': 0.8928667563930014,
 'precision': 0.8928667563930014,
 'recall': 0.8928667563930014,
 'support': 3715}
weighted
{'f1-score': 0.8787121451057891,
 'precision': 0.8704564025874509,
 'recall': 0.8928667563930014,
 'support': 3715}
### report_full
macro
{'f1-score': 0.26152053582386675,
 'precision': 0.3381783309607017,
 'recall': 0.22742819519934296,
 'support': 5259}
micro
{'f1-score': 0.7392467127256519,
 'precision': 0.8928667563930014,
 'recall': 0.6307282753375166,
 'support': 5259}
weighted
{'f1-score': 0.6693201676735233,
 'precision': 0.7679482923302394,
 'recall': 0.6307282753375166,
 'support': 5259}
## test
PPCR: 0.702559
### report
macro
{'f1-score': 0.28675273593909456,
 'precision': 0.32429952326635664,
 'recall': 0.2681727296181631,
 'support': 659}
micro
{'f1-score': 0.8907435508345979,
 'precision': 0.8907435508345979,
 'recall': 0.8907435508345979,
 'support': 659}
weighted
{'f1-score': 0.8731936133662056,
 'precision': 0.8631031895312368,
 'recall': 0.8907435508345979,
 'support': 659}
### report_full
macro
{'f1-score': 0.2026593473779002,
 'precision': 0.32429952326635664,
 'recall': 0.18232875109314986,
 'support': 938}
micro
{'f1-score': 0.7351283656856605,
 'precision': 0.8907435508345979,
 'recall': 0.6257995735607675,
 'support': 938}
weighted
{'f1-score': 0.6528048283435527,
 'precision': 0.7784240749897463,
 'recall': 0.6257995735607675,
 'support': 938}
```

## javascript
### Summary
20 rules, avg.len. 4.9

| | |
|-|-|
|Min support|144|
|Max support|810|
|Min confidence|0.9431372284889221|
|Max confidence|0.9973545074462891|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 766.` |
| 2 | `  -1.internal_type = CommentLine<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 201.` |
| 3 | `  -1.internal_type not in {CommentLine}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 4 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.991. Support: 763.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 217.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 7 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 810.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 188.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 173.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 237.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.label in {<newline>}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 255.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 517.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 277.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 232.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 17 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 2<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 251.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 183.` |
| 20 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.9, "max_conf": 0.9973545074462891, "max_support": 810, "min_conf": 0.9431372284889221, "min_support": 144, "num_rules": 20}}
```
</details>
