# Model report for file:///tmp/top-repos-quality-repos-5yw7tiei/cs-2.1-trees-sorting.git HEAD 6a03ffdeb043435640af62ee52852d0dce635f59

### Dump

```json
{'created_at': '2021-08-21 19:17:42',
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
 'size': '18.1 kB',
 'tags': [],
 'uuid': '79147d0b-cc47-4e37-8955-143c784d3d2a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5yw7tiei/cs-2.1-trees-sorting.git 6a03ffdeb043435640af62ee52852d0dce635f59

# javascript
24 rules, avg.len. 9.6
## train
PPCR: 0.900711
### report
macro
{'f1-score': 0.7279318140045538,
 'precision': 0.7255976970625801,
 'recall': 0.7347857695724463,
 'support': 28630}
micro
{'f1-score': 0.9517988124345093,
 'precision': 0.9517988124345093,
 'recall': 0.9517988124345093,
 'support': 28630}
weighted
{'f1-score': 0.9495382992675457,
 'precision': 0.9495427522549499,
 'recall': 0.9517988124345093,
 'support': 28630}
### report_full
macro
{'f1-score': 0.6225863062947036,
 'precision': 0.7255976970625801,
 'recall': 0.5612741928093723,
 'support': 31786}
micro
{'f1-score': 0.9020789194915255,
 'precision': 0.9517988124345093,
 'recall': 0.8572956647580696,
 'support': 31786}
weighted
{'f1-score': 0.8829944227758857,
 'precision': 0.9224269753213926,
 'recall': 0.8572956647580696,
 'support': 31786}
## test
PPCR: 0.886228
### report
macro
{'f1-score': 0.5672187358481791,
 'precision': 0.586656378600823,
 'recall': 0.5655651552697967,
 'support': 592}
micro
{'f1-score': 0.856418918918919,
 'precision': 0.856418918918919,
 'recall': 0.856418918918919,
 'support': 592}
weighted
{'f1-score': 0.8573279934529693,
 'precision': 0.8833960523023023,
 'recall': 0.856418918918919,
 'support': 592}
### report_full
macro
{'f1-score': 0.5014467767524562,
 'precision': 0.586656378600823,
 'recall': 0.4573020031224366,
 'support': 668}
micro
{'f1-score': 0.8047619047619049,
 'precision': 0.856418918918919,
 'recall': 0.7589820359281437,
 'support': 668}
weighted
{'f1-score': 0.7963801629853842,
 'precision': 0.8697516078953205,
 'recall': 0.7589820359281437,
 'support': 668}
```

## javascript
### Summary
16 rules, avg.len. 8.8

| | |
|-|-|
|Min support|121|
|Max support|5933|
|Min confidence|0.9430146813392639|
|Max confidence|0.9994239807128906|

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
| 1 | `  +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 4768.` |
| 2 | `  +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 2116.` |
| 3 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.996. Support: 133.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1329.` |
| 5 | `  -1.label in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 868.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 791.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 786.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 634.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 596.` |
| 10 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY}<br>	∧ +1.reserved not in {,, ;}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 218.` |
| 11 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 272.` |
| 13 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {KEY}<br>	∧ -2.diff_line = 0<br>	∧ -2.roles in {FUNCTION} and not in {CALLEE}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 121.` |
| 14 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {KEY}<br>	∧ -3.reserved = .<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 751.` |
| 15 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {KEY}<br>	∧ -3.reserved not in {.}<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 5933.` |
| 16 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≤ 26<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 126.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.75, "max_conf": 0.9994239807128906, "max_support": 5933, "min_conf": 0.9430146813392639, "min_support": 121, "num_rules": 16}}
```
</details>
