# Model report for file:///tmp/top-repos-quality-repos-aww_vryg/mica.git HEAD 82d26b9c6b04e1b8cea7d80c054e2cf482a58520

### Dump

```json
{'created_at': '2021-08-21 07:06:09',
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
 'size': '32.0 kB',
 'tags': [],
 'uuid': 'cafecd5e-03cd-40b5-a1c5-f95e4e477b31',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aww_vryg/mica.git 82d26b9c6b04e1b8cea7d80c054e2cf482a58520

# javascript
378 rules, avg.len. 12.7
## train
PPCR: 0.950525
### report
macro
{'f1-score': 0.40213995856499707,
 'precision': 0.4633304015428995,
 'recall': 0.39170912128523877,
 'support': 137521}
micro
{'f1-score': 0.926985696729954,
 'precision': 0.926985696729954,
 'recall': 0.926985696729954,
 'support': 137521}
weighted
{'f1-score': 0.9167976803824258,
 'precision': 0.91913755646479,
 'recall': 0.926985696729954,
 'support': 137521}
### report_full
macro
{'f1-score': 0.35891297147642204,
 'precision': 0.4633304015428995,
 'recall': 0.33369284100174357,
 'support': 144679}
micro
{'f1-score': 0.9034727143869595,
 'precision': 0.926985696729954,
 'recall': 0.8811230378976908,
 'support': 144679}
weighted
{'f1-score': 0.8828421861861603,
 'precision': 0.905997090643048,
 'recall': 0.8811230378976908,
 'support': 144679}
## test
PPCR: 0.952873
### report
macro
{'f1-score': 0.3574932985123194,
 'precision': 0.42641296938025,
 'recall': 0.37396166203004694,
 'support': 31785}
micro
{'f1-score': 0.8934717634104137,
 'precision': 0.8934717634104137,
 'recall': 0.8934717634104137,
 'support': 31785}
weighted
{'f1-score': 0.8794443636249499,
 'precision': 0.9072308272582964,
 'recall': 0.8934717634104137,
 'support': 31785}
### report_full
macro
{'f1-score': 0.3091367215340383,
 'precision': 0.42641296938025,
 'recall': 0.3108995712150261,
 'support': 33357}
micro
{'f1-score': 0.8719105953148506,
 'precision': 0.8934717634104137,
 'recall': 0.8513655304733639,
 'support': 33357}
weighted
{'f1-score': 0.8490683109757091,
 'precision': 0.8969715884172396,
 'recall': 0.8513655304733639,
 'support': 33357}
```

## javascript
### Summary
196 rules, avg.len. 12.6

| | |
|-|-|
|Min support|134|
|Max support|22345|
|Min confidence|0.9205882549285889|
|Max confidence|0.9988864064216614|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 17252.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.974. Support: 287.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 3853.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 6120.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5452.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL, DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 184.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 9327.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2911.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1773.` |
| 10 | `  •••start_col ≥ 23<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 715.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 11<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 582.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 10<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 13 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 10<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 45<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ⏎⏎<br>Confidence: 0.966. Support: 219.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 10<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 595.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 10<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 141.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1514.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 589.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 438.` |
| 19 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -3.reserved = :<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 154.` |
| 20 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1974.` |
| 21 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), ,, ;, ]}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 213.` |
| 22 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -2.diff_offset ≤ 12<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL, FILE}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 224.` |
| 23 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 12<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, EXPRESSION, FILE}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 416.` |
| 24 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, :, ;, ]}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1178.` |
| 25 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {CONDITION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, :, ;}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE, LITERAL}<br>	∧ ^2.roles not in {ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 19263.` |
| 26 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 163.` |
| 27 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.921. Support: 510.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 17323.` |
| 29 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {+}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.935. Support: 5574.` |
| 30 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.970. Support: 288.` |
| 31 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 3937.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 6279.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5371.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL, FUNCTION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 189.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 596.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL} and not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 8697.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 220.` |
| 38 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2863.` |
| 39 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1853.` |
| 40 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 24<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 718.` |
| 41 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 10<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 589.` |
| 42 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 9<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 296.` |
| 43 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 9<br>	∧ -2.roles not in {BLOCK}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 554.` |
| 44 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 9<br>	∧ -2.roles not in {BLOCK}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 155.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1471.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 563.` |
| 47 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 405.` |
| 48 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1940.` |
| 49 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 225.` |
| 50 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {NUMBER} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 160.` |
| 51 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {NUMBER} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 203.` |
| 52 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {NUMBER, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 293.` |
| 53 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {NUMBER, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 2980.` |
| 54 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {NUMBER, STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LEFT} and not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 499.` |
| 55 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {NUMBER, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER, LEFT}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 17675.` |
| 56 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 154.` |
| 57 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.946. Support: 527.` |
| 58 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 17275.` |
| 59 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {+}<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.973. Support: 3328.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.975. Support: 303.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 3904.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 6220.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5376.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ +4.roles in {KEY}<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.987. Support: 514.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles in {CALL} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 177.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 605.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 8614.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1862.` |
| 69 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_offset ≥ 23<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 689.` |
| 70 | `  -1.diff_offset ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 558.` |
| 71 | `  -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 341.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1512.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 574.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 431.` |
| 75 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -2.length ≤ 8<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 299.` |
| 76 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 1971.` |
| 77 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles in {FUNCTION} and not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 78 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 340.` |
| 79 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 4847.` |
| 80 | `  •••start_col ≤ 11<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT} and not in {ARGUMENT, ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 288.` |
| 81 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 18333.` |
| 82 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 152.` |
| 83 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 826.` |
| 84 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 346.` |
| 85 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.928. Support: 552.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 17149.` |
| 87 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.951. Support: 318.` |
| 88 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 3938.` |
| 89 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 6269.` |
| 90 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5366.` |
| 91 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles in {CALL} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 177.` |
| 92 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 5513.` |
| 93 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 3768.` |
| 94 | `  -1.diff_offset ≥ 10<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 592.` |
| 95 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 342.` |
| 96 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 287.` |
| 97 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 599.` |
| 98 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 173.` |
| 99 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 411.` |
| 100 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1962.` |
| 101 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 213.` |
| 102 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 370.` |
| 103 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 4934.` |
| 104 | `  •••start_col ≤ 11<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 299.` |
| 105 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ], return}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LEFT} and not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 488.` |
| 106 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ], return}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, LEFT, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 17578.` |
| 107 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, return}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 198.` |
| 108 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 17307.` |
| 109 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {+}<br>	∧ ^1.roles in {MAP} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.975. Support: 3282.` |
| 110 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {CALL, DECLARATION} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 170.` |
| 111 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 593.` |
| 112 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 8624.` |
| 113 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1782.` |
| 114 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 23<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 764.` |
| 115 | `  -1.diff_offset ≥ 10<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 654.` |
| 116 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 367.` |
| 117 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 627.` |
| 118 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1365.` |
| 119 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 486.` |
| 120 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 422.` |
| 121 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 2031.` |
| 122 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 199.` |
| 123 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 308.` |
| 124 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION, NUMBER} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles in {EXPRESSION} and not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 169.` |
| 125 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION, NUMBER} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles not in {CALL, EXPRESSION, FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 126 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 18455.` |
| 127 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 139.` |
| 128 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {ARITHMETIC} and not in {CALL, FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 349.` |
| 129 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {STRING} and not in {KEY}<br>	∧ +5.reserved = :<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.994. Support: 565.` |
| 130 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL, FUNCTION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 131 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {BLOCK, CALL, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 206.` |
| 132 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 11<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 600.` |
| 133 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 10<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 134 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 10<br>	∧ -2.roles not in {COMMENT}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 633.` |
| 135 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 10<br>	∧ -2.roles not in {COMMENT}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 156.` |
| 136 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 645.` |
| 137 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 2021.` |
| 138 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 203.` |
| 139 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 312.` |
| 140 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SWITCH} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 141 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED, SWITCH}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 3290.` |
| 142 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 18283.` |
| 143 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 181.` |
| 144 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {INITIALIZATION} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.975. Support: 3372.` |
| 145 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 199.` |
| 146 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 22<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 743.` |
| 147 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1964.` |
| 148 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = function<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ,, :, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 210.` |
| 149 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER, STATEMENT}<br>	∧ ^2.roles in {INITIALIZATION, VALUE} and not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1822.` |
| 150 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER, STATEMENT}<br>	∧ ^2.roles not in {ARGUMENT, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 2798.` |
| 151 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER, STATEMENT}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 152 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 18228.` |
| 153 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {+}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.932. Support: 5501.` |
| 154 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ +5.reserved = :<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.993. Support: 536.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {CALL, DECLARATION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 160.` |
| 156 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, function}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 9309.` |
| 157 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 10<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 624.` |
| 158 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.length ≤ 9<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 159 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≤ 9<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 639.` |
| 160 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≤ 9<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 151.` |
| 161 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 621.` |
| 162 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 22345.` |
| 163 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ,, :, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 169.` |
| 164 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {(}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {ARITHMETIC} and not in {CALL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 339.` |
| 165 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.951. Support: 566.` |
| 166 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 17055.` |
| 167 | `  -1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INITIALIZATION}<br>⇒ y = '<br>Confidence: 0.971. Support: 3446.` |
| 168 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.960. Support: 285.` |
| 169 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 3805.` |
| 170 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 6275.` |
| 171 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5360.` |
| 172 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {STRING} and not in {KEY}<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>⇒ y = '<br>Confidence: 0.964. Support: 595.` |
| 173 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL, DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 151.` |
| 174 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 5350.` |
| 175 | `  -1.diff_offset ≤ 1<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 3844.` |
| 176 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 594.` |
| 177 | `  -1.diff_offset ≤ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {COMMENT}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 167.` |
| 178 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 386.` |
| 179 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = :<br>	∧ -4.diff_offset ≤ 11<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 134.` |
| 180 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 2031.` |
| 181 | `  -1.diff_col ≥ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ,, :, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 208.` |
| 182 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {SWITCH} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 183 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL, SWITCH}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 5094.` |
| 184 | `  •••start_col ≤ 11<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL, SWITCH}<br>	∧ ^2.roles not in {ARGUMENT, ARITHMETIC, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 278.` |
| 185 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 18435.` |
| 186 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 171.` |
| 187 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 162.` |
| 188 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, function}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 9306.` |
| 189 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1790.` |
| 190 | `  -1.diff_offset ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 571.` |
| 191 | `  -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 335.` |
| 192 | `  -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -2.length ≥ 2<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 580.` |
| 193 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = .<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 449.` |
| 194 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1463.` |
| 195 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {CONDITION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ,, :, ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CALL, FILE, LITERAL, STATEMENT}<br>	∧ ^2.roles not in {ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 16471.` |
| 196 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ,, :, ]}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {CALL, FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 12.596938775510203, "max_conf": 0.9988864064216614, "max_support": 22345, "min_conf": 0.9205882549285889, "min_support": 134, "num_rules": 196}}
```
</details>
