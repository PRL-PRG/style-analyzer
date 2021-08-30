# Model report for file:///tmp/top-repos-quality-repos-pp738myk/ar.js.git HEAD aaad9847f67a4738b00724c38d9f501d78e0a8af

### Dump

```json
{'created_at': '2021-08-29 11:42:54',
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
 'size': '21.3 kB',
 'tags': [],
 'uuid': '55a7193d-c8ea-4333-9ba9-4814b0742cab',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-pp738myk/ar.js.git aaad9847f67a4738b00724c38d9f501d78e0a8af

# javascript
103 rules, avg.len. 7.2
## train
PPCR: 0.869360
### report
macro
{'f1-score': 0.2998522313346845,
 'precision': 0.30130089203480886,
 'recall': 0.29863684725539624,
 'support': 33972}
micro
{'f1-score': 0.9286471211586012,
 'precision': 0.9286471211586012,
 'recall': 0.9286471211586012,
 'support': 33972}
weighted
{'f1-score': 0.9226948435759701,
 'precision': 0.9171074067603849,
 'recall': 0.9286471211586012,
 'support': 33972}
### report_full
macro
{'f1-score': 0.270224347569837,
 'precision': 0.30130089203480886,
 'recall': 0.2525073028906843,
 'support': 39077}
micro
{'f1-score': 0.8637489904037017,
 'precision': 0.9286471211586012,
 'recall': 0.8073291194308673,
 'support': 39077}
weighted
{'f1-score': 0.8241993894730928,
 'precision': 0.8523130314184103,
 'recall': 0.8073291194308673,
 'support': 39077}
## test
PPCR: 0.886758
### report
macro
{'f1-score': 0.3082271840574727,
 'precision': 0.311784121358837,
 'recall': 0.30589034916044583,
 'support': 2913}
micro
{'f1-score': 0.9474768280123584,
 'precision': 0.9474768280123584,
 'recall': 0.9474768280123584,
 'support': 2913}
weighted
{'f1-score': 0.9381589584431788,
 'precision': 0.9303725332937431,
 'recall': 0.9474768280123584,
 'support': 2913}
### report_full
macro
{'f1-score': 0.2866302659381929,
 'precision': 0.311784121358837,
 'recall': 0.2728275364731148,
 'support': 3285}
micro
{'f1-score': 0.8906098741529526,
 'precision': 0.9474768280123584,
 'recall': 0.8401826484018264,
 'support': 3285}
weighted
{'f1-score': 0.8471271974274869,
 'precision': 0.8661329999934497,
 'recall': 0.8401826484018264,
 'support': 3285}
```

## javascript
### Summary
57 rules, avg.len. 6.5

| | |
|-|-|
|Min support|139|
|Max support|8698|
|Min confidence|0.9204338192939758|
|Max confidence|0.9983108043670654|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 8402.` |
| 2 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 2518.` |
| 3 | `  -1.label in {<space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.949. Support: 146.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.length ≤ 1<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 167.` |
| 5 | `  -2.label in {<newline>}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 727.` |
| 6 | `  •••start_col ≥ 24<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 296.` |
| 7 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.953. Support: 245.` |
| 8 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length ≥ 3<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 293.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.963. Support: 715.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 644.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 141.` |
| 12 | `  -3.diff_line ≥ 1<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 739.` |
| 13 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.length ≥ 3<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 259.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 535.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 5991.` |
| 16 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.923. Support: 188.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 7<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.961. Support: 245.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.977. Support: 555.` |
| 19 | `  •••start_col ≤ 15<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 775.` |
| 20 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.975. Support: 260.` |
| 21 | `  -1.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.972. Support: 685.` |
| 22 | `  -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.976. Support: 565.` |
| 23 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 6039.` |
| 24 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.993. Support: 8698.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 2485.` |
| 26 | `  -1.label not in {<space>}<br>	∧ -1.length ≤ 1<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 163.` |
| 27 | `  •••start_col ≤ 16<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 734.` |
| 28 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.943. Support: 343.` |
| 29 | `  -1.reserved = var<br>	∧ -2.diff_offset ≤ 21<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 207.` |
| 30 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_offset ≤ 21<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, OPERATOR}<br>⇒ y = '<br>Confidence: 0.929. Support: 234.` |
| 31 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.962. Support: 722.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 544.` |
| 33 | `  -3.diff_line ≥ 1<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 796.` |
| 34 | `  -1.reserved = (<br>	∧ +1.roles in {STRING} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.939. Support: 318.` |
| 35 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 220.` |
| 36 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 8<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = '<br>Confidence: 0.952. Support: 241.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 635.` |
| 38 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 2441.` |
| 39 | `  -3.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.921. Support: 171.` |
| 40 | `  -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 139.` |
| 41 | `  -1.reserved not in {(}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 692.` |
| 42 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_col ≥ 38<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.965. Support: 612.` |
| 44 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 8559.` |
| 45 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 2438.` |
| 46 | `  -2.label in {<newline>}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 671.` |
| 47 | `  -1.reserved = (<br>	∧ +1.roles in {STRING} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.938. Support: 297.` |
| 48 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.952. Support: 219.` |
| 49 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.961. Support: 699.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.974. Support: 557.` |
| 51 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 155.` |
| 52 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.933. Support: 233.` |
| 53 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 5<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 221.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.979. Support: 630.` |
| 55 | `  •••start_col ≥ 24<br>	∧ -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 252.` |
| 56 | `  -1.diff_col ≤ 8<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 7<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.952. Support: 241.` |
| 57 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = =<br>	∧ +2.length ≤ 7<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 222.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.526315789473684, "max_conf": 0.9983108043670654, "max_support": 8698, "min_conf": 0.9204338192939758, "min_support": 139, "num_rules": 57}}
```
</details>
