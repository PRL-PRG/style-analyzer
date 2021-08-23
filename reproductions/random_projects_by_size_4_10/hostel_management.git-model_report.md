# Model report for file:///tmp/top-repos-quality-repos-khao_4x6/hostel_management.git HEAD d7707a5cf93972867bcb77ff643ae10d27859230

### Dump

```json
{'created_at': '2021-08-21 20:52:30',
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
 'size': '27.2 kB',
 'tags': [],
 'uuid': '9ae981b4-4c83-4937-a86e-83c557e3296f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-khao_4x6/hostel_management.git d7707a5cf93972867bcb77ff643ae10d27859230

# javascript
214 rules, avg.len. 8.7
## train
PPCR: 0.897643
### report
macro
{'f1-score': 0.3911221267946247,
 'precision': 0.40159184913407564,
 'recall': 0.39119925841470043,
 'support': 86268}
micro
{'f1-score': 0.9242361014512913,
 'precision': 0.9242361014512913,
 'recall': 0.9242361014512913,
 'support': 86268}
weighted
{'f1-score': 0.9162261273929301,
 'precision': 0.9130076270722742,
 'recall': 0.9242361014512913,
 'support': 86268}
### report_full
macro
{'f1-score': 0.34147985564296174,
 'precision': 0.40159184913407564,
 'recall': 0.3139429647255422,
 'support': 96105}
micro
{'f1-score': 0.8743838177800443,
 'precision': 0.9242361014512913,
 'recall': 0.8296342542011341,
 'support': 96105}
weighted
{'f1-score': 0.8461735596088736,
 'precision': 0.8783180957530995,
 'recall': 0.8296342542011341,
 'support': 96105}
## test
PPCR: 0.934587
### report
macro
{'f1-score': 0.38558710688104775,
 'precision': 0.4001539552190553,
 'recall': 0.37509228885219037,
 'support': 14016}
micro
{'f1-score': 0.9521261415525114,
 'precision': 0.9521261415525114,
 'recall': 0.9521261415525114,
 'support': 14016}
weighted
{'f1-score': 0.9484316534625151,
 'precision': 0.9470700023625931,
 'recall': 0.9521261415525114,
 'support': 14016}
### report_full
macro
{'f1-score': 0.35487993749290514,
 'precision': 0.4001539552190553,
 'recall': 0.32700762098092057,
 'support': 14997}
micro
{'f1-score': 0.9199324440767931,
 'precision': 0.9521261415525114,
 'recall': 0.8898446355937855,
 'support': 14997}
weighted
{'f1-score': 0.9079383724041117,
 'precision': 0.9370739160631614,
 'recall': 0.8898446355937855,
 'support': 14997}
```

## javascript
### Summary
90 rules, avg.len. 8.2

| | |
|-|-|
|Min support|162|
|Max support|14725|
|Min confidence|0.9211582541465759|
|Max confidence|0.998046875|

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
                     'min_samples_split': 187,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 14667.` |
| 2 | `  -1.reserved not in {(}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 1300.` |
| 3 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.931. Support: 511.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 853.` |
| 5 | `  -1.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.922. Support: 468.` |
| 6 | `  -1.roles in {STRING} and not in {RIGHT}<br>	∧ -5.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = '<br>Confidence: 0.936. Support: 431.` |
| 7 | `  -1.roles in {STRING} and not in {RIGHT}<br>	∧ -5.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = '<br>Confidence: 0.952. Support: 178.` |
| 8 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1745.` |
| 9 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 12715.` |
| 10 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2743.` |
| 11 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 2281.` |
| 12 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1087.` |
| 13 | `  •••start_line ≤ 245<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 225.` |
| 14 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 227.` |
| 15 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 4602.` |
| 16 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.933. Support: 441.` |
| 17 | `  -1.label not in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 1744.` |
| 18 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 826.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1872.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 12757.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2765.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 2242.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1023.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 256.` |
| 25 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 184.` |
| 26 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 14678.` |
| 27 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 234.` |
| 28 | `  •••start_line ≤ 245<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 190.` |
| 29 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 357.` |
| 30 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 3982.` |
| 31 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≤ 5<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 641.` |
| 32 | `  •••start_col ≥ 8<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ -3.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {ADD} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 182.` |
| 33 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.981. Support: 14725.` |
| 34 | `  -1.reserved not in {(}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1286.` |
| 35 | `  -1.reserved = ,<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 1568.` |
| 36 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 834.` |
| 37 | `  •••start_col ≤ 92<br>	∧ -1.diff_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {SWITCH}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 8208.` |
| 38 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1787.` |
| 39 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 12708.` |
| 40 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2800.` |
| 41 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 2266.` |
| 42 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 1119.` |
| 43 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 251.` |
| 44 | `  •••start_line ≤ 245<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 208.` |
| 45 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 4594.` |
| 46 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, =}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LEFT}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 700.` |
| 47 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LEFT}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 162.` |
| 48 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 14720.` |
| 49 | `  -1.reserved not in {(}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1317.` |
| 50 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 839.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 2390.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 1764.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 326.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = +<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 551.` |
| 56 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 571.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 12623.` |
| 58 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 5757.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 14554.` |
| 60 | `  -1.reserved = ,<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 1560.` |
| 61 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 842.` |
| 62 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 63 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 2328.` |
| 64 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 1786.` |
| 65 | `  -1.diff_col ≤ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {ARITHMETIC} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 264.` |
| 66 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {ADD, OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 178.` |
| 67 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 315.` |
| 68 | `  -1.reserved not in {if}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 12837.` |
| 69 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 5732.` |
| 70 | `  •••start_col ≤ 92<br>	∧ -1.diff_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 8189.` |
| 71 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 12777.` |
| 72 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2757.` |
| 73 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 2259.` |
| 74 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1056.` |
| 75 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 250.` |
| 76 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 174.` |
| 77 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 172.` |
| 78 | `  -1.reserved not in {(, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.935. Support: 466.` |
| 79 | `  -1.roles in {STRING} and not in {KEY, RIGHT}<br>	∧ -5.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.940. Support: 390.` |
| 80 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 12833.` |
| 81 | `  -1.reserved not in {(, ,}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.925. Support: 449.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 12809.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2727.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 2224.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1053.` |
| 86 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 87 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 4575.` |
| 88 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.931. Support: 428.` |
| 89 | `  •••start_col ≤ 92<br>	∧ -1.diff_col ≤ 17<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 8174.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 235.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.2, "max_conf": 0.998046875, "max_support": 14725, "min_conf": 0.9211582541465759, "min_support": 162, "num_rules": 90}}
```
</details>
