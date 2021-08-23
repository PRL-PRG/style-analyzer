# Model report for file:///tmp/top-repos-quality-repos-gz0tm4hq/brooder.git HEAD ed2d140f19bd4598ff6478c1179187039b14009b

### Dump

```json
{'created_at': '2021-08-21 02:19:53',
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
 'size': '59.6 kB',
 'tags': [],
 'uuid': 'd6a16e9d-76e9-4258-8d0f-a5d3013c344e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gz0tm4hq/brooder.git ed2d140f19bd4598ff6478c1179187039b14009b

# javascript
151 rules, avg.len. 13.3
## train
PPCR: 0.939751
### report
macro
{'f1-score': 0.3461187856568472,
 'precision': 0.378176659071206,
 'recall': 0.3376558696375544,
 'support': 450249}
micro
{'f1-score': 0.9585162876541647,
 'precision': 0.9585162876541647,
 'recall': 0.9585162876541647,
 'support': 450249}
weighted
{'f1-score': 0.9530916999759864,
 'precision': 0.9517751882299623,
 'recall': 0.9585162876541647,
 'support': 450249}
### report_full
macro
{'f1-score': 0.3080679045823925,
 'precision': 0.378176659071206,
 'recall': 0.2842038064990824,
 'support': 479115}
micro
{'f1-score': 0.9287448190375354,
 'precision': 0.9585162876541647,
 'recall': 0.9007670392285777,
 'support': 479115}
weighted
{'f1-score': 0.916488293901284,
 'precision': 0.9439935995455577,
 'recall': 0.9007670392285777,
 'support': 479115}
## test
PPCR: 0.942858
### report
macro
{'f1-score': 0.34104749696722564,
 'precision': 0.3773192956480469,
 'recall': 0.33248592742794425,
 'support': 111658}
micro
{'f1-score': 0.9537695462931451,
 'precision': 0.9537695462931451,
 'recall': 0.9537695462931451,
 'support': 111658}
weighted
{'f1-score': 0.9472551151122366,
 'precision': 0.9467321693020252,
 'recall': 0.9537695462931451,
 'support': 111658}
### report_full
macro
{'f1-score': 0.30998509642017275,
 'precision': 0.3773192956480469,
 'recall': 0.2871925262884956,
 'support': 118425}
micro
{'f1-score': 0.9257181104210219,
 'precision': 0.9537695462931451,
 'recall': 0.8992695799028921,
 'support': 118425}
weighted
{'f1-score': 0.913206901929344,
 'precision': 0.9388423406081724,
 'recall': 0.8992695799028921,
 'support': 118425}
```

## javascript
### Summary
98 rules, avg.len. 12.8

| | |
|-|-|
|Min support|91|
|Max support|48074|
|Min confidence|0.9203821420669556|
|Max confidence|0.9990440011024475|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {==}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved = +<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.946. Support: 249.` |
| 2 | `  •••start_line ≥ 121<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {==}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.921. Support: 5934.` |
| 3 | `  •••start_line ≤ 120<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {==}<br>	∧ -5.reserved not in {(}<br>	∧ +2.length ≤ 8<br>	∧ +3.reserved = .<br>	∧ +4.reserved not in {(}<br>	∧ ^1.roles in {BINARY} and not in {FILE}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.998. Support: 330.` |
| 4 | `  •••start_line ≤ 120<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {==}<br>	∧ -5.reserved not in {(}<br>	∧ +2.length ≤ 8<br>	∧ +3.reserved not in {.}<br>	∧ +4.reserved not in {(}<br>	∧ ^1.roles not in {FILE}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.990. Support: 18158.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.992. Support: 48074.` |
| 6 | `  •••start_col ≥ 11<br>	∧ •••start_line ≥ 121<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {ARITHMETIC} and not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.923. Support: 150.` |
| 7 | `  •••start_col ≥ 11<br>	∧ •••start_line ≥ 121<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC, ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.928. Support: 3211.` |
| 8 | `  •••start_col ≥ 11<br>	∧ •••start_line ≤ 120<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ASSIGNMENT}<br>⇒ y = '<br>Confidence: 0.986. Support: 15193.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_offset ≥ 17<br>	∧ -5.roles in {IDENTIFIER}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 720.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -5.roles not in {IDENTIFIER}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 13398.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.963. Support: 2184.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.length ≥ 2<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 661.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 1870.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1089.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 963.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 490.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = )<br>	∧ -3.internal_type = StringLiteral<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 294.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.roles in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1016.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 1122.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.reserved = ;<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 5<br>	∧ +5.reserved = .<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 175.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.reserved = ;<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +5.reserved not in {.}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1778.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -5.reserved not in {!==, ;}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 25216.` |
| 23 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type = Identifier<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.reserved not in {!==, ;}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 2450.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {Identifier, StringLiteral}<br>	∧ -3.roles in {BINARY}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.reserved not in {!==, ;}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 157.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {Identifier, StringLiteral}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.reserved not in {!==, ;}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.internal_type not in {Identifier, StringLiteral}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 6<br>	∧ -5.reserved not in {!==, ;}<br>	∧ -5.roles not in {LEFT}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 20<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 4699.` |
| 27 | `  •••start_line ≥ 155<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 9<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 28 | `  •••start_line ≥ 155<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 9<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1041.` |
| 29 | `  •••start_col ≤ 27<br>	∧ •••start_line ≤ 154<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.length ≥ 9<br>	∧ -4.diff_col ≤ 29<br>	∧ -4.reserved not in {}}<br>	∧ -5.diff_col ≤ 3<br>	∧ -5.diff_offset ≤ 42<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {MAP}<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 30 | `  •••start_col ≤ 27<br>	∧ •••start_line ≤ 154<br>	∧ -1.diff_offset ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.length ≥ 9<br>	∧ -4.diff_col ≤ 29<br>	∧ -4.reserved not in {}}<br>	∧ -5.diff_col ≥ 3<br>	∧ -5.diff_offset ≤ 42<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {MAP}<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 266.` |
| 31 | `  •••start_col ≤ 17<br>	∧ •••start_line ≤ 154<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -2.length ≥ 9<br>	∧ -5.diff_col ≤ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 2425.` |
| 32 | `  •••start_col ≤ 17<br>	∧ •••start_line ≤ 127<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, IDENTIFIER}<br>	∧ -2.length ≥ 9<br>	∧ -5.diff_col ≤ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {LITERAL}<br>	∧ +2.length ≥ 10<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 33 | `  •••start_line ≤ 154<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ -2.length ≥ 9<br>	∧ -5.diff_col ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 34 | `  •••start_line ≤ 154<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ -2.length ≥ 9<br>	∧ -3.diff_offset ≥ 18<br>	∧ -4.diff_col ≤ 4<br>	∧ -5.diff_col ≤ 2<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 104.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≤ 8<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 3439.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≤ 8<br>	∧ +1.reserved = ?<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 1231.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, THEN}<br>	∧ -2.length ≤ 8<br>	∧ +1.reserved not in {?}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 353.` |
| 38 | `  •••start_line ≤ 250<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = function<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 586.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, LITERAL}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = {<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 305.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = {<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 523.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 130.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved = (<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 342.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved not in {(, {}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = {<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 5571.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved not in {(, {}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 931.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {THEN}<br>	∧ -2.length ≤ 8<br>	∧ -3.reserved not in {(, {}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 35954.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 13<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.972. Support: 3016.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ -4.diff_offset ≤ 12<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.930. Support: 291.` |
| 49 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 15530.` |
| 50 | `  •••start_col ≤ 12<br>	∧ •••start_line ≥ 203<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 672.` |
| 51 | `  •••start_col ≤ 12<br>	∧ •••start_line ≤ 202<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 175.` |
| 52 | `  •••start_col ≤ 12<br>	∧ •••start_line ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1246.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 499.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 303.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.951. Support: 766.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 623.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 9<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 361.` |
| 58 | `  •••start_line ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 285.` |
| 59 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved = (<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 445.` |
| 60 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved = (<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 405.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = .<br>	∧ -4.diff_offset ≥ 12<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 267.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,, .}<br>	∧ -4.diff_offset ≥ 12<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 27317.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.diff_col ≥ 9<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,, ;}<br>	∧ -4.diff_offset ≤ 11<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.996. Support: 131.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,, ;}<br>	∧ -4.diff_offset ≤ 11<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.942. Support: 968.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {BINARY} and not in {CALL}<br>	∧ -4.diff_offset ≤ 11<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +3.length ≤ 8<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 445.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {CALL}<br>	∧ -4.diff_offset ≤ 11<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.diff_col ≥ 4<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 10811.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {(, ), ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 11<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -5.diff_col ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclarator<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 155.` |
| 68 | `  •••start_col ≥ 10<br>	∧ •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {CALL} and not in {CALLEE}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.961. Support: 2462.` |
| 69 | `  •••start_col ≥ 10<br>	∧ •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +4.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.996. Support: 810.` |
| 70 | `  •••start_col ≥ 24<br>	∧ •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = '<br>	∧ -3.length ≤ 14<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +4.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.965. Support: 808.` |
| 71 | `  •••start_col ≤ 9<br>	∧ •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.947. Support: 311.` |
| 72 | `  •••start_line ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻<br>Confidence: 0.971. Support: 298.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 10515.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved = exports<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺<br>Confidence: 0.985. Support: 299.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = return<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved not in {exports}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.948. Support: 2306.` |
| 76 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 157.` |
| 77 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 3790.` |
| 78 | `  •••start_col ≥ 34<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 267.` |
| 79 | `  •••start_col ≤ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 2520.` |
| 80 | `  •••start_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 711.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 4804.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 2114.` |
| 83 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 949.` |
| 84 | `  •••start_col ≥ 11<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -3.length ≥ 9<br>	∧ -4.diff_offset ≥ 21<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 898.` |
| 85 | `  •••start_col ≥ 11<br>	∧ •••start_line ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR}<br>⇒ y = ⏎⏎⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.954. Support: 296.` |
| 86 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 15<br>	∧ +1.reserved = return<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.956. Support: 420.` |
| 87 | `  •••start_col ≥ 30<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, return, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 4<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>	∧ ^2.roles not in {IF}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.972. Support: 706.` |
| 88 | `  •••start_col ≤ 30<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 328.` |
| 89 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 10<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 151.` |
| 90 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 9<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≤ 3<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≥ 12<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.949. Support: 148.` |
| 91 | `  •••start_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 387.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CONDITION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 329.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CONDITION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 2640.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ]<br>	∧ +1.reserved = [<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 251.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 274.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 17474.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 1<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.989. Support: 308.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, [, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 422.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 12.806122448979592, "max_conf": 0.9990440011024475, "max_support": 48074, "min_conf": 0.9203821420669556, "min_support": 91, "num_rules": 98}}
```
</details>
