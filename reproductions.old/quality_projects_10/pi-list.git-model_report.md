# Model report for file:///tmp/top-repos-quality-repos-waamnoay/pi-list.git HEAD 0d9d9e3d5b482f03ef34a8ec02f4e20cb2deb998

### Dump

```json
{'created_at': '2021-08-24 20:57:13',
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
 'size': '20.9 kB',
 'tags': [],
 'uuid': '165ca90d-03d9-437d-9385-962646c0297f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-waamnoay/pi-list.git 0d9d9e3d5b482f03ef34a8ec02f4e20cb2deb998

# javascript
62 rules, avg.len. 9.1
## train
PPCR: 0.924920
### report
macro
{'f1-score': 0.7305627380166188,
 'precision': 0.7458960834723991,
 'recall': 0.7172020567175992,
 'support': 102803}
micro
{'f1-score': 0.9627150958629612,
 'precision': 0.9627150958629612,
 'recall': 0.9627150958629612,
 'support': 102803}
weighted
{'f1-score': 0.9611847417432043,
 'precision': 0.960318270017932,
 'recall': 0.9627150958629612,
 'support': 102803}
### report_full
macro
{'f1-score': 0.6609855918004351,
 'precision': 0.7458960834723991,
 'recall': 0.6064844119926375,
 'support': 111148}
micro
{'f1-score': 0.9251651078985375,
 'precision': 0.9627150958629612,
 'recall': 0.8904343757872386,
 'support': 111148}
weighted
{'f1-score': 0.9184023433057366,
 'precision': 0.9571661019837421,
 'recall': 0.8904343757872386,
 'support': 111148}
## test
PPCR: 0.922326
### report
macro
{'f1-score': 0.7252383129947553,
 'precision': 0.7430676565582527,
 'recall': 0.711268438732835,
 'support': 25898}
micro
{'f1-score': 0.9592246505521662,
 'precision': 0.9592246505521662,
 'recall': 0.9592246505521662,
 'support': 25898}
weighted
{'f1-score': 0.9576074568479078,
 'precision': 0.957066078606893,
 'recall': 0.9592246505521662,
 'support': 25898}
### report_full
macro
{'f1-score': 0.6508645572959688,
 'precision': 0.7430676565582527,
 'recall': 0.5913638740248828,
 'support': 28079}
micro
{'f1-score': 0.9204661244604183,
 'precision': 0.9592246505521662,
 'recall': 0.8847181167420493,
 'support': 28079}
weighted
{'f1-score': 0.9134477653264622,
 'precision': 0.9537322974043523,
 'recall': 0.8847181167420493,
 'support': 28079}
```

## javascript
### Summary
46 rules, avg.len. 8.9

| | |
|-|-|
|Min support|91|
|Max support|24887|
|Min confidence|0.9244604110717773|
|Max confidence|0.9993997812271118|

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
| 1 | `  •••start_col ≥ 17<br>	∧ -1.reserved = {<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.938. Support: 888.` |
| 2 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.968. Support: 675.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 833.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 623.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 586.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 399.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 139.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 213.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 407.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved = =<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ,, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 273.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ,, >}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 7993.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.969. Support: 571.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.978. Support: 2974.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5819.` |
| 15 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 542.` |
| 16 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.roles in {PATHNAME}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.955. Support: 146.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.968. Support: 571.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 642.` |
| 19 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.length ≤ 4<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 184.` |
| 20 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.944. Support: 2114.` |
| 21 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 440.` |
| 22 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 23 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 4077.` |
| 24 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 1459.` |
| 25 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 532.` |
| 26 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 612.` |
| 27 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 167.` |
| 28 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 6416.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 691.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 222.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 102.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 662.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 908.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 233.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 91.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 973.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 196.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 219.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 149.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 147.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, return}<br>	∧ -3.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type = JSXIdentifier<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 92.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return}<br>	∧ -3.reserved = return<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {JSXIdentifier}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 256.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {(}<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {CALLEE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, >, if, return}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {CALLEE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 2059.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, if, return}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {IDENTIFIER, LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {CALLEE, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 24887.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length = 0<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 211.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.91304347826087, "max_conf": 0.9993997812271118, "max_support": 24887, "min_conf": 0.9244604110717773, "min_support": 91, "num_rules": 46}}
```
</details>
