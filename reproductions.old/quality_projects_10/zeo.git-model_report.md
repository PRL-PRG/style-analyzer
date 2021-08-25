# Model report for file:///tmp/top-repos-quality-repos-v2r_65fk/zeo.git HEAD 164f4c99451fc62222120f1c8c1887bec2976bad

### Dump

```json
{'created_at': '2021-08-25 07:29:07',
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
 'size': '29.7 kB',
 'tags': [],
 'uuid': 'b958d56f-36c3-4fcd-a6c8-1fc38f189f8e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v2r_65fk/zeo.git 164f4c99451fc62222120f1c8c1887bec2976bad

# javascript
164 rules, avg.len. 11.4
## train
PPCR: 0.942936
### report
macro
{'f1-score': 0.4867967821107209,
 'precision': 0.5645168805187475,
 'recall': 0.4628989495475671,
 'support': 504976}
micro
{'f1-score': 0.9644735591394442,
 'precision': 0.9644735591394442,
 'recall': 0.9644735591394442,
 'support': 504976}
weighted
{'f1-score': 0.9616096364309434,
 'precision': 0.9618725017095244,
 'recall': 0.9644735591394442,
 'support': 504976}
### report_full
macro
{'f1-score': 0.4468596049348206,
 'precision': 0.5645168805187475,
 'recall': 0.41055796590844235,
 'support': 535536}
micro
{'f1-score': 0.9361468200270636,
 'precision': 0.9644735591394442,
 'recall': 0.9094365271428998,
 'support': 535536}
weighted
{'f1-score': 0.925070596578805,
 'precision': 0.9574103146459771,
 'recall': 0.9094365271428998,
 'support': 535536}
## test
PPCR: 0.941069
### report
macro
{'f1-score': 0.493422720183308,
 'precision': 0.5675639421005325,
 'recall': 0.4740477202504396,
 'support': 121397}
micro
{'f1-score': 0.9494550936184585,
 'precision': 0.9494550936184585,
 'recall': 0.9494550936184585,
 'support': 121397}
weighted
{'f1-score': 0.9458020263015368,
 'precision': 0.9484436200245795,
 'recall': 0.9494550936184585,
 'support': 121397}
### report_full
macro
{'f1-score': 0.4540946946825499,
 'precision': 0.5675639421005325,
 'recall': 0.42063917084112534,
 'support': 128999}
micro
{'f1-score': 0.9206297225195291,
 'precision': 0.9494550936184585,
 'recall': 0.8935030504112434,
 'support': 128999}
weighted
{'f1-score': 0.9091376811298727,
 'precision': 0.9439632643696066,
 'recall': 0.8935030504112434,
 'support': 128999}
```

## javascript
### Summary
91 rules, avg.len. 11.9

| | |
|-|-|
|Min support|91|
|Max support|51362|
|Min confidence|0.920195460319519|
|Max confidence|0.99967360496521|

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
                     'min_samples_split': 237,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -3.reserved not in {.}<br>	∧ -4.diff_col ≥ 10<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2764.` |
| 2 | `  •••start_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -3.reserved not in {.}<br>	∧ -4.diff_col ≥ 10<br>	∧ +1.roles in {BINARY}<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 234.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), [}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 265.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), [}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1303.` |
| 5 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1906.` |
| 6 | `  •••start_col ≤ 50<br>	∧ -1.reserved = ;<br>	∧ -2.diff_offset ≤ 8<br>	∧ -2.roles in {INITIALIZATION}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {NAME}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 188.` |
| 7 | `  •••start_col ≤ 50<br>	∧ -1.reserved = ;<br>	∧ -2.diff_offset ≤ 8<br>	∧ -2.roles not in {INITIALIZATION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +4.length = 0<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 203.` |
| 8 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.921. Support: 2335.` |
| 9 | `  •••start_col ≥ 13<br>	∧ -1.reserved = (<br>	∧ -4.diff_offset ≥ 20<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {ASSIGNMENT}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 10 | `  •••start_col ≥ 13<br>	∧ -1.reserved = (<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {ASSIGNMENT}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 13425.` |
| 11 | `  -1.reserved = {<br>	∧ -3.reserved = const<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1590.` |
| 12 | `  -1.reserved = {<br>	∧ -3.reserved not in {const}<br>	∧ -4.roles in {KEY}<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +5.reserved not in {,}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 337.` |
| 13 | `  -1.reserved = {<br>	∧ -3.reserved not in {const}<br>	∧ -4.roles in {KEY}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 293.` |
| 14 | `  -1.reserved = {<br>	∧ -3.reserved not in {const}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved = }<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 15 | `  -1.reserved = {<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.944. Support: 331.` |
| 16 | `  •••start_col ≥ 50<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved = )<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION, MAP}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 92.` |
| 17 | `  •••start_col ≤ 49<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.roles in {EXPRESSION, MAP}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 2976.` |
| 18 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.reserved = ,<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.973. Support: 358.` |
| 19 | `  •••start_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved not in {,}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ +1.length ≤ 6<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALLEE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 125.` |
| 20 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.926. Support: 2879.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -2.label not in {<-space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.935. Support: 116.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER, LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.962. Support: 563.` |
| 23 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -3.reserved = ,<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 210.` |
| 24 | `  -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -3.reserved = ,<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 469.` |
| 25 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1280.` |
| 26 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 590.` |
| 27 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles in {BINARY}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 386.` |
| 28 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles in {BINARY}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 623.` |
| 29 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.label in {<newline>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 2464.` |
| 30 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved = .<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 267.` |
| 31 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {,, .}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 44474.` |
| 32 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved = =<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 100.` |
| 33 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {,, ., =}<br>	∧ -3.roles in {RIGHT}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +5.roles not in {RIGHT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 644.` |
| 34 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {BINARY}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {,, ., =}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 13238.` |
| 35 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 24348.` |
| 36 | `  -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 668.` |
| 37 | `  -1.reserved not in {(, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 12709.` |
| 38 | `  -1.internal_type = NumericLiteral<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved not in {>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {DIVIDE, OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 129.` |
| 39 | `  -1.internal_type = NumericLiteral<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved not in {>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +2.length ≥ 2<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 208.` |
| 40 | `  -1.internal_type = NumericLiteral<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved not in {>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 1028.` |
| 41 | `  -1.internal_type not in {NumericLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 12030.` |
| 42 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1326.` |
| 43 | `  -1.reserved not in {;}<br>	∧ -2.label in {<space>}<br>	∧ -4.reserved = .<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 298.` |
| 44 | `  -1.reserved not in {;}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +3.reserved = =<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 94.` |
| 45 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {CALL}<br>	∧ -2.label in {<space>}<br>	∧ -4.internal_type = Identifier<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 189.` |
| 46 | `  -1.reserved not in {;}<br>	∧ -2.label not in {<space>}<br>	∧ -4.internal_type = Identifier<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 309.` |
| 47 | `  -1.reserved = {<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.976. Support: 565.` |
| 48 | `  -1.reserved = {<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.937. Support: 4684.` |
| 49 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2462.` |
| 50 | `  -1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2291.` |
| 51 | `  -1.reserved not in {;, =, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1424.` |
| 52 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.950. Support: 191.` |
| 53 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = :<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 274.` |
| 54 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, =, {, }}<br>	∧ -2.length ≤ 15<br>	∧ -4.diff_offset ≤ 15<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = const<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 432.` |
| 55 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, =, {, }}<br>	∧ -2.length ≤ 15<br>	∧ -4.diff_offset ≤ 15<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {const}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 56 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.946. Support: 4808.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 2849.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 2195.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎⇥⁻<br>Confidence: 0.920. Support: 307.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 61 | `  •••start_col ≤ 58<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.984. Support: 3182.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -5.reserved = const<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1532.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.971. Support: 325.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {CONDITION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 173.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -4.roles in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 113.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ +1.reserved = +<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 168.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 150.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.diff_col ≥ 27<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 3<br>	∧ +5.roles in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 422.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≥ 27<br>	∧ -2.label in {<newline>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {+, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 146.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≥ 27<br>	∧ -2.label not in {<newline>}<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {+, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 249.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≥ 27<br>	∧ -2.label not in {<newline>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {+, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2074.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -4.label in {<space>}<br>	∧ -5.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 262.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {(, ;, const}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 5<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.roles in {BODY, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 714.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.roles not in {LITERAL}<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {(, ;, const}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 3460.` |
| 76 | `  •••start_col ≥ 34<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label in {<space>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = ArrowFunctionExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 490.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label not in {<space>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2728.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 100.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 360.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 13739.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 315.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles not in {BINARY}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 486.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<space>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 2229.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {(, ), +, =, ], {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 330.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1637.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label in {<newline>}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1083.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {}}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1368.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, if, return}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {}}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = FunctionDeclaration<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 671.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, if, return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {}}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 51362.` |
| 90 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 116.` |
| 91 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 26<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {}}<br>	∧ -4.roles not in {INITIALIZATION}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), +, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 23814.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.945054945054945, "max_conf": 0.99967360496521, "max_support": 51362, "min_conf": 0.920195460319519, "min_support": 91, "num_rules": 91}}
```
</details>
