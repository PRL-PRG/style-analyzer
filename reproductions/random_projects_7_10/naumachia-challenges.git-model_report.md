# Model report for file:///tmp/top-repos-quality-repos-mlrr7gjb/naumachia-challenges.git HEAD 8dc3053b9fb4c4adaf1627fbae31eb393aa61ebd

### Dump

```json
{'created_at': '2021-08-21 06:30:40',
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
 'size': '29.9 kB',
 'tags': [],
 'uuid': '52cc7153-0700-42ef-b17b-44ee16ab328f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mlrr7gjb/naumachia-challenges.git 8dc3053b9fb4c4adaf1627fbae31eb393aa61ebd

# javascript
366 rules, avg.len. 9.9
## train
PPCR: 0.876623
### report
macro
{'f1-score': 0.311403460316168,
 'precision': 0.321123954456865,
 'recall': 0.30710544071178314,
 'support': 94983}
micro
{'f1-score': 0.9223966393986293,
 'precision': 0.9223966393986293,
 'recall': 0.9223966393986293,
 'support': 94983}
weighted
{'f1-score': 0.918524096445704,
 'precision': 0.9194636818944455,
 'recall': 0.9223966393986293,
 'support': 94983}
### report_full
macro
{'f1-score': 0.24537866806085085,
 'precision': 0.321123954456865,
 'recall': 0.22255419120130426,
 'support': 108351}
micro
{'f1-score': 0.8617545516244209,
 'precision': 0.9223966393986293,
 'recall': 0.8085942907771964,
 'support': 108351}
weighted
{'f1-score': 0.8295201182266958,
 'precision': 0.8779485693296598,
 'recall': 0.8085942907771964,
 'support': 108351}
## test
PPCR: 0.899306
### report
macro
{'f1-score': 0.21883808948869432,
 'precision': 0.22624061367025722,
 'recall': 0.2137331637234361,
 'support': 777}
micro
{'f1-score': 0.9575289575289575,
 'precision': 0.9575289575289575,
 'recall': 0.9575289575289575,
 'support': 777}
weighted
{'f1-score': 0.9514213691878901,
 'precision': 0.9470898814238401,
 'recall': 0.9575289575289575,
 'support': 777}
### report_full
macro
{'f1-score': 0.19476194157893906,
 'precision': 0.22624061367025722,
 'recall': 0.18491854967747579,
 'support': 864}
micro
{'f1-score': 0.9067641681901281,
 'precision': 0.9575289575289575,
 'recall': 0.8611111111111112,
 'support': 864}
weighted
{'f1-score': 0.8665017492339735,
 'precision': 0.8899270209268645,
 'recall': 0.8611111111111112,
 'support': 864}
```

## javascript
### Summary
122 rules, avg.len. 8.4

| | |
|-|-|
|Min support|147|
|Max support|14760|
|Min confidence|0.9206548929214478|
|Max confidence|0.999544620513916|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14690.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3973.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1193.` |
| 4 | `  -1.diff_offset ≥ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.995. Support: 332.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 3632.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {RIGHT} and not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 280.` |
| 7 | `  -1.diff_col ≥ 12<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 185.` |
| 8 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 164.` |
| 9 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1417.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≥ 2<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = "<br>Confidence: 0.978. Support: 247.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.944. Support: 276.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 13480.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3563.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 2332.` |
| 15 | `  -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 574.` |
| 16 | `  •••start_col ≤ 32<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ +3.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 520.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14647.` |
| 18 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3981.` |
| 19 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 1183.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.938. Support: 232.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 4798.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {RIGHT} and not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 314.` |
| 23 | `  -1.diff_col ≥ 12<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.985. Support: 170.` |
| 24 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1071.` |
| 25 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 250<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 147.` |
| 26 | `  •••start_line ≥ 216<br>	∧ -1.roles in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.942. Support: 391.` |
| 27 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 450.` |
| 28 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 13633.` |
| 29 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3654.` |
| 30 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 2306.` |
| 31 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 576.` |
| 32 | `  •••start_col ≥ 33<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 6<br>	∧ -3.label in {<space>}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 303.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1196.` |
| 34 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 9<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 352.` |
| 35 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 4803.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 323.` |
| 37 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1085.` |
| 38 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 157.` |
| 39 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 316.` |
| 40 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 604.` |
| 41 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -1.length ≤ 7<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.roles not in {NUMBER}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER, IF}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 483.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 14760.` |
| 43 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 4009.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1230.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = '<br>Confidence: 0.926. Support: 224.` |
| 46 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 3583.` |
| 47 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 286.` |
| 48 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 157.` |
| 49 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 965.` |
| 50 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 240<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {FILE}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 51 | `  -1.roles in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.978. Support: 558.` |
| 52 | `  •••start_line ≥ 218<br>	∧ -1.roles in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.938. Support: 364.` |
| 53 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.940. Support: 326.` |
| 54 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 13440.` |
| 55 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3519.` |
| 56 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 2329.` |
| 57 | `  -1.diff_offset ≤ 6<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 567.` |
| 58 | `  •••start_col ≥ 33<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.roles in {BINARY}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 202.` |
| 59 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 9<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 325.` |
| 60 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = '<br>Confidence: 0.921. Support: 221.` |
| 61 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 4722.` |
| 62 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 308.` |
| 63 | `  -1.diff_col ≥ 12<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>⇒ y = "<br>Confidence: 0.997. Support: 166.` |
| 64 | `  -1.internal_type = StringLiteral<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = "<br>Confidence: 0.982. Support: 575.` |
| 65 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {KEY}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.938. Support: 364.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 584.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14683.` |
| 68 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3930.` |
| 69 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1226.` |
| 70 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 3585.` |
| 71 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 310.` |
| 72 | `  -1.diff_col ≥ 12<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.991. Support: 172.` |
| 73 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 154.` |
| 74 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1098.` |
| 75 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 240<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type not in {File, IfStatement}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 76 | `  •••start_line ≥ 218<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.956. Support: 396.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION, RIGHT}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 535.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 13439.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3540.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2316.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 603.` |
| 82 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 14617.` |
| 83 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {RIGHT} and not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 318.` |
| 84 | `  -1.diff_col ≥ 12<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 152.` |
| 85 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1378.` |
| 86 | `  -1.roles in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = '<br>Confidence: 0.953. Support: 390.` |
| 87 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 13451.` |
| 88 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3528.` |
| 89 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2344.` |
| 90 | `  -1.diff_offset ≤ 6<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 587.` |
| 91 | `  -1.internal_type = StringLiteral<br>	∧ -4.reserved not in {.}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.921. Support: 397.` |
| 92 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 3940.` |
| 93 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 4746.` |
| 94 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 343.` |
| 95 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 181.` |
| 96 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1386.` |
| 97 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 233<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, {, }}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {,}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {OPERATOR} and not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 98 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 233<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {,}<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.roles not in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 207.` |
| 99 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 929.` |
| 100 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -4.label not in {<space>}<br>	∧ -5.internal_type = Identifier<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {IF, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 535.` |
| 101 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3917.` |
| 102 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 9<br>	∧ -2.label in {<newline>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 103 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≤ 8<br>	∧ -2.label in {<newline>}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 288.` |
| 104 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.942. Support: 231.` |
| 105 | `  -1.diff_col ≤ 11<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 175.` |
| 106 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {KEY}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.940. Support: 375.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 13292.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3434.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2332.` |
| 110 | `  -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 571.` |
| 111 | `  •••start_col ≥ 31<br>	∧ -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 340.` |
| 112 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14653.` |
| 113 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION} and not in {ARGUMENT}<br>⇒ y = "<br>Confidence: 0.988. Support: 717.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 4103.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1262.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.length ≥ 9<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 367.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 3527.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 315.` |
| 119 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 170.` |
| 120 | `  -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1049.` |
| 121 | `  •••start_col ≥ 8<br>	∧ •••start_line ≥ 233<br>	∧ -1.diff_col ≤ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, =, var, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {,}<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY, RIGHT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {OPERATOR} and not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 163.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 6<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 591.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.434426229508198, "max_conf": 0.999544620513916, "max_support": 14760, "min_conf": 0.9206548929214478, "min_support": 147, "num_rules": 122}}
```
</details>
