# Model report for file:///tmp/top-repos-quality-repos-ot6yu6mo/veritaswrestlingsystems.git HEAD 393a6f217a98bf3581e298d269563499af7f4100

### Dump

```json
{'created_at': '2021-08-22 03:24:38',
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
 'size': '22.2 kB',
 'tags': [],
 'uuid': 'ca320370-47c1-4823-85f1-b41fdb5bb334',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ot6yu6mo/veritaswrestlingsystems.git 393a6f217a98bf3581e298d269563499af7f4100

# javascript
54 rules, avg.len. 9.7
## train
PPCR: 0.873364
### report
macro
{'f1-score': 0.7386251356892809,
 'precision': 0.7776380653001014,
 'recall': 0.7126420422710145,
 'support': 87298}
micro
{'f1-score': 0.956264748333295,
 'precision': 0.9562647483332951,
 'recall': 0.9562647483332951,
 'support': 87298}
weighted
{'f1-score': 0.9509429684537658,
 'precision': 0.9476309909092059,
 'recall': 0.9562647483332951,
 'support': 87298}
### report_full
macro
{'f1-score': 0.5288086156346027,
 'precision': 0.7776380653001014,
 'recall': 0.45113018174669794,
 'support': 99956}
micro
{'f1-score': 0.891623142896814,
 'precision': 0.9562647483332951,
 'recall': 0.8351674736884229,
 'support': 99956}
weighted
{'f1-score': 0.871080907135239,
 'precision': 0.9332996464566844,
 'recall': 0.8351674736884229,
 'support': 99956}
## test
PPCR: 0.852204
### report
macro
{'f1-score': 0.47751742164983785,
 'precision': 0.4920162486113911,
 'recall': 0.47787797011265587,
 'support': 20781}
micro
{'f1-score': 0.9493768346085366,
 'precision': 0.9493768346085366,
 'recall': 0.9493768346085366,
 'support': 20781}
weighted
{'f1-score': 0.9427448625145229,
 'precision': 0.9384270101669533,
 'recall': 0.9493768346085366,
 'support': 20781}
### report_full
macro
{'f1-score': 0.35042156370080696,
 'precision': 0.4920162486113911,
 'recall': 0.31661993977990077,
 'support': 24385}
micro
{'f1-score': 0.8736217508745516,
 'precision': 0.9493768346085366,
 'recall': 0.8090629485339348,
 'support': 24385}
weighted
{'f1-score': 0.8478421331748275,
 'precision': 0.91072582697281,
 'recall': 0.8090629485339348,
 'support': 24385}
```

## javascript
### Summary
37 rules, avg.len. 9.8

| | |
|-|-|
|Min support|90|
|Max support|15163|
|Min confidence|0.936302125453949|
|Max confidence|0.999475359916687|

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
| 1 | `  -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.roles not in {KEY}<br>	∧ -5.reserved not in {:}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 823.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ -4.roles not in {KEY}<br>	∧ -5.reserved not in {:}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 15163.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {+}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.941. Support: 3685.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.936. Support: 2661.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.959. Support: 205.` |
| 6 | `  •••start_col ≥ 77<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_col ≥ 6<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 91.` |
| 7 | `  •••start_col ≤ 76<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 2515.` |
| 8 | `  •••start_line ≥ 242<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.reserved = )<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.997. Support: 166.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {FOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 139.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 4708.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 2409.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, =}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 1045.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, =}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 90.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 953.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, =, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 7236.` |
| 16 | `  •••start_col ≥ 19<br>	∧ •••start_line ≥ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.length ≤ 6<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 659.` |
| 17 | `  •••start_col ≤ 18<br>	∧ •••start_line ≥ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 6<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.length ≤ 6<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 134.` |
| 18 | `  •••start_col ≥ 9<br>	∧ •••start_line ≥ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.length ≤ 6<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 920.` |
| 19 | `  •••start_col ≥ 9<br>	∧ •••start_line ≤ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2476.` |
| 20 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 188.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.977. Support: 365.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 961.` |
| 23 | `  -1.diff_col ≥ 9<br>	∧ -1.diff_offset ≥ 86<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 145.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.972. Support: 162.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 8<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.989. Support: 134.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 140.` |
| 27 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.985. Support: 306.` |
| 28 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 29 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 581.` |
| 30 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 218.` |
| 31 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.963. Support: 178.` |
| 32 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = +<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.938. Support: 121.` |
| 33 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, KEY}<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 761.` |
| 34 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, KEY}<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 161.` |
| 35 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 351.` |
| 36 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 131.` |
| 37 | `  •••start_line ≥ 2<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>	∧ ^2.internal_type not in {BinaryExpression}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 11229.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.81081081081081, "max_conf": 0.999475359916687, "max_support": 15163, "min_conf": 0.936302125453949, "min_support": 90, "num_rules": 37}}
```
</details>
