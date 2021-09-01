# Model report for file:///tmp/top-repos-quality-repos-ar98v6wy/xnote.git HEAD 919104b746a912ca0365a6b20c7a3d99b16bc147

### Dump

```json
{'created_at': '2021-08-30 06:55:03',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '28.7 kB',
 'tags': [],
 'uuid': '81bd3f42-bc11-47de-8ae1-65cc51cbd3c2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ar98v6wy/xnote.git 919104b746a912ca0365a6b20c7a3d99b16bc147

# javascript
345 rules, avg.len. 12.0
## train
PPCR: 0.914914
### report
macro
{'f1-score': 0.47430820125480994,
 'precision': 0.4896388475529988,
 'recall': 0.4689917962424718,
 'support': 166056}
micro
{'f1-score': 0.9270968829792359,
 'precision': 0.9270968829792359,
 'recall': 0.9270968829792359,
 'support': 166056}
weighted
{'f1-score': 0.9199605825868887,
 'precision': 0.9147429342137859,
 'recall': 0.9270968829792359,
 'support': 166056}
### report_full
macro
{'f1-score': 0.40159501921134166,
 'precision': 0.4896388475529988,
 'recall': 0.35519285600895856,
 'support': 181499}
micro
{'f1-score': 0.8859029506121333,
 'precision': 0.9270968829792359,
 'recall': 0.8482140397467755,
 'support': 181499}
weighted
{'f1-score': 0.8636462630260084,
 'precision': 0.8912263687907787,
 'recall': 0.8482140397467755,
 'support': 181499}
## test
PPCR: 0.888417
### report
macro
{'f1-score': 0.41267699875619474,
 'precision': 0.41216683422565775,
 'recall': 0.44317046700102963,
 'support': 1465}
micro
{'f1-score': 0.9522184300341296,
 'precision': 0.9522184300341296,
 'recall': 0.9522184300341296,
 'support': 1465}
weighted
{'f1-score': 0.9467180318971423,
 'precision': 0.9448605736891224,
 'recall': 0.9522184300341296,
 'support': 1465}
### report_full
macro
{'f1-score': 0.3425809564734471,
 'precision': 0.41216683422565775,
 'recall': 0.32137438671532675,
 'support': 1649}
micro
{'f1-score': 0.8959537572254336,
 'precision': 0.9522184300341296,
 'recall': 0.8459672528805336,
 'support': 1649}
weighted
{'f1-score': 0.8623431835173586,
 'precision': 0.9001113476053725,
 'recall': 0.8459672528805336,
 'support': 1649}
```

## javascript
### Summary
168 rules, avg.len. 10.9

| | |
|-|-|
|Min support|143|
|Max support|26126|
|Min confidence|0.9206695556640625|
|Max confidence|0.9997222423553467|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.989. Support: 25975.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 1052.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 443.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 426.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 8421.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 4375.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {[}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 1340.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 2203.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 333.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 304.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1631.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 530.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 17535.` |
| 14 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5477.` |
| 15 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 16<br>	∧ -4.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 144.` |
| 16 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 2492.` |
| 17 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {CALLEE}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 1215.` |
| 18 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1200.` |
| 19 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.957. Support: 901.` |
| 20 | `  •••start_col ≤ 9<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.927. Support: 320.` |
| 21 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3002.` |
| 22 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 404.` |
| 23 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 2403.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1022.` |
| 25 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION, NAME} and not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 406.` |
| 26 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.966. Support: 164.` |
| 27 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1387.` |
| 28 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,, .}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 8698.` |
| 29 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1380.` |
| 30 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.internal_type = Identifier<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 432.` |
| 31 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 32 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 26039.` |
| 33 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1112.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 529.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 467.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 8395.` |
| 37 | `  -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 4239.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 2260.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 319.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1731.` |
| 41 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 17438.` |
| 42 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.diff_col ≥ 9<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 155.` |
| 43 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 5429.` |
| 44 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3098.` |
| 45 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.949. Support: 886.` |
| 46 | `  •••start_col ≤ 9<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.938. Support: 284.` |
| 47 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 372.` |
| 48 | `  •••start_col ≤ 6<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.941. Support: 398.` |
| 49 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 2355.` |
| 50 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1480.` |
| 51 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION, NAME} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 420.` |
| 52 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {BODY, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 1470.` |
| 53 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.internal_type = Identifier<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {BODY, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 542.` |
| 54 | `  •••start_col ≤ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 327.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 26126.` |
| 56 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {IDENTICAL, OPERATOR}<br>⇒ y = '<br>Confidence: 0.970. Support: 280.` |
| 57 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 488.` |
| 58 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 472.` |
| 59 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +3.reserved not in {]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 8313.` |
| 60 | `  -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +3.reserved not in {]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 4379.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1800.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.length ≥ 8<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 180.` |
| 63 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 21<br>	∧ -5.diff_line ≤ 1<br>	∧ -5.diff_offset ≥ 23<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 352.` |
| 64 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 22<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 6080.` |
| 65 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3109.` |
| 66 | `  •••start_col ≤ 8<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.939. Support: 301.` |
| 67 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 7<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ForStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.926. Support: 397.` |
| 68 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1672.` |
| 69 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,, .}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 6273.` |
| 70 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 659.` |
| 71 | `  •••start_col ≥ 20<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 1288.` |
| 72 | `  •••start_col ≤ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 341.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved = [<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 567.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 469.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 305.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 329.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 465.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 8585.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 4455.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1577.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 491.` |
| 82 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2511.` |
| 83 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1443.` |
| 84 | `  •••start_col ≥ 15<br>	∧ -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1474.` |
| 85 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 173.` |
| 86 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.label in {<space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 5415.` |
| 87 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.label not in {<space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 4981.` |
| 88 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.label in {<-space>} and not in {<space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, VARIABLE}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 836.` |
| 89 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.label not in {<-space>, <newline>, <space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, VARIABLE}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 670.` |
| 90 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.label not in {<-space>, <newline>, <space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), ,, ;, function, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, VARIABLE}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1289.` |
| 91 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 5<br>	∧ -4.label not in {<-space>, <newline>, <space>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), ,, ;, function, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +2.roles in {ASSIGNMENT} and not in {COMMENT, STRING}<br>	∧ +2.length ≤ 5<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, VARIABLE}<br>	∧ ^2.internal_type not in {AssignmentExpression}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 271.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 25769.` |
| 93 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 595.` |
| 94 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 422.` |
| 95 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +3.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 8414.` |
| 96 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +3.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 4229.` |
| 97 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≤ 1<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 1257.` |
| 98 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 2343.` |
| 99 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 304.` |
| 100 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1749.` |
| 101 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 17417.` |
| 102 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label in {<newline>}<br>	∧ -4.length ≤ 16<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 158.` |
| 103 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION, FUNCTION} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 415.` |
| 104 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 10<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1372.` |
| 105 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 640.` |
| 106 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 201.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 25832.` |
| 108 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1110.` |
| 109 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {IDENTICAL, OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.957. Support: 247.` |
| 110 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 555.` |
| 111 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 426.` |
| 112 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 16369.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 2172.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 328.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1733.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, RIGHT}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {CALL, MAP}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 749.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {RIGHT}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {CALL, MAP}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 687.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 17441.` |
| 119 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5555.` |
| 120 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.940. Support: 907.` |
| 121 | `  •••start_col ≤ 9<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.962. Support: 325.` |
| 122 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 393.` |
| 123 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 2318.` |
| 124 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION, NAME} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 431.` |
| 125 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 126 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 25798.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 440.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 8397.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 4332.` |
| 130 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {[}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 980.` |
| 131 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 2222.` |
| 132 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 291.` |
| 133 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1737.` |
| 134 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 17474.` |
| 135 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION, FUNCTION} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 424.` |
| 136 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, FUNCTION, MAP}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.984. Support: 153.` |
| 137 | `  -1.diff_col ≥ 7<br>	∧ -1.diff_offset ≥ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 1514.` |
| 138 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {BODY, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1371.` |
| 139 | `  •••start_col ≥ 12<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.internal_type = Identifier<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {BODY, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 563.` |
| 140 | `  •••start_col ≤ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 319.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 427.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 16419.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 471.` |
| 144 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 22<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {FUNCTION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 5487.` |
| 145 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.944. Support: 913.` |
| 146 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 403.` |
| 147 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 288.` |
| 148 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 16634.` |
| 149 | `  •••start_col ≤ 5<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.957. Support: 294.` |
| 150 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 151 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 577.` |
| 152 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 299.` |
| 153 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 435.` |
| 154 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 8256.` |
| 155 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {[}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 4267.` |
| 156 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {[}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -3.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 1264.` |
| 157 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.length ≥ 8<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 176.` |
| 158 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles in {ARGUMENT}<br>	∧ -4.length ≤ 7<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 474.` |
| 159 | `  -1.roles in {EXPRESSION, RIGHT} and not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ -4.length ≤ 7<br>	∧ -5.label not in {<space>}<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VALUE}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 163.` |
| 160 | `  •••start_col ≥ 8<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 17309.` |
| 161 | `  •••start_col ≤ 7<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -5.diff_col ≥ 7<br>	∧ +1.reserved not in {), =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 162 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 18<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 174.` |
| 163 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.942. Support: 890.` |
| 164 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 1422.` |
| 165 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {', (, )}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {,, .}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 12359.` |
| 166 | `  •••start_col ≥ 11<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {', (, )}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,, .}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER, OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 1630.` |
| 167 | `  •••start_col ≥ 11<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {', (, )}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {,, .}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER, OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 306.` |
| 168 | `  •••start_col ≤ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 185.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.94047619047619, "max_conf": 0.9997222423553467, "max_support": 26126, "min_conf": 0.9206695556640625, "min_support": 143, "num_rules": 168}}
```
</details>
