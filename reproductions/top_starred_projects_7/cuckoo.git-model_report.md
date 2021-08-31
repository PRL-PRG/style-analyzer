# Model report for file:///tmp/top-repos-quality-repos-rv0csyb4/cuckoo.git HEAD 50452a39ff7c3e0c4c94d114bc6317101633b958

### Dump

```json
{'created_at': '2021-08-30 17:45:51',
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
 'size': '29.3 kB',
 'tags': [],
 'uuid': '3fc468b9-e3ac-4bdd-9d6e-429ce2482cf2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rv0csyb4/cuckoo.git 50452a39ff7c3e0c4c94d114bc6317101633b958

# javascript
184 rules, avg.len. 8.8
## train
PPCR: 0.911195
### report
macro
{'f1-score': 0.37244467311958296,
 'precision': 0.38298538697576323,
 'recall': 0.3666315520136376,
 'support': 80690}
micro
{'f1-score': 0.9381955632668236,
 'precision': 0.9381955632668236,
 'recall': 0.9381955632668236,
 'support': 80690}
weighted
{'f1-score': 0.9311861832343681,
 'precision': 0.9261498309114273,
 'recall': 0.9381955632668236,
 'support': 80690}
### report_full
macro
{'f1-score': 0.3227512799638354,
 'precision': 0.38298538697576323,
 'recall': 0.29868331525147773,
 'support': 88554}
micro
{'f1-score': 0.8946018765805582,
 'precision': 0.9381955632668236,
 'recall': 0.8548795085484563,
 'support': 88554}
weighted
{'f1-score': 0.8620126393450588,
 'precision': 0.8853455059228224,
 'recall': 0.8548795085484563,
 'support': 88554}
## test
PPCR: 0.910099
### report
macro
{'f1-score': 0.36908351227293024,
 'precision': 0.3923229738489967,
 'recall': 0.35784933468455704,
 'support': 19700}
micro
{'f1-score': 0.9436548223350254,
 'precision': 0.9436548223350254,
 'recall': 0.9436548223350254,
 'support': 19700}
weighted
{'f1-score': 0.935560719947431,
 'precision': 0.930698833024497,
 'recall': 0.9436548223350254,
 'support': 19700}
### report_full
macro
{'f1-score': 0.3125627668497828,
 'precision': 0.3923229738489967,
 'recall': 0.2887442119878727,
 'support': 21646}
micro
{'f1-score': 0.8992405553136943,
 'precision': 0.9436548223350254,
 'recall': 0.858819181373002,
 'support': 21646}
weighted
{'f1-score': 0.8641652629243852,
 'precision': 0.8917723020307752,
 'recall': 0.858819181373002,
 'support': 21646}
```

## javascript
### Summary
92 rules, avg.len. 9.5

| | |
|-|-|
|Min support|135|
|Max support|20368|
|Min confidence|0.9201030731201172|
|Max confidence|0.9998081922531128|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 13660.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 343.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2629.` |
| 4 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1208.` |
| 5 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 5239.` |
| 6 | `  -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1866.` |
| 7 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1416.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 1089.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1036.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 19730.` |
| 12 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 338.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 5856.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 13471.` |
| 15 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2716.` |
| 16 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 1145.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 5375.` |
| 20 | `  -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1839.` |
| 21 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1391.` |
| 22 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 1095.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 440.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -5.length ≥ 9<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 402.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.roles not in {RIGHT}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 940.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {, }}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.roles not in {RIGHT}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 20368.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 194.` |
| 28 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.length ≥ 2<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 1137.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 158.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 1722.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 19501.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.989. Support: 13390.` |
| 33 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 361.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {ARITHMETIC, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 483.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 4997.` |
| 36 | `  +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2607.` |
| 37 | `  -1.reserved = (<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2588.` |
| 38 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 959.` |
| 39 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_line = 0<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 2689.` |
| 40 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved = :<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 268.` |
| 41 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 1816.` |
| 42 | `  +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1498.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {[, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 17074.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {BINARY}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 5760.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {BINARY}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {BINARY}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 175.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 13522.` |
| 47 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 634.` |
| 48 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 5071.` |
| 49 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1400.` |
| 50 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 1128.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1765.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 19300.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 13618.` |
| 54 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 364.` |
| 55 | `  -1.reserved = (<br>	∧ -3.diff_col ≤ 15<br>	∧ -5.reserved = .<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.991. Support: 165.` |
| 56 | `  -1.reserved = (<br>	∧ -3.diff_col ≤ 15<br>	∧ -4.reserved = )<br>	∧ -5.reserved not in {.}<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.932. Support: 212.` |
| 57 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 2651.` |
| 58 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1174.` |
| 59 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 60 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 5329.` |
| 61 | `  -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 1769.` |
| 62 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 1430.` |
| 63 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 1119.` |
| 64 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -2.length ≥ 5<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.930. Support: 337.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 1760.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 19559.` |
| 67 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 68 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {}<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 69 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 5083.` |
| 70 | `  -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 1449.` |
| 71 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 1128.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BINARY, BLOCK}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1801.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BINARY, BLOCK}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 19405.` |
| 75 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 2873.` |
| 76 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1458.` |
| 77 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 141.` |
| 78 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, LITERAL}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {ASSIGNMENT}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 466.` |
| 79 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2645.` |
| 80 | `  -1.diff_col ≥ 9<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.length ≥ 2<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 1197.` |
| 81 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 183.` |
| 82 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 83 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 135.` |
| 84 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1782.` |
| 85 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 19325.` |
| 86 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 5751.` |
| 87 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 167.` |
| 88 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.roles not in {BINARY}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1440.` |
| 89 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {:, ;, if, {}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.roles not in {BINARY}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 19738.` |
| 90 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 186.` |
| 91 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 92 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 5393.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.456521739130435, "max_conf": 0.9998081922531128, "max_support": 20368, "min_conf": 0.9201030731201172, "min_support": 135, "num_rules": 92}}
```
</details>
