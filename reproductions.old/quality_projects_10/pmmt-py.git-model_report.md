# Model report for file:///tmp/top-repos-quality-repos-sjpkm6xa/pmmt-py.git HEAD 300d596c4c2458b61ecf4226507b23c502eadbe1

### Dump

```json
{'created_at': '2021-08-25 08:44:28',
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
 'size': '24.6 kB',
 'tags': [],
 'uuid': 'cc7fe1c8-f80c-4a76-a417-7545fe252d95',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sjpkm6xa/pmmt-py.git 300d596c4c2458b61ecf4226507b23c502eadbe1

# javascript
250 rules, avg.len. 8.3
## train
PPCR: 0.961338
### report
macro
{'f1-score': 0.4388397043040891,
 'precision': 0.43135795553328715,
 'recall': 0.4480827100115336,
 'support': 69051}
micro
{'f1-score': 0.9259387988588145,
 'precision': 0.9259387988588145,
 'recall': 0.9259387988588145,
 'support': 69051}
weighted
{'f1-score': 0.9106068494657383,
 'precision': 0.8966500735904231,
 'recall': 0.9259387988588145,
 'support': 69051}
### report_full
macro
{'f1-score': 0.4106244774454586,
 'precision': 0.43135795553328715,
 'recall': 0.4030561143193923,
 'support': 71828}
micro
{'f1-score': 0.907686738264752,
 'precision': 0.9259387988588145,
 'recall': 0.8901403352453082,
 'support': 71828}
weighted
{'f1-score': 0.884794197135847,
 'precision': 0.883722166851091,
 'recall': 0.8901403352453082,
 'support': 71828}
## test
PPCR: 0.963130
### report
macro
{'f1-score': 0.43205242274636557,
 'precision': 0.4322154654153973,
 'recall': 0.43447604990059463,
 'support': 16222}
micro
{'f1-score': 0.9217112563185798,
 'precision': 0.9217112563185798,
 'recall': 0.9217112563185798,
 'support': 16222}
weighted
{'f1-score': 0.9028421873952188,
 'precision': 0.8858237447442429,
 'recall': 0.9217112563185798,
 'support': 16222}
### report_full
macro
{'f1-score': 0.4043594893472398,
 'precision': 0.4322154654153973,
 'recall': 0.39201449117767534,
 'support': 16843}
micro
{'f1-score': 0.904400423408438,
 'precision': 0.9217112563185798,
 'recall': 0.8877278394585287,
 'support': 16843}
weighted
{'f1-score': 0.8788336703348327,
 'precision': 0.8752508202599745,
 'recall': 0.8877278394585287,
 'support': 16843}
```

## javascript
### Summary
145 rules, avg.len. 8.2

| | |
|-|-|
|Min support|146|
|Max support|25885|
|Min confidence|0.9203389883041382|
|Max confidence|0.9998046159744263|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1469.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 305.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 2262.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 264.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 1447.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.929. Support: 2177.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 1482.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1005.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 526.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 483.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 280.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -2.length ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.978. Support: 207.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 769.` |
| 14 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 494.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 480.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 5406.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 19008.` |
| 18 | `  -1.roles not in {STRING}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 1519.` |
| 19 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 295.` |
| 20 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -3.roles in {MAP}<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = '<br>Confidence: 0.989. Support: 1341.` |
| 21 | `  •••start_line ≥ 164<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -3.roles not in {MAP}<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = '<br>Confidence: 0.955. Support: 706.` |
| 22 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 294.` |
| 23 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 2360.` |
| 24 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {BINARY}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 212.` |
| 25 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 1373.` |
| 26 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 2088.` |
| 27 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 12626.` |
| 28 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5440.` |
| 29 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 3665.` |
| 30 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2551.` |
| 31 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 802.` |
| 32 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 696.` |
| 33 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 381.` |
| 34 | `  •••start_col ≥ 16<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 249.` |
| 35 | `  •••start_col ≥ 16<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.roles in {LITERAL}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {), ,, ;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 178.` |
| 36 | `  •••start_line ≤ 125<br>	∧ -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.939. Support: 2532.` |
| 37 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.984. Support: 1283.` |
| 38 | `  •••start_line ≥ 181<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.964. Support: 759.` |
| 39 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 1462.` |
| 40 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 969.` |
| 41 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 524.` |
| 42 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 464.` |
| 43 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 8<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.965. Support: 212.` |
| 44 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 276.` |
| 45 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 747.` |
| 46 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = function<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≥ 10<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 498.` |
| 47 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1244.` |
| 48 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 24157.` |
| 49 | `  -1.internal_type = StringLiteral<br>	∧ -5.length ≤ 9<br>	∧ ^1.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.985. Support: 1373.` |
| 50 | `  •••start_line ≥ 242<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.length ≤ 9<br>	∧ ^1.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.974. Support: 474.` |
| 51 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -4.label in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1267.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.932. Support: 288.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 2175.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 12496.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5374.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3660.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2556.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 844.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 684.` |
| 60 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 394.` |
| 61 | `  -1.roles not in {STRING}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.940. Support: 1454.` |
| 62 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.940. Support: 307.` |
| 63 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.978. Support: 1365.` |
| 64 | `  •••start_line ≥ 181<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.950. Support: 768.` |
| 65 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 292.` |
| 66 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 2138.` |
| 67 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1480.` |
| 68 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1001.` |
| 69 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 525.` |
| 70 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 423.` |
| 71 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 185.` |
| 72 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 249.` |
| 73 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 750.` |
| 74 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, if, return, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 25885.` |
| 75 | `  •••start_line ≤ 125<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 24<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.936. Support: 2391.` |
| 76 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 2429.` |
| 77 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 201.` |
| 78 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1450.` |
| 79 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 150.` |
| 80 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.930. Support: 2134.` |
| 81 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 12741.` |
| 82 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5193.` |
| 83 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3683.` |
| 84 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2503.` |
| 85 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 655.` |
| 86 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 219.` |
| 87 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.length ≤ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 3702.` |
| 88 | `  •••start_col ≥ 15<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 240.` |
| 89 | `  •••start_line ≤ 125<br>	∧ -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.933. Support: 2525.` |
| 90 | `  •••start_col ≤ 73<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 1307.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {'}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 299.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 315.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 2420.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {BINARY}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 221.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 1457.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 146.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.927. Support: 2188.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 12596.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 5322.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 3668.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2507.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 796.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 637.` |
| 104 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 105 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +2.length ≥ 4<br>	∧ ^1.roles in {IF} and not in {LITERAL, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 402.` |
| 106 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +2.length ≥ 4<br>	∧ ^1.roles not in {IF, LITERAL, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 242.` |
| 107 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 3728.` |
| 108 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 270.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1471.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 4614.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 2109.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 12655.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5318.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3649.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2559.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 838.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 685.` |
| 118 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.internal_type = Identifier<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 235.` |
| 119 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 5<br>	∧ +2.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 374.` |
| 120 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 243.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.label in {'}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1438.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {'}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 276.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.label not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.988. Support: 1385.` |
| 124 | `  •••start_line ≥ 181<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 16<br>	∧ -4.diff_col ≥ 4<br>	∧ -4.label not in {'}<br>	∧ +1.roles in {STRING}<br>	∧ +3.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.958. Support: 758.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 255.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5334.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3629.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2515.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 775.` |
| 130 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 690.` |
| 131 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1787.` |
| 132 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1332.` |
| 133 | `  •••start_line ≤ 126<br>	∧ -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.940. Support: 2534.` |
| 134 | `  •••start_col ≤ 73<br>	∧ -1.roles not in {STRING}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1275.` |
| 135 | `  •••start_line ≥ 181<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.roles not in {RIGHT}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.948. Support: 581.` |
| 136 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -4.reserved not in {'}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.979. Support: 1427.` |
| 137 | `  -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {BINARY}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 236.` |
| 138 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 12659.` |
| 139 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 5159.` |
| 140 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 3831.` |
| 141 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2472.` |
| 142 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 645.` |
| 143 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.internal_type = Identifier<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 216.` |
| 144 | `  •••start_col ≤ 29<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 3018.` |
| 145 | `  •••start_col ≥ 15<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 260.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.193103448275862, "max_conf": 0.9998046159744263, "max_support": 25885, "min_conf": 0.9203389883041382, "min_support": 146, "num_rules": 145}}
```
</details>
