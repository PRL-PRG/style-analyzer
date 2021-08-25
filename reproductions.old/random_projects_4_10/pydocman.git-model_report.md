# Model report for file:///tmp/top-repos-quality-repos-p4l_i18z/pydocman.git HEAD f899b83cb8d10d86fdd1f661e95af293a3d3573b

### Dump

```json
{'created_at': '2021-08-21 22:06:25',
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
 'size': '35.9 kB',
 'tags': [],
 'uuid': '3c22138a-c9eb-4af7-85d4-3d5dffcde042',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-p4l_i18z/pydocman.git f899b83cb8d10d86fdd1f661e95af293a3d3573b

# javascript
76 rules, avg.len. 9.6
## train
PPCR: 0.918827
### report
macro
{'f1-score': 0.29179056283865656,
 'precision': 0.3048484948570173,
 'recall': 0.2835776824969644,
 'support': 188978}
micro
{'f1-score': 0.9670702409804316,
 'precision': 0.9670702409804316,
 'recall': 0.9670702409804316,
 'support': 188978}
weighted
{'f1-score': 0.963612939167279,
 'precision': 0.9613400516369037,
 'recall': 0.9670702409804316,
 'support': 188978}
### report_full
macro
{'f1-score': 0.26756518186973,
 'precision': 0.3048484948570173,
 'recall': 0.25019549159959603,
 'support': 205673}
micro
{'f1-score': 0.9261600756111095,
 'precision': 0.9670702409804316,
 'recall': 0.8885706923125544,
 'support': 205673}
weighted
{'f1-score': 0.9120806891535251,
 'precision': 0.9431975990708887,
 'recall': 0.8885706923125544,
 'support': 205673}
## test
PPCR: 0.922518
### report
macro
{'f1-score': 0.2924884506709336,
 'precision': 0.3053559566838551,
 'recall': 0.2845932651218702,
 'support': 43327}
micro
{'f1-score': 0.9675491033304867,
 'precision': 0.9675491033304867,
 'recall': 0.9675491033304867,
 'support': 43327}
weighted
{'f1-score': 0.9640884917147851,
 'precision': 0.9618116881813973,
 'recall': 0.9675491033304867,
 'support': 43327}
### report_full
macro
{'f1-score': 0.2687616881489939,
 'precision': 0.3053559566838551,
 'recall': 0.25204825104784817,
 'support': 46966}
micro
{'f1-score': 0.9285548159879503,
 'precision': 0.9675491033304867,
 'recall': 0.8925818677341055,
 'support': 46966}
weighted
{'f1-score': 0.9149773190660883,
 'precision': 0.9446850501937178,
 'recall': 0.8925818677341055,
 'support': 46966}
```

## javascript
### Summary
60 rules, avg.len. 9.3

| | |
|-|-|
|Min support|90|
|Max support|31462|
|Min confidence|0.9218146800994873|
|Max confidence|0.9983966946601868|

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
| 1 | `  -1.reserved = )<br>	∧ -2.reserved = )<br>	∧ +4.length ≥ 3<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 214.` |
| 2 | `  -1.reserved = )<br>	∧ +4.length ≤ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.922. Support: 518.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.996. Support: 120.` |
| 4 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.reserved = ;<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 5 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.reserved = ;<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1330.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 11<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 164.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -1.length ≤ 6<br>	∧ -2.diff_offset ≥ 20<br>	∧ -3.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1149.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -2.diff_offset ≤ 19<br>	∧ -3.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 31462.` |
| 9 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 2183.` |
| 10 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = module<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {[}<br>	∧ +3.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 214.` |
| 11 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {module}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {[}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 7561.` |
| 12 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.965. Support: 5690.` |
| 13 | `  -1.diff_offset ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 99.` |
| 14 | `  •••start_line ≥ 89<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {(}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.964. Support: 6049.` |
| 15 | `  •••start_col ≥ 12<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.997. Support: 189.` |
| 16 | `  •••start_col ≤ 11<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +4.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.927. Support: 337.` |
| 17 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.968. Support: 1951.` |
| 18 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 10<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 861.` |
| 19 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 370.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.935. Support: 559.` |
| 21 | `  •••start_col ≥ 6<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = '<br>Confidence: 0.989. Support: 131.` |
| 22 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -5.diff_col ≤ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LIST} and not in {FILE, STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 111.` |
| 23 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 21<br>	∧ -3.roles in {ARGUMENT, STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 855.` |
| 24 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 21<br>	∧ -3.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -4.label not in {<space>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 15861.` |
| 25 | `  •••start_col ≤ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, STATEMENT}<br>⇒ y = '<br>Confidence: 0.988. Support: 121.` |
| 26 | `  -1.roles in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.993. Support: 207.` |
| 27 | `  -1.roles not in {STRING}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.977. Support: 5846.` |
| 28 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.984. Support: 4785.` |
| 29 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 94.` |
| 30 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ArrayExpression<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 90.` |
| 31 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 5863.` |
| 32 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.990. Support: 353.` |
| 33 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 475.` |
| 34 | `  -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 260.` |
| 35 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 255.` |
| 36 | `  -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {NOT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |
| 37 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {(, ), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 375.` |
| 38 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.980. Support: 128.` |
| 39 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 93.` |
| 40 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = .<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 127.` |
| 41 | `  -1.roles not in {STRING}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 125.` |
| 42 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ?<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 416.` |
| 43 | `  -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 243.` |
| 44 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {ELSE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 177.` |
| 45 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.970. Support: 215.` |
| 46 | `  •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 2664.` |
| 47 | `  •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≤ 13<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 21<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 672.` |
| 48 | `  •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = (<br>	∧ -2.length ≤ 13<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 20<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 699.` |
| 49 | `  •••start_col ≥ 48<br>	∧ •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≤ 13<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 20<br>	∧ -4.reserved = ,<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 463.` |
| 50 | `  •••start_col ≤ 62<br>	∧ •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≤ 13<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 20<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 532.` |
| 51 | `  •••start_col ≤ 47<br>	∧ •••start_line ≥ 21<br>	∧ -1.reserved not in {return}<br>	∧ -1.roles in {RIGHT} and not in {STRING}<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≤ 13<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 20<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 253.` |
| 52 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 128.` |
| 53 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1188.` |
| 54 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 114.` |
| 55 | `  -1.reserved not in {), :, return}<br>	∧ -1.roles not in {STRING}<br>	∧ -5.length ≥ 5<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1745.` |
| 56 | `  -1.reserved not in {), :, return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<+space>}<br>	∧ -5.length ≤ 4<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = function<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 375.` |
| 57 | `  -1.reserved not in {), :, return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<+space>}<br>	∧ -5.length ≤ 4<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.938. Support: 105.` |
| 58 | `  •••start_line ≥ 13<br>	∧ -1.reserved not in {), :}<br>	∧ -1.roles in {BINARY} and not in {STRING}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {;, }}<br>	∧ -5.diff_col ≥ 16<br>	∧ +1.reserved not in {:, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 846.` |
| 59 | `  •••start_line ≥ 13<br>	∧ -1.reserved not in {), :}<br>	∧ -1.roles in {BINARY} and not in {STRING}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.diff_offset ≤ 12<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {;, }}<br>	∧ -5.diff_col ≤ 15<br>	∧ +1.reserved not in {:, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 439.` |
| 60 | `  •••start_line ≥ 13<br>	∧ -1.reserved not in {), :}<br>	∧ -1.roles not in {BINARY, STRING}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {;, }}<br>	∧ +1.reserved not in {:, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 29116.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.266666666666667, "max_conf": 0.9983966946601868, "max_support": 31462, "min_conf": 0.9218146800994873, "min_support": 90, "num_rules": 60}}
```
</details>
