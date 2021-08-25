# Model report for file:///tmp/top-repos-quality-repos-pl9gcvp7/itc-7.git HEAD d2bfcdc3a1faea4cdc1d7e555404445ac8768c66

### Dump

```json
{'created_at': '2021-08-24 11:56:05',
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
 'uuid': '28faa42f-bbc3-4b64-ad0d-dbb4d341f9ec',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-pl9gcvp7/itc-7.git d2bfcdc3a1faea4cdc1d7e555404445ac8768c66

# javascript
92 rules, avg.len. 10.9
## train
PPCR: 0.884527
### report
macro
{'f1-score': 0.4704587527279334,
 'precision': 0.5166138963037532,
 'recall': 0.4427721877351818,
 'support': 213071}
micro
{'f1-score': 0.9425731328993622,
 'precision': 0.9425731328993622,
 'recall': 0.9425731328993622,
 'support': 213071}
weighted
{'f1-score': 0.9397935020898249,
 'precision': 0.9392540582059429,
 'recall': 0.9425731328993622,
 'support': 213071}
### report_full
macro
{'f1-score': 0.3608547233306674,
 'precision': 0.5166138963037532,
 'recall': 0.30281138143926,
 'support': 240887}
micro
{'f1-score': 0.8848175381863521,
 'precision': 0.9425731328993622,
 'recall': 0.833731168556211,
 'support': 240887}
weighted
{'f1-score': 0.8670932812666912,
 'precision': 0.923991592065976,
 'recall': 0.833731168556211,
 'support': 240887}
## test
PPCR: 0.874354
### report
macro
{'f1-score': 0.4926466155883156,
 'precision': 0.5159324169594954,
 'recall': 0.4780610674711448,
 'support': 38726}
micro
{'f1-score': 0.944946547539121,
 'precision': 0.944946547539121,
 'recall': 0.944946547539121,
 'support': 38726}
weighted
{'f1-score': 0.9421766109424988,
 'precision': 0.9410246076973018,
 'recall': 0.944946547539121,
 'support': 38726}
### report_full
macro
{'f1-score': 0.37285828155401435,
 'precision': 0.5159324169594954,
 'recall': 0.3086754153473345,
 'support': 44291}
micro
{'f1-score': 0.8816025633304022,
 'precision': 0.944946547539121,
 'recall': 0.8262175159738999,
 'support': 44291}
weighted
{'f1-score': 0.8608915524400348,
 'precision': 0.9164603645649673,
 'recall': 0.8262175159738999,
 'support': 44291}
```

## javascript
### Summary
46 rules, avg.len. 9.9

| | |
|-|-|
|Min support|95|
|Max support|43846|
|Min confidence|0.9205170273780823|
|Max confidence|0.9991328716278076|

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
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.965. Support: 328.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 43846.` |
| 3 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1608.` |
| 4 | `  •••start_col ≥ 30<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {ASSIGNMENT, OPERATOR} and not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.951. Support: 398.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 649.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = )<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 350.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 114.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 427.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved = =<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 95.` |
| 10 | `  -1.diff_offset ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {DIVIDE, OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 750.` |
| 11 | `  -1.diff_offset ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = (<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {DIVIDE, OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 127.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.reserved = =<br>	∧ +1.reserved = (<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 129.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {OPERATOR} and not in {DIVIDE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 10597.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 140.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {), =}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≥ 4<br>	∧ ^1.roles in {OPERATOR} and not in {DIVIDE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 4658.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {), =}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles in {OPERATOR} and not in {DIVIDE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 2065.` |
| 17 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 7350.` |
| 18 | `  -1.reserved not in {(}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2883.` |
| 19 | `  •••start_col ≥ 7<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 8<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 96.` |
| 20 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 2713.` |
| 21 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 1818.` |
| 22 | `  -1.diff_offset ≥ 14<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 6<br>	∧ +5.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 101.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 117.` |
| 24 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {,}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 98.` |
| 25 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≥ 4<br>	∧ -2.reserved = (<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 126.` |
| 26 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 11772.` |
| 27 | `  •••start_col ≤ 55<br>	∧ •••start_line ≥ 86<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = new<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 145.` |
| 28 | `  •••start_col ≤ 55<br>	∧ •••start_line ≥ 86<br>	∧ -1.internal_type = StringLiteral<br>	∧ -4.reserved = .<br>	∧ -5.diff_col ≤ 25<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {new}<br>	∧ +4.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.965. Support: 127.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.935. Support: 100.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.956. Support: 3744.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1314.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.945. Support: 118.` |
| 33 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, if, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 155.` |
| 34 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 582.` |
| 35 | `  •••start_line ≥ 150<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.length ≤ 5<br>	∧ -5.reserved = function<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 194.` |
| 36 | `  •••start_line ≤ 149<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.length ≤ 5<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 537.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 418.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.length ≤ 6<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {ASSIGNMENT}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 223.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved = ?<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 418.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 208.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 159.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, return}<br>	∧ -3.diff_line ≥ 2<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, return}<br>	∧ -3.diff_line ≤ 1<br>	∧ -4.roles in {KEY}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {RIGHT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 427.` |
| 44 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, return}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.diff_line ≤ 1<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 637.` |
| 45 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, return}<br>	∧ -3.diff_line ≤ 1<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2460.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, return}<br>	∧ -3.diff_line = 0<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {:, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 38278.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.934782608695652, "max_conf": 0.9991328716278076, "max_support": 43846, "min_conf": 0.9205170273780823, "min_support": 95, "num_rules": 46}}
```
</details>
