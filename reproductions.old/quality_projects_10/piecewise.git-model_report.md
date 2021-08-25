# Model report for file:///tmp/top-repos-quality-repos-o2d7vayp/piecewise.git HEAD 0b21a7cccf6a2c2b2d6f9e95a281a39fede932ac

### Dump

```json
{'created_at': '2021-08-24 22:26:42',
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
 'size': '19.4 kB',
 'tags': [],
 'uuid': '5675f0dc-76d8-4cb2-9a61-d4e2867c26cf',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o2d7vayp/piecewise.git 0b21a7cccf6a2c2b2d6f9e95a281a39fede932ac

# javascript
122 rules, avg.len. 9.2
## train
PPCR: 0.968142
### report
macro
{'f1-score': 0.748346476210487,
 'precision': 0.7907803417910286,
 'recall': 0.7207709370002713,
 'support': 39567}
micro
{'f1-score': 0.9346930522910506,
 'precision': 0.9346930522910506,
 'recall': 0.9346930522910506,
 'support': 39567}
weighted
{'f1-score': 0.9314115820309705,
 'precision': 0.9311749565539787,
 'recall': 0.9346930522910506,
 'support': 39567}
### report_full
macro
{'f1-score': 0.7281387023454264,
 'precision': 0.7907803417910286,
 'recall': 0.6875290881623253,
 'support': 40869}
micro
{'f1-score': 0.919563379581282,
 'precision': 0.9346930522910506,
 'recall': 0.9049157062810443,
 'support': 40869}
weighted
{'f1-score': 0.9155247259802914,
 'precision': 0.930546807092275,
 'recall': 0.9049157062810443,
 'support': 40869}
## test
PPCR: 0.959844
### report
macro
{'f1-score': 0.6958065579430353,
 'precision': 0.7381160134931919,
 'recall': 0.6654040169207631,
 'support': 5163}
micro
{'f1-score': 0.9037381367422042,
 'precision': 0.9037381367422042,
 'recall': 0.9037381367422042,
 'support': 5163}
weighted
{'f1-score': 0.8991079137625456,
 'precision': 0.898359034490852,
 'recall': 0.9037381367422042,
 'support': 5163}
### report_full
macro
{'f1-score': 0.6625989092737761,
 'precision': 0.7381160134931919,
 'recall': 0.6158118371042703,
 'support': 5379}
micro
{'f1-score': 0.8852210206791881,
 'precision': 0.9037381367422042,
 'recall': 0.8674474809444135,
 'support': 5379}
weighted
{'f1-score': 0.8775522641676036,
 'precision': 0.8953338401146734,
 'recall': 0.8674474809444135,
 'support': 5379}
```

## javascript
### Summary
67 rules, avg.len. 9.0

| | |
|-|-|
|Min support|149|
|Max support|10581|
|Min confidence|0.9204308390617371|
|Max confidence|0.9998151063919067|

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
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 2704.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = ]<br>⇒ y = ␣<br>Confidence: 0.999. Support: 348.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.999. Support: 454.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {ObjectProperty}<br>⇒ y = '<br>Confidence: 0.998. Support: 226.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, [}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.991. Support: 163.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.967. Support: 2489.` |
| 7 | `  -1.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 2004.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved = :<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 176.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 745.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line = 0<br>	∧ -5.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 192.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 747.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 246.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 838.` |
| 14 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 920.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 10167.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 247.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.925. Support: 194.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {:}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.963. Support: 2519.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 153.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line = 0<br>	∧ -5.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.962. Support: 326.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 531.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 270.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 354.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 4<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 149.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 203.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 10013.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 250.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +2.reserved not in {,}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 608.` |
| 30 | `  •••start_col ≤ 26<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {,}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 245.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1063.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 188.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 187.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 10284.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 4<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 654.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {STATEMENT, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 643.` |
| 37 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 328.` |
| 38 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 256.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 1060.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 168.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 744.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 217.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 184.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 624.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 161.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 1439.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 227.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 185.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.989. Support: 236.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 225.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 469.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 298.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 10507.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = :<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 613.` |
| 55 | `  •••start_col ≤ 26<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {,, :}<br>	∧ +3.roles not in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 206.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 594.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 299.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 877.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 10581.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {LIST}<br>⇒ y = '<br>Confidence: 0.998. Support: 238.` |
| 61 | `  •••start_col ≥ 33<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 211.` |
| 62 | `  •••start_col ≤ 32<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {], }}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 758.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +2.reserved not in {,}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 606.` |
| 64 | `  •••start_col ≤ 24<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {,}<br>	∧ +4.reserved not in {:}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 196.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line = 0<br>	∧ -5.reserved = [<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 187.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles in {CALL}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 221.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {=}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1349.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.970149253731343, "max_conf": 0.9998151063919067, "max_support": 10581, "min_conf": 0.9204308390617371, "min_support": 149, "num_rules": 67}}
```
</details>
