# Model report for file:///tmp/top-repos-quality-repos-cin4i3yl/sdkdemoapp_windows.git HEAD 99425e7d62793e25713955be24ed1802ee00773e

### Dump

```json
{'created_at': '2021-08-25 08:25:47',
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
 'size': '24.9 kB',
 'tags': [],
 'uuid': 'd28aae79-8338-4aef-8966-58f2334b5f11',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-cin4i3yl/sdkdemoapp_windows.git 99425e7d62793e25713955be24ed1802ee00773e

# javascript
64 rules, avg.len. 9.6
## train
PPCR: 0.890360
### report
macro
{'f1-score': 0.8022189320105775,
 'precision': 0.8976576686331028,
 'recall': 0.7546843352028306,
 'support': 105814}
micro
{'f1-score': 0.9552611185665413,
 'precision': 0.9552611185665413,
 'recall': 0.9552611185665413,
 'support': 105814}
weighted
{'f1-score': 0.9532980241040947,
 'precision': 0.955270060953424,
 'recall': 0.9552611185665413,
 'support': 105814}
### report_full
macro
{'f1-score': 0.6416725083807093,
 'precision': 0.8976576686331028,
 'recall': 0.5620885182492962,
 'support': 118844}
micro
{'f1-score': 0.8998566710288527,
 'precision': 0.9552611185665413,
 'recall': 0.8505267409376999,
 'support': 118844}
weighted
{'f1-score': 0.8852267123316014,
 'precision': 0.951424720695176,
 'recall': 0.8505267409376999,
 'support': 118844}
## test
PPCR: 0.872548
### report
macro
{'f1-score': 0.52997855801068,
 'precision': 0.608308871922231,
 'recall': 0.5508522292911485,
 'support': 31629}
micro
{'f1-score': 0.9166903790824876,
 'precision': 0.9166903790824876,
 'recall': 0.9166903790824876,
 'support': 31629}
weighted
{'f1-score': 0.9126057928011609,
 'precision': 0.9171453074655709,
 'recall': 0.9166903790824876,
 'support': 31629}
### report_full
macro
{'f1-score': 0.4122685272024101,
 'precision': 0.608308871922231,
 'recall': 0.3564704678968142,
 'support': 36249}
micro
{'f1-score': 0.8542974159521495,
 'precision': 0.9166903790824876,
 'recall': 0.7998565477668349,
 'support': 36249}
weighted
{'f1-score': 0.8356465434432409,
 'precision': 0.9009106103201326,
 'recall': 0.7998565477668349,
 'support': 36249}
```

## javascript
### Summary
33 rules, avg.len. 10.2

| | |
|-|-|
|Min support|91|
|Max support|22904|
|Min confidence|0.9230310320854187|
|Max confidence|0.9987834692001343|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 19176.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 781.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 6660.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 115.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {(, )}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 838.` |
| 7 | `  •••start_line ≤ 254<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +4.length ≥ 41<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.949. Support: 285.` |
| 8 | `  •••start_col ≥ 4<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.995. Support: 100.` |
| 9 | `  •••start_col ≥ 4<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.931. Support: 94.` |
| 10 | `  •••start_col ≥ 25<br>	∧ •••start_line ≤ 241<br>	∧ -1.reserved = (<br>	∧ -4.reserved = =<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.941. Support: 94.` |
| 11 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3426.` |
| 12 | `  -1.diff_col ≥ 9<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 735.` |
| 13 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.reserved = )<br>	∧ -3.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.943. Support: 220.` |
| 14 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 147.` |
| 15 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 153.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.label in {<newline>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 182.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE, VALUE}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.length ≤ 5<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 176.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE, VALUE}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {., }}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 737.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE, VALUE}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.internal_type not in {File, ObjectExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 5534.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1988.` |
| 21 | `  •••start_col ≥ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 109.` |
| 22 | `  •••start_col ≥ 29<br>	∧ •••start_line ≥ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 1264.` |
| 23 | `  •••start_col ≥ 29<br>	∧ •••start_line ≤ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 454.` |
| 24 | `  •••start_col ≤ 28<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 236.` |
| 25 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 596.` |
| 26 | `  •••start_line ≥ 253<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 546.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 204.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 310.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 411.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 100.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, return, {}<br>	∧ -2.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 91.` |
| 32 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, function, if, return, {}<br>	∧ -2.reserved not in {=}<br>	∧ -3.reserved = ;<br>	∧ +1.reserved not in {=, if, return, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {IMPORT, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1920.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, function, if, return, {}<br>	∧ -2.reserved not in {=}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {=, if, return, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IMPORT, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 22904.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.242424242424242, "max_conf": 0.9987834692001343, "max_support": 22904, "min_conf": 0.9230310320854187, "min_support": 91, "num_rules": 33}}
```
</details>
