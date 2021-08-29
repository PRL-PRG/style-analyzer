# Model report for file:///tmp/top-repos-quality-repos-cnc1awse/kimchi.git HEAD 36ed74be5d2016848ea6e453d7fb990d3d7fe3cd

### Dump

```json
{'created_at': '2021-08-29 19:54:44',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '42.7 kB',
 'tags': [],
 'uuid': '4a0c7fcd-3b2d-454d-997a-d6d22ed0fbde',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-cnc1awse/kimchi.git 36ed74be5d2016848ea6e453d7fb990d3d7fe3cd

# javascript
450 rules, avg.len. 11.8
## train
PPCR: 0.941867
### report
macro
{'f1-score': 0.34626138697516623,
 'precision': 0.3970227412996989,
 'recall': 0.3187295729947637,
 'support': 236062}
micro
{'f1-score': 0.9268539620947039,
 'precision': 0.9268539620947039,
 'recall': 0.9268539620947039,
 'support': 236062}
weighted
{'f1-score': 0.9171702043729653,
 'precision': 0.9144922411269545,
 'recall': 0.9268539620947039,
 'support': 236062}
### report_full
macro
{'f1-score': 0.2890984662543979,
 'precision': 0.3970227412996989,
 'recall': 0.2484953652955456,
 'support': 250632}
micro
{'f1-score': 0.899107036454117,
 'precision': 0.9268539620947039,
 'recall': 0.8729731239426729,
 'support': 250632}
weighted
{'f1-score': 0.8760774145500286,
 'precision': 0.899275233530729,
 'recall': 0.8729731239426729,
 'support': 250632}
## test
PPCR: 0.945599
### report
macro
{'f1-score': 0.3145699603198947,
 'precision': 0.3710196224639637,
 'recall': 0.29298322666814763,
 'support': 64279}
micro
{'f1-score': 0.915866768306912,
 'precision': 0.915866768306912,
 'recall': 0.915866768306912,
 'support': 64279}
weighted
{'f1-score': 0.9051792410398517,
 'precision': 0.9051291819708379,
 'recall': 0.915866768306912,
 'support': 64279}
### report_full
macro
{'f1-score': 0.2605036721025416,
 'precision': 0.3710196224639637,
 'recall': 0.2308619666343982,
 'support': 67977}
micro
{'f1-score': 0.8902582869586256,
 'precision': 0.915866768306912,
 'recall': 0.8660429262838901,
 'support': 67977}
weighted
{'f1-score': 0.8651069606470964,
 'precision': 0.8966291044503406,
 'recall': 0.8660429262838901,
 'support': 67977}
```

## javascript
### Summary
229 rules, avg.len. 10.9

| | |
|-|-|
|Min support|143|
|Max support|43478|
|Min confidence|0.9208144545555115|
|Max confidence|0.9998173713684082|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 593.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 42691.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.967. Support: 289.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1330.` |
| 5 | `  -1.reserved = (<br>	∧ -2.reserved not in {[}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1121.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 893.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 408.` |
| 8 | `  -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 376.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LEFT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 468.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ]<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {), [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +5.reserved = ,<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣␣␣<br>Confidence: 0.992. Support: 197.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ]}<br>	∧ -1.roles not in {LEFT, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1124.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ]}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 317.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ]}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 17519.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = [<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.981. Support: 286.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 2738.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 32905.` |
| 18 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 10606.` |
| 19 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 7561.` |
| 20 | `  •••start_line ≥ 131<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.955. Support: 345.` |
| 21 | `  •••start_col ≤ 41<br>	∧ •••start_line ≥ 131<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line = 0<br>	∧ -5.diff_offset ≤ 6<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.932. Support: 155.` |
| 22 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2397.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 2134.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1796.` |
| 25 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 1856.` |
| 26 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 902.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.937. Support: 390.` |
| 28 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 835.` |
| 29 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 307.` |
| 30 | `  •••start_col ≥ 4<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 31 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 14087.` |
| 32 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved = ,<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1798.` |
| 33 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.973. Support: 607.` |
| 34 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 42992.` |
| 35 | `  -1.roles in {STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.964. Support: 293.` |
| 36 | `  -1.roles not in {STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 1320.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 794.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 414.` |
| 39 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 385.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {BINARY}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 491.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved = '<br>	∧ -3.reserved not in {=}<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣␣␣<br>Confidence: 0.993. Support: 202.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1181.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', [}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 317.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=, [}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 14017.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=, [}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 2768.` |
| 46 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.970. Support: 250.` |
| 47 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 2626.` |
| 48 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 32819.` |
| 49 | `  •••start_col ≥ 9<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {ARGUMENT}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type = NumericLiteral<br>	∧ ^1.roles not in {FILE, FOR, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 151.` |
| 50 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.952. Support: 510.` |
| 51 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.963. Support: 449.` |
| 52 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 8<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 217.` |
| 53 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 7<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1356.` |
| 54 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 286.` |
| 55 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 56 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 11782.` |
| 57 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,, ]}<br>	∧ +1.roles not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VALUE}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 3075.` |
| 58 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {ARGUMENT}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VALUE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 259.` |
| 59 | `  -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.979. Support: 552.` |
| 60 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 43478.` |
| 61 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1127.` |
| 62 | `  -1.reserved not in {(}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.968. Support: 269.` |
| 63 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1087.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 827.` |
| 65 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 392.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LEFT}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 526.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved = '<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣␣␣<br>Confidence: 0.977. Support: 196.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier, NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier, NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 347.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier, NumericLiteral}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 17472.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 2670.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 32673.` |
| 73 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 10544.` |
| 74 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 7493.` |
| 75 | `  •••start_line ≥ 122<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.946. Support: 360.` |
| 76 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2401.` |
| 77 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 2083.` |
| 78 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1807.` |
| 79 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.948. Support: 395.` |
| 80 | `  •••start_col ≤ 46<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.length ≥ 22<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 156.` |
| 81 | `  •••start_col ≥ 4<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 197.` |
| 82 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,, ]}<br>	∧ +2.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2643.` |
| 83 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.976. Support: 600.` |
| 84 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 43076.` |
| 85 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.983. Support: 262.` |
| 86 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1131.` |
| 87 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1022.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 829.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 358.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 536.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved = '<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣␣␣<br>Confidence: 0.984. Support: 222.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 356.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 17308.` |
| 95 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 2673.` |
| 96 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.length ≤ 27<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 32496.` |
| 97 | `  •••start_line ≥ 95<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.951. Support: 379.` |
| 98 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 874.` |
| 99 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.966. Support: 280.` |
| 100 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 306.` |
| 101 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.length ≥ 22<br>	∧ -3.reserved not in {,}<br>	∧ -5.internal_type = NumericLiteral<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 172.` |
| 102 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved = (<br>	∧ -2.length ≤ 21<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_col ≥ 8<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 165.` |
| 103 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 14276.` |
| 104 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LIST} and not in {OPERATOR, VALUE}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 2158.` |
| 105 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LIST, OPERATOR, VALUE}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 2018.` |
| 106 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {(, )}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LIST, OPERATOR, VALUE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 311.` |
| 107 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.971. Support: 332.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1126.` |
| 109 | `  -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1064.` |
| 110 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 853.` |
| 111 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 376.` |
| 112 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ∅<br>Confidence: 0.964. Support: 464.` |
| 113 | `  •••start_col ≤ 19<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved = '<br>	∧ -3.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣␣␣<br>Confidence: 0.997. Support: 190.` |
| 114 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 115 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 397.` |
| 116 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 19408.` |
| 117 | `  •••start_line ≥ 163<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.956. Support: 330.` |
| 118 | `  •••start_line ≤ 162<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.958. Support: 155.` |
| 119 | `  •••start_col ≤ 62<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 818.` |
| 120 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.973. Support: 428.` |
| 121 | `  -1.diff_col ≥ 7<br>	∧ -1.diff_offset ≥ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 8<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 213.` |
| 122 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 123 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,, ]}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2738.` |
| 124 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ -5.reserved = var<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 2531.` |
| 125 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {LIST}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1999.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.980. Support: 675.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 43102.` |
| 128 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.966. Support: 280.` |
| 129 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1147.` |
| 130 | `  -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1056.` |
| 131 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 818.` |
| 132 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 422.` |
| 133 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LEFT, STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 538.` |
| 134 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = '<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣␣␣<br>Confidence: 0.986. Support: 183.` |
| 135 | `  -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 149.` |
| 136 | `  -1.diff_offset ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 361.` |
| 137 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {NumericLiteral}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 17448.` |
| 138 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.970. Support: 279.` |
| 139 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 2666.` |
| 140 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 32675.` |
| 141 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 10590.` |
| 142 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 7689.` |
| 143 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.967. Support: 504.` |
| 144 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2288.` |
| 145 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 2116.` |
| 146 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1788.` |
| 147 | `  •••start_col ≤ 65<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 932.` |
| 148 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.946. Support: 451.` |
| 149 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 272.` |
| 150 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.length ≤ 21<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 192.` |
| 151 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 21<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 221.` |
| 152 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.reserved not in {(, )}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≥ 3<br>	∧ +2.length ≤ 21<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 11265.` |
| 153 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 903.` |
| 154 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 360.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved = )<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {BINARY}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 158.` |
| 156 | `  -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', [}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 157 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', [}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 345.` |
| 158 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=, [}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 13183.` |
| 159 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=, [}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.diff_col ≥ 12<br>	∧ -5.diff_offset ≥ 13<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 164.` |
| 160 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -3.reserved not in {=, [}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.diff_col ≤ 11<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.internal_type not in {NumericLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 2505.` |
| 161 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -4.length ≤ 25<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 32271.` |
| 162 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 10626.` |
| 163 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.949. Support: 503.` |
| 164 | `  -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.936. Support: 416.` |
| 165 | `  -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 805.` |
| 166 | `  -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 323.` |
| 167 | `  •••start_col ≥ 4<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {(, ), ,, ;, ], }}<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 12923.` |
| 168 | `  •••start_col ≥ 4<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {(, ), ,, ;, ], }}<br>	∧ +1.length ≤ 13<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles in {LIST} and not in {IDENTIFIER, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 2146.` |
| 169 | `  •••start_col ≥ 4<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {(, ), ,, ;, ], }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {IDENTIFIER, LIST, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 267.` |
| 170 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.975. Support: 575.` |
| 171 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 43124.` |
| 172 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.985. Support: 292.` |
| 173 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1222.` |
| 174 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 439.` |
| 175 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 774.` |
| 176 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 463.` |
| 177 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved = '<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(}<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣␣␣<br>Confidence: 0.967. Support: 165.` |
| 178 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {IDENTIFIER, NUMBER}<br>	∧ +1.reserved not in {(, ), ], {}<br>	∧ +2.roles in {NUMBER}<br>	∧ ^1.roles in {ARITHMETIC, OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 205.` |
| 179 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {IDENTIFIER, NUMBER}<br>	∧ +1.reserved not in {(, ), ], {}<br>	∧ +2.roles in {NUMBER}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1282.` |
| 180 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -2.reserved not in {', ), [}<br>	∧ -2.roles not in {IDENTIFIER, NUMBER}<br>	∧ +1.reserved not in {(, ), ], {}<br>	∧ +2.roles not in {NUMBER}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 16013.` |
| 181 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 10363.` |
| 182 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 848.` |
| 183 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.951. Support: 415.` |
| 184 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STATEMENT}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 280.` |
| 185 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ]}<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 147.` |
| 186 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≤ 14<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles in {LIST} and not in {MAP, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 2047.` |
| 187 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≤ 1<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≤ 14<br>	∧ +2.length ≤ 22<br>	∧ ^1.roles not in {LIST, MAP, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 891.` |
| 188 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.972. Support: 586.` |
| 189 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 43133.` |
| 190 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.983. Support: 270.` |
| 191 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1168.` |
| 192 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 373.` |
| 193 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +4.roles in {NUMBER}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣␣␣<br>Confidence: 0.982. Support: 193.` |
| 194 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -5.diff_offset ≤ 13<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 343.` |
| 195 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 196 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 430.` |
| 197 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {NUMBER}<br>	∧ -3.reserved not in {=}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 19528.` |
| 198 | `  •••start_line ≥ 118<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.946. Support: 362.` |
| 199 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.length ≤ 7<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1969.` |
| 200 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 173.` |
| 201 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,, ]}<br>	∧ +2.length ≤ 19<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2615.` |
| 202 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ -5.reserved = var<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 19<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2589.` |
| 203 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 19<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {LIST}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2006.` |
| 204 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.982. Support: 578.` |
| 205 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 42669.` |
| 206 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.975. Support: 296.` |
| 207 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1179.` |
| 208 | `  -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 1058.` |
| 209 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 848.` |
| 210 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 390.` |
| 211 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 527.` |
| 212 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {'}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +4.internal_type = NumericLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣␣␣<br>Confidence: 0.978. Support: 206.` |
| 213 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>} and not in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1194.` |
| 214 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 215 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 331.` |
| 216 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.roles in {NUMBER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1091.` |
| 217 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {=, [}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +2.roles not in {NUMBER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 15820.` |
| 218 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = [<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.973. Support: 237.` |
| 219 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.roles in {CALL, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.966. Support: 222.` |
| 220 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 10694.` |
| 221 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.946. Support: 494.` |
| 222 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.927. Support: 389.` |
| 223 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 8<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 233.` |
| 224 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 7<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1331.` |
| 225 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 277.` |
| 226 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 153.` |
| 227 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,, ]}<br>	∧ +2.length ≤ 22<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2666.` |
| 228 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ -5.diff_col ≥ 2<br>	∧ -5.reserved = var<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2559.` |
| 229 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ -5.diff_col ≥ 2<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1953.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.851528384279476, "max_conf": 0.9998173713684082, "max_support": 43478, "min_conf": 0.9208144545555115, "min_support": 143, "num_rules": 229}}
```
</details>
