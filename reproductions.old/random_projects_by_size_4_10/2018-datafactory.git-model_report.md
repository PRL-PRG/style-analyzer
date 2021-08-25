# Model report for file:///tmp/top-repos-quality-repos-ozlyf1ce/2018-datafactory.git HEAD 730a808554866c4d2312b6a3d9fae0c1f23c8929

### Dump

```json
{'created_at': '2021-08-21 19:53:01',
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
 'size': '45.6 kB',
 'tags': [],
 'uuid': '5c341a5e-13ac-4262-8918-9f7488d302f1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ozlyf1ce/2018-datafactory.git 730a808554866c4d2312b6a3d9fae0c1f23c8929

# javascript
381 rules, avg.len. 9.5
## train
PPCR: 0.950733
### report
macro
{'f1-score': 0.2724804254342156,
 'precision': 0.34526542435310653,
 'recall': 0.26084459889218986,
 'support': 157411}
micro
{'f1-score': 0.9259136909110545,
 'precision': 0.9259136909110545,
 'recall': 0.9259136909110545,
 'support': 157411}
weighted
{'f1-score': 0.9158868280972102,
 'precision': 0.9167486578776413,
 'recall': 0.9259136909110545,
 'support': 157411}
### report_full
macro
{'f1-score': 0.25792443511377605,
 'precision': 0.34526542435310653,
 'recall': 0.2436961743716335,
 'support': 165568}
micro
{'f1-score': 0.9025292666086651,
 'precision': 0.9259136909110545,
 'recall': 0.8802969172787012,
 'support': 165568}
weighted
{'f1-score': 0.8822348968358893,
 'precision': 0.9045804172299178,
 'recall': 0.8802969172787012,
 'support': 165568}
## test
PPCR: 0.941020
### report
macro
{'f1-score': 0.22857452660073163,
 'precision': 0.25414593785804823,
 'recall': 0.23706850287521125,
 'support': 30043}
micro
{'f1-score': 0.9184835069733382,
 'precision': 0.9184835069733382,
 'recall': 0.9184835069733382,
 'support': 30043}
weighted
{'f1-score': 0.9146648115286256,
 'precision': 0.9173623625403355,
 'recall': 0.9184835069733382,
 'support': 30043}
### report_full
macro
{'f1-score': 0.2170544676420736,
 'precision': 0.25414593785804823,
 'recall': 0.2126532525420362,
 'support': 31926}
micro
{'f1-score': 0.8905743194177734,
 'precision': 0.9184835069733382,
 'recall': 0.8643112196955459,
 'support': 31926}
weighted
{'f1-score': 0.8761785537302472,
 'precision': 0.9018843147045889,
 'recall': 0.8643112196955459,
 'support': 31926}
```

## javascript
### Summary
159 rules, avg.len. 9.1

| | |
|-|-|
|Min support|143|
|Max support|27517|
|Min confidence|0.9206815361976624|
|Max confidence|0.9997975826263428|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = )<br>	∧ -2.reserved = )<br>	∧ +4.length ≥ 3<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 227.` |
| 2 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 27469.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.943. Support: 1646.` |
| 4 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 6263.` |
| 5 | `  -1.reserved not in {(, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.999. Support: 372.` |
| 6 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles not in {KEY}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.946. Support: 1366.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 389.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 332.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 12738.` |
| 10 | `  •••start_col ≥ 20<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 1188.` |
| 11 | `  -1.roles in {STRING}<br>	∧ -4.roles in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.934. Support: 615.` |
| 12 | `  -1.roles in {STRING}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.946. Support: 3484.` |
| 13 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 4326.` |
| 14 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.940. Support: 1043.` |
| 15 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {VALUE}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 413.` |
| 16 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.923. Support: 200.` |
| 17 | `  -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 152.` |
| 18 | `  -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 16363.` |
| 19 | `  -1.reserved = {<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 877.` |
| 20 | `  -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 1106.` |
| 21 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.941. Support: 1421.` |
| 22 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 539.` |
| 23 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 13722.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.962. Support: 596.` |
| 25 | `  -1.internal_type = StringLiteral<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.953. Support: 3475.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 4475.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.952. Support: 1007.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.959. Support: 183.` |
| 29 | `  •••start_line ≤ 166<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {ADD, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 372.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.927. Support: 938.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 150.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 16533.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 22<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2302.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 22<br>	∧ +1.reserved not in {,, :, ;, ?, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2100.` |
| 35 | `  -1.reserved = )<br>	∧ -2.reserved = )<br>	∧ +4.length ≥ 3<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 238.` |
| 36 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 27249.` |
| 37 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.935. Support: 1716.` |
| 38 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 6329.` |
| 39 | `  -1.reserved = {<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INITIALIZATION, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 808.` |
| 40 | `  •••start_col ≥ 10<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED}<br>	∧ ^2.internal_type = IfStatement<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 1369.` |
| 41 | `  •••start_col ≥ 10<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<newline>} and not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {IfStatement}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 1080.` |
| 42 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles in {KEY}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 9<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = "<br>Confidence: 0.941. Support: 315.` |
| 43 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles not in {KEY}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = '<br>Confidence: 0.941. Support: 1408.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 10<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 842.` |
| 45 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 438.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 359.` |
| 47 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LIST, QUALIFIED, STATEMENT}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 606.` |
| 48 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LIST, QUALIFIED, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 12773.` |
| 49 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.947. Support: 636.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INITIALIZATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 4538.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.952. Support: 219.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 188.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 441.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {), :, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.925. Support: 286.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 143.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 17650.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2461.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2717.` |
| 59 | `  -1.reserved = {<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 846.` |
| 60 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.999. Support: 335.` |
| 61 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 380.` |
| 62 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 300.` |
| 63 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.reserved not in {:}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 12604.` |
| 64 | `  •••start_col ≥ 28<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.reserved not in {:}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 637.` |
| 65 | `  -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {{}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.940. Support: 3620.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 998.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 751.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 899.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 16088.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2424.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {,, :, ;, ?, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2594.` |
| 72 | `  -1.reserved = )<br>	∧ -2.reserved = )<br>	∧ +4.length ≥ 3<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 237.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 27408.` |
| 74 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.935. Support: 1775.` |
| 75 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 6040.` |
| 76 | `  -1.reserved = {<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 831.` |
| 77 | `  •••start_col ≥ 8<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>	∧ ^2.internal_type = IfStatement<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 1387.` |
| 78 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = "<br>Confidence: 0.999. Support: 359.` |
| 79 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles not in {KEY}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = '<br>Confidence: 0.941. Support: 1406.` |
| 80 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 10<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 851.` |
| 81 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 373.` |
| 82 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 342.` |
| 83 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 12600.` |
| 84 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1109.` |
| 85 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 1<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 170.` |
| 86 | `  -1.roles in {STRING}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.942. Support: 3492.` |
| 87 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<newline>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.955. Support: 1043.` |
| 88 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <newline>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 374.` |
| 89 | `  •••start_line ≥ 114<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <newline>, <space>}<br>	∧ -5.diff_col ≥ 14<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.922. Support: 737.` |
| 90 | `  •••start_line ≥ 114<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <newline>, <space>}<br>	∧ -5.diff_col ≤ 13<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {RIGHT} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.941. Support: 196.` |
| 91 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 4267.` |
| 92 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.925. Support: 206.` |
| 93 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 736.` |
| 94 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 912.` |
| 95 | `  -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 172.` |
| 96 | `  -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 16158.` |
| 97 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, :, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 4999.` |
| 98 | `  -1.reserved = {<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 848.` |
| 99 | `  -1.reserved not in {(, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.999. Support: 343.` |
| 100 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = '<br>Confidence: 0.951. Support: 1410.` |
| 101 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 11<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 835.` |
| 102 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 10<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 388.` |
| 103 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 12751.` |
| 104 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<-space>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 938.` |
| 105 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 3<br>	∧ +3.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 269.` |
| 106 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 16293.` |
| 107 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.diff_offset ≤ 18<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, :, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 4893.` |
| 108 | `  -1.label in {<newline>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED, STATEMENT}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = "<br>Confidence: 0.999. Support: 349.` |
| 109 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 10<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 934.` |
| 110 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 362.` |
| 111 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 330.` |
| 112 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LIST, QUALIFIED, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 12638.` |
| 113 | `  -1.internal_type = StringLiteral<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.943. Support: 3547.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 987.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ObjectProperty<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.943. Support: 484.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 923.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 6<br>	∧ +1.reserved not in {,, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 433.` |
| 118 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.935. Support: 1672.` |
| 119 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 6100.` |
| 120 | `  •••start_line ≥ 93<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {module}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>	∧ ^2.roles in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.928. Support: 352.` |
| 121 | `  •••start_col ≥ 10<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<-space>} and not in {<-space>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 1378.` |
| 122 | `  •••start_col ≥ 10<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ -4.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>	∧ ^2.internal_type = IfStatement<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 981.` |
| 123 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {FILE, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 522.` |
| 124 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 12693.` |
| 125 | `  -1.internal_type = StringLiteral<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.roles in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = "<br>Confidence: 0.935. Support: 620.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.962. Support: 955.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 604.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 16220.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 18<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, :, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 4871.` |
| 130 | `  •••start_line ≥ 95<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.reserved not in {module}<br>	∧ +1.length ≤ 3<br>	∧ +3.length ≥ 6<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 294.` |
| 131 | `  •••start_line ≥ 95<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≥ 9<br>	∧ +1.reserved not in {module}<br>	∧ +1.length ≤ 3<br>	∧ +3.length ≤ 5<br>	∧ +4.length ≥ 6<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 205.` |
| 132 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 11<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 829.` |
| 133 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 10<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 402.` |
| 134 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 353.` |
| 135 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.reserved not in {:}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 12441.` |
| 136 | `  •••start_col ≥ 29<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.reserved not in {:}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 638.` |
| 137 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LIST, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 322.` |
| 138 | `  -1.internal_type = StringLiteral<br>	∧ -4.label not in {<newline>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.947. Support: 3419.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 4476.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<-space>} and not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 976.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {MAP} and not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.936. Support: 864.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.969. Support: 208.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 719.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.926. Support: 928.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 144.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -2.diff_offset ≤ 18<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2255.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -2.diff_offset ≤ 18<br>	∧ -2.reserved not in {)}<br>	∧ -3.length ≥ 6<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 1378.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -2.diff_offset ≤ 18<br>	∧ -2.label in {<-space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.length ≤ 5<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 167.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 16346.` |
| 150 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2470.` |
| 151 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2846.` |
| 152 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 27517.` |
| 153 | `  •••start_col ≥ 9<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>	∧ ^2.internal_type = IfStatement<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 492.` |
| 154 | `  •••start_col ≥ 21<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, var, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1178.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 673.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 181.` |
| 157 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.942. Support: 215.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 18<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2428.` |
| 159 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 18<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 2781.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.08805031446541, "max_conf": 0.9997975826263428, "max_support": 27517, "min_conf": 0.9206815361976624, "min_support": 143, "num_rules": 159}}
```
</details>
